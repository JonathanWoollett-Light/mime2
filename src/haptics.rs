use super::*;
#[doc = "\\[RFC-ietf-mediaman-haptics-05\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::haptics::IVS;"]
#[doc = "assert_eq!(media.ttype, \"haptics\");"]
#[doc = "assert_eq!(media.subtype, \"ivs\");"]
#[doc = "assert_eq!(media.to_string(), \"haptics/ivs\");"]
#[doc = r" ```"]
pub const IVS: Mime = Mime {
    ttype: "haptics",
    subtype: "ivs",
};
#[doc = "\\[RFC-ietf-mediaman-haptics-05\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::haptics::HJIF;"]
#[doc = "assert_eq!(media.ttype, \"haptics\");"]
#[doc = "assert_eq!(media.subtype, \"hjif\");"]
#[doc = "assert_eq!(media.to_string(), \"haptics/hjif\");"]
#[doc = r" ```"]
pub const HJIF: Mime = Mime {
    ttype: "haptics",
    subtype: "hjif",
};
#[doc = "\\[RFC-ietf-mediaman-haptics-05\\]"]
#[doc = r" ```no_run"]
#[doc = "let media = mime2::haptics::HMPG;"]
#[doc = "assert_eq!(media.ttype, \"haptics\");"]
#[doc = "assert_eq!(media.subtype, \"hmpg\");"]
#[doc = "assert_eq!(media.to_string(), \"haptics/hmpg\");"]
#[doc = r" ```"]
pub const HMPG: Mime = Mime {
    ttype: "haptics",
    subtype: "hmpg",
};
