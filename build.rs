use quote::format_ident;
use quote::quote;
use std::fs;

fn to_identifiable(s: &str) -> String {
    let mut identifier = s
        .replace("+", "_")
        .replace("/", "_")
        .replace("-", "_")
        .replace(".", "_")
        .to_uppercase();
    if identifier.chars().next().unwrap().is_numeric() {
        identifier.insert(0, '_');
    }
    identifier
}

fn main() {
    #[cfg(not(docsrs))]
    inner();
}

fn inner() {
    let paths = fs::read_dir("assets").unwrap();
    let mut outer = Vec::new();
    for path_result in paths {
        let mut inner = Vec::new();
        let path = path_result.unwrap().path();

        let module = path.file_stem().unwrap().to_str().unwrap();
        let inner_ident = format_ident!("{}", module);
        let body = fs::read_to_string(path).unwrap();
        let mut reader = csv::Reader::from_reader(body.as_bytes());
        for record_result in reader.records() {
            let record = record_result.unwrap();
            let name = record.get(0).unwrap();
            let template = record.get(1).unwrap();
            let mut template_iter = template.split("/");
            let ttype = template_iter.next().unwrap();
            let subtype = template_iter.next().unwrap();
            assert!(template_iter.next().is_none());
            let reference = record
                .get(2)
                .unwrap()
                .replace("[", "\\[")
                .replace("]", "\\]");
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
            })
        }
        outer.push(quote! {
            pub mod #inner_ident {
                use super::*;
                #(#inner)*
            }
        });
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
    std::fs::write("src/lib.rs", out.to_string()).unwrap();
    std::process::Command::new("rustfmt")
        .arg("src/lib.rs")
        .spawn()
        .unwrap()
        .wait()
        .unwrap();
}
