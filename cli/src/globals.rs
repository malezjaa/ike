pub const VERSION: &str = env!("CARGO_PKG_VERSION");
pub const ALLOWED_EXTENSIONS: [&str; 8] = ["js", "mjs", "ts", "mts", "cjs", "cts", "jsx", "tsx"];

pub const CODE_TO_INJECT: &str = include_str!("./inject.js");