#![allow(rustdoc::bare_urls)]
#![warn(clippy::pedantic)]
#![warn(missing_docs)]
#![cfg_attr(docsrs, feature(doc_cfg))]
#![doc = r" ```"]
#![doc = r" use std::str::FromStr;"]
#![doc = r" let text = mime2::text::PLAIN;"]
#![doc = r#" assert_eq!(text.ttype, "text");"#]
#![doc = r#" assert_eq!(text.subtype,"plain");"#]
#![doc = r#" assert_eq!(text.to_string(),"text/plain");"#]
#![doc = r#" assert_eq!(mime2::Mime::from_str("text/plain"), Ok(text));"#]
#![doc = r" ```"]
#[doc = r" A media type (also known as a Multipurpose Internet Mail Extensions or MIME type)"]
#[doc = r" indicates the nature and format of a document, file, or assortment of bytes. MIME types"]
#[doc = r" are defined and standardized in IETF's"]
#[doc = r" [RFC 6838](https://datatracker.ietf.org/doc/html/rfc6838)."]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct Mime {
    #[doc = r" Type."]
    pub ttype: &'static str,
    #[doc = r" Subtype."]
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
#[doc = "Media types for the application type."]
pub mod application;
#[doc = "Media types for the font type."]
pub mod font;
#[doc = "Media types for the haptics type."]
pub mod haptics;
#[doc = "Media types for the image type."]
pub mod image;
#[doc = "Media types for the message type."]
pub mod message;
#[doc = "Media types for the model type."]
pub mod model;
#[doc = "Media types for the multipart type."]
pub mod multipart;
#[doc = "Media types for the text type."]
pub mod text;
#[doc = "Media types for the video type."]
pub mod video;
