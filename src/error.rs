use error_chain::error_chain;
pub use error_chain::{bail, ensure};

error_chain! {
    foreign_links {
        Fmt(::std::fmt::Error);
        ParseIntError(::std::num::ParseIntError);
        ParseFloatError(::std::num::ParseFloatError);
        SerdeJson(serde_json::Error);
        Base64(base64::DecodeError);
        Png(png::DecodingError);
    }
}
