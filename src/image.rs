use super::Mime;
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
