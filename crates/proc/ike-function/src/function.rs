use std::iter::zip;

use proc_macro2::{Span, TokenStream};
use quote::{format_ident, quote};
use syn::{parse2, parse_str, Ident, ItemFn, Type};
use thiserror::Error;

use crate::parse::{parse_signature, Arg, ArgError, NumberType, SignatureError, Strings};
use std::iter::Iterator;

#[derive(Debug, Error)]
pub enum MacroFunctionError {
    #[error("Invalid argument: '{0}'")]
    InvalidArgument(String, #[source] ArgError),
    #[error("Parse error: {0}")]
    ParseError(#[from] syn::Error),
    #[error("Failed to parse signature: {0}")]
    SignatureError(#[from] SignatureError),
    #[error("Failed to map function: {0}")]
    MappingError(#[from] MappingError),
}

#[derive(Debug, Error)]
#[allow(clippy::enum_variant_names)]
pub enum MappingError {
    #[error("Unable to map argument {1:?} to {0}")]
    NoArgMapping(&'static str, Arg),
}

pub fn macro_function(
    _: TokenStream,
    item: TokenStream,
) -> Result<TokenStream, MacroFunctionError> {
    let input = parse2::<ItemFn>(item)?;

    let ItemFn {
        sig,
        vis,
        block,
        attrs,
    } = input;
    let signature = parse_signature(attrs.clone(), sig.clone())?;

    let processed_args = zip(signature.args.iter(), sig.inputs.iter()).collect::<Vec<_>>();

    let function_identifier = sig.ident.clone();

    let generic = signature
        .generic_bounds
        .keys()
        .map(|s| format_ident!("{s}"))
        .collect::<Vec<_>>();
    let bound = signature
        .generic_bounds
        .values()
        .map(|p| parse_str::<Type>(p).expect("Failed to reparse type bounds"))
        .collect::<Vec<_>>();

    let mut names = signature.names.iter();
    let mut call_args: Vec<TokenStream> = vec![];
    let iterator = processed_args.iter().cloned();
    for (index_out, arg) in iterator.enumerate() {
        let name_in = names.next().expect("Failed to get next name");
        let ident = Ident::new(&name_in, Span::call_site());

        let arg = &arg.0.clone();

        let arg_type = match arg {
            Arg::String(Strings::String) => {
                let msg = format!("Missing string argument: {}", name_in);
                quote! {
                    let #ident = match args.get(#index_out) {
                        Some(arg) => arg.to_string(ctx)?.to_std_string_escaped(),
                        None => {
                            return Err(::boa_engine::JsError::from_native(
                                ::boa_engine::JsNativeError::error().with_message(#msg),
                            ));
                        }
                    };
                }
            }
            Arg::OptionString(Strings::String) => {
                quote! {
                    let #ident = match args.get(#index_out) {
                        Some(arg) => Some(arg.to_string(ctx)?.to_std_string_escaped()),
                        None => None,
                    };
                }
            }
            Arg::Function => {
                let msg = format!("Missing function argument: {}", name_in);
                quote! {
                    let func = args.get(#index_out);
                    if func.is_none() {
                        return Err(::boa_engine::JsError::from_native(
                            ::boa_engine::JsNativeError::error()
                            .with_message(#msg)
                            .into()));
                    }

                    let func = func.unwrap();
                    let func = func.as_object().unwrap();
                    let func = JsFunction::from_object(func.clone()).expect("Function not found");

                    let #ident = func;
                }
            }
            Arg::OptionFunction => {
                quote! {
                    let func = args.get(#index_out);
                    let #ident = if let Some(func) = func {
                        let func = func.as_object().unwrap();
                        let func = JsFunction::from_object(func.clone()).expect("Function not found");
                        Some(func)
                    } else {
                        None
                    };
                }
            }
            Arg::Number(typ) => {
                let msg = format!("Missing number argument: {}", name_in);
                let number_type = match typ {
                    NumberType::I32 => quote! { to_i32 },
                    // NumberType::U32 => quote! { to_u32 },
                    // NumberType::F64 => quote! { to_f64 },
                };
                quote! {
                    let #ident = match args.get(#index_out) {
                        Some(arg) => arg.#number_type(ctx).expect("Failed to convert to number"),
                        None => {
                            return Err(::boa_engine::JsError::from_native(
                                ::boa_engine::JsNativeError::error().with_message(#msg),
                            ));
                        }
                    };
                }
            }
            Arg::OptionNumber(typ) => {
                let number_type = match typ {
                    NumberType::I32 => quote! { to_i32 },
                    // NumberType::U32 => quote! { to_u32 },
                    // NumberType::F64 => quote! { to_f64 },
                };
                quote! {
                    let #ident = match args.get(#index_out) {
                        Some(arg) => Some(arg.#number_type(ctx).expect("Failed to convert to number")),
                        None => None,
                    };
                }
            }
            Arg::Bool => {
                let msg = format!("Missing boolean argument: {}", name_in);
                quote! {
                    let #ident = match args.get(#index_out) {
                        Some(arg) => arg.to_boolean(),
                        None => {
                            return Err(::boa_engine::JsError::from_native(
                                ::boa_engine::JsNativeError::error().with_message(#msg),
                            ));
                        }
                    };
                }
            }
            Arg::OptionBool => {
                quote! {
                    let #ident = match args.get(#index_out) {
                        Some(arg) => Some(arg.to_boolean()),
                        None => None,
                    };
                }
            }
            _ => {
                return Err(MacroFunctionError::MappingError(
                    MappingError::NoArgMapping("context", arg.clone()),
                ));
            }
        };

        call_args.push(quote! {
            #arg_type
        });
    }

    Ok(quote!(
        #(#attrs)*
        #vis fn #function_identifier <#(#generic : #bound),*> (this: &::boa_engine::JsValue, args: &[::boa_engine::JsValue], ctx: &mut ::boa_engine::Context) -> ::boa_engine::JsResult<::boa_engine::JsValue> {
            #(#call_args)*
            #block
        }
    ))
}
