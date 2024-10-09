#![allow(rustdoc::bare_urls)]
#![warn(clippy::pedantic)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = r" ```"]
#![doc = r" let text = mime2::text::PLAIN;"]
#![doc = r#" assert_eq!(text.ttype, "text");"#]
#![doc = r#" assert_eq!(text.subtype,"plain");"#]
#![doc = r#" assert_eq!(text.to_string(),"text/plain");"#]
#![doc = r" ```"]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mime {
    pub ttype: &'static str,
    pub subtype: &'static str,
}
impl std::fmt::Display for Mime {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}/{}", self.ttype, self.subtype)
    }
}
mod from_str;
pub use from_str::*;
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
impl TryFrom<Mime> for http::header::HeaderValue {
    type Error = http::header::InvalidHeaderValue;
    fn try_from(value: Mime) -> Result<Self, Self::Error> {
        http::header::HeaderValue::from_str(&value.to_string())
    }
}
pub mod application;
pub mod font;
pub mod haptics;
pub mod image;
pub mod message;
pub mod model;
pub mod multipart;
pub mod text;
pub mod video;
