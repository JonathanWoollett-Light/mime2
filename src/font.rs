use super::*;
#[doc = "\\[RFC8081\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::font::COLLECTION;"]
#[doc = "assert_eq!(media.ttype, \"font\");"]
#[doc = "assert_eq!(media.subtype, \"collection\");"]
#[doc = "assert_eq!(media.to_string(), \"font/collection\");"]
#[doc = r" ```"]
pub const COLLECTION: Mime = Mime {
    ttype: "font",
    subtype: "collection",
};
#[doc = "\\[RFC8081\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::font::OTF;"]
#[doc = "assert_eq!(media.ttype, \"font\");"]
#[doc = "assert_eq!(media.subtype, \"otf\");"]
#[doc = "assert_eq!(media.to_string(), \"font/otf\");"]
#[doc = r" ```"]
pub const OTF: Mime = Mime {
    ttype: "font",
    subtype: "otf",
};
#[doc = "\\[RFC8081\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::font::SFNT;"]
#[doc = "assert_eq!(media.ttype, \"font\");"]
#[doc = "assert_eq!(media.subtype, \"sfnt\");"]
#[doc = "assert_eq!(media.to_string(), \"font/sfnt\");"]
#[doc = r" ```"]
pub const SFNT: Mime = Mime {
    ttype: "font",
    subtype: "sfnt",
};
#[doc = "\\[RFC8081\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::font::TTF;"]
#[doc = "assert_eq!(media.ttype, \"font\");"]
#[doc = "assert_eq!(media.subtype, \"ttf\");"]
#[doc = "assert_eq!(media.to_string(), \"font/ttf\");"]
#[doc = r" ```"]
pub const TTF: Mime = Mime {
    ttype: "font",
    subtype: "ttf",
};
#[doc = "\\[RFC8081\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::font::WOFF;"]
#[doc = "assert_eq!(media.ttype, \"font\");"]
#[doc = "assert_eq!(media.subtype, \"woff\");"]
#[doc = "assert_eq!(media.to_string(), \"font/woff\");"]
#[doc = r" ```"]
pub const WOFF: Mime = Mime {
    ttype: "font",
    subtype: "woff",
};
#[doc = "\\[RFC8081\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::font::WOFF2;"]
#[doc = "assert_eq!(media.ttype, \"font\");"]
#[doc = "assert_eq!(media.subtype, \"woff2\");"]
#[doc = "assert_eq!(media.to_string(), \"font/woff2\");"]
#[doc = r" ```"]
pub const WOFF2: Mime = Mime {
    ttype: "font",
    subtype: "woff2",
};
