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
