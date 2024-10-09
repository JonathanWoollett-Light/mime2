use super::Mime;
#[doc = "\\[RFC2046\\]\\[RFC2045\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::ALTERNATIVE;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"alternative\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/alternative\");"]
#[doc = r" ```"]
pub const ALTERNATIVE: Mime = Mime {
    ttype: "multipart",
    subtype: "alternative",
};
#[doc = "\\[Patrik_Faltstrom\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::APPLEDOUBLE;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"appledouble\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/appledouble\");"]
#[doc = r" ```"]
pub const APPLEDOUBLE: Mime = Mime {
    ttype: "multipart",
    subtype: "appledouble",
};
#[doc = "\\[RFC9110\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::BYTERANGES;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"byteranges\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/byteranges\");"]
#[doc = r" ```"]
pub const BYTERANGES: Mime = Mime {
    ttype: "multipart",
    subtype: "byteranges",
};
#[doc = "\\[RFC2046\\]\\[RFC2045\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::DIGEST;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"digest\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/digest\");"]
#[doc = r" ```"]
pub const DIGEST: Mime = Mime {
    ttype: "multipart",
    subtype: "digest",
};
#[doc = "\\[RFC1847\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::ENCRYPTED;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"encrypted\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/encrypted\");"]
#[doc = r" ```"]
pub const ENCRYPTED: Mime = Mime {
    ttype: "multipart",
    subtype: "encrypted",
};
#[doc = "\\[RFC4735\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::EXAMPLE;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"example\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/example\");"]
#[doc = r" ```"]
pub const EXAMPLE: Mime = Mime {
    ttype: "multipart",
    subtype: "example",
};
#[doc = "\\[RFC7578\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::FORM_DATA;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"form-data\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/form-data\");"]
#[doc = r" ```"]
pub const FORM_DATA: Mime = Mime {
    ttype: "multipart",
    subtype: "form-data",
};
#[doc = "\\[Dave_Crocker\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::HEADER_SET;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"header-set\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/header-set\");"]
#[doc = r" ```"]
pub const HEADER_SET: Mime = Mime {
    ttype: "multipart",
    subtype: "header-set",
};
#[doc = "\\[RFC2046\\]\\[RFC2045\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::MIXED;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"mixed\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/mixed\");"]
#[doc = r" ```"]
pub const MIXED: Mime = Mime {
    ttype: "multipart",
    subtype: "mixed",
};
#[doc = "\\[RFC8255\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::MULTILINGUAL;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"multilingual\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/multilingual\");"]
#[doc = r" ```"]
pub const MULTILINGUAL: Mime = Mime {
    ttype: "multipart",
    subtype: "multilingual",
};
#[doc = "\\[RFC2046\\]\\[RFC2045\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::PARALLEL;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"parallel\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/parallel\");"]
#[doc = r" ```"]
pub const PARALLEL: Mime = Mime {
    ttype: "multipart",
    subtype: "parallel",
};
#[doc = "\\[RFC2387\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::RELATED;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"related\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/related\");"]
#[doc = r" ```"]
pub const RELATED: Mime = Mime {
    ttype: "multipart",
    subtype: "related",
};
#[doc = "\\[RFC6522\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::REPORT;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"report\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/report\");"]
#[doc = r" ```"]
pub const REPORT: Mime = Mime {
    ttype: "multipart",
    subtype: "report",
};
#[doc = "\\[RFC1847\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::SIGNED;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"signed\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/signed\");"]
#[doc = r" ```"]
pub const SIGNED: Mime = Mime {
    ttype: "multipart",
    subtype: "signed",
};
#[doc = "\\[Heinz-Peter_Schütz\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::VND_BINT_MED_PLUS;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"vnd.bint.med-plus\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/vnd.bint.med-plus\");"]
#[doc = r" ```"]
pub const VND_BINT_MED_PLUS: Mime = Mime {
    ttype: "multipart",
    subtype: "vnd.bint.med-plus",
};
#[doc = "\\[RFC3801\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::VOICE_MESSAGE;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"voice-message\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/voice-message\");"]
#[doc = r" ```"]
pub const VOICE_MESSAGE: Mime = Mime {
    ttype: "multipart",
    subtype: "voice-message",
};
#[doc = "\\[W3C\\]\\[Robin_Berjon\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::multipart::X_MIXED_REPLACE;"]
#[doc = "assert_eq!(media.ttype, \"multipart\");"]
#[doc = "assert_eq!(media.subtype, \"x-mixed-replace\");"]
#[doc = "assert_eq!(media.to_string(), \"multipart/x-mixed-replace\");"]
#[doc = r" ```"]
pub const X_MIXED_REPLACE: Mime = Mime {
    ttype: "multipart",
    subtype: "x-mixed-replace",
};
