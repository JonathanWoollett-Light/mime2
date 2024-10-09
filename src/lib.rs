#![allow(warnings)]
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
#[cfg(feature = "http")]
#[cfg_attr(docsrs, doc(cfg(feature = "http")))]
impl TryFrom<Mime> for http::header::HeaderValue {
    type Error = http::header::InvalidHeaderValue;
    fn try_from(value: Mime) -> Result<Self, Self::Error> {
        http::header::HeaderValue::from_str(&value.to_string())
    }
}
pub mod font {
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
}
pub mod haptics {
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
}
pub mod image {
    use super::*;
    #[doc = "\\[SMPTE\\]\\[Howard_Lukk\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::ACES;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"aces\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/aces\");"]
    #[doc = r" ```"]
    pub const ACES: Mime = Mime {
        ttype: "image",
        subtype: "aces",
    };
    #[doc = "\\[W3C\\]\\[W3C_PNG_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::APNG;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"apng\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/apng\");"]
    #[doc = r" ```"]
    pub const APNG: Mime = Mime {
        ttype: "image",
        subtype: "apng",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[David_Singer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::AVCI;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"avci\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/avci\");"]
    #[doc = r" ```"]
    pub const AVCI: Mime = Mime {
        ttype: "image",
        subtype: "avci",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[David_Singer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::AVCS;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"avcs\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/avcs\");"]
    #[doc = r" ```"]
    pub const AVCS: Mime = Mime {
        ttype: "image",
        subtype: "avcs",
    };
    #[doc = "\\[Alliance_for_Open_Media\\]\\[Cyril_Concolato\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::AVIF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"avif\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/avif\");"]
    #[doc = r" ```"]
    pub const AVIF: Mime = Mime {
        ttype: "image",
        subtype: "avif",
    };
    #[doc = "\\[RFC7903\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::BMP;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"bmp\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/bmp\");"]
    #[doc = r" ```"]
    pub const BMP: Mime = Mime {
        ttype: "image",
        subtype: "bmp",
    };
    #[doc = "\\[Alan_Francis\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::CGM;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"cgm\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/cgm\");"]
    #[doc = r" ```"]
    pub const CGM: Mime = Mime {
        ttype: "image",
        subtype: "cgm",
    };
    #[doc = "\\[DICOM_Standard_Committee\\]\\[David_Clunie\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::DICOM_RLE;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"dicom-rle\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/dicom-rle\");"]
    #[doc = r" ```"]
    pub const DICOM_RLE: Mime = Mime {
        ttype: "image",
        subtype: "dicom-rle",
    };
    #[doc = "\\[SMPTE\\]\\[SMPTE_Director_of_Standards_Development\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::DPX;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"dpx\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/dpx\");"]
    #[doc = r" ```"]
    pub const DPX: Mime = Mime {
        ttype: "image",
        subtype: "dpx",
    };
    #[doc = "\\[RFC7903\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::EMF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"emf\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/emf\");"]
    #[doc = r" ```"]
    pub const EMF: Mime = Mime {
        ttype: "image",
        subtype: "emf",
    };
    #[doc = "\\[RFC4735\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::EXAMPLE;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"example\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/example\");"]
    #[doc = r" ```"]
    pub const EXAMPLE: Mime = Mime {
        ttype: "image",
        subtype: "example",
    };
    #[doc = "\\[RFC4047\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::FITS;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"fits\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/fits\");"]
    #[doc = r" ```"]
    pub const FITS: Mime = Mime {
        ttype: "image",
        subtype: "fits",
    };
    #[doc = "\\[RFC1494\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::G3FAX;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"g3fax\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/g3fax\");"]
    #[doc = r" ```"]
    pub const G3FAX: Mime = Mime {
        ttype: "image",
        subtype: "g3fax",
    };
    #[doc = "\\[RFC2045\\]\\[RFC2046\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::GIF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"gif\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/gif\");"]
    #[doc = r" ```"]
    pub const GIF: Mime = Mime {
        ttype: "image",
        subtype: "gif",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[David_Singer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::HEIC;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"heic\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/heic\");"]
    #[doc = r" ```"]
    pub const HEIC: Mime = Mime {
        ttype: "image",
        subtype: "heic",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[David_Singer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::HEIC_SEQUENCE;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"heic-sequence\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/heic-sequence\");"]
    #[doc = r" ```"]
    pub const HEIC_SEQUENCE: Mime = Mime {
        ttype: "image",
        subtype: "heic-sequence",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[David_Singer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::HEIF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"heif\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/heif\");"]
    #[doc = r" ```"]
    pub const HEIF: Mime = Mime {
        ttype: "image",
        subtype: "heif",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[David_Singer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::HEIF_SEQUENCE;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"heif-sequence\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/heif-sequence\");"]
    #[doc = r" ```"]
    pub const HEIF_SEQUENCE: Mime = Mime {
        ttype: "image",
        subtype: "heif-sequence",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[ITU-T\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::HEJ2K;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"hej2k\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/hej2k\");"]
    #[doc = r" ```"]
    pub const HEJ2K: Mime = Mime {
        ttype: "image",
        subtype: "hej2k",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[ITU-T\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::HSJ2;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"hsj2\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/hsj2\");"]
    #[doc = r" ```"]
    pub const HSJ2: Mime = Mime {
        ttype: "image",
        subtype: "hsj2",
    };
    #[doc = "\\[RFC1314\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::IEF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"ief\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/ief\");"]
    #[doc = r" ```"]
    pub const IEF: Mime = Mime {
        ttype: "image",
        subtype: "ief",
    };
    #[doc = "\\[ISO-IEC_JTC_1_SC_29_WG_1\\]\\[ISO-IEC_JTC_1\\]\\[ITU-T\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::J2C;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"j2c\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/j2c\");"]
    #[doc = r" ```"]
    pub const J2C: Mime = Mime {
        ttype: "image",
        subtype: "j2c",
    };
    #[doc = "\\[DICOM_Standard_Committee\\]\\[David_Clunie\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JLS;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jls\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jls\");"]
    #[doc = r" ```"]
    pub const JLS: Mime = Mime {
        ttype: "image",
        subtype: "jls",
    };
    #[doc = "\\[RFC3745\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JP2;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jp2\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jp2\");"]
    #[doc = r" ```"]
    pub const JP2: Mime = Mime {
        ttype: "image",
        subtype: "jp2",
    };
    #[doc = "\\[RFC2045\\]\\[RFC2046\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JPEG;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jpeg\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jpeg\");"]
    #[doc = r" ```"]
    pub const JPEG: Mime = Mime {
        ttype: "image",
        subtype: "jpeg",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[ITU-T\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JPH;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jph\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jph\");"]
    #[doc = r" ```"]
    pub const JPH: Mime = Mime {
        ttype: "image",
        subtype: "jph",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[ITU-T\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JPHC;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jphc\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jphc\");"]
    #[doc = r" ```"]
    pub const JPHC: Mime = Mime {
        ttype: "image",
        subtype: "jphc",
    };
    #[doc = "\\[RFC3745\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JPM;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jpm\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jpm\");"]
    #[doc = r" ```"]
    pub const JPM: Mime = Mime {
        ttype: "image",
        subtype: "jpm",
    };
    #[doc = "\\[RFC3745\\]\\[ISO-IEC_JTC_1_SC_29_WG_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JPX;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jpx\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jpx\");"]
    #[doc = r" ```"]
    pub const JPX: Mime = Mime {
        ttype: "image",
        subtype: "jpx",
    };
    #[doc = "\\[ISO-IEC_JTC_1_SC_29_WG_1\\]\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JXL;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jxl\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jxl\");"]
    #[doc = r" ```"]
    pub const JXL: Mime = Mime {
        ttype: "image",
        subtype: "jxl",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[ITU-T\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JXR;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jxr\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jxr\");"]
    #[doc = r" ```"]
    pub const JXR: Mime = Mime {
        ttype: "image",
        subtype: "jxr",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[ITU-T\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JXRA;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jxrA\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jxrA\");"]
    #[doc = r" ```"]
    pub const JXRA: Mime = Mime {
        ttype: "image",
        subtype: "jxrA",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[ITU-T\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JXRS;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jxrS\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jxrS\");"]
    #[doc = r" ```"]
    pub const JXRS: Mime = Mime {
        ttype: "image",
        subtype: "jxrS",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JXS;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jxs\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jxs\");"]
    #[doc = r" ```"]
    pub const JXS: Mime = Mime {
        ttype: "image",
        subtype: "jxs",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JXSC;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jxsc\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jxsc\");"]
    #[doc = r" ```"]
    pub const JXSC: Mime = Mime {
        ttype: "image",
        subtype: "jxsc",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JXSI;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jxsi\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jxsi\");"]
    #[doc = r" ```"]
    pub const JXSI: Mime = Mime {
        ttype: "image",
        subtype: "jxsi",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::JXSS;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"jxss\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/jxss\");"]
    #[doc = r" ```"]
    pub const JXSS: Mime = Mime {
        ttype: "image",
        subtype: "jxss",
    };
    #[doc = "\\[Khronos\\]\\[Mark_Callow\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::KTX;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"ktx\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/ktx\");"]
    #[doc = r" ```"]
    pub const KTX: Mime = Mime {
        ttype: "image",
        subtype: "ktx",
    };
    #[doc = "\\[Khronos\\]\\[Mark_Callow\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::KTX2;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"ktx2\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/ktx2\");"]
    #[doc = r" ```"]
    pub const KTX2: Mime = Mime {
        ttype: "image",
        subtype: "ktx2",
    };
    #[doc = "\\[Ilya_Ferber\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::NAPLPS;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"naplps\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/naplps\");"]
    #[doc = r" ```"]
    pub const NAPLPS: Mime = Mime {
        ttype: "image",
        subtype: "naplps",
    };
    #[doc = "\\[W3C\\]\\[PNG_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::PNG;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"png\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/png\");"]
    #[doc = r" ```"]
    pub const PNG: Mime = Mime {
        ttype: "image",
        subtype: "png",
    };
    #[doc = "\\[Ben_Simon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::PRS_BTIF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.btif\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/prs.btif\");"]
    #[doc = r" ```"]
    pub const PRS_BTIF: Mime = Mime {
        ttype: "image",
        subtype: "prs.btif",
    };
    #[doc = "\\[Juern_Laun\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::PRS_PTI;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.pti\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/prs.pti\");"]
    #[doc = r" ```"]
    pub const PRS_PTI: Mime = Mime {
        ttype: "image",
        subtype: "prs.pti",
    };
    #[doc = "\\[Michael_Sweet\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::PWG_RASTER;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"pwg-raster\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/pwg-raster\");"]
    #[doc = r" ```"]
    pub const PWG_RASTER: Mime = Mime {
        ttype: "image",
        subtype: "pwg-raster",
    };
    #[doc = "\\[W3C\\]\\[http://www.w3.org/TR/SVG/mimereg.html\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::SVG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"svg+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/svg+xml\");"]
    #[doc = r" ```"]
    pub const SVG_XML: Mime = Mime {
        ttype: "image",
        subtype: "svg+xml",
    };
    #[doc = "\\[RFC3362\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::T38;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"t38\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/t38\");"]
    #[doc = r" ```"]
    pub const T38: Mime = Mime {
        ttype: "image",
        subtype: "t38",
    };
    #[doc = "\\[RFC3302\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::TIFF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"tiff\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/tiff\");"]
    #[doc = r" ```"]
    pub const TIFF: Mime = Mime {
        ttype: "image",
        subtype: "tiff",
    };
    #[doc = "\\[RFC3950\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::TIFF_FX;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"tiff-fx\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/tiff-fx\");"]
    #[doc = r" ```"]
    pub const TIFF_FX: Mime = Mime {
        ttype: "image",
        subtype: "tiff-fx",
    };
    #[doc = "\\[Kim_Scarborough\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_ADOBE_PHOTOSHOP;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.adobe.photoshop\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.adobe.photoshop\");"]
    #[doc = r" ```"]
    pub const VND_ADOBE_PHOTOSHOP: Mime = Mime {
        ttype: "image",
        subtype: "vnd.adobe.photoshop",
    };
    #[doc = "\\[Gary_Clueit\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_AIRZIP_ACCELERATOR_AZV;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.airzip.accelerator.azv\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.airzip.accelerator.azv\");"]
    #[doc = r" ```"]
    pub const VND_AIRZIP_ACCELERATOR_AZV: Mime = Mime {
        ttype: "image",
        subtype: "vnd.airzip.accelerator.azv",
    };
    #[doc = "\\[Ann_McLaughlin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_CNS_INF2;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cns.inf2\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.cns.inf2\");"]
    #[doc = r" ```"]
    pub const VND_CNS_INF2: Mime = Mime {
        ttype: "image",
        subtype: "vnd.cns.inf2",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_DECE_GRAPHIC;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.graphic\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.dece.graphic\");"]
    #[doc = r" ```"]
    pub const VND_DECE_GRAPHIC: Mime = Mime {
        ttype: "image",
        subtype: "vnd.dece.graphic",
    };
    #[doc = "\\[Leon_Bottou\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_DJVU;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.djvu\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.djvu\");"]
    #[doc = r" ```"]
    pub const VND_DJVU: Mime = Mime {
        ttype: "image",
        subtype: "vnd.djvu",
    };
    #[doc = "\\[Jodi_Moline\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_DWG;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dwg\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.dwg\");"]
    #[doc = r" ```"]
    pub const VND_DWG: Mime = Mime {
        ttype: "image",
        subtype: "vnd.dwg",
    };
    #[doc = "\\[Jodi_Moline\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_DXF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dxf\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.dxf\");"]
    #[doc = r" ```"]
    pub const VND_DXF: Mime = Mime {
        ttype: "image",
        subtype: "vnd.dxf",
    };
    #[doc = "\\[Peter_Siebert\\]\\[Michael_Lagally\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_DVB_SUBTITLE;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.subtitle\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.dvb.subtitle\");"]
    #[doc = r" ```"]
    pub const VND_DVB_SUBTITLE: Mime = Mime {
        ttype: "image",
        subtype: "vnd.dvb.subtitle",
    };
    #[doc = "\\[Scott_Becker\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_FASTBIDSHEET;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fastbidsheet\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.fastbidsheet\");"]
    #[doc = r" ```"]
    pub const VND_FASTBIDSHEET: Mime = Mime {
        ttype: "image",
        subtype: "vnd.fastbidsheet",
    };
    #[doc = "\\[Marc_Douglas_Spencer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_FPX;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fpx\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.fpx\");"]
    #[doc = r" ```"]
    pub const VND_FPX: Mime = Mime {
        ttype: "image",
        subtype: "vnd.fpx",
    };
    #[doc = "\\[Arild_Fuldseth\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_FST;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fst\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.fst\");"]
    #[doc = r" ```"]
    pub const VND_FST: Mime = Mime {
        ttype: "image",
        subtype: "vnd.fst",
    };
    #[doc = "\\[Masanori_Onda\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_FUJIXEROX_EDMICS_MMR;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujixerox.edmics-mmr\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.fujixerox.edmics-mmr\");"]
    #[doc = r" ```"]
    pub const VND_FUJIXEROX_EDMICS_MMR: Mime = Mime {
        ttype: "image",
        subtype: "vnd.fujixerox.edmics-mmr",
    };
    #[doc = "\\[Masanori_Onda\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_FUJIXEROX_EDMICS_RLC;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujixerox.edmics-rlc\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.fujixerox.edmics-rlc\");"]
    #[doc = r" ```"]
    pub const VND_FUJIXEROX_EDMICS_RLC: Mime = Mime {
        ttype: "image",
        subtype: "vnd.fujixerox.edmics-rlc",
    };
    #[doc = "\\[Martin_Bailey\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_GLOBALGRAPHICS_PGB;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.globalgraphics.pgb\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.globalgraphics.pgb\");"]
    #[doc = r" ```"]
    pub const VND_GLOBALGRAPHICS_PGB: Mime = Mime {
        ttype: "image",
        subtype: "vnd.globalgraphics.pgb",
    };
    #[doc = "\\[Simon_Butcher\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_MICROSOFT_ICON;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.microsoft.icon\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.microsoft.icon\");"]
    #[doc = r" ```"]
    pub const VND_MICROSOFT_ICON: Mime = Mime {
        ttype: "image",
        subtype: "vnd.microsoft.icon",
    };
    #[doc = "\\[Saveen_Reddy\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_MIX;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mix\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.mix\");"]
    #[doc = r" ```"]
    pub const VND_MIX: Mime = Mime {
        ttype: "image",
        subtype: "vnd.mix",
    };
    #[doc = "\\[Gregory_Vaughan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_MS_MODI;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-modi\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.ms-modi\");"]
    #[doc = r" ```"]
    pub const VND_MS_MODI: Mime = Mime {
        ttype: "image",
        subtype: "vnd.ms-modi",
    };
    #[doc = "\\[Stuart_Parmenter\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_MOZILLA_APNG;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mozilla.apng\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.mozilla.apng\");"]
    #[doc = r" ```"]
    pub const VND_MOZILLA_APNG: Mime = Mime {
        ttype: "image",
        subtype: "vnd.mozilla.apng",
    };
    #[doc = "\\[Marc_Douglas_Spencer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_NET_FPX;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.net-fpx\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.net-fpx\");"]
    #[doc = r" ```"]
    pub const VND_NET_FPX: Mime = Mime {
        ttype: "image",
        subtype: "vnd.net-fpx",
    };
    #[doc = "\\[PCO_AG\\]\\[Jan_Zeman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_PCO_B16;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pco.b16\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.pco.b16\");"]
    #[doc = r" ```"]
    pub const VND_PCO_B16: Mime = Mime {
        ttype: "image",
        subtype: "vnd.pco.b16",
    };
    #[doc = "\\[Randolph_Fritz\\]\\[Greg_Ward\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_RADIANCE;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radiance\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.radiance\");"]
    #[doc = r" ```"]
    pub const VND_RADIANCE: Mime = Mime {
        ttype: "image",
        subtype: "vnd.radiance",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_SEALED_PNG;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.png\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.sealed.png\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_PNG: Mime = Mime {
        ttype: "image",
        subtype: "vnd.sealed.png",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_SEALEDMEDIA_SOFTSEAL_GIF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealedmedia.softseal.gif\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.sealedmedia.softseal.gif\");"]
    #[doc = r" ```"]
    pub const VND_SEALEDMEDIA_SOFTSEAL_GIF: Mime = Mime {
        ttype: "image",
        subtype: "vnd.sealedmedia.softseal.gif",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_SEALEDMEDIA_SOFTSEAL_JPG;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealedmedia.softseal.jpg\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.sealedmedia.softseal.jpg\");"]
    #[doc = r" ```"]
    pub const VND_SEALEDMEDIA_SOFTSEAL_JPG: Mime = Mime {
        ttype: "image",
        subtype: "vnd.sealedmedia.softseal.jpg",
    };
    #[doc = "\\[Jodi_Moline\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_SVF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.svf\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.svf\");"]
    #[doc = r" ```"]
    pub const VND_SVF: Mime = Mime {
        ttype: "image",
        subtype: "vnd.svf",
    };
    #[doc = "\\[Ni_Hui\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_TENCENT_TAP;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.tencent.tap\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.tencent.tap\");"]
    #[doc = r" ```"]
    pub const VND_TENCENT_TAP: Mime = Mime {
        ttype: "image",
        subtype: "vnd.tencent.tap",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_VALVE_SOURCE_TEXTURE;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.valve.source.texture\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.valve.source.texture\");"]
    #[doc = r" ```"]
    pub const VND_VALVE_SOURCE_TEXTURE: Mime = Mime {
        ttype: "image",
        subtype: "vnd.valve.source.texture",
    };
    #[doc = "\\[Peter_Stark\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_WAP_WBMP;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wap.wbmp\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.wap.wbmp\");"]
    #[doc = r" ```"]
    pub const VND_WAP_WBMP: Mime = Mime {
        ttype: "image",
        subtype: "vnd.wap.wbmp",
    };
    #[doc = "\\[Steven_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_XIFF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xiff\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.xiff\");"]
    #[doc = r" ```"]
    pub const VND_XIFF: Mime = Mime {
        ttype: "image",
        subtype: "vnd.xiff",
    };
    #[doc = "\\[Chris_Charabaruk\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::VND_ZBRUSH_PCX;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.zbrush.pcx\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/vnd.zbrush.pcx\");"]
    #[doc = r" ```"]
    pub const VND_ZBRUSH_PCX: Mime = Mime {
        ttype: "image",
        subtype: "vnd.zbrush.pcx",
    };
    #[doc = "\\[RFC-zern-webp-15\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::WEBP;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"webp\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/webp\");"]
    #[doc = r" ```"]
    pub const WEBP: Mime = Mime {
        ttype: "image",
        subtype: "webp",
    };
    #[doc = "\\[RFC7903\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::WMF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"wmf\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/wmf\");"]
    #[doc = r" ```"]
    pub const WMF: Mime = Mime {
        ttype: "image",
        subtype: "wmf",
    };
    #[doc = "\\[RFC7903\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::X_EMF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"x-emf\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/x-emf\");"]
    #[doc = r" ```"]
    pub const X_EMF: Mime = Mime {
        ttype: "image",
        subtype: "x-emf",
    };
    #[doc = "\\[RFC7903\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::image::X_WMF;"]
    #[doc = "assert_eq!(media.ttype, \"image\");"]
    #[doc = "assert_eq!(media.subtype, \"x-wmf\");"]
    #[doc = "assert_eq!(media.to_string(), \"image/x-wmf\");"]
    #[doc = r" ```"]
    pub const X_WMF: Mime = Mime {
        ttype: "image",
        subtype: "x-wmf",
    };
}
pub mod message {
    use super::*;
    #[doc = "\\[RFC9292\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::BHTTP;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"bhttp\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/bhttp\");"]
    #[doc = r" ```"]
    pub const BHTTP: Mime = Mime {
        ttype: "message",
        subtype: "bhttp",
    };
    #[doc = "\\[RFC3862\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::CPIM;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"CPIM\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/CPIM\");"]
    #[doc = r" ```"]
    pub const CPIM: Mime = Mime {
        ttype: "message",
        subtype: "CPIM",
    };
    #[doc = "\\[RFC1894\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::DELIVERY_STATUS;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"delivery-status\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/delivery-status\");"]
    #[doc = r" ```"]
    pub const DELIVERY_STATUS: Mime = Mime {
        ttype: "message",
        subtype: "delivery-status",
    };
    #[doc = "\\[RFC8098\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::DISPOSITION_NOTIFICATION;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"disposition-notification\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/disposition-notification\");"]
    #[doc = r" ```"]
    pub const DISPOSITION_NOTIFICATION: Mime = Mime {
        ttype: "message",
        subtype: "disposition-notification",
    };
    #[doc = "\\[RFC4735\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::EXAMPLE;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"example\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/example\");"]
    #[doc = r" ```"]
    pub const EXAMPLE: Mime = Mime {
        ttype: "message",
        subtype: "example",
    };
    #[doc = "\\[RFC2045\\]\\[RFC2046\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::EXTERNAL_BODY;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"external-body\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/external-body\");"]
    #[doc = r" ```"]
    pub const EXTERNAL_BODY: Mime = Mime {
        ttype: "message",
        subtype: "external-body",
    };
    #[doc = "\\[RFC5965\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::FEEDBACK_REPORT;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"feedback-report\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/feedback-report\");"]
    #[doc = r" ```"]
    pub const FEEDBACK_REPORT: Mime = Mime {
        ttype: "message",
        subtype: "feedback-report",
    };
    #[doc = "\\[RFC6532\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::GLOBAL;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"global\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/global\");"]
    #[doc = r" ```"]
    pub const GLOBAL: Mime = Mime {
        ttype: "message",
        subtype: "global",
    };
    #[doc = "\\[RFC6533\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::GLOBAL_DELIVERY_STATUS;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"global-delivery-status\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/global-delivery-status\");"]
    #[doc = r" ```"]
    pub const GLOBAL_DELIVERY_STATUS: Mime = Mime {
        ttype: "message",
        subtype: "global-delivery-status",
    };
    #[doc = "\\[RFC6533\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::GLOBAL_DISPOSITION_NOTIFICATION;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"global-disposition-notification\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/global-disposition-notification\");"]
    #[doc = r" ```"]
    pub const GLOBAL_DISPOSITION_NOTIFICATION: Mime = Mime {
        ttype: "message",
        subtype: "global-disposition-notification",
    };
    #[doc = "\\[RFC6533\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::GLOBAL_HEADERS;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"global-headers\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/global-headers\");"]
    #[doc = r" ```"]
    pub const GLOBAL_HEADERS: Mime = Mime {
        ttype: "message",
        subtype: "global-headers",
    };
    #[doc = "\\[RFC9112\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::HTTP;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"http\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/http\");"]
    #[doc = r" ```"]
    pub const HTTP: Mime = Mime {
        ttype: "message",
        subtype: "http",
    };
    #[doc = "\\[RFC5438\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::IMDN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"imdn+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/imdn+xml\");"]
    #[doc = r" ```"]
    pub const IMDN_XML: Mime = Mime {
        ttype: "message",
        subtype: "imdn+xml",
    };
    #[doc = "\\[RFC9420\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::MLS;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"mls\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/mls\");"]
    #[doc = r" ```"]
    pub const MLS: Mime = Mime {
        ttype: "message",
        subtype: "mls",
    };
    #[doc = "\\[RFC5537\\]\\[Henry_Spencer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::NEWS;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"news\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/news\");"]
    #[doc = r" ```"]
    pub const NEWS: Mime = Mime {
        ttype: "message",
        subtype: "news",
    };
    #[doc = "\\[RFC9458\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::OHTTP_REQ;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"ohttp-req\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/ohttp-req\");"]
    #[doc = r" ```"]
    pub const OHTTP_REQ: Mime = Mime {
        ttype: "message",
        subtype: "ohttp-req",
    };
    #[doc = "\\[RFC9458\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::OHTTP_RES;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"ohttp-res\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/ohttp-res\");"]
    #[doc = r" ```"]
    pub const OHTTP_RES: Mime = Mime {
        ttype: "message",
        subtype: "ohttp-res",
    };
    #[doc = "\\[RFC2045\\]\\[RFC2046\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::PARTIAL;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"partial\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/partial\");"]
    #[doc = r" ```"]
    pub const PARTIAL: Mime = Mime {
        ttype: "message",
        subtype: "partial",
    };
    #[doc = "\\[RFC2045\\]\\[RFC2046\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::RFC822;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"rfc822\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/rfc822\");"]
    #[doc = r" ```"]
    pub const RFC822: Mime = Mime {
        ttype: "message",
        subtype: "rfc822",
    };
    #[doc = "\\[RFC2660\\]\\[Status change of HTTP experiments to Historic\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::S_HTTP;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"s-http\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/s-http\");"]
    #[doc = r" ```"]
    pub const S_HTTP: Mime = Mime {
        ttype: "message",
        subtype: "s-http",
    };
    #[doc = "\\[RFC3261\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::SIP;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"sip\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/sip\");"]
    #[doc = r" ```"]
    pub const SIP: Mime = Mime {
        ttype: "message",
        subtype: "sip",
    };
    #[doc = "\\[RFC3420\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::SIPFRAG;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"sipfrag\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/sipfrag\");"]
    #[doc = r" ```"]
    pub const SIPFRAG: Mime = Mime {
        ttype: "message",
        subtype: "sipfrag",
    };
    #[doc = "\\[RFC3886\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::TRACKING_STATUS;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"tracking-status\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/tracking-status\");"]
    #[doc = r" ```"]
    pub const TRACKING_STATUS: Mime = Mime {
        ttype: "message",
        subtype: "tracking-status",
    };
    #[doc = "\\[Nicholas_Parks_Young\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::VND_SI_SIMP;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.si.simp\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/vnd.si.simp\");"]
    #[doc = r" ```"]
    pub const VND_SI_SIMP: Mime = Mime {
        ttype: "message",
        subtype: "vnd.si.simp",
    };
    #[doc = "\\[Mick_Conley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::message::VND_WFA_WSC;"]
    #[doc = "assert_eq!(media.ttype, \"message\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wfa.wsc\");"]
    #[doc = "assert_eq!(media.to_string(), \"message/vnd.wfa.wsc\");"]
    #[doc = r" ```"]
    pub const VND_WFA_WSC: Mime = Mime {
        ttype: "message",
        subtype: "vnd.wfa.wsc",
    };
}
pub mod model {
    use super::*;
    #[doc = "\\[http://www.3mf.io/specification\\]\\[_3MF\\]\\[Michael_Sweet\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::_3MF;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"3mf\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/3mf\");"]
    #[doc = r" ```"]
    pub const _3MF: Mime = Mime {
        ttype: "model",
        subtype: "3mf",
    };
    #[doc = "\\[ASTM\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::E57;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"e57\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/e57\");"]
    #[doc = r" ```"]
    pub const E57: Mime = Mime {
        ttype: "model",
        subtype: "e57",
    };
    #[doc = "\\[RFC4735\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::EXAMPLE;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"example\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/example\");"]
    #[doc = r" ```"]
    pub const EXAMPLE: Mime = Mime {
        ttype: "model",
        subtype: "example",
    };
    #[doc = "\\[Khronos\\]\\[Saurabh_Bhatia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::GLTF_BINARY;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"gltf-binary\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/gltf-binary\");"]
    #[doc = r" ```"]
    pub const GLTF_BINARY: Mime = Mime {
        ttype: "model",
        subtype: "gltf-binary",
    };
    #[doc = "\\[Khronos\\]\\[Uli_Klumpp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::GLTF_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"gltf+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/gltf+json\");"]
    #[doc = r" ```"]
    pub const GLTF_JSON: Mime = Mime {
        ttype: "model",
        subtype: "gltf+json",
    };
    #[doc = "\\[ISO-TC_184-SC_4\\]\\[Michael_Zink\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::JT;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"JT\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/JT\");"]
    #[doc = r" ```"]
    pub const JT: Mime = Mime {
        ttype: "model",
        subtype: "JT",
    };
    #[doc = "\\[Curtis_Parks\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::IGES;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"iges\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/iges\");"]
    #[doc = r" ```"]
    pub const IGES: Mime = Mime {
        ttype: "model",
        subtype: "iges",
    };
    #[doc = "\\[RFC2077\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::MESH;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"mesh\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/mesh\");"]
    #[doc = r" ```"]
    pub const MESH: Mime = Mime {
        ttype: "model",
        subtype: "mesh",
    };
    #[doc = "\\[DICOM_Standard_Committee\\]\\[DICOM_WG_17\\]\\[Carolyn_Hull\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::MTL;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"mtl\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/mtl\");"]
    #[doc = r" ```"]
    pub const MTL: Mime = Mime {
        ttype: "model",
        subtype: "mtl",
    };
    #[doc = "\\[DICOM_Standard_Committee\\]\\[DICOM_WG_17\\]\\[Carolyn_Hull\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::OBJ;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"obj\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/obj\");"]
    #[doc = r" ```"]
    pub const OBJ: Mime = Mime {
        ttype: "model",
        subtype: "obj",
    };
    #[doc = "\\[ISO-TC_171-SC_2\\]\\[Betsy_Fanning\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::PRC;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"prc\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/prc\");"]
    #[doc = r" ```"]
    pub const PRC: Mime = Mime {
        ttype: "model",
        subtype: "prc",
    };
    #[doc = "\\[ISO-TC_184-SC_4\\]\\[Dana_Tripp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::STEP;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"step\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/step\");"]
    #[doc = r" ```"]
    pub const STEP: Mime = Mime {
        ttype: "model",
        subtype: "step",
    };
    #[doc = "\\[ISO-TC_184-SC_4\\]\\[Dana_Tripp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::STEP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"step+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/step+xml\");"]
    #[doc = r" ```"]
    pub const STEP_XML: Mime = Mime {
        ttype: "model",
        subtype: "step+xml",
    };
    #[doc = "\\[ISO-TC_184-SC_4\\]\\[Dana_Tripp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::STEP_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"step+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/step+zip\");"]
    #[doc = r" ```"]
    pub const STEP_ZIP: Mime = Mime {
        ttype: "model",
        subtype: "step+zip",
    };
    #[doc = "\\[ISO-TC_184-SC_4\\]\\[Dana_Tripp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::STEP_XML_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"step-xml+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/step-xml+zip\");"]
    #[doc = r" ```"]
    pub const STEP_XML_ZIP: Mime = Mime {
        ttype: "model",
        subtype: "step-xml+zip",
    };
    #[doc = "\\[DICOM_Standard_Committee\\]\\[DICOM_WG_17\\]\\[Carolyn_Hull\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::STL;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"stl\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/stl\");"]
    #[doc = r" ```"]
    pub const STL: Mime = Mime {
        ttype: "model",
        subtype: "stl",
    };
    #[doc = "\\[PDF_Association\\]\\[Peter_Wyatt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::U3D;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"u3d\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/u3d\");"]
    #[doc = r" ```"]
    pub const U3D: Mime = Mime {
        ttype: "model",
        subtype: "u3d",
    };
    #[doc = "\\[Displaced_Micro-Mesh_SDK_Support\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_BARY;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bary\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.bary\");"]
    #[doc = r" ```"]
    pub const VND_BARY: Mime = Mime {
        ttype: "model",
        subtype: "vnd.bary",
    };
    #[doc = "\\[Robert_Monaghan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_CLD;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cld\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.cld\");"]
    #[doc = r" ```"]
    pub const VND_CLD: Mime = Mime {
        ttype: "model",
        subtype: "vnd.cld",
    };
    #[doc = "\\[James_Riordon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_COLLADA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.collada+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.collada+xml\");"]
    #[doc = r" ```"]
    pub const VND_COLLADA_XML: Mime = Mime {
        ttype: "model",
        subtype: "vnd.collada+xml",
    };
    #[doc = "\\[Jason_Pratt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_DWF;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dwf\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.dwf\");"]
    #[doc = r" ```"]
    pub const VND_DWF: Mime = Mime {
        ttype: "model",
        subtype: "vnd.dwf",
    };
    #[doc = "\\[Michael_Powers\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_FLATLAND_3DML;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.flatland.3dml\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.flatland.3dml\");"]
    #[doc = r" ```"]
    pub const VND_FLATLAND_3DML: Mime = Mime {
        ttype: "model",
        subtype: "vnd.flatland.3dml",
    };
    #[doc = "\\[Attila_Babits\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_GDL;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gdl\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.gdl\");"]
    #[doc = r" ```"]
    pub const VND_GDL: Mime = Mime {
        ttype: "model",
        subtype: "vnd.gdl",
    };
    #[doc = "\\[Attila_Babits\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_GS_GDL;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gs-gdl\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.gs-gdl\");"]
    #[doc = r" ```"]
    pub const VND_GS_GDL: Mime = Mime {
        ttype: "model",
        subtype: "vnd.gs-gdl",
    };
    #[doc = "\\[Yutaka_Ozaki\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_GTW;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gtw\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.gtw\");"]
    #[doc = r" ```"]
    pub const VND_GTW: Mime = Mime {
        ttype: "model",
        subtype: "vnd.gtw",
    };
    #[doc = "\\[Christopher_Brooks\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_MOML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.moml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.moml+xml\");"]
    #[doc = r" ```"]
    pub const VND_MOML_XML: Mime = Mime {
        ttype: "model",
        subtype: "vnd.moml+xml",
    };
    #[doc = "\\[Boris_Rabinovitch\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_MTS;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mts\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.mts\");"]
    #[doc = r" ```"]
    pub const VND_MTS: Mime = Mime {
        ttype: "model",
        subtype: "vnd.mts",
    };
    #[doc = "\\[Eric_Lengyel\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_OPENGEX;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.opengex\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.opengex\");"]
    #[doc = r" ```"]
    pub const VND_OPENGEX: Mime = Mime {
        ttype: "model",
        subtype: "vnd.opengex",
    };
    #[doc = "\\[Parasolid\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_PARASOLID_TRANSMIT_BINARY;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.parasolid.transmit.binary\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.parasolid.transmit.binary\");"]
    #[doc = r" ```"]
    pub const VND_PARASOLID_TRANSMIT_BINARY: Mime = Mime {
        ttype: "model",
        subtype: "vnd.parasolid.transmit.binary",
    };
    #[doc = "\\[Parasolid\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_PARASOLID_TRANSMIT_TEXT;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.parasolid.transmit.text\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.parasolid.transmit.text\");"]
    #[doc = r" ```"]
    pub const VND_PARASOLID_TRANSMIT_TEXT: Mime = Mime {
        ttype: "model",
        subtype: "vnd.parasolid.transmit.text",
    };
    #[doc = "\\[Daniel_Flassig\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_PYTHA_PYOX;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pytha.pyox\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.pytha.pyox\");"]
    #[doc = r" ```"]
    pub const VND_PYTHA_PYOX: Mime = Mime {
        ttype: "model",
        subtype: "vnd.pytha.pyox",
    };
    #[doc = "\\[Benson_Margulies\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_ROSETTE_ANNOTATED_DATA_MODEL;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.rosette.annotated-data-model\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.rosette.annotated-data-model\");"]
    #[doc = r" ```"]
    pub const VND_ROSETTE_ANNOTATED_DATA_MODEL: Mime = Mime {
        ttype: "model",
        subtype: "vnd.rosette.annotated-data-model",
    };
    #[doc = "\\[SAP_SE\\]\\[Igor_Afanasyev\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_SAP_VDS;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sap.vds\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.sap.vds\");"]
    #[doc = r" ```"]
    pub const VND_SAP_VDS: Mime = Mime {
        ttype: "model",
        subtype: "vnd.sap.vds",
    };
    #[doc = "\\[Sebastian_Grassia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_USDA;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.usda\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.usda\");"]
    #[doc = r" ```"]
    pub const VND_USDA: Mime = Mime {
        ttype: "model",
        subtype: "vnd.usda",
    };
    #[doc = "\\[Sebastian_Grassia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_USDZ_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.usdz+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.usdz+zip\");"]
    #[doc = r" ```"]
    pub const VND_USDZ_ZIP: Mime = Mime {
        ttype: "model",
        subtype: "vnd.usdz+zip",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_VALVE_SOURCE_COMPILED_MAP;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.valve.source.compiled-map\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.valve.source.compiled-map\");"]
    #[doc = r" ```"]
    pub const VND_VALVE_SOURCE_COMPILED_MAP: Mime = Mime {
        ttype: "model",
        subtype: "vnd.valve.source.compiled-map",
    };
    #[doc = "\\[Boris_Rabinovitch\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VND_VTU;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.vtu\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vnd.vtu\");"]
    #[doc = r" ```"]
    pub const VND_VTU: Mime = Mime {
        ttype: "model",
        subtype: "vnd.vtu",
    };
    #[doc = "\\[RFC2077\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::VRML;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"vrml\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/vrml\");"]
    #[doc = r" ```"]
    pub const VRML: Mime = Mime {
        ttype: "model",
        subtype: "vrml",
    };
    #[doc = "\\[Web3D\\]\\[Web3D_X3D\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::X3D_VRML;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"x3d-vrml\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/x3d-vrml\");"]
    #[doc = r" ```"]
    pub const X3D_VRML: Mime = Mime {
        ttype: "model",
        subtype: "x3d-vrml",
    };
    #[doc = "\\[Web3D_X3D\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::X3D_FASTINFOSET;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"x3d+fastinfoset\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/x3d+fastinfoset\");"]
    #[doc = r" ```"]
    pub const X3D_FASTINFOSET: Mime = Mime {
        ttype: "model",
        subtype: "x3d+fastinfoset",
    };
    #[doc = "\\[Web3D\\]\\[Web3D_X3D\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::model::X3D_XML;"]
    #[doc = "assert_eq!(media.ttype, \"model\");"]
    #[doc = "assert_eq!(media.subtype, \"x3d+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"model/x3d+xml\");"]
    #[doc = r" ```"]
    pub const X3D_XML: Mime = Mime {
        ttype: "model",
        subtype: "x3d+xml",
    };
}
pub mod multipart {
    use super::*;
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
}
pub mod text {
    use super::*;
    #[doc = "\\[RFC6015\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::_1D_INTERLEAVED_PARITYFEC;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"1d-interleaved-parityfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/1d-interleaved-parityfec\");"]
    #[doc = r" ```"]
    pub const _1D_INTERLEAVED_PARITYFEC: Mime = Mime {
        ttype: "text",
        subtype: "1d-interleaved-parityfec",
    };
    #[doc = "\\[W3C\\]\\[Robin_Berjon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::CACHE_MANIFEST;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"cache-manifest\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/cache-manifest\");"]
    #[doc = r" ```"]
    pub const CACHE_MANIFEST: Mime = Mime {
        ttype: "text",
        subtype: "cache-manifest",
    };
    #[doc = "\\[RFC5545\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::CALENDAR;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"calendar\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/calendar\");"]
    #[doc = r" ```"]
    pub const CALENDAR: Mime = Mime {
        ttype: "text",
        subtype: "calendar",
    };
    #[doc = "\\[HL7\\]\\[Bryn_Rhodes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::CQL;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"cql\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/cql\");"]
    #[doc = r" ```"]
    pub const CQL: Mime = Mime {
        ttype: "text",
        subtype: "cql",
    };
    #[doc = "\\[HL7\\]\\[Bryn_Rhodes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::CQL_EXPRESSION;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"cql-expression\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/cql-expression\");"]
    #[doc = r" ```"]
    pub const CQL_EXPRESSION: Mime = Mime {
        ttype: "text",
        subtype: "cql-expression",
    };
    #[doc = "\\[HL7\\]\\[Bryn_Rhodes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::CQL_IDENTIFIER;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"cql-identifier\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/cql-identifier\");"]
    #[doc = r" ```"]
    pub const CQL_IDENTIFIER: Mime = Mime {
        ttype: "text",
        subtype: "cql-identifier",
    };
    #[doc = "\\[RFC2318\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::CSS;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"css\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/css\");"]
    #[doc = r" ```"]
    pub const CSS: Mime = Mime {
        ttype: "text",
        subtype: "css",
    };
    #[doc = "\\[RFC4180\\]\\[RFC7111\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::CSV;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"csv\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/csv\");"]
    #[doc = r" ```"]
    pub const CSV: Mime = Mime {
        ttype: "text",
        subtype: "csv",
    };
    #[doc = "\\[National_Archives_UK\\]\\[David_Underdown\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::CSV_SCHEMA;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"csv-schema\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/csv-schema\");"]
    #[doc = r" ```"]
    pub const CSV_SCHEMA: Mime = Mime {
        ttype: "text",
        subtype: "csv-schema",
    };
    #[doc = "\\[RFC2425\\]\\[RFC6350\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::DIRECTORY;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"directory\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/directory\");"]
    #[doc = r" ```"]
    pub const DIRECTORY: Mime = Mime {
        ttype: "text",
        subtype: "directory",
    };
    #[doc = "\\[RFC4027\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::DNS;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"dns\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/dns\");"]
    #[doc = r" ```"]
    pub const DNS: Mime = Mime {
        ttype: "text",
        subtype: "dns",
    };
    #[doc = "\\[RFC9239\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::ECMASCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"ecmascript\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/ecmascript\");"]
    #[doc = r" ```"]
    pub const ECMASCRIPT: Mime = Mime {
        ttype: "text",
        subtype: "ecmascript",
    };
    #[doc = "\\[RFC6849\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::ENCAPRTP;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"encaprtp\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/encaprtp\");"]
    #[doc = r" ```"]
    pub const ENCAPRTP: Mime = Mime {
        ttype: "text",
        subtype: "encaprtp",
    };
    #[doc = "\\[RFC1896\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::ENRICHED;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"enriched\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/enriched\");"]
    #[doc = r" ```"]
    pub const ENRICHED: Mime = Mime {
        ttype: "text",
        subtype: "enriched",
    };
    #[doc = "\\[RFC4735\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::EXAMPLE;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"example\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/example\");"]
    #[doc = r" ```"]
    pub const EXAMPLE: Mime = Mime {
        ttype: "text",
        subtype: "example",
    };
    #[doc = "\\[HL7\\]\\[Bryn_Rhodes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::FHIRPATH;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"fhirpath\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/fhirpath\");"]
    #[doc = r" ```"]
    pub const FHIRPATH: Mime = Mime {
        ttype: "text",
        subtype: "fhirpath",
    };
    #[doc = "\\[RFC8627\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::FLEXFEC;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"flexfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/flexfec\");"]
    #[doc = r" ```"]
    pub const FLEXFEC: Mime = Mime {
        ttype: "text",
        subtype: "flexfec",
    };
    #[doc = "\\[RFC6354\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::FWDRED;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"fwdred\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/fwdred\");"]
    #[doc = r" ```"]
    pub const FWDRED: Mime = Mime {
        ttype: "text",
        subtype: "fwdred",
    };
    #[doc = "\\[Sequence_Ontology\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::GFF3;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"gff3\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/gff3\");"]
    #[doc = r" ```"]
    pub const GFF3: Mime = Mime {
        ttype: "text",
        subtype: "gff3",
    };
    #[doc = "\\[RFC6787\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::GRAMMAR_REF_LIST;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"grammar-ref-list\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/grammar-ref-list\");"]
    #[doc = r" ```"]
    pub const GRAMMAR_REF_LIST: Mime = Mime {
        ttype: "text",
        subtype: "grammar-ref-list",
    };
    #[doc = "\\[HL7\\]\\[Marc_Duteau\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::HL7V2;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"hl7v2\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/hl7v2\");"]
    #[doc = r" ```"]
    pub const HL7V2: Mime = Mime {
        ttype: "text",
        subtype: "hl7v2",
    };
    #[doc = "\\[W3C\\]\\[Robin_Berjon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::HTML;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"html\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/html\");"]
    #[doc = r" ```"]
    pub const HTML: Mime = Mime {
        ttype: "text",
        subtype: "html",
    };
    #[doc = "\\[RFC9239\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::JAVASCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"javascript\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/javascript\");"]
    #[doc = r" ```"]
    pub const JAVASCRIPT: Mime = Mime {
        ttype: "text",
        subtype: "javascript",
    };
    #[doc = "\\[Peeter_Piegaze\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::JCR_CND;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"jcr-cnd\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/jcr-cnd\");"]
    #[doc = r" ```"]
    pub const JCR_CND: Mime = Mime {
        ttype: "text",
        subtype: "jcr-cnd",
    };
    #[doc = "\\[RFC7763\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::MARKDOWN;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"markdown\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/markdown\");"]
    #[doc = r" ```"]
    pub const MARKDOWN: Mime = Mime {
        ttype: "text",
        subtype: "markdown",
    };
    #[doc = "\\[Jesse_Alama\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::MIZAR;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"mizar\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/mizar\");"]
    #[doc = r" ```"]
    pub const MIZAR: Mime = Mime {
        ttype: "text",
        subtype: "mizar",
    };
    #[doc = "\\[W3C\\]\\[Eric_Prudhommeaux\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::N3;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"n3\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/n3\");"]
    #[doc = r" ```"]
    pub const N3: Mime = Mime {
        ttype: "text",
        subtype: "n3",
    };
    #[doc = "\\[RFC7826\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::PARAMETERS;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"parameters\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/parameters\");"]
    #[doc = r" ```"]
    pub const PARAMETERS: Mime = Mime {
        ttype: "text",
        subtype: "parameters",
    };
    #[doc = "\\[RFC3009\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::PARITYFEC;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"parityfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/parityfec\");"]
    #[doc = r" ```"]
    pub const PARITYFEC: Mime = Mime {
        ttype: "text",
        subtype: "parityfec",
    };
    #[doc = "\\[RFC2046\\]\\[RFC3676\\]\\[RFC5147\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::PLAIN;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"plain\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/plain\");"]
    #[doc = r" ```"]
    pub const PLAIN: Mime = Mime {
        ttype: "text",
        subtype: "plain",
    };
    #[doc = "\\[W3C\\]\\[Ivan_Herman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::PROVENANCE_NOTATION;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"provenance-notation\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/provenance-notation\");"]
    #[doc = r" ```"]
    pub const PROVENANCE_NOTATION: Mime = Mime {
        ttype: "text",
        subtype: "provenance-notation",
    };
    #[doc = "\\[Benja_Fallenstein\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::PRS_FALLENSTEIN_RST;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.fallenstein.rst\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/prs.fallenstein.rst\");"]
    #[doc = r" ```"]
    pub const PRS_FALLENSTEIN_RST: Mime = Mime {
        ttype: "text",
        subtype: "prs.fallenstein.rst",
    };
    #[doc = "\\[John_Lines\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::PRS_LINES_TAG;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.lines.tag\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/prs.lines.tag\");"]
    #[doc = r" ```"]
    pub const PRS_LINES_TAG: Mime = Mime {
        ttype: "text",
        subtype: "prs.lines.tag",
    };
    #[doc = "\\[Hans-Dieter_A._Hiep\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::PRS_PROP_LOGIC;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.prop.logic\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/prs.prop.logic\");"]
    #[doc = r" ```"]
    pub const PRS_PROP_LOGIC: Mime = Mime {
        ttype: "text",
        subtype: "prs.prop.logic",
    };
    #[doc = "\\[Matin_Bavardi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::PRS_TEXI;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.texi\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/prs.texi\");"]
    #[doc = r" ```"]
    pub const PRS_TEXI: Mime = Mime {
        ttype: "text",
        subtype: "prs.texi",
    };
    #[doc = "\\[RFC6682\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::RAPTORFEC;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"raptorfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/raptorfec\");"]
    #[doc = r" ```"]
    pub const RAPTORFEC: Mime = Mime {
        ttype: "text",
        subtype: "raptorfec",
    };
    #[doc = "\\[RFC4102\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::RED;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"RED\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/RED\");"]
    #[doc = r" ```"]
    pub const RED: Mime = Mime {
        ttype: "text",
        subtype: "RED",
    };
    #[doc = "\\[RFC6522\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::RFC822_HEADERS;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"rfc822-headers\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/rfc822-headers\");"]
    #[doc = r" ```"]
    pub const RFC822_HEADERS: Mime = Mime {
        ttype: "text",
        subtype: "rfc822-headers",
    };
    #[doc = "\\[RFC2045\\]\\[RFC2046\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::RICHTEXT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"richtext\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/richtext\");"]
    #[doc = r" ```"]
    pub const RICHTEXT: Mime = Mime {
        ttype: "text",
        subtype: "richtext",
    };
    #[doc = "\\[Paul_Lindner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::RTF;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"rtf\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/rtf\");"]
    #[doc = r" ```"]
    pub const RTF: Mime = Mime {
        ttype: "text",
        subtype: "rtf",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::RTP_ENC_AESCM128;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"rtp-enc-aescm128\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/rtp-enc-aescm128\");"]
    #[doc = r" ```"]
    pub const RTP_ENC_AESCM128: Mime = Mime {
        ttype: "text",
        subtype: "rtp-enc-aescm128",
    };
    #[doc = "\\[RFC6849\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::RTPLOOPBACK;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"rtploopback\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/rtploopback\");"]
    #[doc = r" ```"]
    pub const RTPLOOPBACK: Mime = Mime {
        ttype: "text",
        subtype: "rtploopback",
    };
    #[doc = "\\[RFC4588\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::RTX;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"rtx\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/rtx\");"]
    #[doc = r" ```"]
    pub const RTX: Mime = Mime {
        ttype: "text",
        subtype: "rtx",
    };
    #[doc = "\\[RFC1874\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::SGML;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"SGML\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/SGML\");"]
    #[doc = r" ```"]
    pub const SGML: Mime = Mime {
        ttype: "text",
        subtype: "SGML",
    };
    #[doc = "\\[W3C_SHACL_Community_Group\\]\\[Vladimir_Alexiev\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::SHACLC;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"shaclc\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/shaclc\");"]
    #[doc = r" ```"]
    pub const SHACLC: Mime = Mime {
        ttype: "text",
        subtype: "shaclc",
    };
    #[doc = "\\[W3C\\]\\[Eric_Prudhommeaux\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::SHEX;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"shex\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/shex\");"]
    #[doc = r" ```"]
    pub const SHEX: Mime = Mime {
        ttype: "text",
        subtype: "shex",
    };
    #[doc = "\\[Linux_Foundation\\]\\[Rose_Judge\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::SPDX;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"spdx\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/spdx\");"]
    #[doc = r" ```"]
    pub const SPDX: Mime = Mime {
        ttype: "text",
        subtype: "spdx",
    };
    #[doc = "\\[IEEE-ISTO-PWG-PPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::STRINGS;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"strings\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/strings\");"]
    #[doc = r" ```"]
    pub const STRINGS: Mime = Mime {
        ttype: "text",
        subtype: "strings",
    };
    #[doc = "\\[RFC4103\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::T140;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"t140\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/t140\");"]
    #[doc = r" ```"]
    pub const T140: Mime = Mime {
        ttype: "text",
        subtype: "t140",
    };
    #[doc = "\\[Paul_Lindner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::TAB_SEPARATED_VALUES;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"tab-separated-values\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/tab-separated-values\");"]
    #[doc = r" ```"]
    pub const TAB_SEPARATED_VALUES: Mime = Mime {
        ttype: "text",
        subtype: "tab-separated-values",
    };
    #[doc = "\\[RFC4263\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::TROFF;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"troff\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/troff\");"]
    #[doc = r" ```"]
    pub const TROFF: Mime = Mime {
        ttype: "text",
        subtype: "troff",
    };
    #[doc = "\\[W3C\\]\\[Eric_Prudhommeaux\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::TURTLE;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"turtle\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/turtle\");"]
    #[doc = r" ```"]
    pub const TURTLE: Mime = Mime {
        ttype: "text",
        subtype: "turtle",
    };
    #[doc = "\\[RFC5109\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::ULPFEC;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"ulpfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/ulpfec\");"]
    #[doc = r" ```"]
    pub const ULPFEC: Mime = Mime {
        ttype: "text",
        subtype: "ulpfec",
    };
    #[doc = "\\[RFC2483\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::URI_LIST;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"uri-list\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/uri-list\");"]
    #[doc = r" ```"]
    pub const URI_LIST: Mime = Mime {
        ttype: "text",
        subtype: "uri-list",
    };
    #[doc = "\\[RFC6350\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VCARD;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vcard\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vcard\");"]
    #[doc = r" ```"]
    pub const VCARD: Mime = Mime {
        ttype: "text",
        subtype: "vcard",
    };
    #[doc = "\\[Regis_Dehoux\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_A;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.a\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.a\");"]
    #[doc = r" ```"]
    pub const VND_A: Mime = Mime {
        ttype: "text",
        subtype: "vnd.a",
    };
    #[doc = "\\[Steve_Allen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_ABC;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.abc\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.abc\");"]
    #[doc = r" ```"]
    pub const VND_ABC: Mime = Mime {
        ttype: "text",
        subtype: "vnd.abc",
    };
    #[doc = "\\[Kim_Scarborough\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_ASCII_ART;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ascii-art\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.ascii-art\");"]
    #[doc = r" ```"]
    pub const VND_ASCII_ART: Mime = Mime {
        ttype: "text",
        subtype: "vnd.ascii-art",
    };
    #[doc = "\\[Robert_Byrnes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_CURL;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.curl\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.curl\");"]
    #[doc = r" ```"]
    pub const VND_CURL: Mime = Mime {
        ttype: "text",
        subtype: "vnd.curl",
    };
    #[doc = "\\[Charles_Plessy\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_DEBIAN_COPYRIGHT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.debian.copyright\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.debian.copyright\");"]
    #[doc = r" ```"]
    pub const VND_DEBIAN_COPYRIGHT: Mime = Mime {
        ttype: "text",
        subtype: "vnd.debian.copyright",
    };
    #[doc = "\\[Dan_Bradley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_DMCLIENTSCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.DMClientScript\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.DMClientScript\");"]
    #[doc = r" ```"]
    pub const VND_DMCLIENTSCRIPT: Mime = Mime {
        ttype: "text",
        subtype: "vnd.DMClientScript",
    };
    #[doc = "\\[Peter_Siebert\\]\\[Michael_Lagally\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_DVB_SUBTITLE;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.subtitle\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.dvb.subtitle\");"]
    #[doc = r" ```"]
    pub const VND_DVB_SUBTITLE: Mime = Mime {
        ttype: "text",
        subtype: "vnd.dvb.subtitle",
    };
    #[doc = "\\[Stefan_Eilemann\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_ESMERTEC_THEME_DESCRIPTOR;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.esmertec.theme-descriptor\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.esmertec.theme-descriptor\");"]
    #[doc = r" ```"]
    pub const VND_ESMERTEC_THEME_DESCRIPTOR: Mime = Mime {
        ttype: "text",
        subtype: "vnd.esmertec.theme-descriptor",
    };
    #[doc = "\\[Martin_Cizek\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_EXCHANGEABLE;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.exchangeable\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.exchangeable\");"]
    #[doc = r" ```"]
    pub const VND_EXCHANGEABLE: Mime = Mime {
        ttype: "text",
        subtype: "vnd.exchangeable",
    };
    #[doc = "\\[Gordon_Clarke\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_FAMILYSEARCH_GEDCOM;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.familysearch.gedcom\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.familysearch.gedcom\");"]
    #[doc = r" ```"]
    pub const VND_FAMILYSEARCH_GEDCOM: Mime = Mime {
        ttype: "text",
        subtype: "vnd.familysearch.gedcom",
    };
    #[doc = "\\[Steve_Gilberd\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_FICLAB_FLT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ficlab.flt\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.ficlab.flt\");"]
    #[doc = r" ```"]
    pub const VND_FICLAB_FLT: Mime = Mime {
        ttype: "text",
        subtype: "vnd.ficlab.flt",
    };
    #[doc = "\\[John-Mark_Gurney\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_FLY;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fly\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.fly\");"]
    #[doc = r" ```"]
    pub const VND_FLY: Mime = Mime {
        ttype: "text",
        subtype: "vnd.fly",
    };
    #[doc = "\\[Kari_E._Hurtta\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_FMI_FLEXSTOR;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fmi.flexstor\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.fmi.flexstor\");"]
    #[doc = r" ```"]
    pub const VND_FMI_FLEXSTOR: Mime = Mime {
        ttype: "text",
        subtype: "vnd.fmi.flexstor",
    };
    #[doc = "\\[Mi_Tar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_GML;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gml\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.gml\");"]
    #[doc = r" ```"]
    pub const VND_GML: Mime = Mime {
        ttype: "text",
        subtype: "vnd.gml",
    };
    #[doc = "\\[John_Ellson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_GRAPHVIZ;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.graphviz\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.graphviz\");"]
    #[doc = r" ```"]
    pub const VND_GRAPHVIZ: Mime = Mime {
        ttype: "text",
        subtype: "vnd.graphviz",
    };
    #[doc = "\\[Hill_Hanxv\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_HANS;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hans\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.hans\");"]
    #[doc = r" ```"]
    pub const VND_HANS: Mime = Mime {
        ttype: "text",
        subtype: "vnd.hans",
    };
    #[doc = "\\[Heungsub_Lee\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_HGL;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hgl\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.hgl\");"]
    #[doc = r" ```"]
    pub const VND_HGL: Mime = Mime {
        ttype: "text",
        subtype: "vnd.hgl",
    };
    #[doc = "\\[Michael_Powers\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_IN3D_3DML;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.in3d.3dml\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.in3d.3dml\");"]
    #[doc = r" ```"]
    pub const VND_IN3D_3DML: Mime = Mime {
        ttype: "text",
        subtype: "vnd.in3d.3dml",
    };
    #[doc = "\\[Michael_Powers\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_IN3D_SPOT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.in3d.spot\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.in3d.spot\");"]
    #[doc = r" ```"]
    pub const VND_IN3D_SPOT: Mime = Mime {
        ttype: "text",
        subtype: "vnd.in3d.spot",
    };
    #[doc = "\\[IPTC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_IPTC_NEWSML;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.IPTC.NewsML\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.IPTC.NewsML\");"]
    #[doc = r" ```"]
    pub const VND_IPTC_NEWSML: Mime = Mime {
        ttype: "text",
        subtype: "vnd.IPTC.NewsML",
    };
    #[doc = "\\[IPTC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_IPTC_NITF;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.IPTC.NITF\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.IPTC.NITF\");"]
    #[doc = r" ```"]
    pub const VND_IPTC_NITF: Mime = Mime {
        ttype: "text",
        subtype: "vnd.IPTC.NITF",
    };
    #[doc = "\\[Mikusiak_Lubos\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_LATEX_Z;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.latex-z\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.latex-z\");"]
    #[doc = r" ```"]
    pub const VND_LATEX_Z: Mime = Mime {
        ttype: "text",
        subtype: "vnd.latex-z",
    };
    #[doc = "\\[Mark_Patton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_MOTOROLA_REFLEX;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.reflex\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.motorola.reflex\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_REFLEX: Mime = Mime {
        ttype: "text",
        subtype: "vnd.motorola.reflex",
    };
    #[doc = "\\[Jan_Nelson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_MS_MEDIAPACKAGE;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-mediapackage\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.ms-mediapackage\");"]
    #[doc = r" ```"]
    pub const VND_MS_MEDIAPACKAGE: Mime = Mime {
        ttype: "text",
        subtype: "vnd.ms-mediapackage",
    };
    #[doc = "\\[Feiyu_Xie\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_NET2PHONE_COMMCENTER_COMMAND;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.net2phone.commcenter.command\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.net2phone.commcenter.command\");"]
    #[doc = r" ```"]
    pub const VND_NET2PHONE_COMMCENTER_COMMAND: Mime = Mime {
        ttype: "text",
        subtype: "vnd.net2phone.commcenter.command",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_RADISYS_MSML_BASIC_LAYOUT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-basic-layout\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.radisys.msml-basic-layout\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_BASIC_LAYOUT: Mime = Mime {
        ttype: "text",
        subtype: "vnd.radisys.msml-basic-layout",
    };
    #[doc = "\\[Pierre_Papin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_SENX_WARPSCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.senx.warpscript\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.senx.warpscript\");"]
    #[doc = r" ```"]
    pub const VND_SENX_WARPSCRIPT: Mime = Mime {
        ttype: "text",
        subtype: "vnd.senx.warpscript",
    };
    #[doc = "\\[Nicholas_Parks_Young\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_SI_URICATALOGUE;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.si.uricatalogue\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.si.uricatalogue\");"]
    #[doc = r" ```"]
    pub const VND_SI_URICATALOGUE: Mime = Mime {
        ttype: "text",
        subtype: "vnd.si.uricatalogue",
    };
    #[doc = "\\[Gary_Adams\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_SUN_J2ME_APP_DESCRIPTOR;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sun.j2me.app-descriptor\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.sun.j2me.app-descriptor\");"]
    #[doc = r" ```"]
    pub const VND_SUN_J2ME_APP_DESCRIPTOR: Mime = Mime {
        ttype: "text",
        subtype: "vnd.sun.j2me.app-descriptor",
    };
    #[doc = "\\[Petter_Reinholdtsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_SOSI;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sosi\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.sosi\");"]
    #[doc = r" ```"]
    pub const VND_SOSI: Mime = Mime {
        ttype: "text",
        subtype: "vnd.sosi",
    };
    #[doc = "\\[David_Lee_Lambert\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_TROLLTECH_LINGUIST;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.trolltech.linguist\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.trolltech.linguist\");"]
    #[doc = r" ```"]
    pub const VND_TROLLTECH_LINGUIST: Mime = Mime {
        ttype: "text",
        subtype: "vnd.trolltech.linguist",
    };
    #[doc = "\\[Anand_Jahagirdar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_VCF;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.vcf\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.vcf\");"]
    #[doc = r" ```"]
    pub const VND_VCF: Mime = Mime {
        ttype: "text",
        subtype: "vnd.vcf",
    };
    #[doc = "\\[WAP-Forum\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_WAP_SI;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wap.si\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.wap.si\");"]
    #[doc = r" ```"]
    pub const VND_WAP_SI: Mime = Mime {
        ttype: "text",
        subtype: "vnd.wap.si",
    };
    #[doc = "\\[WAP-Forum\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_WAP_SL;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wap.sl\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.wap.sl\");"]
    #[doc = r" ```"]
    pub const VND_WAP_SL: Mime = Mime {
        ttype: "text",
        subtype: "vnd.wap.sl",
    };
    #[doc = "\\[Peter_Stark\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_WAP_WML;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wap.wml\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.wap.wml\");"]
    #[doc = r" ```"]
    pub const VND_WAP_WML: Mime = Mime {
        ttype: "text",
        subtype: "vnd.wap.wml",
    };
    #[doc = "\\[Peter_Stark\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_WAP_WMLSCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wap.wmlscript\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.wap.wmlscript\");"]
    #[doc = r" ```"]
    pub const VND_WAP_WMLSCRIPT: Mime = Mime {
        ttype: "text",
        subtype: "vnd.wap.wmlscript",
    };
    #[doc = "\\[Jessie_Frazelle\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VND_ZOO_KCL;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.zoo.kcl\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vnd.zoo.kcl\");"]
    #[doc = r" ```"]
    pub const VND_ZOO_KCL: Mime = Mime {
        ttype: "text",
        subtype: "vnd.zoo.kcl",
    };
    #[doc = "\\[W3C\\]\\[Silvia_Pfeiffer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::VTT;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"vtt\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/vtt\");"]
    #[doc = r" ```"]
    pub const VTT: Mime = Mime {
        ttype: "text",
        subtype: "vtt",
    };
    #[doc = "\\[W3C\\]\\[David_Neto\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::WGSL;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"wgsl\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/wgsl\");"]
    #[doc = r" ```"]
    pub const WGSL: Mime = Mime {
        ttype: "text",
        subtype: "wgsl",
    };
    #[doc = "\\[RFC7303\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::XML;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/xml\");"]
    #[doc = r" ```"]
    pub const XML: Mime = Mime {
        ttype: "text",
        subtype: "xml",
    };
    #[doc = "\\[RFC7303\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::text::XML_EXTERNAL_PARSED_ENTITY;"]
    #[doc = "assert_eq!(media.ttype, \"text\");"]
    #[doc = "assert_eq!(media.subtype, \"xml-external-parsed-entity\");"]
    #[doc = "assert_eq!(media.to_string(), \"text/xml-external-parsed-entity\");"]
    #[doc = r" ```"]
    pub const XML_EXTERNAL_PARSED_ENTITY: Mime = Mime {
        ttype: "text",
        subtype: "xml-external-parsed-entity",
    };
}
pub mod video {
    use super::*;
    #[doc = "\\[RFC6015\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::_1D_INTERLEAVED_PARITYFEC;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"1d-interleaved-parityfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/1d-interleaved-parityfec\");"]
    #[doc = r" ```"]
    pub const _1D_INTERLEAVED_PARITYFEC: Mime = Mime {
        ttype: "video",
        subtype: "1d-interleaved-parityfec",
    };
    #[doc = "\\[RFC3839\\]\\[RFC6381\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::_3GPP;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"3gpp\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/3gpp\");"]
    #[doc = r" ```"]
    pub const _3GPP: Mime = Mime {
        ttype: "video",
        subtype: "3gpp",
    };
    #[doc = "\\[RFC4393\\]\\[RFC6381\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::_3GPP2;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"3gpp2\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/3gpp2\");"]
    #[doc = r" ```"]
    pub const _3GPP2: Mime = Mime {
        ttype: "video",
        subtype: "3gpp2",
    };
    #[doc = "\\[RFC4396\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::_3GPP_TT;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"3gpp-tt\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/3gpp-tt\");"]
    #[doc = r" ```"]
    pub const _3GPP_TT: Mime = Mime {
        ttype: "video",
        subtype: "3gpp-tt",
    };
    #[doc = "\\[Alliance_for_Open_Media\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::AV1;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"AV1\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/AV1\");"]
    #[doc = r" ```"]
    pub const AV1: Mime = Mime {
        ttype: "video",
        subtype: "AV1",
    };
    #[doc = "\\[RFC3555\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::BMPEG;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"BMPEG\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/BMPEG\");"]
    #[doc = r" ```"]
    pub const BMPEG: Mime = Mime {
        ttype: "video",
        subtype: "BMPEG",
    };
    #[doc = "\\[RFC3555\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::BT656;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"BT656\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/BT656\");"]
    #[doc = r" ```"]
    pub const BT656: Mime = Mime {
        ttype: "video",
        subtype: "BT656",
    };
    #[doc = "\\[RFC3555\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::CELB;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"CelB\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/CelB\");"]
    #[doc = r" ```"]
    pub const CELB: Mime = Mime {
        ttype: "video",
        subtype: "CelB",
    };
    #[doc = "\\[RFC6469\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::DV;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"DV\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/DV\");"]
    #[doc = r" ```"]
    pub const DV: Mime = Mime {
        ttype: "video",
        subtype: "DV",
    };
    #[doc = "\\[RFC6849\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::ENCAPRTP;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"encaprtp\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/encaprtp\");"]
    #[doc = r" ```"]
    pub const ENCAPRTP: Mime = Mime {
        ttype: "video",
        subtype: "encaprtp",
    };
    #[doc = "\\[RFC9584\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::EVC;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"evc\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/evc\");"]
    #[doc = r" ```"]
    pub const EVC: Mime = Mime {
        ttype: "video",
        subtype: "evc",
    };
    #[doc = "\\[RFC4735\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::EXAMPLE;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"example\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/example\");"]
    #[doc = r" ```"]
    pub const EXAMPLE: Mime = Mime {
        ttype: "video",
        subtype: "example",
    };
    #[doc = "\\[RFC9043\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::FFV1;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"FFV1\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/FFV1\");"]
    #[doc = r" ```"]
    pub const FFV1: Mime = Mime {
        ttype: "video",
        subtype: "FFV1",
    };
    #[doc = "\\[RFC8627\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::FLEXFEC;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"flexfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/flexfec\");"]
    #[doc = r" ```"]
    pub const FLEXFEC: Mime = Mime {
        ttype: "video",
        subtype: "flexfec",
    };
    #[doc = "\\[RFC4587\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::H261;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"H261\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/H261\");"]
    #[doc = r" ```"]
    pub const H261: Mime = Mime {
        ttype: "video",
        subtype: "H261",
    };
    #[doc = "\\[RFC3555\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::H263;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"H263\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/H263\");"]
    #[doc = r" ```"]
    pub const H263: Mime = Mime {
        ttype: "video",
        subtype: "H263",
    };
    #[doc = "\\[RFC4629\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::H263_1998;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"H263-1998\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/H263-1998\");"]
    #[doc = r" ```"]
    pub const H263_1998: Mime = Mime {
        ttype: "video",
        subtype: "H263-1998",
    };
    #[doc = "\\[RFC4629\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::H263_2000;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"H263-2000\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/H263-2000\");"]
    #[doc = r" ```"]
    pub const H263_2000: Mime = Mime {
        ttype: "video",
        subtype: "H263-2000",
    };
    #[doc = "\\[RFC6184\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::H264;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"H264\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/H264\");"]
    #[doc = r" ```"]
    pub const H264: Mime = Mime {
        ttype: "video",
        subtype: "H264",
    };
    #[doc = "\\[RFC6185\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::H264_RCDO;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"H264-RCDO\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/H264-RCDO\");"]
    #[doc = r" ```"]
    pub const H264_RCDO: Mime = Mime {
        ttype: "video",
        subtype: "H264-RCDO",
    };
    #[doc = "\\[RFC6190\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::H264_SVC;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"H264-SVC\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/H264-SVC\");"]
    #[doc = r" ```"]
    pub const H264_SVC: Mime = Mime {
        ttype: "video",
        subtype: "H264-SVC",
    };
    #[doc = "\\[RFC7798\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::H265;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"H265\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/H265\");"]
    #[doc = r" ```"]
    pub const H265: Mime = Mime {
        ttype: "video",
        subtype: "H265",
    };
    #[doc = "\\[RFC9328\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::H266;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"H266\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/H266\");"]
    #[doc = r" ```"]
    pub const H266: Mime = Mime {
        ttype: "video",
        subtype: "H266",
    };
    #[doc = "\\[David_Singer\\]\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::ISO_SEGMENT;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"iso.segment\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/iso.segment\");"]
    #[doc = r" ```"]
    pub const ISO_SEGMENT: Mime = Mime {
        ttype: "video",
        subtype: "iso.segment",
    };
    #[doc = "\\[RFC3555\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::JPEG;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"JPEG\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/JPEG\");"]
    #[doc = r" ```"]
    pub const JPEG: Mime = Mime {
        ttype: "video",
        subtype: "JPEG",
    };
    #[doc = "\\[RFC5371\\]\\[RFC5372\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::JPEG2000;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"jpeg2000\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/jpeg2000\");"]
    #[doc = r" ```"]
    pub const JPEG2000: Mime = Mime {
        ttype: "video",
        subtype: "jpeg2000",
    };
    #[doc = "\\[RFC9134\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::JXSV;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"jxsv\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/jxsv\");"]
    #[doc = r" ```"]
    pub const JXSV: Mime = Mime {
        ttype: "video",
        subtype: "jxsv",
    };
    #[doc = "\\[RFC-ietf-cellar-matroska-21\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MATROSKA;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"matroska\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/matroska\");"]
    #[doc = r" ```"]
    pub const MATROSKA: Mime = Mime {
        ttype: "video",
        subtype: "matroska",
    };
    #[doc = "\\[RFC-ietf-cellar-matroska-21\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MATROSKA_3D;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"matroska-3d\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/matroska-3d\");"]
    #[doc = r" ```"]
    pub const MATROSKA_3D: Mime = Mime {
        ttype: "video",
        subtype: "matroska-3d",
    };
    #[doc = "\\[RFC3745\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MJ2;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"mj2\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/mj2\");"]
    #[doc = r" ```"]
    pub const MJ2: Mime = Mime {
        ttype: "video",
        subtype: "mj2",
    };
    #[doc = "\\[RFC3555\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MP1S;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"MP1S\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/MP1S\");"]
    #[doc = r" ```"]
    pub const MP1S: Mime = Mime {
        ttype: "video",
        subtype: "MP1S",
    };
    #[doc = "\\[RFC3555\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MP2P;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"MP2P\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/MP2P\");"]
    #[doc = r" ```"]
    pub const MP2P: Mime = Mime {
        ttype: "video",
        subtype: "MP2P",
    };
    #[doc = "\\[RFC3555\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MP2T;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"MP2T\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/MP2T\");"]
    #[doc = r" ```"]
    pub const MP2T: Mime = Mime {
        ttype: "video",
        subtype: "MP2T",
    };
    #[doc = "\\[RFC4337\\]\\[RFC6381\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MP4;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"mp4\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/mp4\");"]
    #[doc = r" ```"]
    pub const MP4: Mime = Mime {
        ttype: "video",
        subtype: "mp4",
    };
    #[doc = "\\[RFC6416\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MP4V_ES;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"MP4V-ES\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/MP4V-ES\");"]
    #[doc = r" ```"]
    pub const MP4V_ES: Mime = Mime {
        ttype: "video",
        subtype: "MP4V-ES",
    };
    #[doc = "\\[RFC3555\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MPV;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"MPV\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/MPV\");"]
    #[doc = r" ```"]
    pub const MPV: Mime = Mime {
        ttype: "video",
        subtype: "MPV",
    };
    #[doc = "\\[RFC2045\\]\\[RFC2046\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MPEG;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"mpeg\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/mpeg\");"]
    #[doc = r" ```"]
    pub const MPEG: Mime = Mime {
        ttype: "video",
        subtype: "mpeg",
    };
    #[doc = "\\[RFC3640\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::MPEG4_GENERIC;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"mpeg4-generic\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/mpeg4-generic\");"]
    #[doc = r" ```"]
    pub const MPEG4_GENERIC: Mime = Mime {
        ttype: "video",
        subtype: "mpeg4-generic",
    };
    #[doc = "\\[RFC4856\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::NV;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"nv\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/nv\");"]
    #[doc = r" ```"]
    pub const NV: Mime = Mime {
        ttype: "video",
        subtype: "nv",
    };
    #[doc = "\\[RFC5334\\]\\[RFC7845\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::OGG;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"ogg\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/ogg\");"]
    #[doc = r" ```"]
    pub const OGG: Mime = Mime {
        ttype: "video",
        subtype: "ogg",
    };
    #[doc = "\\[RFC3009\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::PARITYFEC;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"parityfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/parityfec\");"]
    #[doc = r" ```"]
    pub const PARITYFEC: Mime = Mime {
        ttype: "video",
        subtype: "parityfec",
    };
    #[doc = "\\[RFC2862\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::POINTER;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"pointer\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/pointer\");"]
    #[doc = r" ```"]
    pub const POINTER: Mime = Mime {
        ttype: "video",
        subtype: "pointer",
    };
    #[doc = "\\[RFC6381\\]\\[Paul_Lindner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::QUICKTIME;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"quicktime\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/quicktime\");"]
    #[doc = r" ```"]
    pub const QUICKTIME: Mime = Mime {
        ttype: "video",
        subtype: "quicktime",
    };
    #[doc = "\\[RFC6682\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::RAPTORFEC;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"raptorfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/raptorfec\");"]
    #[doc = r" ```"]
    pub const RAPTORFEC: Mime = Mime {
        ttype: "video",
        subtype: "raptorfec",
    };
    #[doc = "\\[RFC4175\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::RAW;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"raw\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/raw\");"]
    #[doc = r" ```"]
    pub const RAW: Mime = Mime {
        ttype: "video",
        subtype: "raw",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::RTP_ENC_AESCM128;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"rtp-enc-aescm128\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/rtp-enc-aescm128\");"]
    #[doc = r" ```"]
    pub const RTP_ENC_AESCM128: Mime = Mime {
        ttype: "video",
        subtype: "rtp-enc-aescm128",
    };
    #[doc = "\\[RFC6849\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::RTPLOOPBACK;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"rtploopback\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/rtploopback\");"]
    #[doc = r" ```"]
    pub const RTPLOOPBACK: Mime = Mime {
        ttype: "video",
        subtype: "rtploopback",
    };
    #[doc = "\\[RFC4588\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::RTX;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"rtx\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/rtx\");"]
    #[doc = r" ```"]
    pub const RTX: Mime = Mime {
        ttype: "video",
        subtype: "rtx",
    };
    #[doc = "\\[RFC9607\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::SCIP;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"scip\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/scip\");"]
    #[doc = r" ```"]
    pub const SCIP: Mime = Mime {
        ttype: "video",
        subtype: "scip",
    };
    #[doc = "\\[RFC8331\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::SMPTE291;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"smpte291\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/smpte291\");"]
    #[doc = r" ```"]
    pub const SMPTE291: Mime = Mime {
        ttype: "video",
        subtype: "smpte291",
    };
    #[doc = "\\[RFC3497\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::SMPTE292M;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"SMPTE292M\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/SMPTE292M\");"]
    #[doc = r" ```"]
    pub const SMPTE292M: Mime = Mime {
        ttype: "video",
        subtype: "SMPTE292M",
    };
    #[doc = "\\[RFC5109\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::ULPFEC;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"ulpfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/ulpfec\");"]
    #[doc = r" ```"]
    pub const ULPFEC: Mime = Mime {
        ttype: "video",
        subtype: "ulpfec",
    };
    #[doc = "\\[RFC4425\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VC1;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vc1\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vc1\");"]
    #[doc = r" ```"]
    pub const VC1: Mime = Mime {
        ttype: "video",
        subtype: "vc1",
    };
    #[doc = "\\[RFC8450\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VC2;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vc2\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vc2\");"]
    #[doc = r" ```"]
    pub const VC2: Mime = Mime {
        ttype: "video",
        subtype: "vc2",
    };
    #[doc = "\\[Frank_Rottmann\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_CCTV;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.CCTV\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.CCTV\");"]
    #[doc = r" ```"]
    pub const VND_CCTV: Mime = Mime {
        ttype: "video",
        subtype: "vnd.CCTV",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_DECE_HD;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.hd\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.dece.hd\");"]
    #[doc = r" ```"]
    pub const VND_DECE_HD: Mime = Mime {
        ttype: "video",
        subtype: "vnd.dece.hd",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_DECE_MOBILE;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.mobile\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.dece.mobile\");"]
    #[doc = r" ```"]
    pub const VND_DECE_MOBILE: Mime = Mime {
        ttype: "video",
        subtype: "vnd.dece.mobile",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_DECE_MP4;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.mp4\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.dece.mp4\");"]
    #[doc = r" ```"]
    pub const VND_DECE_MP4: Mime = Mime {
        ttype: "video",
        subtype: "vnd.dece.mp4",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_DECE_PD;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.pd\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.dece.pd\");"]
    #[doc = r" ```"]
    pub const VND_DECE_PD: Mime = Mime {
        ttype: "video",
        subtype: "vnd.dece.pd",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_DECE_SD;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.sd\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.dece.sd\");"]
    #[doc = r" ```"]
    pub const VND_DECE_SD: Mime = Mime {
        ttype: "video",
        subtype: "vnd.dece.sd",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_DECE_VIDEO;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.video\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.dece.video\");"]
    #[doc = r" ```"]
    pub const VND_DECE_VIDEO: Mime = Mime {
        ttype: "video",
        subtype: "vnd.dece.video",
    };
    #[doc = "\\[Nathan_Zerbe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_DIRECTV_MPEG;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.directv.mpeg\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.directv.mpeg\");"]
    #[doc = r" ```"]
    pub const VND_DIRECTV_MPEG: Mime = Mime {
        ttype: "video",
        subtype: "vnd.directv.mpeg",
    };
    #[doc = "\\[Nathan_Zerbe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_DIRECTV_MPEG_TTS;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.directv.mpeg-tts\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.directv.mpeg-tts\");"]
    #[doc = r" ```"]
    pub const VND_DIRECTV_MPEG_TTS: Mime = Mime {
        ttype: "video",
        subtype: "vnd.directv.mpeg-tts",
    };
    #[doc = "\\[Edwin_Heredia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_DLNA_MPEG_TTS;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dlna.mpeg-tts\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.dlna.mpeg-tts\");"]
    #[doc = r" ```"]
    pub const VND_DLNA_MPEG_TTS: Mime = Mime {
        ttype: "video",
        subtype: "vnd.dlna.mpeg-tts",
    };
    #[doc = "\\[Peter_Siebert\\]\\[Kevin_Murray\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_DVB_FILE;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.file\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.dvb.file\");"]
    #[doc = r" ```"]
    pub const VND_DVB_FILE: Mime = Mime {
        ttype: "video",
        subtype: "vnd.dvb.file",
    };
    #[doc = "\\[Arild_Fuldseth\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_FVT;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fvt\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.fvt\");"]
    #[doc = r" ```"]
    pub const VND_FVT: Mime = Mime {
        ttype: "video",
        subtype: "vnd.fvt",
    };
    #[doc = "\\[Swaminathan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_HNS_VIDEO;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hns.video\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.hns.video\");"]
    #[doc = r" ```"]
    pub const VND_HNS_VIDEO: Mime = Mime {
        ttype: "video",
        subtype: "vnd.hns.video",
    };
    #[doc = "\\[Shuji_Nakamura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_IPTVFORUM_1DPARITYFEC_1010;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptvforum.1dparityfec-1010\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.iptvforum.1dparityfec-1010\");"]
    #[doc = r" ```"]
    pub const VND_IPTVFORUM_1DPARITYFEC_1010: Mime = Mime {
        ttype: "video",
        subtype: "vnd.iptvforum.1dparityfec-1010",
    };
    #[doc = "\\[Shuji_Nakamura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_IPTVFORUM_1DPARITYFEC_2005;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptvforum.1dparityfec-2005\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.iptvforum.1dparityfec-2005\");"]
    #[doc = r" ```"]
    pub const VND_IPTVFORUM_1DPARITYFEC_2005: Mime = Mime {
        ttype: "video",
        subtype: "vnd.iptvforum.1dparityfec-2005",
    };
    #[doc = "\\[Shuji_Nakamura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_IPTVFORUM_2DPARITYFEC_1010;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptvforum.2dparityfec-1010\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.iptvforum.2dparityfec-1010\");"]
    #[doc = r" ```"]
    pub const VND_IPTVFORUM_2DPARITYFEC_1010: Mime = Mime {
        ttype: "video",
        subtype: "vnd.iptvforum.2dparityfec-1010",
    };
    #[doc = "\\[Shuji_Nakamura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_IPTVFORUM_2DPARITYFEC_2005;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptvforum.2dparityfec-2005\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.iptvforum.2dparityfec-2005\");"]
    #[doc = r" ```"]
    pub const VND_IPTVFORUM_2DPARITYFEC_2005: Mime = Mime {
        ttype: "video",
        subtype: "vnd.iptvforum.2dparityfec-2005",
    };
    #[doc = "\\[Shuji_Nakamura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_IPTVFORUM_TTSAVC;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptvforum.ttsavc\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.iptvforum.ttsavc\");"]
    #[doc = r" ```"]
    pub const VND_IPTVFORUM_TTSAVC: Mime = Mime {
        ttype: "video",
        subtype: "vnd.iptvforum.ttsavc",
    };
    #[doc = "\\[Shuji_Nakamura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_IPTVFORUM_TTSMPEG2;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptvforum.ttsmpeg2\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.iptvforum.ttsmpeg2\");"]
    #[doc = r" ```"]
    pub const VND_IPTVFORUM_TTSMPEG2: Mime = Mime {
        ttype: "video",
        subtype: "vnd.iptvforum.ttsmpeg2",
    };
    #[doc = "\\[Tom_McGinty\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_MOTOROLA_VIDEO;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.video\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.motorola.video\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_VIDEO: Mime = Mime {
        ttype: "video",
        subtype: "vnd.motorola.video",
    };
    #[doc = "\\[Tom_McGinty\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_MOTOROLA_VIDEOP;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.videop\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.motorola.videop\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_VIDEOP: Mime = Mime {
        ttype: "video",
        subtype: "vnd.motorola.videop",
    };
    #[doc = "\\[Heiko_Recktenwald\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_MPEGURL;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mpegurl\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.mpegurl\");"]
    #[doc = r" ```"]
    pub const VND_MPEGURL: Mime = Mime {
        ttype: "video",
        subtype: "vnd.mpegurl",
    };
    #[doc = "\\[Steve_DiAcetis\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_MS_PLAYREADY_MEDIA_PYV;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-playready.media.pyv\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.ms-playready.media.pyv\");"]
    #[doc = r" ```"]
    pub const VND_MS_PLAYREADY_MEDIA_PYV: Mime = Mime {
        ttype: "video",
        subtype: "vnd.ms-playready.media.pyv",
    };
    #[doc = "\\[Petteri_Kangaslampi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_NOKIA_INTERLEAVED_MULTIMEDIA;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.interleaved-multimedia\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.nokia.interleaved-multimedia\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_INTERLEAVED_MULTIMEDIA: Mime = Mime {
        ttype: "video",
        subtype: "vnd.nokia.interleaved-multimedia",
    };
    #[doc = "\\[Miska_M._Hannuksela\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_NOKIA_MP4VR;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.mp4vr\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.nokia.mp4vr\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_MP4VR: Mime = Mime {
        ttype: "video",
        subtype: "vnd.nokia.mp4vr",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_NOKIA_VIDEOVOIP;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.videovoip\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.nokia.videovoip\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_VIDEOVOIP: Mime = Mime {
        ttype: "video",
        subtype: "vnd.nokia.videovoip",
    };
    #[doc = "\\[John_Clark\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_OBJECTVIDEO;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.objectvideo\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.objectvideo\");"]
    #[doc = r" ```"]
    pub const VND_OBJECTVIDEO: Mime = Mime {
        ttype: "video",
        subtype: "vnd.objectvideo",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_RADGAMETTOOLS_BINK;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radgamettools.bink\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.radgamettools.bink\");"]
    #[doc = r" ```"]
    pub const VND_RADGAMETTOOLS_BINK: Mime = Mime {
        ttype: "video",
        subtype: "vnd.radgamettools.bink",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_RADGAMETTOOLS_SMACKER;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radgamettools.smacker\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.radgamettools.smacker\");"]
    #[doc = r" ```"]
    pub const VND_RADGAMETTOOLS_SMACKER: Mime = Mime {
        ttype: "video",
        subtype: "vnd.radgamettools.smacker",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_SEALED_MPEG1;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.mpeg1\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.sealed.mpeg1\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_MPEG1: Mime = Mime {
        ttype: "video",
        subtype: "vnd.sealed.mpeg1",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_SEALED_MPEG4;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.mpeg4\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.sealed.mpeg4\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_MPEG4: Mime = Mime {
        ttype: "video",
        subtype: "vnd.sealed.mpeg4",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_SEALED_SWF;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.swf\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.sealed.swf\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_SWF: Mime = Mime {
        ttype: "video",
        subtype: "vnd.sealed.swf",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_SEALEDMEDIA_SOFTSEAL_MOV;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealedmedia.softseal.mov\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.sealedmedia.softseal.mov\");"]
    #[doc = r" ```"]
    pub const VND_SEALEDMEDIA_SOFTSEAL_MOV: Mime = Mime {
        ttype: "video",
        subtype: "vnd.sealedmedia.softseal.mov",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_UVVU_MP4;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uvvu.mp4\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.uvvu.mp4\");"]
    #[doc = r" ```"]
    pub const VND_UVVU_MP4: Mime = Mime {
        ttype: "video",
        subtype: "vnd.uvvu.mp4",
    };
    #[doc = "\\[Google\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_YOUTUBE_YT;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.youtube.yt\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.youtube.yt\");"]
    #[doc = r" ```"]
    pub const VND_YOUTUBE_YT: Mime = Mime {
        ttype: "video",
        subtype: "vnd.youtube.yt",
    };
    #[doc = "\\[John_Wolfe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VND_VIVO;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.vivo\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/vnd.vivo\");"]
    #[doc = r" ```"]
    pub const VND_VIVO: Mime = Mime {
        ttype: "video",
        subtype: "vnd.vivo",
    };
    #[doc = "\\[RFC7741\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VP8;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"VP8\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/VP8\");"]
    #[doc = r" ```"]
    pub const VP8: Mime = Mime {
        ttype: "video",
        subtype: "VP8",
    };
    #[doc = "\\[RFC-ietf-payload-vp9-16\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::video::VP9;"]
    #[doc = "assert_eq!(media.ttype, \"video\");"]
    #[doc = "assert_eq!(media.subtype, \"VP9\");"]
    #[doc = "assert_eq!(media.to_string(), \"video/VP9\");"]
    #[doc = r" ```"]
    pub const VP9: Mime = Mime {
        ttype: "video",
        subtype: "VP9",
    };
}
