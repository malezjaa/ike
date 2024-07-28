#[macro_export]
macro_rules! create_method_with_state {
    ($func:expr, immutable $state:expr) => {
        unsafe {
            NativeFunction::from_closure(move |this, args, context| {
                $func(this, args, &$state.borrow(), context)
            })
        }
    };
    ($func:expr, mutable $state:expr) => {
        unsafe {
            let cloned_state = $state.clone();
            NativeFunction::from_closure(move |this, args, context| {
                $func(this, args, &mut cloned_state.borrow_mut(), context)
            })
        }
    };
}

#[macro_export]
macro_rules! create_method {
    ($func:expr) => {
        unsafe { NativeFunction::from_closure($func) }
    };
}

// This converts a JsValue to a Rust string
#[macro_export]
macro_rules! str_from_jsvalue {
    ($value:expr, $context:expr) => {
        $value.to_string($context)?.to_std_string_escaped()
    };
}

#[macro_export]
macro_rules! js_str_to_string {
    ($value:expr) => {
        $value.to_std_string_escaped()
    };
}

#[macro_export]
macro_rules! throw {
    (
        typ,
        $message:expr
    ) => {
        return Err(JsNativeError::typ().with_message($message).into())
    };
    (
        ref,
        $message:expr
    ) => {
        return Err(JsNativeError::reference().with_message($message).into())
    };
    (
        err,
        $message:expr
    ) => {
        return Err(JsNativeError::error().with_message($message).into())
    };
}

#[macro_export]
macro_rules! assert_arg_type {
    (string, $arg:expr) => {
        if !$arg.is_string() {
            throw!(typ, format!("Expected a string, got {:?}", $arg.get_type()));
        }
    };
    (function, $arg:expr) => {
        if !$arg.is_callable() {
            throw!(
                typ,
                format!("Expected a function, got {:?}", $arg.get_type())
            );
        }
    };
}

#[macro_export]
macro_rules! get_prototype_name {
    ($proto:expr, $ctx:expr) => {{
        let proto_name = $proto
            .get(js_string!("constructor"), $ctx)
            .unwrap()
            .to_object($ctx)
            .unwrap()
            .get(js_string!("name"), $ctx)
            .unwrap();
        let str_name = js_str_to_string!(proto_name.to_string($ctx).unwrap());
        str_name
    }};
}
