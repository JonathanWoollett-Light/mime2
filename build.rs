#![warn(clippy::pedantic)]

use proc_macro2::TokenStream;
use quote::format_ident;
use quote::quote;
use std::fs;
use std::fs::DirEntry;
use std::process::Command;

fn to_identifiable(s: &str) -> String {
    let mut identifier = s.replace(['+', '/', '-', '.'], "_").to_uppercase();
    if identifier.chars().next().unwrap().is_numeric() {
        identifier.insert(0, '_');
    }
    identifier
}

fn rustfmt(s: &[&str]) {
    Command::new("rustfmt")
        .args(s)
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}

fn module(entry: &DirEntry, outer: &mut Vec<TokenStream>, outer_from_str: &mut Vec<TokenStream>) {
    let path = entry.path();
    let mut inner = Vec::new();
    let mut inner_from_str = Vec::new();
    let module = path.file_stem().unwrap().to_str().unwrap();
    let inner_ident = format_ident!("{}", module);
    let body = fs::read_to_string(&path).unwrap();
    let mut reader = csv::Reader::from_reader(body.as_bytes());
    for record_result in reader.records() {
        let record = record_result.unwrap();
        let name = record.get(0).unwrap();
        let template = record.get(1).unwrap();
        let mut template_iter = template.split('/');
        let ttype = template_iter.next().unwrap();
        let subtype = template_iter.next().unwrap();
        assert!(template_iter.next().is_none());
        let reference = record
            .get(2)
            .unwrap()
            .replace('[', "\\[")
            .replace(']', "\\]");
        let identifier = to_identifiable(name)
            .chars()
            .take_while(|c| *c != ' ')
            .collect::<String>();

        let media_type = format!("let media = mime2::{ttype}::{identifier};");
        let type_test = format!("assert_eq!(media.ttype, {ttype:?});");
        let subtype_test = format!("assert_eq!(media.subtype, {subtype:?});");
        let template_test = format!("assert_eq!(media.to_string(), {template:?});");

        let identifier = format_ident!("{identifier}");
        inner.push(quote! {
            #[doc = #reference]
            /// ```no_run
            #[doc = #media_type]
            #[doc = #type_test]
            #[doc = #subtype_test]
            #[doc = #template_test]
            /// ```
            pub const #identifier: Mime = Mime { ttype: #ttype, subtype: #subtype };
        });
        inner_from_str.push(quote! {
            #subtype => Ok(crate::#inner_ident::#identifier),
        });
    }
    outer.push(quote! {
        pub mod #inner_ident;
    });
    let subtype_module = quote! {
        use super::*;
        #(#inner)*
    };
    let module_file = format!("src/{module}.rs");
    fs::write(&module_file, subtype_module.to_string()).unwrap();
    rustfmt(&[&module_file]);
    outer_from_str.push(quote! {
        #module => match subtype {
            #(#inner_from_str)*
            _ => Err(ParseMimeError),
        }
    });
}

fn main() {
    if std::env::var("DOCS_RS").is_ok() {
        return;
    }
    let paths = fs::read_dir("assets").unwrap();
    let mut outer = Vec::new();
    let mut outer_from_str = Vec::new();
    for entry in paths.map(Result::unwrap) {
        module(&entry, &mut outer, &mut outer_from_str);
    }
    let out = quote! {
        #![allow(warnings)]
        #![cfg_attr(docsrs, feature(doc_cfg))]
        //! ```
        //! let text = mime2::text::PLAIN;
        //! assert_eq!(text.ttype, "text");
        //! assert_eq!(text.subtype,"plain");
        //! assert_eq!(text.to_string(),"text/plain");
        //! ```
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
        #[cfg(feature="http")]
        #[cfg_attr(docsrs, doc(cfg(feature = "http")))]
        impl TryFrom<Mime> for http::header::HeaderValue {
            type Error = http::header::InvalidHeaderValue;
            fn try_from(value: Mime) -> Result<Self, Self::Error> {
                http::header::HeaderValue::from_str(&value.to_string())
            }
        }
        #(#outer)*
    };
    let from_str = quote! {
        use super::*;
        #[derive(Debug, PartialEq, Eq)]
        pub struct ParseMimeError;
        impl std::fmt::Display for ParseMimeError {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
                write!(f, "Failed to find matching media type.")
            }
        }
        impl std::error::Error for ParseMimeError { }
        impl std::str::FromStr for Mime {
            type Err = ParseMimeError;
            fn from_str(s: &str) -> Result<Self, Self::Err> {
                let mut iter = s.split('/');
                let ttype = iter.next().ok_or(ParseMimeError)?;
                let subtype = iter.next().ok_or(ParseMimeError)?;
                if iter.next().is_some() {
                    return Err(ParseMimeError);
                }
                match ttype {
                    #(#outer_from_str)*
                    _ => Err(ParseMimeError),
                }
            }
        }
    };
    fs::write("src/from_str.rs", from_str.to_string()).unwrap();
    fs::write("src/lib.rs", out.to_string()).unwrap();
    rustfmt(&["src/lib.rs", "src/from_str.rs"]);
}
