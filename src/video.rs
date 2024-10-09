use super::Mime;
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
