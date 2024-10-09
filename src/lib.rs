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
pub mod application {
    use super::*;
    #[doc = "\\[RFC6015\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::_1D_INTERLEAVED_PARITYFEC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"1d-interleaved-parityfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/1d-interleaved-parityfec\");"]
    #[doc = r" ```"]
    pub const _1D_INTERLEAVED_PARITYFEC: Mime = Mime {
        ttype: "application",
        subtype: "1d-interleaved-parityfec",
    };
    #[doc = "\\[_3GPP\\]\\[Ozgur_Oyman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::_3GPDASH_QOE_REPORT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"3gpdash-qoe-report+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/3gpdash-qoe-report+xml\");"]
    #[doc = r" ```"]
    pub const _3GPDASH_QOE_REPORT_XML: Mime = Mime {
        ttype: "application",
        subtype: "3gpdash-qoe-report+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Ulrich_Wiehe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::_3GPPHAL_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"3gppHal+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/3gppHal+json\");"]
    #[doc = r" ```"]
    pub const _3GPPHAL_JSON: Mime = Mime {
        ttype: "application",
        subtype: "3gppHal+json",
    };
    #[doc = "\\[_3GPP\\]\\[Ulrich_Wiehe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::_3GPPHALFORMS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"3gppHalForms+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/3gppHalForms+json\");"]
    #[doc = r" ```"]
    pub const _3GPPHALFORMS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "3gppHalForms+json",
    };
    #[doc = "\\[_3GPP\\]\\[John_M_Meredith\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::_3GPP_IMS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"3gpp-ims+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/3gpp-ims+xml\");"]
    #[doc = r" ```"]
    pub const _3GPP_IMS_XML: Mime = Mime {
        ttype: "application",
        subtype: "3gpp-ims+xml",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::A2L;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"A2L\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/A2L\");"]
    #[doc = r" ```"]
    pub const A2L: Mime = Mime {
        ttype: "application",
        subtype: "A2L",
    };
    #[doc = "\\[RFC9594\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ACE_GROUPCOMM_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ace-groupcomm+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ace-groupcomm+cbor\");"]
    #[doc = r" ```"]
    pub const ACE_GROUPCOMM_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "ace-groupcomm+cbor",
    };
    #[doc = "\\[RFC9200\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ACE_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ace+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ace+cbor\");"]
    #[doc = r" ```"]
    pub const ACE_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "ace+cbor",
    };
    #[doc = "\\[RFC9431\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ACE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ace+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ace+json\");"]
    #[doc = r" ```"]
    pub const ACE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "ace+json",
    };
    #[doc = "\\[Ehud_Shapiro\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ACTIVEMESSAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"activemessage\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/activemessage\");"]
    #[doc = r" ```"]
    pub const ACTIVEMESSAGE: Mime = Mime {
        ttype: "application",
        subtype: "activemessage",
    };
    #[doc = "\\[W3C\\]\\[Benjamin_Goering\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ACTIVITY_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"activity+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/activity+json\");"]
    #[doc = r" ```"]
    pub const ACTIVITY_JSON: Mime = Mime {
        ttype: "application",
        subtype: "activity+json",
    };
    #[doc = "\\[RFC9237\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::AIF_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"aif+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/aif+cbor\");"]
    #[doc = r" ```"]
    pub const AIF_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "aif+cbor",
    };
    #[doc = "\\[RFC9237\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::AIF_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"aif+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/aif+json\");"]
    #[doc = r" ```"]
    pub const AIF_JSON: Mime = Mime {
        ttype: "application",
        subtype: "aif+json",
    };
    #[doc = "\\[RFC9241\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_CDNI_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-cdni+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-cdni+json\");"]
    #[doc = r" ```"]
    pub const ALTO_CDNI_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-cdni+json",
    };
    #[doc = "\\[RFC9241\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_CDNIFILTER_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-cdnifilter+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-cdnifilter+json\");"]
    #[doc = r" ```"]
    pub const ALTO_CDNIFILTER_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-cdnifilter+json",
    };
    #[doc = "\\[RFC7285\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_COSTMAP_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-costmap+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-costmap+json\");"]
    #[doc = r" ```"]
    pub const ALTO_COSTMAP_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-costmap+json",
    };
    #[doc = "\\[RFC7285\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_COSTMAPFILTER_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-costmapfilter+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-costmapfilter+json\");"]
    #[doc = r" ```"]
    pub const ALTO_COSTMAPFILTER_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-costmapfilter+json",
    };
    #[doc = "\\[RFC7285\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_DIRECTORY_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-directory+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-directory+json\");"]
    #[doc = r" ```"]
    pub const ALTO_DIRECTORY_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-directory+json",
    };
    #[doc = "\\[RFC7285\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_ENDPOINTPROP_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-endpointprop+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-endpointprop+json\");"]
    #[doc = r" ```"]
    pub const ALTO_ENDPOINTPROP_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-endpointprop+json",
    };
    #[doc = "\\[RFC7285\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_ENDPOINTPROPPARAMS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-endpointpropparams+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-endpointpropparams+json\");"]
    #[doc = r" ```"]
    pub const ALTO_ENDPOINTPROPPARAMS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-endpointpropparams+json",
    };
    #[doc = "\\[RFC7285\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_ENDPOINTCOST_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-endpointcost+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-endpointcost+json\");"]
    #[doc = r" ```"]
    pub const ALTO_ENDPOINTCOST_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-endpointcost+json",
    };
    #[doc = "\\[RFC7285\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_ENDPOINTCOSTPARAMS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-endpointcostparams+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-endpointcostparams+json\");"]
    #[doc = r" ```"]
    pub const ALTO_ENDPOINTCOSTPARAMS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-endpointcostparams+json",
    };
    #[doc = "\\[RFC7285\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_ERROR_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-error+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-error+json\");"]
    #[doc = r" ```"]
    pub const ALTO_ERROR_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-error+json",
    };
    #[doc = "\\[RFC7285\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_NETWORKMAPFILTER_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-networkmapfilter+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-networkmapfilter+json\");"]
    #[doc = r" ```"]
    pub const ALTO_NETWORKMAPFILTER_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-networkmapfilter+json",
    };
    #[doc = "\\[RFC7285\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_NETWORKMAP_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-networkmap+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-networkmap+json\");"]
    #[doc = r" ```"]
    pub const ALTO_NETWORKMAP_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-networkmap+json",
    };
    #[doc = "\\[RFC9240\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_PROPMAP_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-propmap+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-propmap+json\");"]
    #[doc = r" ```"]
    pub const ALTO_PROPMAP_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-propmap+json",
    };
    #[doc = "\\[RFC9240\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_PROPMAPPARAMS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-propmapparams+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-propmapparams+json\");"]
    #[doc = r" ```"]
    pub const ALTO_PROPMAPPARAMS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-propmapparams+json",
    };
    #[doc = "\\[RFC9569\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_TIPS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-tips+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-tips+json\");"]
    #[doc = r" ```"]
    pub const ALTO_TIPS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-tips+json",
    };
    #[doc = "\\[RFC9569\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_TIPSPARAMS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-tipsparams+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-tipsparams+json\");"]
    #[doc = r" ```"]
    pub const ALTO_TIPSPARAMS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-tipsparams+json",
    };
    #[doc = "\\[RFC8895\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_UPDATESTREAMCONTROL_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-updatestreamcontrol+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-updatestreamcontrol+json\");"]
    #[doc = r" ```"]
    pub const ALTO_UPDATESTREAMCONTROL_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-updatestreamcontrol+json",
    };
    #[doc = "\\[RFC8895\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ALTO_UPDATESTREAMPARAMS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"alto-updatestreamparams+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/alto-updatestreamparams+json\");"]
    #[doc = r" ```"]
    pub const ALTO_UPDATESTREAMPARAMS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "alto-updatestreamparams+json",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::AML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"AML\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/AML\");"]
    #[doc = r" ```"]
    pub const AML: Mime = Mime {
        ttype: "application",
        subtype: "AML",
    };
    #[doc = "\\[Nathaniel_Borenstein\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ANDREW_INSET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"andrew-inset\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/andrew-inset\");"]
    #[doc = r" ```"]
    pub const ANDREW_INSET: Mime = Mime {
        ttype: "application",
        subtype: "andrew-inset",
    };
    #[doc = "\\[Patrik_Faltstrom\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::APPLEFILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"applefile\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/applefile\");"]
    #[doc = r" ```"]
    pub const APPLEFILE: Mime = Mime {
        ttype: "application",
        subtype: "applefile",
    };
    #[doc = "\\[RFC9068\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::AT_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"at+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/at+jwt\");"]
    #[doc = r" ```"]
    pub const AT_JWT: Mime = Mime {
        ttype: "application",
        subtype: "at+jwt",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ATF\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ATF\");"]
    #[doc = r" ```"]
    pub const ATF: Mime = Mime {
        ttype: "application",
        subtype: "ATF",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATFX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ATFX\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ATFX\");"]
    #[doc = r" ```"]
    pub const ATFX: Mime = Mime {
        ttype: "application",
        subtype: "ATFX",
    };
    #[doc = "\\[RFC4287\\]\\[RFC5023\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATOM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"atom+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/atom+xml\");"]
    #[doc = r" ```"]
    pub const ATOM_XML: Mime = Mime {
        ttype: "application",
        subtype: "atom+xml",
    };
    #[doc = "\\[RFC5023\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATOMCAT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"atomcat+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/atomcat+xml\");"]
    #[doc = r" ```"]
    pub const ATOMCAT_XML: Mime = Mime {
        ttype: "application",
        subtype: "atomcat+xml",
    };
    #[doc = "\\[RFC6721\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATOMDELETED_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"atomdeleted+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/atomdeleted+xml\");"]
    #[doc = r" ```"]
    pub const ATOMDELETED_XML: Mime = Mime {
        ttype: "application",
        subtype: "atomdeleted+xml",
    };
    #[doc = "\\[Nathaniel_Borenstein\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATOMICMAIL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"atomicmail\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/atomicmail\");"]
    #[doc = r" ```"]
    pub const ATOMICMAIL: Mime = Mime {
        ttype: "application",
        subtype: "atomicmail",
    };
    #[doc = "\\[RFC5023\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATOMSVC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"atomsvc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/atomsvc+xml\");"]
    #[doc = r" ```"]
    pub const ATOMSVC_XML: Mime = Mime {
        ttype: "application",
        subtype: "atomsvc+xml",
    };
    #[doc = "\\[ATSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATSC_DWD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"atsc-dwd+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/atsc-dwd+xml\");"]
    #[doc = r" ```"]
    pub const ATSC_DWD_XML: Mime = Mime {
        ttype: "application",
        subtype: "atsc-dwd+xml",
    };
    #[doc = "\\[ATSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATSC_DYNAMIC_EVENT_MESSAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"atsc-dynamic-event-message\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/atsc-dynamic-event-message\");"]
    #[doc = r" ```"]
    pub const ATSC_DYNAMIC_EVENT_MESSAGE: Mime = Mime {
        ttype: "application",
        subtype: "atsc-dynamic-event-message",
    };
    #[doc = "\\[ATSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATSC_HELD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"atsc-held+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/atsc-held+xml\");"]
    #[doc = r" ```"]
    pub const ATSC_HELD_XML: Mime = Mime {
        ttype: "application",
        subtype: "atsc-held+xml",
    };
    #[doc = "\\[ATSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATSC_RDT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"atsc-rdt+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/atsc-rdt+json\");"]
    #[doc = r" ```"]
    pub const ATSC_RDT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "atsc-rdt+json",
    };
    #[doc = "\\[ATSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATSC_RSAT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"atsc-rsat+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/atsc-rsat+xml\");"]
    #[doc = r" ```"]
    pub const ATSC_RSAT_XML: Mime = Mime {
        ttype: "application",
        subtype: "atsc-rsat+xml",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ATXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ATXML\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ATXML\");"]
    #[doc = r" ```"]
    pub const ATXML: Mime = Mime {
        ttype: "application",
        subtype: "ATXML",
    };
    #[doc = "\\[RFC4745\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::AUTH_POLICY_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"auth-policy+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/auth-policy+xml\");"]
    #[doc = r" ```"]
    pub const AUTH_POLICY_XML: Mime = Mime {
        ttype: "application",
        subtype: "auth-policy+xml",
    };
    #[doc = "\\[AutomationML_e.V.\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::AUTOMATIONML_AML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"automationml-aml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/automationml-aml+xml\");"]
    #[doc = r" ```"]
    pub const AUTOMATIONML_AML_XML: Mime = Mime {
        ttype: "application",
        subtype: "automationml-aml+xml",
    };
    #[doc = "\\[AutomationML_e.V.\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::AUTOMATIONML_AMLX_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"automationml-amlx+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/automationml-amlx+zip\");"]
    #[doc = r" ```"]
    pub const AUTOMATIONML_AMLX_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "automationml-amlx+zip",
    };
    #[doc = "\\[ASHRAE\\]\\[Dave_Robin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::BACNET_XDD_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"bacnet-xdd+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/bacnet-xdd+zip\");"]
    #[doc = r" ```"]
    pub const BACNET_XDD_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "bacnet-xdd+zip",
    };
    #[doc = "\\[RFC2442\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::BATCH_SMTP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"batch-SMTP\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/batch-SMTP\");"]
    #[doc = r" ```"]
    pub const BATCH_SMTP: Mime = Mime {
        ttype: "application",
        subtype: "batch-SMTP",
    };
    #[doc = "\\[RFC3080\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::BEEP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"beep+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/beep+xml\");"]
    #[doc = r" ```"]
    pub const BEEP_XML: Mime = Mime {
        ttype: "application",
        subtype: "beep+xml",
    };
    #[doc = "\\[World_Meterological_Organization\\]\\[Anna_Milan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::BUFR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"bufr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/bufr\");"]
    #[doc = r" ```"]
    pub const BUFR: Mime = Mime {
        ttype: "application",
        subtype: "bufr",
    };
    #[doc = "\\[C2PA\\]\\[Leonard_Rosenthol\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::C2PA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"c2pa\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/c2pa\");"]
    #[doc = r" ```"]
    pub const C2PA: Mime = Mime {
        ttype: "application",
        subtype: "c2pa",
    };
    #[doc = "\\[RFC7265\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CALENDAR_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"calendar+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/calendar+json\");"]
    #[doc = r" ```"]
    pub const CALENDAR_JSON: Mime = Mime {
        ttype: "application",
        subtype: "calendar+json",
    };
    #[doc = "\\[RFC6321\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CALENDAR_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"calendar+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/calendar+xml\");"]
    #[doc = r" ```"]
    pub const CALENDAR_XML: Mime = Mime {
        ttype: "application",
        subtype: "calendar+xml",
    };
    #[doc = "\\[RFC6910\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CALL_COMPLETION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"call-completion\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/call-completion\");"]
    #[doc = r" ```"]
    pub const CALL_COMPLETION: Mime = Mime {
        ttype: "application",
        subtype: "call-completion",
    };
    #[doc = "\\[RFC1895\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CALS_1840;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"CALS-1840\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/CALS-1840\");"]
    #[doc = r" ```"]
    pub const CALS_1840: Mime = Mime {
        ttype: "application",
        subtype: "CALS-1840",
    };
    #[doc = "\\[RFC8908\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CAPTIVE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"captive+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/captive+json\");"]
    #[doc = r" ```"]
    pub const CAPTIVE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "captive+json",
    };
    #[doc = "\\[RFC8949\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cbor\");"]
    #[doc = r" ```"]
    pub const CBOR: Mime = Mime {
        ttype: "application",
        subtype: "cbor",
    };
    #[doc = "\\[RFC8742\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CBOR_SEQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cbor-seq\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cbor-seq\");"]
    #[doc = r" ```"]
    pub const CBOR_SEQ: Mime = Mime {
        ttype: "application",
        subtype: "cbor-seq",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CCCEX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cccex\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cccex\");"]
    #[doc = r" ```"]
    pub const CCCEX: Mime = Mime {
        ttype: "application",
        subtype: "cccex",
    };
    #[doc = "\\[RFC6503\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CCMP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ccmp+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ccmp+xml\");"]
    #[doc = r" ```"]
    pub const CCMP_XML: Mime = Mime {
        ttype: "application",
        subtype: "ccmp+xml",
    };
    #[doc = "\\[RFC4267\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CCXML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ccxml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ccxml+xml\");"]
    #[doc = r" ```"]
    pub const CCXML_XML: Mime = Mime {
        ttype: "application",
        subtype: "ccxml+xml",
    };
    #[doc = "\\[HL7\\]\\[Marc_Duteau\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CDA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cda+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cda+xml\");"]
    #[doc = r" ```"]
    pub const CDA_XML: Mime = Mime {
        ttype: "application",
        subtype: "cda+xml",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CDFX_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"CDFX+XML\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/CDFX+XML\");"]
    #[doc = r" ```"]
    pub const CDFX_XML: Mime = Mime {
        ttype: "application",
        subtype: "CDFX+XML",
    };
    #[doc = "\\[RFC6208\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CDMI_CAPABILITY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cdmi-capability\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cdmi-capability\");"]
    #[doc = r" ```"]
    pub const CDMI_CAPABILITY: Mime = Mime {
        ttype: "application",
        subtype: "cdmi-capability",
    };
    #[doc = "\\[RFC6208\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CDMI_CONTAINER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cdmi-container\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cdmi-container\");"]
    #[doc = r" ```"]
    pub const CDMI_CONTAINER: Mime = Mime {
        ttype: "application",
        subtype: "cdmi-container",
    };
    #[doc = "\\[RFC6208\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CDMI_DOMAIN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cdmi-domain\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cdmi-domain\");"]
    #[doc = r" ```"]
    pub const CDMI_DOMAIN: Mime = Mime {
        ttype: "application",
        subtype: "cdmi-domain",
    };
    #[doc = "\\[RFC6208\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CDMI_OBJECT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cdmi-object\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cdmi-object\");"]
    #[doc = r" ```"]
    pub const CDMI_OBJECT: Mime = Mime {
        ttype: "application",
        subtype: "cdmi-object",
    };
    #[doc = "\\[RFC6208\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CDMI_QUEUE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cdmi-queue\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cdmi-queue\");"]
    #[doc = r" ```"]
    pub const CDMI_QUEUE: Mime = Mime {
        ttype: "application",
        subtype: "cdmi-queue",
    };
    #[doc = "\\[RFC7736\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CDNI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cdni\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cdni\");"]
    #[doc = r" ```"]
    pub const CDNI: Mime = Mime {
        ttype: "application",
        subtype: "cdni",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CEA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"CEA\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/CEA\");"]
    #[doc = r" ```"]
    pub const CEA: Mime = Mime {
        ttype: "application",
        subtype: "CEA",
    };
    #[doc = "\\[Gottfried_Zimmermann\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CEA_2018_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cea-2018+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cea-2018+xml\");"]
    #[doc = r" ```"]
    pub const CEA_2018_XML: Mime = Mime {
        ttype: "application",
        subtype: "cea-2018+xml",
    };
    #[doc = "\\[RFC4708\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CELLML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cellml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cellml+xml\");"]
    #[doc = r" ```"]
    pub const CELLML_XML: Mime = Mime {
        ttype: "application",
        subtype: "cellml+xml",
    };
    #[doc = "\\[RFC6230\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CFW;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cfw\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cfw\");"]
    #[doc = r" ```"]
    pub const CFW: Mime = Mime {
        ttype: "application",
        subtype: "cfw",
    };
    #[doc = "\\[RFC9528\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CID_EDHOC_CBOR_SEQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cid-edhoc+cbor-seq\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cid-edhoc+cbor-seq\");"]
    #[doc = r" ```"]
    pub const CID_EDHOC_CBOR_SEQ: Mime = Mime {
        ttype: "application",
        subtype: "cid-edhoc+cbor-seq",
    };
    #[doc = "\\[OGC\\]\\[Scott_Simmons\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CITY_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"city+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/city+json\");"]
    #[doc = r" ```"]
    pub const CITY_JSON: Mime = Mime {
        ttype: "application",
        subtype: "city+json",
    };
    #[doc = "\\[IMS_Global\\]\\[Andy_Miller\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CLR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"clr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/clr\");"]
    #[doc = r" ```"]
    pub const CLR: Mime = Mime {
        ttype: "application",
        subtype: "clr",
    };
    #[doc = "\\[RFC8846\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CLUE_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"clue_info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/clue_info+xml\");"]
    #[doc = r" ```"]
    pub const CLUE_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "clue_info+xml",
    };
    #[doc = "\\[RFC8847\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CLUE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"clue+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/clue+xml\");"]
    #[doc = r" ```"]
    pub const CLUE_XML: Mime = Mime {
        ttype: "application",
        subtype: "clue+xml",
    };
    #[doc = "\\[RFC7193\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CMS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cms\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cms\");"]
    #[doc = r" ```"]
    pub const CMS: Mime = Mime {
        ttype: "application",
        subtype: "cms",
    };
    #[doc = "\\[RFC3367\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CNRP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cnrp+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cnrp+xml\");"]
    #[doc = r" ```"]
    pub const CNRP_XML: Mime = Mime {
        ttype: "application",
        subtype: "cnrp+xml",
    };
    #[doc = "\\[RFC7390\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::COAP_GROUP_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"coap-group+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/coap-group+json\");"]
    #[doc = r" ```"]
    pub const COAP_GROUP_JSON: Mime = Mime {
        ttype: "application",
        subtype: "coap-group+json",
    };
    #[doc = "\\[RFC8075\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::COAP_PAYLOAD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"coap-payload\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/coap-payload\");"]
    #[doc = r" ```"]
    pub const COAP_PAYLOAD: Mime = Mime {
        ttype: "application",
        subtype: "coap-payload",
    };
    #[doc = "\\[David_Glazer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::COMMONGROUND;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"commonground\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/commonground\");"]
    #[doc = r" ```"]
    pub const COMMONGROUND: Mime = Mime {
        ttype: "application",
        subtype: "commonground",
    };
    #[doc = "\\[RFC9290, Section 6.3\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CONCISE_PROBLEM_DETAILS_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"concise-problem-details+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/concise-problem-details+cbor\");"]
    #[doc = r" ```"]
    pub const CONCISE_PROBLEM_DETAILS_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "concise-problem-details+cbor",
    };
    #[doc = "\\[RFC4575\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CONFERENCE_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"conference-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/conference-info+xml\");"]
    #[doc = r" ```"]
    pub const CONFERENCE_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "conference-info+xml",
    };
    #[doc = "\\[RFC3880\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CPL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cpl+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cpl+xml\");"]
    #[doc = r" ```"]
    pub const CPL_XML: Mime = Mime {
        ttype: "application",
        subtype: "cpl+xml",
    };
    #[doc = "\\[RFC9052\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::COSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cose\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cose\");"]
    #[doc = r" ```"]
    pub const COSE: Mime = Mime {
        ttype: "application",
        subtype: "cose",
    };
    #[doc = "\\[RFC9052\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::COSE_KEY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cose-key\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cose-key\");"]
    #[doc = r" ```"]
    pub const COSE_KEY: Mime = Mime {
        ttype: "application",
        subtype: "cose-key",
    };
    #[doc = "\\[RFC9052\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::COSE_KEY_SET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cose-key-set\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cose-key-set\");"]
    #[doc = r" ```"]
    pub const COSE_KEY_SET: Mime = Mime {
        ttype: "application",
        subtype: "cose-key-set",
    };
    #[doc = "\\[RFC9360\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::COSE_X509;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cose-x509\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cose-x509\");"]
    #[doc = r" ```"]
    pub const COSE_X509: Mime = Mime {
        ttype: "application",
        subtype: "cose-x509",
    };
    #[doc = "\\[RFC7030\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CSRATTRS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"csrattrs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/csrattrs\");"]
    #[doc = r" ```"]
    pub const CSRATTRS: Mime = Mime {
        ttype: "application",
        subtype: "csrattrs",
    };
    #[doc = "\\[Ecma_International_Helpdesk\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CSTA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"csta+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/csta+xml\");"]
    #[doc = r" ```"]
    pub const CSTA_XML: Mime = Mime {
        ttype: "application",
        subtype: "csta+xml",
    };
    #[doc = "\\[Ecma_International_Helpdesk\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CSTADATA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"CSTAdata+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/CSTAdata+xml\");"]
    #[doc = r" ```"]
    pub const CSTADATA_XML: Mime = Mime {
        ttype: "application",
        subtype: "CSTAdata+xml",
    };
    #[doc = "\\[W3C\\]\\[Ivan_Herman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CSVM_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"csvm+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/csvm+json\");"]
    #[doc = r" ```"]
    pub const CSVM_JSON: Mime = Mime {
        ttype: "application",
        subtype: "csvm+json",
    };
    #[doc = "\\[CWL_Project\\]\\[Michael_R._Crusoe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CWL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cwl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cwl\");"]
    #[doc = r" ```"]
    pub const CWL: Mime = Mime {
        ttype: "application",
        subtype: "cwl",
    };
    #[doc = "\\[CWL_Project\\]\\[Michael_R._Crusoe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CWL_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cwl+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cwl+json\");"]
    #[doc = r" ```"]
    pub const CWL_JSON: Mime = Mime {
        ttype: "application",
        subtype: "cwl+json",
    };
    #[doc = "\\[CWL_Project\\]\\[Michael_R._Crusoe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CWL_YAML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cwl+yaml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cwl+yaml\");"]
    #[doc = r" ```"]
    pub const CWL_YAML: Mime = Mime {
        ttype: "application",
        subtype: "cwl+yaml",
    };
    #[doc = "\\[RFC8392\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cwt\");"]
    #[doc = r" ```"]
    pub const CWT: Mime = Mime {
        ttype: "application",
        subtype: "cwt",
    };
    #[doc = "\\[Donald_E._Eastlake_3rd\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::CYBERCASH;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"cybercash\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/cybercash\");"]
    #[doc = r" ```"]
    pub const CYBERCASH: Mime = Mime {
        ttype: "application",
        subtype: "cybercash",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[Thomas_Stockhammer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DASH_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dash+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dash+xml\");"]
    #[doc = r" ```"]
    pub const DASH_XML: Mime = Mime {
        ttype: "application",
        subtype: "dash+xml",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DASH_PATCH_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dash-patch+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dash-patch+xml\");"]
    #[doc = r" ```"]
    pub const DASH_PATCH_XML: Mime = Mime {
        ttype: "application",
        subtype: "dash-patch+xml",
    };
    #[doc = "\\[David_Furbeck\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DASHDELTA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dashdelta\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dashdelta\");"]
    #[doc = r" ```"]
    pub const DASHDELTA: Mime = Mime {
        ttype: "application",
        subtype: "dashdelta",
    };
    #[doc = "\\[RFC4709\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DAVMOUNT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"davmount+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/davmount+xml\");"]
    #[doc = r" ```"]
    pub const DAVMOUNT_XML: Mime = Mime {
        ttype: "application",
        subtype: "davmount+xml",
    };
    #[doc = "\\[Larry_Campbell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DCA_RFT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dca-rft\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dca-rft\");"]
    #[doc = r" ```"]
    pub const DCA_RFT: Mime = Mime {
        ttype: "application",
        subtype: "dca-rft",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DCD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"DCD\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/DCD\");"]
    #[doc = r" ```"]
    pub const DCD: Mime = Mime {
        ttype: "application",
        subtype: "DCD",
    };
    #[doc = "\\[Larry_Campbell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DEC_DX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dec-dx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dec-dx\");"]
    #[doc = r" ```"]
    pub const DEC_DX: Mime = Mime {
        ttype: "application",
        subtype: "dec-dx",
    };
    #[doc = "\\[RFC4235\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DIALOG_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dialog-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dialog-info+xml\");"]
    #[doc = r" ```"]
    pub const DIALOG_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "dialog-info+xml",
    };
    #[doc = "\\[RFC3240\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DICOM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dicom\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dicom\");"]
    #[doc = r" ```"]
    pub const DICOM: Mime = Mime {
        ttype: "application",
        subtype: "dicom",
    };
    #[doc = "\\[DICOM_Standard_Committee\\]\\[David_Clunie\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DICOM_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dicom+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dicom+json\");"]
    #[doc = r" ```"]
    pub const DICOM_JSON: Mime = Mime {
        ttype: "application",
        subtype: "dicom+json",
    };
    #[doc = "\\[DICOM_Standard_Committee\\]\\[David_Clunie\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DICOM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dicom+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dicom+xml\");"]
    #[doc = r" ```"]
    pub const DICOM_XML: Mime = Mime {
        ttype: "application",
        subtype: "dicom+xml",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DII;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"DII\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/DII\");"]
    #[doc = r" ```"]
    pub const DII: Mime = Mime {
        ttype: "application",
        subtype: "DII",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DIT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"DIT\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/DIT\");"]
    #[doc = r" ```"]
    pub const DIT: Mime = Mime {
        ttype: "application",
        subtype: "DIT",
    };
    #[doc = "\\[RFC4027\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DNS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dns\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dns\");"]
    #[doc = r" ```"]
    pub const DNS: Mime = Mime {
        ttype: "application",
        subtype: "dns",
    };
    #[doc = "\\[RFC8427\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DNS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dns+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dns+json\");"]
    #[doc = r" ```"]
    pub const DNS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "dns+json",
    };
    #[doc = "\\[RFC8484\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DNS_MESSAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dns-message\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dns-message\");"]
    #[doc = r" ```"]
    pub const DNS_MESSAGE: Mime = Mime {
        ttype: "application",
        subtype: "dns-message",
    };
    #[doc = "\\[RFC9132\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DOTS_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dots+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dots+cbor\");"]
    #[doc = r" ```"]
    pub const DOTS_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "dots+cbor",
    };
    #[doc = "\\[RFC9449\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DPOP_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dpop+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dpop+jwt\");"]
    #[doc = r" ```"]
    pub const DPOP_JWT: Mime = Mime {
        ttype: "application",
        subtype: "dpop+jwt",
    };
    #[doc = "\\[RFC6063\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DSKPP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dskpp+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dskpp+xml\");"]
    #[doc = r" ```"]
    pub const DSKPP_XML: Mime = Mime {
        ttype: "application",
        subtype: "dskpp+xml",
    };
    #[doc = "\\[RFC5698\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DSSC_DER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dssc+der\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dssc+der\");"]
    #[doc = r" ```"]
    pub const DSSC_DER: Mime = Mime {
        ttype: "application",
        subtype: "dssc+der",
    };
    #[doc = "\\[RFC5698\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DSSC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dssc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dssc+xml\");"]
    #[doc = r" ```"]
    pub const DSSC_XML: Mime = Mime {
        ttype: "application",
        subtype: "dssc+xml",
    };
    #[doc = "\\[RFC3029\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::DVCS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"dvcs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/dvcs\");"]
    #[doc = r" ```"]
    pub const DVCS: Mime = Mime {
        ttype: "application",
        subtype: "dvcs",
    };
    #[doc = "\\[RFC4329\\]\\[RFC9239\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ECMASCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ecmascript\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ecmascript\");"]
    #[doc = r" ```"]
    pub const ECMASCRIPT: Mime = Mime {
        ttype: "application",
        subtype: "ecmascript",
    };
    #[doc = "\\[RFC9528\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EDHOC_CBOR_SEQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"edhoc+cbor-seq\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/edhoc+cbor-seq\");"]
    #[doc = r" ```"]
    pub const EDHOC_CBOR_SEQ: Mime = Mime {
        ttype: "application",
        subtype: "edhoc+cbor-seq",
    };
    #[doc = "\\[RFC1767\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EDI_CONSENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EDI-consent\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EDI-consent\");"]
    #[doc = r" ```"]
    pub const EDI_CONSENT: Mime = Mime {
        ttype: "application",
        subtype: "EDI-consent",
    };
    #[doc = "\\[RFC1767\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EDIFACT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EDIFACT\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EDIFACT\");"]
    #[doc = r" ```"]
    pub const EDIFACT: Mime = Mime {
        ttype: "application",
        subtype: "EDIFACT",
    };
    #[doc = "\\[RFC1767\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EDI_X12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EDI-X12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EDI-X12\");"]
    #[doc = r" ```"]
    pub const EDI_X12: Mime = Mime {
        ttype: "application",
        subtype: "EDI-X12",
    };
    #[doc = "\\[UEFI_Forum\\]\\[Samer_El-Haj-Mahmoud\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EFI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"efi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/efi\");"]
    #[doc = r" ```"]
    pub const EFI: Mime = Mime {
        ttype: "application",
        subtype: "efi",
    };
    #[doc = "\\[HL7\\]\\[Bryn_Rhodes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ELM_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"elm+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/elm+json\");"]
    #[doc = r" ```"]
    pub const ELM_JSON: Mime = Mime {
        ttype: "application",
        subtype: "elm+json",
    };
    #[doc = "\\[HL7\\]\\[Bryn_Rhodes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ELM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"elm+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/elm+xml\");"]
    #[doc = r" ```"]
    pub const ELM_XML: Mime = Mime {
        ttype: "application",
        subtype: "elm+xml",
    };
    #[doc = "\\[RFC8876\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMERGENCYCALLDATA_CAP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EmergencyCallData.cap+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EmergencyCallData.cap+xml\");"]
    #[doc = r" ```"]
    pub const EMERGENCYCALLDATA_CAP_XML: Mime = Mime {
        ttype: "application",
        subtype: "EmergencyCallData.cap+xml",
    };
    #[doc = "\\[RFC7852\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMERGENCYCALLDATA_COMMENT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EmergencyCallData.Comment+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EmergencyCallData.Comment+xml\");"]
    #[doc = r" ```"]
    pub const EMERGENCYCALLDATA_COMMENT_XML: Mime = Mime {
        ttype: "application",
        subtype: "EmergencyCallData.Comment+xml",
    };
    #[doc = "\\[RFC8147\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMERGENCYCALLDATA_CONTROL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EmergencyCallData.Control+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EmergencyCallData.Control+xml\");"]
    #[doc = r" ```"]
    pub const EMERGENCYCALLDATA_CONTROL_XML: Mime = Mime {
        ttype: "application",
        subtype: "EmergencyCallData.Control+xml",
    };
    #[doc = "\\[RFC7852\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMERGENCYCALLDATA_DEVICEINFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EmergencyCallData.DeviceInfo+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EmergencyCallData.DeviceInfo+xml\");"]
    #[doc = r" ```"]
    pub const EMERGENCYCALLDATA_DEVICEINFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "EmergencyCallData.DeviceInfo+xml",
    };
    #[doc = "\\[RFC8147\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMERGENCYCALLDATA_ECALL_MSD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EmergencyCallData.eCall.MSD\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EmergencyCallData.eCall.MSD\");"]
    #[doc = r" ```"]
    pub const EMERGENCYCALLDATA_ECALL_MSD: Mime = Mime {
        ttype: "application",
        subtype: "EmergencyCallData.eCall.MSD",
    };
    #[doc = "\\[NENA\\]\\[Randall_Gellens\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMERGENCYCALLDATA_LEGACYESN_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EmergencyCallData.LegacyESN+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EmergencyCallData.LegacyESN+json\");"]
    #[doc = r" ```"]
    pub const EMERGENCYCALLDATA_LEGACYESN_JSON: Mime = Mime {
        ttype: "application",
        subtype: "EmergencyCallData.LegacyESN+json",
    };
    #[doc = "\\[RFC7852\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMERGENCYCALLDATA_PROVIDERINFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EmergencyCallData.ProviderInfo+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EmergencyCallData.ProviderInfo+xml\");"]
    #[doc = r" ```"]
    pub const EMERGENCYCALLDATA_PROVIDERINFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "EmergencyCallData.ProviderInfo+xml",
    };
    #[doc = "\\[RFC7852\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMERGENCYCALLDATA_SERVICEINFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EmergencyCallData.ServiceInfo+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EmergencyCallData.ServiceInfo+xml\");"]
    #[doc = r" ```"]
    pub const EMERGENCYCALLDATA_SERVICEINFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "EmergencyCallData.ServiceInfo+xml",
    };
    #[doc = "\\[RFC7852\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMERGENCYCALLDATA_SUBSCRIBERINFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EmergencyCallData.SubscriberInfo+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EmergencyCallData.SubscriberInfo+xml\");"]
    #[doc = r" ```"]
    pub const EMERGENCYCALLDATA_SUBSCRIBERINFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "EmergencyCallData.SubscriberInfo+xml",
    };
    #[doc = "\\[RFC8148\\]\\[RFC Errata 6500\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMERGENCYCALLDATA_VEDS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"EmergencyCallData.VEDS+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/EmergencyCallData.VEDS+xml\");"]
    #[doc = r" ```"]
    pub const EMERGENCYCALLDATA_VEDS_XML: Mime = Mime {
        ttype: "application",
        subtype: "EmergencyCallData.VEDS+xml",
    };
    #[doc = "\\[W3C\\]\\[http://www.w3.org/TR/2007/CR-emma-20071211/#media-type-registration\\]\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMMA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"emma+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/emma+xml\");"]
    #[doc = r" ```"]
    pub const EMMA_XML: Mime = Mime {
        ttype: "application",
        subtype: "emma+xml",
    };
    #[doc = "\\[W3C\\]\\[Kazuyuki_Ashimura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EMOTIONML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"emotionml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/emotionml+xml\");"]
    #[doc = r" ```"]
    pub const EMOTIONML_XML: Mime = Mime {
        ttype: "application",
        subtype: "emotionml+xml",
    };
    #[doc = "\\[RFC6849\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ENCAPRTP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"encaprtp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/encaprtp\");"]
    #[doc = r" ```"]
    pub const ENCAPRTP: Mime = Mime {
        ttype: "application",
        subtype: "encaprtp",
    };
    #[doc = "\\[OpenID_Foundation_Artifact_Binding_WG\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ENTITY_STATEMENT_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"entity-statement+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/entity-statement+jwt\");"]
    #[doc = r" ```"]
    pub const ENTITY_STATEMENT_JWT: Mime = Mime {
        ttype: "application",
        subtype: "entity-statement+jwt",
    };
    #[doc = "\\[RFC5730\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EPP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"epp+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/epp+xml\");"]
    #[doc = r" ```"]
    pub const EPP_XML: Mime = Mime {
        ttype: "application",
        subtype: "epp+xml",
    };
    #[doc = "\\[W3C\\]\\[EPUB_3_WG\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EPUB_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"epub+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/epub+zip\");"]
    #[doc = r" ```"]
    pub const EPUB_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "epub+zip",
    };
    #[doc = "\\[Steve_Katz\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ESHOP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"eshop\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/eshop\");"]
    #[doc = r" ```"]
    pub const ESHOP: Mime = Mime {
        ttype: "application",
        subtype: "eshop",
    };
    #[doc = "\\[RFC4735\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EXAMPLE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"example\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/example\");"]
    #[doc = r" ```"]
    pub const EXAMPLE: Mime = Mime {
        ttype: "application",
        subtype: "example",
    };
    #[doc = "\\[W3C\\]\\[http://www.w3.org/TR/2009/CR-exi-20091208/#mediaTypeRegistration\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EXI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"exi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/exi\");"]
    #[doc = r" ```"]
    pub const EXI: Mime = Mime {
        ttype: "application",
        subtype: "exi",
    };
    #[doc = "\\[RFC9163\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EXPECT_CT_REPORT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"expect-ct-report+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/expect-ct-report+json\");"]
    #[doc = r" ```"]
    pub const EXPECT_CT_REPORT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "expect-ct-report+json",
    };
    #[doc = "\\[ISO-TC_184-SC_4\\]\\[Dana_Tripp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::EXPRESS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"express\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/express\");"]
    #[doc = r" ```"]
    pub const EXPRESS: Mime = Mime {
        ttype: "application",
        subtype: "express",
    };
    #[doc = "\\[ITU-T_ASN.1_Rapporteur\\]\\[ISO-IEC_JTC_1_SC_6_ASN.1_Rapporteur\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FASTINFOSET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"fastinfoset\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/fastinfoset\");"]
    #[doc = r" ```"]
    pub const FASTINFOSET: Mime = Mime {
        ttype: "application",
        subtype: "fastinfoset",
    };
    #[doc = "\\[ITU-T_ASN.1_Rapporteur\\]\\[ISO-IEC_JTC_1_SC_6_ASN.1_Rapporteur\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FASTSOAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"fastsoap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/fastsoap\");"]
    #[doc = r" ```"]
    pub const FASTSOAP: Mime = Mime {
        ttype: "application",
        subtype: "fastsoap",
    };
    #[doc = "\\[ISO-TC_171-SC_2\\]\\[Betsy_Fanning\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FDF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"fdf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/fdf\");"]
    #[doc = r" ```"]
    pub const FDF: Mime = Mime {
        ttype: "application",
        subtype: "fdf",
    };
    #[doc = "\\[RFC6726\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FDT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"fdt+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/fdt+xml\");"]
    #[doc = r" ```"]
    pub const FDT_XML: Mime = Mime {
        ttype: "application",
        subtype: "fdt+xml",
    };
    #[doc = "\\[HL7\\]\\[Grahame_Grieve\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FHIR_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"fhir+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/fhir+json\");"]
    #[doc = r" ```"]
    pub const FHIR_JSON: Mime = Mime {
        ttype: "application",
        subtype: "fhir+json",
    };
    #[doc = "\\[HL7\\]\\[Grahame_Grieve\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FHIR_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"fhir+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/fhir+xml\");"]
    #[doc = r" ```"]
    pub const FHIR_XML: Mime = Mime {
        ttype: "application",
        subtype: "fhir+xml",
    };
    #[doc = "\\[RFC4047\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FITS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"fits\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/fits\");"]
    #[doc = r" ```"]
    pub const FITS: Mime = Mime {
        ttype: "application",
        subtype: "fits",
    };
    #[doc = "\\[RFC8627\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FLEXFEC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"flexfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/flexfec\");"]
    #[doc = r" ```"]
    pub const FLEXFEC: Mime = Mime {
        ttype: "application",
        subtype: "flexfec",
    };
    #[doc = "\\[Levantovsky\\]\\[ISO-IEC_JTC_1\\]\\[RFC8081\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FONT_SFNT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"font-sfnt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/font-sfnt\");"]
    #[doc = r" ```"]
    pub const FONT_SFNT: Mime = Mime {
        ttype: "application",
        subtype: "font-sfnt",
    };
    #[doc = "\\[RFC3073\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FONT_TDPFR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"font-tdpfr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/font-tdpfr\");"]
    #[doc = r" ```"]
    pub const FONT_TDPFR: Mime = Mime {
        ttype: "application",
        subtype: "font-tdpfr",
    };
    #[doc = "\\[W3C\\]\\[RFC8081\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FONT_WOFF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"font-woff\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/font-woff\");"]
    #[doc = r" ```"]
    pub const FONT_WOFF: Mime = Mime {
        ttype: "application",
        subtype: "font-woff",
    };
    #[doc = "\\[RFC6230\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::FRAMEWORK_ATTRIBUTES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"framework-attributes+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/framework-attributes+xml\");"]
    #[doc = r" ```"]
    pub const FRAMEWORK_ATTRIBUTES_XML: Mime = Mime {
        ttype: "application",
        subtype: "framework-attributes+xml",
    };
    #[doc = "\\[RFC7946\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GEO_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"geo+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/geo+json\");"]
    #[doc = r" ```"]
    pub const GEO_JSON: Mime = Mime {
        ttype: "application",
        subtype: "geo+json",
    };
    #[doc = "\\[RFC8142\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GEO_JSON_SEQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"geo+json-seq\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/geo+json-seq\");"]
    #[doc = r" ```"]
    pub const GEO_JSON_SEQ: Mime = Mime {
        ttype: "application",
        subtype: "geo+json-seq",
    };
    #[doc = "\\[OGC\\]\\[Scott_Simmons\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GEOPACKAGE_SQLITE3;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"geopackage+sqlite3\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/geopackage+sqlite3\");"]
    #[doc = r" ```"]
    pub const GEOPACKAGE_SQLITE3: Mime = Mime {
        ttype: "application",
        subtype: "geopackage+sqlite3",
    };
    #[doc = "\\[OGC\\]\\[Scott_Simmons\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GEOPOSE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"geopose+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/geopose+json\");"]
    #[doc = r" ```"]
    pub const GEOPOSE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "geopose+json",
    };
    #[doc = "\\[OGC\\]\\[Scott_Simmons\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GEOXACML_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"geoxacml+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/geoxacml+json\");"]
    #[doc = r" ```"]
    pub const GEOXACML_JSON: Mime = Mime {
        ttype: "application",
        subtype: "geoxacml+json",
    };
    #[doc = "\\[OGC\\]\\[Scott_Simmons\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GEOXACML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"geoxacml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/geoxacml+xml\");"]
    #[doc = r" ```"]
    pub const GEOXACML_XML: Mime = Mime {
        ttype: "application",
        subtype: "geoxacml+xml",
    };
    #[doc = "\\[Khronos\\]\\[Saurabh_Bhatia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GLTF_BUFFER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"gltf-buffer\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/gltf-buffer\");"]
    #[doc = r" ```"]
    pub const GLTF_BUFFER: Mime = Mime {
        ttype: "application",
        subtype: "gltf-buffer",
    };
    #[doc = "\\[OGC\\]\\[Clemens_Portele\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"gml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/gml+xml\");"]
    #[doc = r" ```"]
    pub const GML_XML: Mime = Mime {
        ttype: "application",
        subtype: "gml+xml",
    };
    #[doc = "\\[RFC-ietf-gnap-core-protocol-20\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GNAP_BINDING_JWS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"gnap-binding-jws\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/gnap-binding-jws\");"]
    #[doc = r" ```"]
    pub const GNAP_BINDING_JWS: Mime = Mime {
        ttype: "application",
        subtype: "gnap-binding-jws",
    };
    #[doc = "\\[RFC-ietf-gnap-core-protocol-20\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GNAP_BINDING_JWSD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"gnap-binding-jwsd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/gnap-binding-jwsd\");"]
    #[doc = r" ```"]
    pub const GNAP_BINDING_JWSD: Mime = Mime {
        ttype: "application",
        subtype: "gnap-binding-jwsd",
    };
    #[doc = "\\[RFC-ietf-gnap-core-protocol-20\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GNAP_BINDING_ROTATION_JWS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"gnap-binding-rotation-jws\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/gnap-binding-rotation-jws\");"]
    #[doc = r" ```"]
    pub const GNAP_BINDING_ROTATION_JWS: Mime = Mime {
        ttype: "application",
        subtype: "gnap-binding-rotation-jws",
    };
    #[doc = "\\[RFC-ietf-gnap-core-protocol-20\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GNAP_BINDING_ROTATION_JWSD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"gnap-binding-rotation-jwsd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/gnap-binding-rotation-jwsd\");"]
    #[doc = r" ```"]
    pub const GNAP_BINDING_ROTATION_JWSD: Mime = Mime {
        ttype: "application",
        subtype: "gnap-binding-rotation-jwsd",
    };
    #[doc = "\\[World_Meterological_Organization\\]\\[Anna_Milan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GRIB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"grib\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/grib\");"]
    #[doc = r" ```"]
    pub const GRIB: Mime = Mime {
        ttype: "application",
        subtype: "grib",
    };
    #[doc = "\\[RFC6713\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::GZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"gzip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/gzip\");"]
    #[doc = r" ```"]
    pub const GZIP: Mime = Mime {
        ttype: "application",
        subtype: "gzip",
    };
    #[doc = "\\[RFC4573\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::H224;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"H224\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/H224\");"]
    #[doc = r" ```"]
    pub const H224: Mime = Mime {
        ttype: "application",
        subtype: "H224",
    };
    #[doc = "\\[RFC5985\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::HELD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"held+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/held+xml\");"]
    #[doc = r" ```"]
    pub const HELD_XML: Mime = Mime {
        ttype: "application",
        subtype: "held+xml",
    };
    #[doc = "\\[HL7\\]\\[Marc_Duteau\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::HL7V2_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"hl7v2+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/hl7v2+xml\");"]
    #[doc = r" ```"]
    pub const HL7V2_XML: Mime = Mime {
        ttype: "application",
        subtype: "hl7v2+xml",
    };
    #[doc = "\\[RFC9112\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::HTTP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"http\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/http\");"]
    #[doc = r" ```"]
    pub const HTTP: Mime = Mime {
        ttype: "application",
        subtype: "http",
    };
    #[doc = "\\[Michael_Domino\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::HYPERSTUDIO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"hyperstudio\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/hyperstudio\");"]
    #[doc = r" ```"]
    pub const HYPERSTUDIO: Mime = Mime {
        ttype: "application",
        subtype: "hyperstudio",
    };
    #[doc = "\\[RFC5408\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::IBE_KEY_REQUEST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ibe-key-request+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ibe-key-request+xml\");"]
    #[doc = r" ```"]
    pub const IBE_KEY_REQUEST_XML: Mime = Mime {
        ttype: "application",
        subtype: "ibe-key-request+xml",
    };
    #[doc = "\\[RFC5408\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::IBE_PKG_REPLY_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ibe-pkg-reply+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ibe-pkg-reply+xml\");"]
    #[doc = r" ```"]
    pub const IBE_PKG_REPLY_XML: Mime = Mime {
        ttype: "application",
        subtype: "ibe-pkg-reply+xml",
    };
    #[doc = "\\[RFC5408\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::IBE_PP_DATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ibe-pp-data\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ibe-pp-data\");"]
    #[doc = r" ```"]
    pub const IBE_PP_DATA: Mime = Mime {
        ttype: "application",
        subtype: "ibe-pp-data",
    };
    #[doc = "\\[Curtis_Parks\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::IGES;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"iges\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/iges\");"]
    #[doc = r" ```"]
    pub const IGES: Mime = Mime {
        ttype: "application",
        subtype: "iges",
    };
    #[doc = "\\[RFC3994\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::IM_ISCOMPOSING_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"im-iscomposing+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/im-iscomposing+xml\");"]
    #[doc = r" ```"]
    pub const IM_ISCOMPOSING_XML: Mime = Mime {
        ttype: "application",
        subtype: "im-iscomposing+xml",
    };
    #[doc = "\\[RFC2652\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::INDEX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"index\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/index\");"]
    #[doc = r" ```"]
    pub const INDEX: Mime = Mime {
        ttype: "application",
        subtype: "index",
    };
    #[doc = "\\[RFC2652\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::INDEX_CMD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"index.cmd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/index.cmd\");"]
    #[doc = r" ```"]
    pub const INDEX_CMD: Mime = Mime {
        ttype: "application",
        subtype: "index.cmd",
    };
    #[doc = "\\[RFC2652\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::INDEX_OBJ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"index.obj\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/index.obj\");"]
    #[doc = r" ```"]
    pub const INDEX_OBJ: Mime = Mime {
        ttype: "application",
        subtype: "index.obj",
    };
    #[doc = "\\[RFC2652\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::INDEX_RESPONSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"index.response\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/index.response\");"]
    #[doc = r" ```"]
    pub const INDEX_RESPONSE: Mime = Mime {
        ttype: "application",
        subtype: "index.response",
    };
    #[doc = "\\[RFC2652\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::INDEX_VND;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"index.vnd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/index.vnd\");"]
    #[doc = r" ```"]
    pub const INDEX_VND: Mime = Mime {
        ttype: "application",
        subtype: "index.vnd",
    };
    #[doc = "\\[Kazuyuki_Ashimura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::INKML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"inkml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/inkml+xml\");"]
    #[doc = r" ```"]
    pub const INKML_XML: Mime = Mime {
        ttype: "application",
        subtype: "inkml+xml",
    };
    #[doc = "\\[RFC2935\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::IOTP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"IOTP\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/IOTP\");"]
    #[doc = r" ```"]
    pub const IOTP: Mime = Mime {
        ttype: "application",
        subtype: "IOTP",
    };
    #[doc = "\\[RFC5655\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::IPFIX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ipfix\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ipfix\");"]
    #[doc = r" ```"]
    pub const IPFIX: Mime = Mime {
        ttype: "application",
        subtype: "ipfix",
    };
    #[doc = "\\[RFC8010\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::IPP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ipp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ipp\");"]
    #[doc = r" ```"]
    pub const IPP: Mime = Mime {
        ttype: "application",
        subtype: "ipp",
    };
    #[doc = "\\[RFC3204\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ISUP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ISUP\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ISUP\");"]
    #[doc = r" ```"]
    pub const ISUP: Mime = Mime {
        ttype: "application",
        subtype: "ISUP",
    };
    #[doc = "\\[W3C\\]\\[ITS-IG-W3C\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ITS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"its+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/its+xml\");"]
    #[doc = r" ```"]
    pub const ITS_XML: Mime = Mime {
        ttype: "application",
        subtype: "its+xml",
    };
    #[doc = "\\[JCP\\]\\[Iris_Clark\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JAVA_ARCHIVE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"java-archive\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/java-archive\");"]
    #[doc = r" ```"]
    pub const JAVA_ARCHIVE: Mime = Mime {
        ttype: "application",
        subtype: "java-archive",
    };
    #[doc = "\\[RFC4329\\]\\[RFC9239\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JAVASCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"javascript\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/javascript\");"]
    #[doc = r" ```"]
    pub const JAVASCRIPT: Mime = Mime {
        ttype: "application",
        subtype: "javascript",
    };
    #[doc = "\\[W3C\\]\\[Ivan_Herman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JF2FEED_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jf2feed+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jf2feed+json\");"]
    #[doc = r" ```"]
    pub const JF2FEED_JSON: Mime = Mime {
        ttype: "application",
        subtype: "jf2feed+json",
    };
    #[doc = "\\[RFC7515\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JOSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jose\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jose\");"]
    #[doc = r" ```"]
    pub const JOSE: Mime = Mime {
        ttype: "application",
        subtype: "jose",
    };
    #[doc = "\\[RFC7515\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JOSE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jose+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jose+json\");"]
    #[doc = r" ```"]
    pub const JOSE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "jose+json",
    };
    #[doc = "\\[RFC7033\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JRD_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jrd+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jrd+json\");"]
    #[doc = r" ```"]
    pub const JRD_JSON: Mime = Mime {
        ttype: "application",
        subtype: "jrd+json",
    };
    #[doc = "\\[RFC8984\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JSCALENDAR_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jscalendar+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jscalendar+json\");"]
    #[doc = r" ```"]
    pub const JSCALENDAR_JSON: Mime = Mime {
        ttype: "application",
        subtype: "jscalendar+json",
    };
    #[doc = "\\[RFC9553\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JSCONTACT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jscontact+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jscontact+json\");"]
    #[doc = r" ```"]
    pub const JSCONTACT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "jscontact+json",
    };
    #[doc = "\\[RFC8259\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/json\");"]
    #[doc = r" ```"]
    pub const JSON: Mime = Mime {
        ttype: "application",
        subtype: "json",
    };
    #[doc = "\\[RFC6902\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JSON_PATCH_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"json-patch+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/json-patch+json\");"]
    #[doc = r" ```"]
    pub const JSON_PATCH_JSON: Mime = Mime {
        ttype: "application",
        subtype: "json-patch+json",
    };
    #[doc = "\\[RFC7464\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JSON_SEQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"json-seq\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/json-seq\");"]
    #[doc = r" ```"]
    pub const JSON_SEQ: Mime = Mime {
        ttype: "application",
        subtype: "json-seq",
    };
    #[doc = "\\[RFC9535\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JSONPATH;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jsonpath\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jsonpath\");"]
    #[doc = r" ```"]
    pub const JSONPATH: Mime = Mime {
        ttype: "application",
        subtype: "jsonpath",
    };
    #[doc = "\\[RFC7517\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JWK_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jwk+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jwk+json\");"]
    #[doc = r" ```"]
    pub const JWK_JSON: Mime = Mime {
        ttype: "application",
        subtype: "jwk+json",
    };
    #[doc = "\\[RFC7517\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JWK_SET_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jwk-set+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jwk-set+json\");"]
    #[doc = r" ```"]
    pub const JWK_SET_JSON: Mime = Mime {
        ttype: "application",
        subtype: "jwk-set+json",
    };
    #[doc = "\\[OpenID_Foundation_Artifact_Binding_WG\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::APPLICATION_JWK_SET_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jwk-set+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jwk-set+jwt\");"]
    #[doc = r" ```"]
    pub const APPLICATION_JWK_SET_JWT: Mime = Mime {
        ttype: "application",
        subtype: "jwk-set+jwt",
    };
    #[doc = "\\[RFC7519\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/jwt\");"]
    #[doc = r" ```"]
    pub const JWT: Mime = Mime {
        ttype: "application",
        subtype: "jwt",
    };
    #[doc = "\\[RFC4730\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::KPML_REQUEST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"kpml-request+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/kpml-request+xml\");"]
    #[doc = r" ```"]
    pub const KPML_REQUEST_XML: Mime = Mime {
        ttype: "application",
        subtype: "kpml-request+xml",
    };
    #[doc = "\\[RFC4730\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::KPML_RESPONSE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"kpml-response+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/kpml-response+xml\");"]
    #[doc = r" ```"]
    pub const KPML_RESPONSE_XML: Mime = Mime {
        ttype: "application",
        subtype: "kpml-response+xml",
    };
    #[doc = "\\[W3C\\]\\[Ivan_Herman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LD_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ld+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ld+json\");"]
    #[doc = r" ```"]
    pub const LD_JSON: Mime = Mime {
        ttype: "application",
        subtype: "ld+json",
    };
    #[doc = "\\[RFC7940\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LGR_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"lgr+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/lgr+xml\");"]
    #[doc = r" ```"]
    pub const LGR_XML: Mime = Mime {
        ttype: "application",
        subtype: "lgr+xml",
    };
    #[doc = "\\[RFC6690\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LINK_FORMAT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"link-format\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/link-format\");"]
    #[doc = r" ```"]
    pub const LINK_FORMAT: Mime = Mime {
        ttype: "application",
        subtype: "link-format",
    };
    #[doc = "\\[RFC9264\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LINKSET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"linkset\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/linkset\");"]
    #[doc = r" ```"]
    pub const LINKSET: Mime = Mime {
        ttype: "application",
        subtype: "linkset",
    };
    #[doc = "\\[RFC9264\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LINKSET_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"linkset+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/linkset+json\");"]
    #[doc = r" ```"]
    pub const LINKSET_JSON: Mime = Mime {
        ttype: "application",
        subtype: "linkset+json",
    };
    #[doc = "\\[RFC7200\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LOAD_CONTROL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"load-control+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/load-control+xml\");"]
    #[doc = r" ```"]
    pub const LOAD_CONTROL_XML: Mime = Mime {
        ttype: "application",
        subtype: "load-control+xml",
    };
    #[doc = "\\[OpenID_Foundation_Artifact_Binding_WG\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LOGOUT_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"logout+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/logout+jwt\");"]
    #[doc = r" ```"]
    pub const LOGOUT_JWT: Mime = Mime {
        ttype: "application",
        subtype: "logout+jwt",
    };
    #[doc = "\\[RFC5222\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LOST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"lost+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/lost+xml\");"]
    #[doc = r" ```"]
    pub const LOST_XML: Mime = Mime {
        ttype: "application",
        subtype: "lost+xml",
    };
    #[doc = "\\[RFC6739\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LOSTSYNC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"lostsync+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/lostsync+xml\");"]
    #[doc = r" ```"]
    pub const LOSTSYNC_XML: Mime = Mime {
        ttype: "application",
        subtype: "lostsync+xml",
    };
    #[doc = "\\[W3C\\]\\[Ivan_Herman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LPF_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"lpf+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/lpf+zip\");"]
    #[doc = r" ```"]
    pub const LPF_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "lpf+zip",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::LXF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"LXF\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/LXF\");"]
    #[doc = r" ```"]
    pub const LXF: Mime = Mime {
        ttype: "application",
        subtype: "LXF",
    };
    #[doc = "\\[Patrik_Faltstrom\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MAC_BINHEX40;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mac-binhex40\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mac-binhex40\");"]
    #[doc = r" ```"]
    pub const MAC_BINHEX40: Mime = Mime {
        ttype: "application",
        subtype: "mac-binhex40",
    };
    #[doc = "\\[Paul_Lindner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MACWRITEII;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"macwriteii\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/macwriteii\");"]
    #[doc = r" ```"]
    pub const MACWRITEII: Mime = Mime {
        ttype: "application",
        subtype: "macwriteii",
    };
    #[doc = "\\[RFC6207\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MADS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mads+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mads+xml\");"]
    #[doc = r" ```"]
    pub const MADS_XML: Mime = Mime {
        ttype: "application",
        subtype: "mads+xml",
    };
    #[doc = "\\[W3C\\]\\[Marcos_Caceres\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MANIFEST_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"manifest+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/manifest+json\");"]
    #[doc = r" ```"]
    pub const MANIFEST_JSON: Mime = Mime {
        ttype: "application",
        subtype: "manifest+json",
    };
    #[doc = "\\[RFC2220\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MARC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"marc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/marc\");"]
    #[doc = r" ```"]
    pub const MARC: Mime = Mime {
        ttype: "application",
        subtype: "marc",
    };
    #[doc = "\\[RFC6207\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MARCXML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"marcxml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/marcxml+xml\");"]
    #[doc = r" ```"]
    pub const MARCXML_XML: Mime = Mime {
        ttype: "application",
        subtype: "marcxml+xml",
    };
    #[doc = "\\[Wolfram\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MATHEMATICA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mathematica\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mathematica\");"]
    #[doc = r" ```"]
    pub const MATHEMATICA: Mime = Mime {
        ttype: "application",
        subtype: "mathematica",
    };
    #[doc = "\\[W3C\\]\\[http://www.w3.org/TR/MathML3/appendixb.html\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MATHML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mathml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mathml+xml\");"]
    #[doc = r" ```"]
    pub const MATHML_XML: Mime = Mime {
        ttype: "application",
        subtype: "mathml+xml",
    };
    #[doc = "\\[W3C\\]\\[http://www.w3.org/TR/MathML3/appendixb.html\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MATHML_CONTENT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mathml-content+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mathml-content+xml\");"]
    #[doc = r" ```"]
    pub const MATHML_CONTENT_XML: Mime = Mime {
        ttype: "application",
        subtype: "mathml-content+xml",
    };
    #[doc = "\\[W3C\\]\\[http://www.w3.org/TR/MathML3/appendixb.html\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MATHML_PRESENTATION_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mathml-presentation+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mathml-presentation+xml\");"]
    #[doc = r" ```"]
    pub const MATHML_PRESENTATION_XML: Mime = Mime {
        ttype: "application",
        subtype: "mathml-presentation+xml",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_ASSOCIATED_PROCEDURE_DESCRIPTION_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-associated-procedure-description+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-associated-procedure-description+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_ASSOCIATED_PROCEDURE_DESCRIPTION_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-associated-procedure-description+xml",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_DEREGISTER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-deregister+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-deregister+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_DEREGISTER_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-deregister+xml",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_ENVELOPE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-envelope+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-envelope+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_ENVELOPE_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-envelope+xml",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_MSK_RESPONSE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-msk-response+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-msk-response+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_MSK_RESPONSE_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-msk-response+xml",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_MSK_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-msk+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-msk+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_MSK_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-msk+xml",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_PROTECTION_DESCRIPTION_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-protection-description+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-protection-description+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_PROTECTION_DESCRIPTION_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-protection-description+xml",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_RECEPTION_REPORT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-reception-report+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-reception-report+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_RECEPTION_REPORT_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-reception-report+xml",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_REGISTER_RESPONSE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-register-response+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-register-response+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_REGISTER_RESPONSE_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-register-response+xml",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_REGISTER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-register+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-register+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_REGISTER_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-register+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Eric_Turcotte\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_SCHEDULE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-schedule+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-schedule+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_SCHEDULE_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-schedule+xml",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBMS_USER_SERVICE_DESCRIPTION_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbms-user-service-description+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbms-user-service-description+xml\");"]
    #[doc = r" ```"]
    pub const MBMS_USER_SERVICE_DESCRIPTION_XML: Mime = Mime {
        ttype: "application",
        subtype: "mbms-user-service-description+xml",
    };
    #[doc = "\\[RFC4155\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MBOX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mbox\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mbox\");"]
    #[doc = r" ```"]
    pub const MBOX: Mime = Mime {
        ttype: "application",
        subtype: "mbox",
    };
    #[doc = "\\[RFC5168\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MEDIA_CONTROL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"media_control+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/media_control+xml\");"]
    #[doc = r" ```"]
    pub const MEDIA_CONTROL_XML: Mime = Mime {
        ttype: "application",
        subtype: "media_control+xml",
    };
    #[doc = "\\[RFC6796\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MEDIA_POLICY_DATASET_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"media-policy-dataset+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/media-policy-dataset+xml\");"]
    #[doc = r" ```"]
    pub const MEDIA_POLICY_DATASET_XML: Mime = Mime {
        ttype: "application",
        subtype: "media-policy-dataset+xml",
    };
    #[doc = "\\[RFC5022\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MEDIASERVERCONTROL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mediaservercontrol+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mediaservercontrol+xml\");"]
    #[doc = r" ```"]
    pub const MEDIASERVERCONTROL_XML: Mime = Mime {
        ttype: "application",
        subtype: "mediaservercontrol+xml",
    };
    #[doc = "\\[RFC7396\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MERGE_PATCH_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"merge-patch+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/merge-patch+json\");"]
    #[doc = r" ```"]
    pub const MERGE_PATCH_JSON: Mime = Mime {
        ttype: "application",
        subtype: "merge-patch+json",
    };
    #[doc = "\\[RFC5854\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::METALINK4_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"metalink4+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/metalink4+xml\");"]
    #[doc = r" ```"]
    pub const METALINK4_XML: Mime = Mime {
        ttype: "application",
        subtype: "metalink4+xml",
    };
    #[doc = "\\[RFC6207\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::METS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mets+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mets+xml\");"]
    #[doc = r" ```"]
    pub const METS_XML: Mime = Mime {
        ttype: "application",
        subtype: "mets+xml",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MF4;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"MF4\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/MF4\");"]
    #[doc = r" ```"]
    pub const MF4: Mime = Mime {
        ttype: "application",
        subtype: "MF4",
    };
    #[doc = "\\[RFC3830\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MIKEY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mikey\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mikey\");"]
    #[doc = r" ```"]
    pub const MIKEY: Mime = Mime {
        ttype: "application",
        subtype: "mikey",
    };
    #[doc = "\\[NCGIS\\]\\[Bryan_Blank\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MIPC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mipc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mipc\");"]
    #[doc = r" ```"]
    pub const MIPC: Mime = Mime {
        ttype: "application",
        subtype: "mipc",
    };
    #[doc = "\\[RFC9177\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MISSING_BLOCKS_CBOR_SEQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"missing-blocks+cbor-seq\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/missing-blocks+cbor-seq\");"]
    #[doc = r" ```"]
    pub const MISSING_BLOCKS_CBOR_SEQ: Mime = Mime {
        ttype: "application",
        subtype: "missing-blocks+cbor-seq",
    };
    #[doc = "\\[ATSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MMT_AEI_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mmt-aei+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mmt-aei+xml\");"]
    #[doc = r" ```"]
    pub const MMT_AEI_XML: Mime = Mime {
        ttype: "application",
        subtype: "mmt-aei+xml",
    };
    #[doc = "\\[ATSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MMT_USD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mmt-usd+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mmt-usd+xml\");"]
    #[doc = r" ```"]
    pub const MMT_USD_XML: Mime = Mime {
        ttype: "application",
        subtype: "mmt-usd+xml",
    };
    #[doc = "\\[RFC6207\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MODS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mods+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mods+xml\");"]
    #[doc = r" ```"]
    pub const MODS_XML: Mime = Mime {
        ttype: "application",
        subtype: "mods+xml",
    };
    #[doc = "\\[RFC1848\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MOSS_KEYS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"moss-keys\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/moss-keys\");"]
    #[doc = r" ```"]
    pub const MOSS_KEYS: Mime = Mime {
        ttype: "application",
        subtype: "moss-keys",
    };
    #[doc = "\\[RFC1848\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MOSS_SIGNATURE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"moss-signature\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/moss-signature\");"]
    #[doc = r" ```"]
    pub const MOSS_SIGNATURE: Mime = Mime {
        ttype: "application",
        subtype: "moss-signature",
    };
    #[doc = "\\[RFC1848\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MOSSKEY_DATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mosskey-data\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mosskey-data\");"]
    #[doc = r" ```"]
    pub const MOSSKEY_DATA: Mime = Mime {
        ttype: "application",
        subtype: "mosskey-data",
    };
    #[doc = "\\[RFC1848\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MOSSKEY_REQUEST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mosskey-request\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mosskey-request\");"]
    #[doc = r" ```"]
    pub const MOSSKEY_REQUEST: Mime = Mime {
        ttype: "application",
        subtype: "mosskey-request",
    };
    #[doc = "\\[RFC6381\\]\\[David_Singer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MP21;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mp21\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mp21\");"]
    #[doc = r" ```"]
    pub const MP21: Mime = Mime {
        ttype: "application",
        subtype: "mp21",
    };
    #[doc = "\\[RFC4337\\]\\[RFC6381\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MP4;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mp4\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mp4\");"]
    #[doc = r" ```"]
    pub const MP4: Mime = Mime {
        ttype: "application",
        subtype: "mp4",
    };
    #[doc = "\\[RFC3640\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MPEG4_GENERIC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mpeg4-generic\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mpeg4-generic\");"]
    #[doc = r" ```"]
    pub const MPEG4_GENERIC: Mime = Mime {
        ttype: "application",
        subtype: "mpeg4-generic",
    };
    #[doc = "\\[RFC4337\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MPEG4_IOD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mpeg4-iod\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mpeg4-iod\");"]
    #[doc = r" ```"]
    pub const MPEG4_IOD: Mime = Mime {
        ttype: "application",
        subtype: "mpeg4-iod",
    };
    #[doc = "\\[RFC4337\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MPEG4_IOD_XMT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mpeg4-iod-xmt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mpeg4-iod-xmt\");"]
    #[doc = r" ```"]
    pub const MPEG4_IOD_XMT: Mime = Mime {
        ttype: "application",
        subtype: "mpeg4-iod-xmt",
    };
    #[doc = "\\[RFC6917\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MRB_CONSUMER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mrb-consumer+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mrb-consumer+xml\");"]
    #[doc = r" ```"]
    pub const MRB_CONSUMER_XML: Mime = Mime {
        ttype: "application",
        subtype: "mrb-consumer+xml",
    };
    #[doc = "\\[RFC6917\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MRB_PUBLISH_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mrb-publish+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mrb-publish+xml\");"]
    #[doc = r" ```"]
    pub const MRB_PUBLISH_XML: Mime = Mime {
        ttype: "application",
        subtype: "mrb-publish+xml",
    };
    #[doc = "\\[RFC6231\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MSC_IVR_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"msc-ivr+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/msc-ivr+xml\");"]
    #[doc = r" ```"]
    pub const MSC_IVR_XML: Mime = Mime {
        ttype: "application",
        subtype: "msc-ivr+xml",
    };
    #[doc = "\\[RFC6505\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MSC_MIXER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"msc-mixer+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/msc-mixer+xml\");"]
    #[doc = r" ```"]
    pub const MSC_MIXER_XML: Mime = Mime {
        ttype: "application",
        subtype: "msc-mixer+xml",
    };
    #[doc = "\\[Paul_Lindner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MSWORD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"msword\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/msword\");"]
    #[doc = r" ```"]
    pub const MSWORD: Mime = Mime {
        ttype: "application",
        subtype: "msword",
    };
    #[doc = "\\[RFC8520\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MUD_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mud+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mud+json\");"]
    #[doc = r" ```"]
    pub const MUD_JSON: Mime = Mime {
        ttype: "application",
        subtype: "mud+json",
    };
    #[doc = "\\[RFC8710\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MULTIPART_CORE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"multipart-core\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/multipart-core\");"]
    #[doc = r" ```"]
    pub const MULTIPART_CORE: Mime = Mime {
        ttype: "application",
        subtype: "multipart-core",
    };
    #[doc = "\\[RFC4539\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::MXF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"mxf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/mxf\");"]
    #[doc = r" ```"]
    pub const MXF: Mime = Mime {
        ttype: "application",
        subtype: "mxf",
    };
    #[doc = "\\[W3C\\]\\[Eric_Prudhommeaux\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::N_QUADS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"n-quads\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/n-quads\");"]
    #[doc = r" ```"]
    pub const N_QUADS: Mime = Mime {
        ttype: "application",
        subtype: "n-quads",
    };
    #[doc = "\\[W3C\\]\\[Eric_Prudhommeaux\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::N_TRIPLES;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"n-triples\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/n-triples\");"]
    #[doc = r" ```"]
    pub const N_TRIPLES: Mime = Mime {
        ttype: "application",
        subtype: "n-triples",
    };
    #[doc = "\\[RFC4707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::NASDATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"nasdata\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/nasdata\");"]
    #[doc = r" ```"]
    pub const NASDATA: Mime = Mime {
        ttype: "application",
        subtype: "nasdata",
    };
    #[doc = "\\[RFC5537\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::NEWS_CHECKGROUPS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"news-checkgroups\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/news-checkgroups\");"]
    #[doc = r" ```"]
    pub const NEWS_CHECKGROUPS: Mime = Mime {
        ttype: "application",
        subtype: "news-checkgroups",
    };
    #[doc = "\\[RFC5537\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::NEWS_GROUPINFO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"news-groupinfo\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/news-groupinfo\");"]
    #[doc = r" ```"]
    pub const NEWS_GROUPINFO: Mime = Mime {
        ttype: "application",
        subtype: "news-groupinfo",
    };
    #[doc = "\\[RFC5537\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::NEWS_TRANSMISSION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"news-transmission\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/news-transmission\");"]
    #[doc = r" ```"]
    pub const NEWS_TRANSMISSION: Mime = Mime {
        ttype: "application",
        subtype: "news-transmission",
    };
    #[doc = "\\[RFC6787\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::NLSML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"nlsml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/nlsml+xml\");"]
    #[doc = r" ```"]
    pub const NLSML_XML: Mime = Mime {
        ttype: "application",
        subtype: "nlsml+xml",
    };
    #[doc = "\\[Node.js_TSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::NODE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"node\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/node\");"]
    #[doc = r" ```"]
    pub const NODE: Mime = Mime {
        ttype: "application",
        subtype: "node",
    };
    #[doc = "\\[Michael_Hammer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::NSS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"nss\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/nss\");"]
    #[doc = r" ```"]
    pub const NSS: Mime = Mime {
        ttype: "application",
        subtype: "nss",
    };
    #[doc = "\\[RFC9101\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OAUTH_AUTHZ_REQ_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"oauth-authz-req+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/oauth-authz-req+jwt\");"]
    #[doc = r" ```"]
    pub const OAUTH_AUTHZ_REQ_JWT: Mime = Mime {
        ttype: "application",
        subtype: "oauth-authz-req+jwt",
    };
    #[doc = "\\[RFC9230\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OBLIVIOUS_DNS_MESSAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"oblivious-dns-message\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/oblivious-dns-message\");"]
    #[doc = r" ```"]
    pub const OBLIVIOUS_DNS_MESSAGE: Mime = Mime {
        ttype: "application",
        subtype: "oblivious-dns-message",
    };
    #[doc = "\\[RFC6960\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OCSP_REQUEST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ocsp-request\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ocsp-request\");"]
    #[doc = r" ```"]
    pub const OCSP_REQUEST: Mime = Mime {
        ttype: "application",
        subtype: "ocsp-request",
    };
    #[doc = "\\[RFC6960\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OCSP_RESPONSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ocsp-response\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ocsp-response\");"]
    #[doc = r" ```"]
    pub const OCSP_RESPONSE: Mime = Mime {
        ttype: "application",
        subtype: "ocsp-response",
    };
    #[doc = "\\[RFC2045\\]\\[RFC2046\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OCTET_STREAM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"octet-stream\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/octet-stream\");"]
    #[doc = r" ```"]
    pub const OCTET_STREAM: Mime = Mime {
        ttype: "application",
        subtype: "octet-stream",
    };
    #[doc = "\\[RFC1494\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ODA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ODA\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ODA\");"]
    #[doc = r" ```"]
    pub const ODA: Mime = Mime {
        ttype: "application",
        subtype: "ODA",
    };
    #[doc = "\\[CDISC\\]\\[Sam_Hume\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ODM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"odm+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/odm+xml\");"]
    #[doc = r" ```"]
    pub const ODM_XML: Mime = Mime {
        ttype: "application",
        subtype: "odm+xml",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ODX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ODX\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ODX\");"]
    #[doc = r" ```"]
    pub const ODX: Mime = Mime {
        ttype: "application",
        subtype: "ODX",
    };
    #[doc = "\\[W3C\\]\\[EPUB_3_WG\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OEBPS_PACKAGE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"oebps-package+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/oebps-package+xml\");"]
    #[doc = r" ```"]
    pub const OEBPS_PACKAGE_XML: Mime = Mime {
        ttype: "application",
        subtype: "oebps-package+xml",
    };
    #[doc = "\\[RFC5334\\]\\[RFC7845\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OGG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ogg\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ogg\");"]
    #[doc = r" ```"]
    pub const OGG: Mime = Mime {
        ttype: "application",
        subtype: "ogg",
    };
    #[doc = "\\[RFC9458\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OHTTP_KEYS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ohttp-keys\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ohttp-keys\");"]
    #[doc = r" ```"]
    pub const OHTTP_KEYS: Mime = Mime {
        ttype: "application",
        subtype: "ohttp-keys",
    };
    #[doc = "\\[OPC_Foundation\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OPC_NODESET_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"opc-nodeset+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/opc-nodeset+xml\");"]
    #[doc = r" ```"]
    pub const OPC_NODESET_XML: Mime = Mime {
        ttype: "application",
        subtype: "opc-nodeset+xml",
    };
    #[doc = "\\[RFC8613\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OSCORE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"oscore\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/oscore\");"]
    #[doc = r" ```"]
    pub const OSCORE: Mime = Mime {
        ttype: "application",
        subtype: "oscore",
    };
    #[doc = "\\[Ecma_International_Helpdesk\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::OXPS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"oxps\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/oxps\");"]
    #[doc = r" ```"]
    pub const OXPS: Mime = Mime {
        ttype: "application",
        subtype: "oxps",
    };
    #[doc = "\\[ISO-TC_184-SC_4\\]\\[Dana_Tripp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::P21;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"p21\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/p21\");"]
    #[doc = r" ```"]
    pub const P21: Mime = Mime {
        ttype: "application",
        subtype: "p21",
    };
    #[doc = "\\[ISO-TC_184-SC_4\\]\\[Dana_Tripp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::P21_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"p21+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/p21+zip\");"]
    #[doc = r" ```"]
    pub const P21_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "p21+zip",
    };
    #[doc = "\\[RFC6940\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::P2P_OVERLAY_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"p2p-overlay+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/p2p-overlay+xml\");"]
    #[doc = r" ```"]
    pub const P2P_OVERLAY_XML: Mime = Mime {
        ttype: "application",
        subtype: "p2p-overlay+xml",
    };
    #[doc = "\\[RFC3009\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PARITYFEC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"parityfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/parityfec\");"]
    #[doc = r" ```"]
    pub const PARITYFEC: Mime = Mime {
        ttype: "application",
        subtype: "parityfec",
    };
    #[doc = "\\[RFC8225\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PASSPORT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"passport\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/passport\");"]
    #[doc = r" ```"]
    pub const PASSPORT: Mime = Mime {
        ttype: "application",
        subtype: "passport",
    };
    #[doc = "\\[RFC5261\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PATCH_OPS_ERROR_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"patch-ops-error+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/patch-ops-error+xml\");"]
    #[doc = r" ```"]
    pub const PATCH_OPS_ERROR_XML: Mime = Mime {
        ttype: "application",
        subtype: "patch-ops-error+xml",
    };
    #[doc = "\\[RFC8118\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PDF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pdf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pdf\");"]
    #[doc = r" ```"]
    pub const PDF: Mime = Mime {
        ttype: "application",
        subtype: "pdf",
    };
    #[doc = "\\[ASAM\\]\\[Thomas_Thomsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PDX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"PDX\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/PDX\");"]
    #[doc = r" ```"]
    pub const PDX: Mime = Mime {
        ttype: "application",
        subtype: "PDX",
    };
    #[doc = "\\[RFC8555\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PEM_CERTIFICATE_CHAIN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pem-certificate-chain\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pem-certificate-chain\");"]
    #[doc = r" ```"]
    pub const PEM_CERTIFICATE_CHAIN: Mime = Mime {
        ttype: "application",
        subtype: "pem-certificate-chain",
    };
    #[doc = "\\[RFC3156\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PGP_ENCRYPTED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pgp-encrypted\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pgp-encrypted\");"]
    #[doc = r" ```"]
    pub const PGP_ENCRYPTED: Mime = Mime {
        ttype: "application",
        subtype: "pgp-encrypted",
    };
    #[doc = "\\[RFC3156\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PGP_KEYS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pgp-keys\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pgp-keys\");"]
    #[doc = r" ```"]
    pub const PGP_KEYS: Mime = Mime {
        ttype: "application",
        subtype: "pgp-keys",
    };
    #[doc = "\\[RFC3156\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PGP_SIGNATURE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pgp-signature\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pgp-signature\");"]
    #[doc = r" ```"]
    pub const PGP_SIGNATURE: Mime = Mime {
        ttype: "application",
        subtype: "pgp-signature",
    };
    #[doc = "\\[RFC5262\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PIDF_DIFF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pidf-diff+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pidf-diff+xml\");"]
    #[doc = r" ```"]
    pub const PIDF_DIFF_XML: Mime = Mime {
        ttype: "application",
        subtype: "pidf-diff+xml",
    };
    #[doc = "\\[RFC3863\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PIDF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pidf+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pidf+xml\");"]
    #[doc = r" ```"]
    pub const PIDF_XML: Mime = Mime {
        ttype: "application",
        subtype: "pidf+xml",
    };
    #[doc = "\\[RFC5967\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKCS10;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkcs10\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkcs10\");"]
    #[doc = r" ```"]
    pub const PKCS10: Mime = Mime {
        ttype: "application",
        subtype: "pkcs10",
    };
    #[doc = "\\[RFC8551\\]\\[RFC7114\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKCS7_MIME;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkcs7-mime\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkcs7-mime\");"]
    #[doc = r" ```"]
    pub const PKCS7_MIME: Mime = Mime {
        ttype: "application",
        subtype: "pkcs7-mime",
    };
    #[doc = "\\[RFC8551\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKCS7_SIGNATURE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkcs7-signature\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkcs7-signature\");"]
    #[doc = r" ```"]
    pub const PKCS7_SIGNATURE: Mime = Mime {
        ttype: "application",
        subtype: "pkcs7-signature",
    };
    #[doc = "\\[RFC5958\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKCS8;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkcs8\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkcs8\");"]
    #[doc = r" ```"]
    pub const PKCS8: Mime = Mime {
        ttype: "application",
        subtype: "pkcs8",
    };
    #[doc = "\\[RFC8351\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKCS8_ENCRYPTED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkcs8-encrypted\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkcs8-encrypted\");"]
    #[doc = r" ```"]
    pub const PKCS8_ENCRYPTED: Mime = Mime {
        ttype: "application",
        subtype: "pkcs8-encrypted",
    };
    #[doc = "\\[IETF\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKCS12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkcs12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkcs12\");"]
    #[doc = r" ```"]
    pub const PKCS12: Mime = Mime {
        ttype: "application",
        subtype: "pkcs12",
    };
    #[doc = "\\[RFC5877\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKIX_ATTR_CERT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkix-attr-cert\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkix-attr-cert\");"]
    #[doc = r" ```"]
    pub const PKIX_ATTR_CERT: Mime = Mime {
        ttype: "application",
        subtype: "pkix-attr-cert",
    };
    #[doc = "\\[RFC2585\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKIX_CERT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkix-cert\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkix-cert\");"]
    #[doc = r" ```"]
    pub const PKIX_CERT: Mime = Mime {
        ttype: "application",
        subtype: "pkix-cert",
    };
    #[doc = "\\[RFC2585\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKIX_CRL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkix-crl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkix-crl\");"]
    #[doc = r" ```"]
    pub const PKIX_CRL: Mime = Mime {
        ttype: "application",
        subtype: "pkix-crl",
    };
    #[doc = "\\[RFC6066\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKIX_PKIPATH;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkix-pkipath\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkix-pkipath\");"]
    #[doc = r" ```"]
    pub const PKIX_PKIPATH: Mime = Mime {
        ttype: "application",
        subtype: "pkix-pkipath",
    };
    #[doc = "\\[RFC2510\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PKIXCMP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pkixcmp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pkixcmp\");"]
    #[doc = r" ```"]
    pub const PKIXCMP: Mime = Mime {
        ttype: "application",
        subtype: "pkixcmp",
    };
    #[doc = "\\[RFC4267\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PLS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pls+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pls+xml\");"]
    #[doc = r" ```"]
    pub const PLS_XML: Mime = Mime {
        ttype: "application",
        subtype: "pls+xml",
    };
    #[doc = "\\[RFC4354\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::POC_SETTINGS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"poc-settings+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/poc-settings+xml\");"]
    #[doc = r" ```"]
    pub const POC_SETTINGS_XML: Mime = Mime {
        ttype: "application",
        subtype: "poc-settings+xml",
    };
    #[doc = "\\[RFC2045\\]\\[RFC2046\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::POSTSCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"postscript\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/postscript\");"]
    #[doc = r" ```"]
    pub const POSTSCRIPT: Mime = Mime {
        ttype: "application",
        subtype: "postscript",
    };
    #[doc = "\\[RFC7846\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PPSP_TRACKER_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ppsp-tracker+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ppsp-tracker+json\");"]
    #[doc = r" ```"]
    pub const PPSP_TRACKER_JSON: Mime = Mime {
        ttype: "application",
        subtype: "ppsp-tracker+json",
    };
    #[doc = "\\[RFC9578\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRIVATE_TOKEN_ISSUER_DIRECTORY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"private-token-issuer-directory\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/private-token-issuer-directory\");"]
    #[doc = r" ```"]
    pub const PRIVATE_TOKEN_ISSUER_DIRECTORY: Mime = Mime {
        ttype: "application",
        subtype: "private-token-issuer-directory",
    };
    #[doc = "\\[RFC9578\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRIVATE_TOKEN_REQUEST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"private-token-request\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/private-token-request\");"]
    #[doc = r" ```"]
    pub const PRIVATE_TOKEN_REQUEST: Mime = Mime {
        ttype: "application",
        subtype: "private-token-request",
    };
    #[doc = "\\[RFC9578\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRIVATE_TOKEN_RESPONSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"private-token-response\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/private-token-response\");"]
    #[doc = r" ```"]
    pub const PRIVATE_TOKEN_RESPONSE: Mime = Mime {
        ttype: "application",
        subtype: "private-token-response",
    };
    #[doc = "\\[RFC9457\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PROBLEM_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"problem+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/problem+json\");"]
    #[doc = r" ```"]
    pub const PROBLEM_JSON: Mime = Mime {
        ttype: "application",
        subtype: "problem+json",
    };
    #[doc = "\\[RFC9457\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PROBLEM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"problem+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/problem+xml\");"]
    #[doc = r" ```"]
    pub const PROBLEM_XML: Mime = Mime {
        ttype: "application",
        subtype: "problem+xml",
    };
    #[doc = "\\[W3C\\]\\[Ivan_Herman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PROVENANCE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"provenance+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/provenance+xml\");"]
    #[doc = r" ```"]
    pub const PROVENANCE_XML: Mime = Mime {
        ttype: "application",
        subtype: "provenance+xml",
    };
    #[doc = "\\[Harald_T._Alvestrand\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_ALVESTRAND_TITRAX_SHEET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.alvestrand.titrax-sheet\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.alvestrand.titrax-sheet\");"]
    #[doc = r" ```"]
    pub const PRS_ALVESTRAND_TITRAX_SHEET: Mime = Mime {
        ttype: "application",
        subtype: "prs.alvestrand.titrax-sheet",
    };
    #[doc = "\\[Khemchart_Rungchavalnont\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_CWW;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.cww\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.cww\");"]
    #[doc = r" ```"]
    pub const PRS_CWW: Mime = Mime {
        ttype: "application",
        subtype: "prs.cww",
    };
    #[doc = "\\[Cynthia_Revström\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_CYN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.cyn\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.cyn\");"]
    #[doc = r" ```"]
    pub const PRS_CYN: Mime = Mime {
        ttype: "application",
        subtype: "prs.cyn",
    };
    #[doc = "\\[Giulio_Zambon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_HPUB_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.hpub+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.hpub+zip\");"]
    #[doc = r" ```"]
    pub const PRS_HPUB_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "prs.hpub+zip",
    };
    #[doc = "\\[Marek_Čermák\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_IMPLIED_DOCUMENT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.implied-document+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.implied-document+xml\");"]
    #[doc = r" ```"]
    pub const PRS_IMPLIED_DOCUMENT_XML: Mime = Mime {
        ttype: "application",
        subtype: "prs.implied-document+xml",
    };
    #[doc = "\\[Marek_Čermák\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_IMPLIED_EXECUTABLE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.implied-executable\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.implied-executable\");"]
    #[doc = r" ```"]
    pub const PRS_IMPLIED_EXECUTABLE: Mime = Mime {
        ttype: "application",
        subtype: "prs.implied-executable",
    };
    #[doc = "\\[Marek_Čermák\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_IMPLIED_OBJECT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.implied-object+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.implied-object+json\");"]
    #[doc = r" ```"]
    pub const PRS_IMPLIED_OBJECT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "prs.implied-object+json",
    };
    #[doc = "\\[Marek_Čermák\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_IMPLIED_OBJECT_JSON_SEQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.implied-object+json-seq\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.implied-object+json-seq\");"]
    #[doc = r" ```"]
    pub const PRS_IMPLIED_OBJECT_JSON_SEQ: Mime = Mime {
        ttype: "application",
        subtype: "prs.implied-object+json-seq",
    };
    #[doc = "\\[Marek_Čermák\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_IMPLIED_OBJECT_YAML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.implied-object+yaml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.implied-object+yaml\");"]
    #[doc = r" ```"]
    pub const PRS_IMPLIED_OBJECT_YAML: Mime = Mime {
        ttype: "application",
        subtype: "prs.implied-object+yaml",
    };
    #[doc = "\\[Marek_Čermák\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_IMPLIED_STRUCTURE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.implied-structure\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.implied-structure\");"]
    #[doc = r" ```"]
    pub const PRS_IMPLIED_STRUCTURE: Mime = Mime {
        ttype: "application",
        subtype: "prs.implied-structure",
    };
    #[doc = "\\[Azalea_Gardenia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_MAYFILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.mayfile\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.mayfile\");"]
    #[doc = r" ```"]
    pub const PRS_MAYFILE: Mime = Mime {
        ttype: "application",
        subtype: "prs.mayfile",
    };
    #[doc = "\\[Jay_Doggett\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_NPREND;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.nprend\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.nprend\");"]
    #[doc = r" ```"]
    pub const PRS_NPREND: Mime = Mime {
        ttype: "application",
        subtype: "prs.nprend",
    };
    #[doc = "\\[Bill_Janssen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_PLUCKER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.plucker\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.plucker\");"]
    #[doc = r" ```"]
    pub const PRS_PLUCKER: Mime = Mime {
        ttype: "application",
        subtype: "prs.plucker",
    };
    #[doc = "\\[Toby_Inkster\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_RDF_XML_CRYPT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.rdf-xml-crypt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.rdf-xml-crypt\");"]
    #[doc = r" ```"]
    pub const PRS_RDF_XML_CRYPT: Mime = Mime {
        ttype: "application",
        subtype: "prs.rdf-xml-crypt",
    };
    #[doc = "\\[Paolo_Marcheschi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_VCFBZIP2;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.vcfbzip2\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.vcfbzip2\");"]
    #[doc = r" ```"]
    pub const PRS_VCFBZIP2: Mime = Mime {
        ttype: "application",
        subtype: "prs.vcfbzip2",
    };
    #[doc = "\\[Maik_Stührenberg\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PRS_XSF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"prs.xsf+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/prs.xsf+xml\");"]
    #[doc = r" ```"]
    pub const PRS_XSF_XML: Mime = Mime {
        ttype: "application",
        subtype: "prs.xsf+xml",
    };
    #[doc = "\\[RFC6030\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PSKC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pskc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pskc+xml\");"]
    #[doc = r" ```"]
    pub const PSKC_XML: Mime = Mime {
        ttype: "application",
        subtype: "pskc+xml",
    };
    #[doc = "\\[RFC8801\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::PVD_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"pvd+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/pvd+json\");"]
    #[doc = r" ```"]
    pub const PVD_JSON: Mime = Mime {
        ttype: "application",
        subtype: "pvd+json",
    };
    #[doc = "\\[RFC3870\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RDF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rdf+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rdf+xml\");"]
    #[doc = r" ```"]
    pub const RDF_XML: Mime = Mime {
        ttype: "application",
        subtype: "rdf+xml",
    };
    #[doc = "\\[ATSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ROUTE_APD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"route-apd+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/route-apd+xml\");"]
    #[doc = r" ```"]
    pub const ROUTE_APD_XML: Mime = Mime {
        ttype: "application",
        subtype: "route-apd+xml",
    };
    #[doc = "\\[ATSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ROUTE_S_TSID_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"route-s-tsid+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/route-s-tsid+xml\");"]
    #[doc = r" ```"]
    pub const ROUTE_S_TSID_XML: Mime = Mime {
        ttype: "application",
        subtype: "route-s-tsid+xml",
    };
    #[doc = "\\[ATSC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ROUTE_USD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"route-usd+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/route-usd+xml\");"]
    #[doc = r" ```"]
    pub const ROUTE_USD_XML: Mime = Mime {
        ttype: "application",
        subtype: "route-usd+xml",
    };
    #[doc = "\\[RFC3204\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::QSIG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"QSIG\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/QSIG\");"]
    #[doc = r" ```"]
    pub const QSIG: Mime = Mime {
        ttype: "application",
        subtype: "QSIG",
    };
    #[doc = "\\[RFC6682\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RAPTORFEC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"raptorfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/raptorfec\");"]
    #[doc = r" ```"]
    pub const RAPTORFEC: Mime = Mime {
        ttype: "application",
        subtype: "raptorfec",
    };
    #[doc = "\\[RFC9083\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RDAP_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rdap+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rdap+json\");"]
    #[doc = r" ```"]
    pub const RDAP_JSON: Mime = Mime {
        ttype: "application",
        subtype: "rdap+json",
    };
    #[doc = "\\[RFC3680\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::REGINFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"reginfo+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/reginfo+xml\");"]
    #[doc = r" ```"]
    pub const REGINFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "reginfo+xml",
    };
    #[doc = "\\[http://www.JTC_1sc34.org/repository/0661.pdf\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RELAX_NG_COMPACT_SYNTAX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"relax-ng-compact-syntax\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/relax-ng-compact-syntax\");"]
    #[doc = r" ```"]
    pub const RELAX_NG_COMPACT_SYNTAX: Mime = Mime {
        ttype: "application",
        subtype: "relax-ng-compact-syntax",
    };
    #[doc = "\\[RFC1486\\]\\[Marshall_Rose\\]\\[Moving TPC.INT and NSAP.INT infrastructure domains to historic\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::REMOTE_PRINTING;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"remote-printing\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/remote-printing\");"]
    #[doc = r" ```"]
    pub const REMOTE_PRINTING: Mime = Mime {
        ttype: "application",
        subtype: "remote-printing",
    };
    #[doc = "\\[RFC7071\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::REPUTON_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"reputon+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/reputon+json\");"]
    #[doc = r" ```"]
    pub const REPUTON_JSON: Mime = Mime {
        ttype: "application",
        subtype: "reputon+json",
    };
    #[doc = "\\[OpenID_Foundation_Artifact_Binding_WG\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::APPLICATION_RESOLVE_RESPONSE_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"resolve-response+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/resolve-response+jwt\");"]
    #[doc = r" ```"]
    pub const APPLICATION_RESOLVE_RESPONSE_JWT: Mime = Mime {
        ttype: "application",
        subtype: "resolve-response+jwt",
    };
    #[doc = "\\[RFC5362\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RESOURCE_LISTS_DIFF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"resource-lists-diff+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/resource-lists-diff+xml\");"]
    #[doc = r" ```"]
    pub const RESOURCE_LISTS_DIFF_XML: Mime = Mime {
        ttype: "application",
        subtype: "resource-lists-diff+xml",
    };
    #[doc = "\\[RFC4826\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RESOURCE_LISTS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"resource-lists+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/resource-lists+xml\");"]
    #[doc = r" ```"]
    pub const RESOURCE_LISTS_XML: Mime = Mime {
        ttype: "application",
        subtype: "resource-lists+xml",
    };
    #[doc = "\\[RFC7991\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RFC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rfc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rfc+xml\");"]
    #[doc = r" ```"]
    pub const RFC_XML: Mime = Mime {
        ttype: "application",
        subtype: "rfc+xml",
    };
    #[doc = "\\[Nick_Smith\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RISCOS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"riscos\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/riscos\");"]
    #[doc = r" ```"]
    pub const RISCOS: Mime = Mime {
        ttype: "application",
        subtype: "riscos",
    };
    #[doc = "\\[RFC4662\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RLMI_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rlmi+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rlmi+xml\");"]
    #[doc = r" ```"]
    pub const RLMI_XML: Mime = Mime {
        ttype: "application",
        subtype: "rlmi+xml",
    };
    #[doc = "\\[RFC4826\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RLS_SERVICES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rls-services+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rls-services+xml\");"]
    #[doc = r" ```"]
    pub const RLS_SERVICES_XML: Mime = Mime {
        ttype: "application",
        subtype: "rls-services+xml",
    };
    #[doc = "\\[RFC9323\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RPKI_CHECKLIST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rpki-checklist\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rpki-checklist\");"]
    #[doc = r" ```"]
    pub const RPKI_CHECKLIST: Mime = Mime {
        ttype: "application",
        subtype: "rpki-checklist",
    };
    #[doc = "\\[RFC6493\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RPKI_GHOSTBUSTERS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rpki-ghostbusters\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rpki-ghostbusters\");"]
    #[doc = r" ```"]
    pub const RPKI_GHOSTBUSTERS: Mime = Mime {
        ttype: "application",
        subtype: "rpki-ghostbusters",
    };
    #[doc = "\\[RFC6481\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RPKI_MANIFEST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rpki-manifest\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rpki-manifest\");"]
    #[doc = r" ```"]
    pub const RPKI_MANIFEST: Mime = Mime {
        ttype: "application",
        subtype: "rpki-manifest",
    };
    #[doc = "\\[RFC8181\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RPKI_PUBLICATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rpki-publication\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rpki-publication\");"]
    #[doc = r" ```"]
    pub const RPKI_PUBLICATION: Mime = Mime {
        ttype: "application",
        subtype: "rpki-publication",
    };
    #[doc = "\\[RFC9582\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RPKI_ROA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rpki-roa\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rpki-roa\");"]
    #[doc = r" ```"]
    pub const RPKI_ROA: Mime = Mime {
        ttype: "application",
        subtype: "rpki-roa",
    };
    #[doc = "\\[RFC-ietf-sidrops-signed-tal-16\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RPKI_SIGNED_TAL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rpki-signed-tal\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rpki-signed-tal\");"]
    #[doc = r" ```"]
    pub const RPKI_SIGNED_TAL: Mime = Mime {
        ttype: "application",
        subtype: "rpki-signed-tal",
    };
    #[doc = "\\[RFC6492\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RPKI_UPDOWN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rpki-updown\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rpki-updown\");"]
    #[doc = r" ```"]
    pub const RPKI_UPDOWN: Mime = Mime {
        ttype: "application",
        subtype: "rpki-updown",
    };
    #[doc = "\\[Paul_Lindner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RTF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rtf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rtf\");"]
    #[doc = r" ```"]
    pub const RTF: Mime = Mime {
        ttype: "application",
        subtype: "rtf",
    };
    #[doc = "\\[RFC6849\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RTPLOOPBACK;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rtploopback\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rtploopback\");"]
    #[doc = r" ```"]
    pub const RTPLOOPBACK: Mime = Mime {
        ttype: "application",
        subtype: "rtploopback",
    };
    #[doc = "\\[RFC4588\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::RTX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"rtx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/rtx\");"]
    #[doc = r" ```"]
    pub const RTX: Mime = Mime {
        ttype: "application",
        subtype: "rtx",
    };
    #[doc = "\\[OASIS_Security_Services_Technical_Committee_SSTC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SAMLASSERTION_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"samlassertion+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/samlassertion+xml\");"]
    #[doc = r" ```"]
    pub const SAMLASSERTION_XML: Mime = Mime {
        ttype: "application",
        subtype: "samlassertion+xml",
    };
    #[doc = "\\[OASIS_Security_Services_Technical_Committee_SSTC\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SAMLMETADATA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"samlmetadata+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/samlmetadata+xml\");"]
    #[doc = r" ```"]
    pub const SAMLMETADATA_XML: Mime = Mime {
        ttype: "application",
        subtype: "samlmetadata+xml",
    };
    #[doc = "\\[OASIS\\]\\[David_Keaton\\]\\[Michael_C._Fanning\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SARIF_EXTERNAL_PROPERTIES_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sarif-external-properties+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sarif-external-properties+json\");"]
    #[doc = r" ```"]
    pub const SARIF_EXTERNAL_PROPERTIES_JSON: Mime = Mime {
        ttype: "application",
        subtype: "sarif-external-properties+json",
    };
    #[doc = "\\[OASIS\\]\\[Michael_C._Fanning\\]\\[Laurence_J._Golding\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SARIF_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sarif+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sarif+json\");"]
    #[doc = r" ```"]
    pub const SARIF_JSON: Mime = Mime {
        ttype: "application",
        subtype: "sarif+json",
    };
    #[doc = "\\[FIX_Trading_Community\\]\\[Donald_L._Mendelson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SBE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sbe\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sbe\");"]
    #[doc = r" ```"]
    pub const SBE: Mime = Mime {
        ttype: "application",
        subtype: "sbe",
    };
    #[doc = "\\[RFC3823\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SBML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sbml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sbml+xml\");"]
    #[doc = r" ```"]
    pub const SBML_XML: Mime = Mime {
        ttype: "application",
        subtype: "sbml+xml",
    };
    #[doc = "\\[SIS\\]\\[Oskar_Jonsson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SCAIP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"scaip+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/scaip+xml\");"]
    #[doc = r" ```"]
    pub const SCAIP_XML: Mime = Mime {
        ttype: "application",
        subtype: "scaip+xml",
    };
    #[doc = "\\[RFC7644\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SCIM_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"scim+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/scim+json\");"]
    #[doc = r" ```"]
    pub const SCIM_JSON: Mime = Mime {
        ttype: "application",
        subtype: "scim+json",
    };
    #[doc = "\\[RFC5055\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SCVP_CV_REQUEST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"scvp-cv-request\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/scvp-cv-request\");"]
    #[doc = r" ```"]
    pub const SCVP_CV_REQUEST: Mime = Mime {
        ttype: "application",
        subtype: "scvp-cv-request",
    };
    #[doc = "\\[RFC5055\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SCVP_CV_RESPONSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"scvp-cv-response\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/scvp-cv-response\");"]
    #[doc = r" ```"]
    pub const SCVP_CV_RESPONSE: Mime = Mime {
        ttype: "application",
        subtype: "scvp-cv-response",
    };
    #[doc = "\\[RFC5055\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SCVP_VP_REQUEST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"scvp-vp-request\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/scvp-vp-request\");"]
    #[doc = r" ```"]
    pub const SCVP_VP_REQUEST: Mime = Mime {
        ttype: "application",
        subtype: "scvp-vp-request",
    };
    #[doc = "\\[RFC5055\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SCVP_VP_RESPONSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"scvp-vp-response\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/scvp-vp-response\");"]
    #[doc = r" ```"]
    pub const SCVP_VP_RESPONSE: Mime = Mime {
        ttype: "application",
        subtype: "scvp-vp-response",
    };
    #[doc = "\\[RFC8866\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SDP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sdp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sdp\");"]
    #[doc = r" ```"]
    pub const SDP: Mime = Mime {
        ttype: "application",
        subtype: "sdp",
    };
    #[doc = "\\[RFC8417\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SECEVENT_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"secevent+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/secevent+jwt\");"]
    #[doc = r" ```"]
    pub const SECEVENT_JWT: Mime = Mime {
        ttype: "application",
        subtype: "secevent+jwt",
    };
    #[doc = "\\[RFC8790\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SENML_ETCH_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"senml-etch+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/senml-etch+cbor\");"]
    #[doc = r" ```"]
    pub const SENML_ETCH_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "senml-etch+cbor",
    };
    #[doc = "\\[RFC8790\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SENML_ETCH_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"senml-etch+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/senml-etch+json\");"]
    #[doc = r" ```"]
    pub const SENML_ETCH_JSON: Mime = Mime {
        ttype: "application",
        subtype: "senml-etch+json",
    };
    #[doc = "\\[RFC8428\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SENML_EXI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"senml-exi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/senml-exi\");"]
    #[doc = r" ```"]
    pub const SENML_EXI: Mime = Mime {
        ttype: "application",
        subtype: "senml-exi",
    };
    #[doc = "\\[RFC8428\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SENML_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"senml+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/senml+cbor\");"]
    #[doc = r" ```"]
    pub const SENML_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "senml+cbor",
    };
    #[doc = "\\[RFC8428\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SENML_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"senml+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/senml+json\");"]
    #[doc = r" ```"]
    pub const SENML_JSON: Mime = Mime {
        ttype: "application",
        subtype: "senml+json",
    };
    #[doc = "\\[RFC8428\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SENML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"senml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/senml+xml\");"]
    #[doc = r" ```"]
    pub const SENML_XML: Mime = Mime {
        ttype: "application",
        subtype: "senml+xml",
    };
    #[doc = "\\[RFC8428\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SENSML_EXI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sensml-exi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sensml-exi\");"]
    #[doc = r" ```"]
    pub const SENSML_EXI: Mime = Mime {
        ttype: "application",
        subtype: "sensml-exi",
    };
    #[doc = "\\[RFC8428\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SENSML_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sensml+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sensml+cbor\");"]
    #[doc = r" ```"]
    pub const SENSML_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "sensml+cbor",
    };
    #[doc = "\\[RFC8428\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SENSML_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sensml+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sensml+json\");"]
    #[doc = r" ```"]
    pub const SENSML_JSON: Mime = Mime {
        ttype: "application",
        subtype: "sensml+json",
    };
    #[doc = "\\[RFC8428\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SENSML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sensml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sensml+xml\");"]
    #[doc = r" ```"]
    pub const SENSML_XML: Mime = Mime {
        ttype: "application",
        subtype: "sensml+xml",
    };
    #[doc = "\\[Robby_Simpson\\]\\[IEEE\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SEP_EXI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sep-exi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sep-exi\");"]
    #[doc = r" ```"]
    pub const SEP_EXI: Mime = Mime {
        ttype: "application",
        subtype: "sep-exi",
    };
    #[doc = "\\[Robby_Simpson\\]\\[IEEE\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SEP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sep+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sep+xml\");"]
    #[doc = r" ```"]
    pub const SEP_XML: Mime = Mime {
        ttype: "application",
        subtype: "sep+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SESSION_INFO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"session-info\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/session-info\");"]
    #[doc = r" ```"]
    pub const SESSION_INFO: Mime = Mime {
        ttype: "application",
        subtype: "session-info",
    };
    #[doc = "\\[Brian_Korver\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SET_PAYMENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"set-payment\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/set-payment\");"]
    #[doc = r" ```"]
    pub const SET_PAYMENT: Mime = Mime {
        ttype: "application",
        subtype: "set-payment",
    };
    #[doc = "\\[Brian_Korver\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SET_PAYMENT_INITIATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"set-payment-initiation\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/set-payment-initiation\");"]
    #[doc = r" ```"]
    pub const SET_PAYMENT_INITIATION: Mime = Mime {
        ttype: "application",
        subtype: "set-payment-initiation",
    };
    #[doc = "\\[Brian_Korver\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SET_REGISTRATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"set-registration\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/set-registration\");"]
    #[doc = r" ```"]
    pub const SET_REGISTRATION: Mime = Mime {
        ttype: "application",
        subtype: "set-registration",
    };
    #[doc = "\\[Brian_Korver\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SET_REGISTRATION_INITIATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"set-registration-initiation\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/set-registration-initiation\");"]
    #[doc = r" ```"]
    pub const SET_REGISTRATION_INITIATION: Mime = Mime {
        ttype: "application",
        subtype: "set-registration-initiation",
    };
    #[doc = "\\[RFC1874\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SGML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"SGML\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/SGML\");"]
    #[doc = r" ```"]
    pub const SGML: Mime = Mime {
        ttype: "application",
        subtype: "SGML",
    };
    #[doc = "\\[Paul_Grosso\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SGML_OPEN_CATALOG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sgml-open-catalog\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sgml-open-catalog\");"]
    #[doc = r" ```"]
    pub const SGML_OPEN_CATALOG: Mime = Mime {
        ttype: "application",
        subtype: "sgml-open-catalog",
    };
    #[doc = "\\[RFC4194\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SHF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"shf+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/shf+xml\");"]
    #[doc = r" ```"]
    pub const SHF_XML: Mime = Mime {
        ttype: "application",
        subtype: "shf+xml",
    };
    #[doc = "\\[RFC5228\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SIEVE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sieve\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sieve\");"]
    #[doc = r" ```"]
    pub const SIEVE: Mime = Mime {
        ttype: "application",
        subtype: "sieve",
    };
    #[doc = "\\[RFC4661\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SIMPLE_FILTER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"simple-filter+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/simple-filter+xml\");"]
    #[doc = r" ```"]
    pub const SIMPLE_FILTER_XML: Mime = Mime {
        ttype: "application",
        subtype: "simple-filter+xml",
    };
    #[doc = "\\[RFC3842\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SIMPLE_MESSAGE_SUMMARY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"simple-message-summary\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/simple-message-summary\");"]
    #[doc = r" ```"]
    pub const SIMPLE_MESSAGE_SUMMARY: Mime = Mime {
        ttype: "application",
        subtype: "simple-message-summary",
    };
    #[doc = "\\[_3GPP\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SIMPLESYMBOLCONTAINER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"simpleSymbolContainer\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/simpleSymbolContainer\");"]
    #[doc = r" ```"]
    pub const SIMPLESYMBOLCONTAINER: Mime = Mime {
        ttype: "application",
        subtype: "simpleSymbolContainer",
    };
    #[doc = "\\[NCGIS\\]\\[Bryan_Blank\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SIPC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sipc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sipc\");"]
    #[doc = r" ```"]
    pub const SIPC: Mime = Mime {
        ttype: "application",
        subtype: "sipc",
    };
    #[doc = "\\[Terry_Crowley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"slate\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/slate\");"]
    #[doc = r" ```"]
    pub const SLATE: Mime = Mime {
        ttype: "application",
        subtype: "slate",
    };
    #[doc = "\\[RFC4536\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SMIL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"smil\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/smil\");"]
    #[doc = r" ```"]
    pub const SMIL: Mime = Mime {
        ttype: "application",
        subtype: "smil",
    };
    #[doc = "\\[RFC4536\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SMIL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"smil+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/smil+xml\");"]
    #[doc = r" ```"]
    pub const SMIL_XML: Mime = Mime {
        ttype: "application",
        subtype: "smil+xml",
    };
    #[doc = "\\[RFC6597\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SMPTE336M;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"smpte336m\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/smpte336m\");"]
    #[doc = r" ```"]
    pub const SMPTE336M: Mime = Mime {
        ttype: "application",
        subtype: "smpte336m",
    };
    #[doc = "\\[ITU-T_ASN.1_Rapporteur\\]\\[ISO-IEC_JTC_1_SC_6_ASN.1_Rapporteur\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SOAP_FASTINFOSET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"soap+fastinfoset\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/soap+fastinfoset\");"]
    #[doc = r" ```"]
    pub const SOAP_FASTINFOSET: Mime = Mime {
        ttype: "application",
        subtype: "soap+fastinfoset",
    };
    #[doc = "\\[RFC3902\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SOAP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"soap+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/soap+xml\");"]
    #[doc = r" ```"]
    pub const SOAP_XML: Mime = Mime {
        ttype: "application",
        subtype: "soap+xml",
    };
    #[doc = "\\[W3C\\]\\[http://www.w3.org/TR/2007/CR-rdf-sparql-query-20070614/#mediaType\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SPARQL_QUERY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sparql-query\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sparql-query\");"]
    #[doc = r" ```"]
    pub const SPARQL_QUERY: Mime = Mime {
        ttype: "application",
        subtype: "sparql-query",
    };
    #[doc = "\\[Linux_Foundation\\]\\[Rose_Judge\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SPDX_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"spdx+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/spdx+json\");"]
    #[doc = r" ```"]
    pub const SPDX_JSON: Mime = Mime {
        ttype: "application",
        subtype: "spdx+json",
    };
    #[doc = "\\[W3C\\]\\[http://www.w3.org/TR/2007/CR-rdf-sparql-XMLres-20070925/#mime\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SPARQL_RESULTS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sparql-results+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sparql-results+xml\");"]
    #[doc = r" ```"]
    pub const SPARQL_RESULTS_XML: Mime = Mime {
        ttype: "application",
        subtype: "sparql-results+xml",
    };
    #[doc = "\\[RFC3910\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SPIRITS_EVENT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"spirits-event+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/spirits-event+xml\");"]
    #[doc = r" ```"]
    pub const SPIRITS_EVENT_XML: Mime = Mime {
        ttype: "application",
        subtype: "spirits-event+xml",
    };
    #[doc = "\\[RFC6922\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SQL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sql\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sql\");"]
    #[doc = r" ```"]
    pub const SQL: Mime = Mime {
        ttype: "application",
        subtype: "sql",
    };
    #[doc = "\\[RFC4267\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SRGS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"srgs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/srgs\");"]
    #[doc = r" ```"]
    pub const SRGS: Mime = Mime {
        ttype: "application",
        subtype: "srgs",
    };
    #[doc = "\\[RFC4267\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SRGS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"srgs+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/srgs+xml\");"]
    #[doc = r" ```"]
    pub const SRGS_XML: Mime = Mime {
        ttype: "application",
        subtype: "srgs+xml",
    };
    #[doc = "\\[RFC6207\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SRU_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sru+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sru+xml\");"]
    #[doc = r" ```"]
    pub const SRU_XML: Mime = Mime {
        ttype: "application",
        subtype: "sru+xml",
    };
    #[doc = "\\[RFC-ietf-tls-keylogfile-02\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SSLKEYLOGFILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"sslkeylogfile\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/sslkeylogfile\");"]
    #[doc = r" ```"]
    pub const SSLKEYLOGFILE: Mime = Mime {
        ttype: "application",
        subtype: "sslkeylogfile",
    };
    #[doc = "\\[RFC4267\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SSML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ssml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ssml+xml\");"]
    #[doc = r" ```"]
    pub const SSML_XML: Mime = Mime {
        ttype: "application",
        subtype: "ssml+xml",
    };
    #[doc = "\\[SMPTE\\]\\[SMPTE_Director_of_Standards_Development\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ST2110_41;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ST2110-41\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ST2110-41\");"]
    #[doc = r" ```"]
    pub const ST2110_41: Mime = Mime {
        ttype: "application",
        subtype: "ST2110-41",
    };
    #[doc = "\\[OASIS\\]\\[Chet_Ensign\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::STIX_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"stix+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/stix+json\");"]
    #[doc = r" ```"]
    pub const STIX_JSON: Mime = Mime {
        ttype: "application",
        subtype: "stix+json",
    };
    #[doc = "\\[Ben_van_Hartingsveldt\\]\\[1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::STRATUM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"stratum\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/stratum\");"]
    #[doc = r" ```"]
    pub const STRATUM: Mime = Mime {
        ttype: "application",
        subtype: "stratum",
    };
    #[doc = "\\[RFC9393\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SWID_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"swid+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/swid+cbor\");"]
    #[doc = r" ```"]
    pub const SWID_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "swid+cbor",
    };
    #[doc = "\\[ISO-IEC_JTC_1\\]\\[David_Waltermire\\]\\[Ron_Brill\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::SWID_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"swid+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/swid+xml\");"]
    #[doc = r" ```"]
    pub const SWID_XML: Mime = Mime {
        ttype: "application",
        subtype: "swid+xml",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_APEX_UPDATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-apex-update\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-apex-update\");"]
    #[doc = r" ```"]
    pub const TAMP_APEX_UPDATE: Mime = Mime {
        ttype: "application",
        subtype: "tamp-apex-update",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_APEX_UPDATE_CONFIRM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-apex-update-confirm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-apex-update-confirm\");"]
    #[doc = r" ```"]
    pub const TAMP_APEX_UPDATE_CONFIRM: Mime = Mime {
        ttype: "application",
        subtype: "tamp-apex-update-confirm",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_COMMUNITY_UPDATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-community-update\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-community-update\");"]
    #[doc = r" ```"]
    pub const TAMP_COMMUNITY_UPDATE: Mime = Mime {
        ttype: "application",
        subtype: "tamp-community-update",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_COMMUNITY_UPDATE_CONFIRM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-community-update-confirm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-community-update-confirm\");"]
    #[doc = r" ```"]
    pub const TAMP_COMMUNITY_UPDATE_CONFIRM: Mime = Mime {
        ttype: "application",
        subtype: "tamp-community-update-confirm",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_ERROR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-error\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-error\");"]
    #[doc = r" ```"]
    pub const TAMP_ERROR: Mime = Mime {
        ttype: "application",
        subtype: "tamp-error",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_SEQUENCE_ADJUST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-sequence-adjust\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-sequence-adjust\");"]
    #[doc = r" ```"]
    pub const TAMP_SEQUENCE_ADJUST: Mime = Mime {
        ttype: "application",
        subtype: "tamp-sequence-adjust",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_SEQUENCE_ADJUST_CONFIRM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-sequence-adjust-confirm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-sequence-adjust-confirm\");"]
    #[doc = r" ```"]
    pub const TAMP_SEQUENCE_ADJUST_CONFIRM: Mime = Mime {
        ttype: "application",
        subtype: "tamp-sequence-adjust-confirm",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_STATUS_QUERY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-status-query\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-status-query\");"]
    #[doc = r" ```"]
    pub const TAMP_STATUS_QUERY: Mime = Mime {
        ttype: "application",
        subtype: "tamp-status-query",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_STATUS_RESPONSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-status-response\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-status-response\");"]
    #[doc = r" ```"]
    pub const TAMP_STATUS_RESPONSE: Mime = Mime {
        ttype: "application",
        subtype: "tamp-status-response",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_UPDATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-update\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-update\");"]
    #[doc = r" ```"]
    pub const TAMP_UPDATE: Mime = Mime {
        ttype: "application",
        subtype: "tamp-update",
    };
    #[doc = "\\[RFC5934\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAMP_UPDATE_CONFIRM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tamp-update-confirm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tamp-update-confirm\");"]
    #[doc = r" ```"]
    pub const TAMP_UPDATE_CONFIRM: Mime = Mime {
        ttype: "application",
        subtype: "tamp-update-confirm",
    };
    #[doc = "\\[OASIS\\]\\[Chet_Ensign\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TAXII_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"taxii+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/taxii+json\");"]
    #[doc = r" ```"]
    pub const TAXII_JSON: Mime = Mime {
        ttype: "application",
        subtype: "taxii+json",
    };
    #[doc = "\\[W3C\\]\\[Matthias_Kovatsch\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TD_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"td+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/td+json\");"]
    #[doc = r" ```"]
    pub const TD_JSON: Mime = Mime {
        ttype: "application",
        subtype: "td+json",
    };
    #[doc = "\\[RFC6129\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TEI_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tei+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tei+xml\");"]
    #[doc = r" ```"]
    pub const TEI_XML: Mime = Mime {
        ttype: "application",
        subtype: "tei+xml",
    };
    #[doc = "\\[ETSI\\]\\[Miguel_Angel_Reina_Ortega\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TETRA_ISI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"TETRA_ISI\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/TETRA_ISI\");"]
    #[doc = r" ```"]
    pub const TETRA_ISI: Mime = Mime {
        ttype: "application",
        subtype: "TETRA_ISI",
    };
    #[doc = "\\[RFC5941\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::THRAUD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"thraud+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/thraud+xml\");"]
    #[doc = r" ```"]
    pub const THRAUD_XML: Mime = Mime {
        ttype: "application",
        subtype: "thraud+xml",
    };
    #[doc = "\\[RFC3161\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TIMESTAMP_QUERY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"timestamp-query\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/timestamp-query\");"]
    #[doc = r" ```"]
    pub const TIMESTAMP_QUERY: Mime = Mime {
        ttype: "application",
        subtype: "timestamp-query",
    };
    #[doc = "\\[RFC3161\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TIMESTAMP_REPLY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"timestamp-reply\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/timestamp-reply\");"]
    #[doc = r" ```"]
    pub const TIMESTAMP_REPLY: Mime = Mime {
        ttype: "application",
        subtype: "timestamp-reply",
    };
    #[doc = "\\[RFC5955\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TIMESTAMPED_DATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"timestamped-data\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/timestamped-data\");"]
    #[doc = r" ```"]
    pub const TIMESTAMPED_DATA: Mime = Mime {
        ttype: "application",
        subtype: "timestamped-data",
    };
    #[doc = "\\[RFC8460\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TLSRPT_GZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tlsrpt+gzip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tlsrpt+gzip\");"]
    #[doc = r" ```"]
    pub const TLSRPT_GZIP: Mime = Mime {
        ttype: "application",
        subtype: "tlsrpt+gzip",
    };
    #[doc = "\\[RFC8460\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TLSRPT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tlsrpt+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tlsrpt+json\");"]
    #[doc = r" ```"]
    pub const TLSRPT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "tlsrpt+json",
    };
    #[doc = "\\[W3C\\]\\[Sebastian_Kaebisch\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TM_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tm+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tm+json\");"]
    #[doc = r" ```"]
    pub const TM_JSON: Mime = Mime {
        ttype: "application",
        subtype: "tm+json",
    };
    #[doc = "\\[RFC8226\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TNAUTHLIST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tnauthlist\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tnauthlist\");"]
    #[doc = r" ```"]
    pub const TNAUTHLIST: Mime = Mime {
        ttype: "application",
        subtype: "tnauthlist",
    };
    #[doc = "\\[RFC-oauth-jwt-introspection-response-12\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TOKEN_INTROSPECTION_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"token-introspection+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/token-introspection+jwt\");"]
    #[doc = r" ```"]
    pub const TOKEN_INTROSPECTION_JWT: Mime = Mime {
        ttype: "application",
        subtype: "token-introspection+jwt",
    };
    #[doc = "\\[RFC8840\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TRICKLE_ICE_SDPFRAG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"trickle-ice-sdpfrag\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/trickle-ice-sdpfrag\");"]
    #[doc = r" ```"]
    pub const TRICKLE_ICE_SDPFRAG: Mime = Mime {
        ttype: "application",
        subtype: "trickle-ice-sdpfrag",
    };
    #[doc = "\\[W3C\\]\\[W3C_RDF_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TRIG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"trig\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/trig\");"]
    #[doc = r" ```"]
    pub const TRIG: Mime = Mime {
        ttype: "application",
        subtype: "trig",
    };
    #[doc = "\\[OpenID_Foundation_Artifact_Binding_WG\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::APPLICATION_TRUST_CHAIN_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"trust-chain+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/trust-chain+json\");"]
    #[doc = r" ```"]
    pub const APPLICATION_TRUST_CHAIN_JSON: Mime = Mime {
        ttype: "application",
        subtype: "trust-chain+json",
    };
    #[doc = "\\[OpenID_Foundation_Artifact_Binding_WG\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::APPLICATION_TRUST_MARK_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"trust-mark+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/trust-mark+jwt\");"]
    #[doc = r" ```"]
    pub const APPLICATION_TRUST_MARK_JWT: Mime = Mime {
        ttype: "application",
        subtype: "trust-mark+jwt",
    };
    #[doc = "\\[OpenID_Foundation_Artifact_Binding_WG\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::APPLICATION_TRUST_MARK_DELEGATION_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"trust-mark-delegation+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/trust-mark-delegation+jwt\");"]
    #[doc = r" ```"]
    pub const APPLICATION_TRUST_MARK_DELEGATION_JWT: Mime = Mime {
        ttype: "application",
        subtype: "trust-mark-delegation+jwt",
    };
    #[doc = "\\[W3C\\]\\[W3C_Timed_Text_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TTML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ttml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ttml+xml\");"]
    #[doc = r" ```"]
    pub const TTML_XML: Mime = Mime {
        ttype: "application",
        subtype: "ttml+xml",
    };
    #[doc = "\\[Linda_Welsh\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TVE_TRIGGER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tve-trigger\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tve-trigger\");"]
    #[doc = r" ```"]
    pub const TVE_TRIGGER: Mime = Mime {
        ttype: "application",
        subtype: "tve-trigger",
    };
    #[doc = "\\[RFC-murchison-rfc8536bis-15\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TZIF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tzif\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tzif\");"]
    #[doc = r" ```"]
    pub const TZIF: Mime = Mime {
        ttype: "application",
        subtype: "tzif",
    };
    #[doc = "\\[RFC-murchison-rfc8536bis-15\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::TZIF_LEAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"tzif-leap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/tzif-leap\");"]
    #[doc = r" ```"]
    pub const TZIF_LEAP: Mime = Mime {
        ttype: "application",
        subtype: "tzif-leap",
    };
    #[doc = "\\[RFC5109\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ULPFEC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"ulpfec\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/ulpfec\");"]
    #[doc = r" ```"]
    pub const ULPFEC: Mime = Mime {
        ttype: "application",
        subtype: "ulpfec",
    };
    #[doc = "\\[Gottfried_Zimmermann\\]\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::URC_GRPSHEET_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"urc-grpsheet+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/urc-grpsheet+xml\");"]
    #[doc = r" ```"]
    pub const URC_GRPSHEET_XML: Mime = Mime {
        ttype: "application",
        subtype: "urc-grpsheet+xml",
    };
    #[doc = "\\[Gottfried_Zimmermann\\]\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::URC_RESSHEET_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"urc-ressheet+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/urc-ressheet+xml\");"]
    #[doc = r" ```"]
    pub const URC_RESSHEET_XML: Mime = Mime {
        ttype: "application",
        subtype: "urc-ressheet+xml",
    };
    #[doc = "\\[Gottfried_Zimmermann\\]\\[ISO-IEC_JTC_1\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::URC_TARGETDESC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"urc-targetdesc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/urc-targetdesc+xml\");"]
    #[doc = r" ```"]
    pub const URC_TARGETDESC_XML: Mime = Mime {
        ttype: "application",
        subtype: "urc-targetdesc+xml",
    };
    #[doc = "\\[Gottfried_Zimmermann\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::URC_UISOCKETDESC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"urc-uisocketdesc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/urc-uisocketdesc+xml\");"]
    #[doc = r" ```"]
    pub const URC_UISOCKETDESC_XML: Mime = Mime {
        ttype: "application",
        subtype: "urc-uisocketdesc+xml",
    };
    #[doc = "\\[W3C\\]\\[Ivan_Herman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vc\");"]
    #[doc = r" ```"]
    pub const VC: Mime = Mime {
        ttype: "application",
        subtype: "vc",
    };
    #[doc = "\\[RFC7095\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VCARD_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vcard+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vcard+json\");"]
    #[doc = r" ```"]
    pub const VCARD_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vcard+json",
    };
    #[doc = "\\[RFC6351\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VCARD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vcard+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vcard+xml\");"]
    #[doc = r" ```"]
    pub const VCARD_XML: Mime = Mime {
        ttype: "application",
        subtype: "vcard+xml",
    };
    #[doc = "\\[RFC2122\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VEMMI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vemmi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vemmi\");"]
    #[doc = r" ```"]
    pub const VEMMI: Mime = Mime {
        ttype: "application",
        subtype: "vemmi",
    };
    #[doc = "\\[Franz_Ombler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_1000MINDS_DECISION_MODEL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.1000minds.decision-model+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.1000minds.decision-model+xml\");"]
    #[doc = r" ```"]
    pub const VND_1000MINDS_DECISION_MODEL_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.1000minds.decision-model+xml",
    };
    #[doc = "\\[Rob_Coyle\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_1OB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.1ob\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.1ob\");"]
    #[doc = r" ```"]
    pub const VND_1OB: Mime = Mime {
        ttype: "application",
        subtype: "vnd.1ob",
    };
    #[doc = "\\[_3GPP\\]\\[Jones_Lu_Yunjie\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_5GNAS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.5gnas\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.5gnas\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_5GNAS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.5gnas",
    };
    #[doc = "\\[_3GPP_CT1\\]\\[Dongwook_Kim\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_5GSA2X;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.5gsa2x\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.5gsa2x\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_5GSA2X: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.5gsa2x",
    };
    #[doc = "\\[_3GPP_CT1\\]\\[Dongwook_Kim\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_5GSA2X_LOCAL_SERVICE_INFORMATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.5gsa2x-local-service-information\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.5gsa2x-local-service-information\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_5GSA2X_LOCAL_SERVICE_INFORMATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.5gsa2x-local-service-information",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_ACCESS_TRANSFER_EVENTS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.access-transfer-events+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.access-transfer-events+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_ACCESS_TRANSFER_EVENTS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.access-transfer-events+xml",
    };
    #[doc = "\\[John_M_Meredith\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_BSF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.bsf+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.bsf+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_BSF_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.bsf+xml",
    };
    #[doc = "\\[Xu_Chen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_CRS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.crs+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.crs+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_CRS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.crs+xml",
    };
    #[doc = "\\[Peter_Leis\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_CURRENT_LOCATION_DISCOVERY_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.current-location-discovery+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.current-location-discovery+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_CURRENT_LOCATION_DISCOVERY_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.current-location-discovery+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_GMOP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.GMOP+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.GMOP+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_GMOP_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.GMOP+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Yang_Yong\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_GTPC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.gtpc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.gtpc\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_GTPC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.gtpc",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_INTERWORKING_DATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.interworking-data\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.interworking-data\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_INTERWORKING_DATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.interworking-data",
    };
    #[doc = "\\[_3GPP\\]\\[Jones_Lu_Yunjie\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_LPP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.lpp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.lpp\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_LPP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.lpp",
    };
    #[doc = "\\[Tim_Woodward\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MC_SIGNALLING_EAR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mc-signalling-ear\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mc-signalling-ear\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MC_SIGNALLING_EAR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mc-signalling-ear",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCDATA_AFFILIATION_COMMAND_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcdata-affiliation-command+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcdata-affiliation-command+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCDATA_AFFILIATION_COMMAND_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcdata-affiliation-command+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCDATA_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcdata-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcdata-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCDATA_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcdata-info+xml",
    };
    #[doc = "\\[Kiran_Kapale\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCDATA_MSGSTORE_CTRL_REQUEST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcdata-msgstore-ctrl-request+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcdata-msgstore-ctrl-request+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCDATA_MSGSTORE_CTRL_REQUEST_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcdata-msgstore-ctrl-request+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCDATA_PAYLOAD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcdata-payload\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcdata-payload\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCDATA_PAYLOAD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcdata-payload",
    };
    #[doc = "\\[Kiran_Kapale\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCDATA_REGROUP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcdata-regroup+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcdata-regroup+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCDATA_REGROUP_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcdata-regroup+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCDATA_SERVICE_CONFIG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcdata-service-config+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcdata-service-config+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCDATA_SERVICE_CONFIG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcdata-service-config+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCDATA_SIGNALLING;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcdata-signalling\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcdata-signalling\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCDATA_SIGNALLING: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcdata-signalling",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCDATA_UE_CONFIG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcdata-ue-config+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcdata-ue-config+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCDATA_UE_CONFIG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcdata-ue-config+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCDATA_USER_PROFILE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcdata-user-profile+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcdata-user-profile+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCDATA_USER_PROFILE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcdata-user-profile+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_AFFILIATION_COMMAND_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-affiliation-command+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-affiliation-command+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_AFFILIATION_COMMAND_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-affiliation-command+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_FLOOR_REQUEST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-floor-request+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-floor-request+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_FLOOR_REQUEST_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-floor-request+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-info+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_LOCATION_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-location-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-location-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_LOCATION_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-location-info+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_MBMS_USAGE_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-mbms-usage-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-mbms-usage-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_MBMS_USAGE_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-mbms-usage-info+xml",
    };
    #[doc = "\\[Kiran_Kapale\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_REGROUP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-regroup+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-regroup+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_REGROUP_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-regroup+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_SERVICE_CONFIG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-service-config+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-service-config+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_SERVICE_CONFIG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-service-config+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_SIGNED_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-signed+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-signed+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_SIGNED_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-signed+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_UE_CONFIG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-ue-config+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-ue-config+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_UE_CONFIG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-ue-config+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_UE_INIT_CONFIG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-ue-init-config+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-ue-init-config+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_UE_INIT_CONFIG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-ue-init-config+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCPTT_USER_PROFILE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcptt-user-profile+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcptt-user-profile+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCPTT_USER_PROFILE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcptt-user-profile+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCVIDEO_AFFILIATION_COMMAND_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcvideo-affiliation-command+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcvideo-affiliation-command+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCVIDEO_AFFILIATION_COMMAND_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcvideo-affiliation-command+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCVIDEO_AFFILIATION_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcvideo-affiliation-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcvideo-affiliation-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCVIDEO_AFFILIATION_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcvideo-affiliation-info+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCVIDEO_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcvideo-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcvideo-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCVIDEO_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcvideo-info+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCVIDEO_LOCATION_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcvideo-location-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcvideo-location-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCVIDEO_LOCATION_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcvideo-location-info+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCVIDEO_MBMS_USAGE_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcvideo-mbms-usage-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcvideo-mbms-usage-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCVIDEO_MBMS_USAGE_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcvideo-mbms-usage-info+xml",
    };
    #[doc = "\\[Kiran_Kapale\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCVIDEO_REGROUP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcvideo-regroup+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcvideo-regroup+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCVIDEO_REGROUP_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcvideo-regroup+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCVIDEO_SERVICE_CONFIG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcvideo-service-config+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcvideo-service-config+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCVIDEO_SERVICE_CONFIG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcvideo-service-config+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCVIDEO_TRANSMISSION_REQUEST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcvideo-transmission-request+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcvideo-transmission-request+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCVIDEO_TRANSMISSION_REQUEST_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcvideo-transmission-request+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCVIDEO_UE_CONFIG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcvideo-ue-config+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcvideo-ue-config+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCVIDEO_UE_CONFIG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcvideo-ue-config+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MCVIDEO_USER_PROFILE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mcvideo-user-profile+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mcvideo-user-profile+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MCVIDEO_USER_PROFILE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mcvideo-user-profile+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_MID_CALL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.mid-call+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.mid-call+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_MID_CALL_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.mid-call+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Yang_Yong\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_NGAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.ngap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.ngap\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_NGAP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.ngap",
    };
    #[doc = "\\[_3GPP\\]\\[Bruno_Landais\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_PFCP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.pfcp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.pfcp\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_PFCP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.pfcp",
    };
    #[doc = "\\[John_M_Meredith\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_PIC_BW_LARGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.pic-bw-large\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.pic-bw-large\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_PIC_BW_LARGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.pic-bw-large",
    };
    #[doc = "\\[John_M_Meredith\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_PIC_BW_SMALL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.pic-bw-small\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.pic-bw-small\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_PIC_BW_SMALL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.pic-bw-small",
    };
    #[doc = "\\[John_M_Meredith\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_PIC_BW_VAR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.pic-bw-var\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.pic-bw-var\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_PIC_BW_VAR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.pic-bw-var",
    };
    #[doc = "\\[_3GPP_CT1\\]\\[Dongwook_Kim\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_PINAPP_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.pinapp-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.pinapp-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_PINAPP_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.pinapp-info+xml",
    };
    #[doc = "\\[Haorui_Yang\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_PROSE_PC3A_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp-prose-pc3a+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp-prose-pc3a+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_PROSE_PC3A_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp-prose-pc3a+xml",
    };
    #[doc = "\\[Haorui_Yang\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_PROSE_PC3ACH_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp-prose-pc3ach+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp-prose-pc3ach+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_PROSE_PC3ACH_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp-prose-pc3ach+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_PROSE_PC3CH_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp-prose-pc3ch+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp-prose-pc3ch+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_PROSE_PC3CH_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp-prose-pc3ch+xml",
    };
    #[doc = "\\[Haorui_Yang\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_PROSE_PC8_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp-prose-pc8+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp-prose-pc8+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_PROSE_PC8_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp-prose-pc8+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_PROSE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp-prose+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp-prose+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_PROSE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp-prose+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Yang_Yong\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_S1AP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.s1ap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.s1ap\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_S1AP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.s1ap",
    };
    #[doc = "\\[Sapan_Shah\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SEAL_GROUP_DOC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.seal-group-doc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.seal-group-doc+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SEAL_GROUP_DOC_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.seal-group-doc+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Christian_Herrero-Veron\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SEAL_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.seal-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.seal-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SEAL_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.seal-info+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Christian_Herrero-Veron\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SEAL_LOCATION_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.seal-location-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.seal-location-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SEAL_LOCATION_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.seal-location-info+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Christian_Herrero-Veron\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SEAL_MBMS_USAGE_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.seal-mbms-usage-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.seal-mbms-usage-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SEAL_MBMS_USAGE_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.seal-mbms-usage-info+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Christian_Herrero-Veron\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SEAL_NETWORK_QOS_MANAGEMENT_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.seal-network-QoS-management-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.seal-network-QoS-management-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SEAL_NETWORK_QOS_MANAGEMENT_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.seal-network-QoS-management-info+xml",
    };
    #[doc = "\\[Sapan_Shah\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SEAL_UE_CONFIG_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.seal-ue-config-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.seal-ue-config-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SEAL_UE_CONFIG_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.seal-ue-config-info+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Christian_Herrero-Veron\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SEAL_UNICAST_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.seal-unicast-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.seal-unicast-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SEAL_UNICAST_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.seal-unicast-info+xml",
    };
    #[doc = "\\[Sapan_Shah\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SEAL_USER_PROFILE_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.seal-user-profile-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.seal-user-profile-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SEAL_USER_PROFILE_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.seal-user-profile-info+xml",
    };
    #[doc = "\\[John_M_Meredith\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SMS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.sms\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.sms\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SMS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.sms",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SMS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.sms+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.sms+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SMS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.sms+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SRVCC_EXT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.srvcc-ext+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.srvcc-ext+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SRVCC_EXT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.srvcc-ext+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_SRVCC_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.SRVCC-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.SRVCC-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_SRVCC_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.SRVCC-info+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_STATE_AND_EVENT_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.state-and-event-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.state-and-event-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_STATE_AND_EVENT_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.state-and-event-info+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_USSD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.ussd+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.ussd+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_USSD_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.ussd+xml",
    };
    #[doc = "\\[_3GPP\\]\\[Christian_Herrero-Veron\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_VAE_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.vae-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.vae-info+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_VAE_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.vae-info+xml",
    };
    #[doc = "\\[Frederic_Firmin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_V2X_LOCAL_SERVICE_INFORMATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp-v2x-local-service-information\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp-v2x-local-service-information\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_V2X_LOCAL_SERVICE_INFORMATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp-v2x-local-service-information",
    };
    #[doc = "\\[AC_Mahendran\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP2_BCMCSINFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp2.bcmcsinfo+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp2.bcmcsinfo+xml\");"]
    #[doc = r" ```"]
    pub const VND_3GPP2_BCMCSINFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp2.bcmcsinfo+xml",
    };
    #[doc = "\\[AC_Mahendran\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP2_SMS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp2.sms\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp2.sms\");"]
    #[doc = r" ```"]
    pub const VND_3GPP2_SMS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp2.sms",
    };
    #[doc = "\\[AC_Mahendran\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP2_TCAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp2.tcap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp2.tcap\");"]
    #[doc = r" ```"]
    pub const VND_3GPP2_TCAP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp2.tcap",
    };
    #[doc = "\\[Sang_Min_Park\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3GPP_V2X;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3gpp.v2x\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3gpp.v2x\");"]
    #[doc = r" ```"]
    pub const VND_3GPP_V2X: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3gpp.v2x",
    };
    #[doc = "\\[Gus_Asadi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3LIGHTSSOFTWARE_IMAGESCAL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3lightssoftware.imagescal\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3lightssoftware.imagescal\");"]
    #[doc = r" ```"]
    pub const VND_3LIGHTSSOFTWARE_IMAGESCAL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3lightssoftware.imagescal",
    };
    #[doc = "\\[Michael_OBrien\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_3M_POST_IT_NOTES;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.3M.Post-it-Notes\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.3M.Post-it-Notes\");"]
    #[doc = r" ```"]
    pub const VND_3M_POST_IT_NOTES: Mime = Mime {
        ttype: "application",
        subtype: "vnd.3M.Post-it-Notes",
    };
    #[doc = "\\[Steve_Leow\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ACCPAC_SIMPLY_ASO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.accpac.simply.aso\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.accpac.simply.aso\");"]
    #[doc = r" ```"]
    pub const VND_ACCPAC_SIMPLY_ASO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.accpac.simply.aso",
    };
    #[doc = "\\[Steve_Leow\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ACCPAC_SIMPLY_IMP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.accpac.simply.imp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.accpac.simply.imp\");"]
    #[doc = r" ```"]
    pub const VND_ACCPAC_SIMPLY_IMP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.accpac.simply.imp",
    };
    #[doc = "\\[Sridhar_Ramakrishnan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ACM_ADDRESSXFER_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.acm.addressxfer+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.acm.addressxfer+json\");"]
    #[doc = r" ```"]
    pub const VND_ACM_ADDRESSXFER_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.acm.addressxfer+json",
    };
    #[doc = "\\[Sridhar_Ramakrishnan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ACM_CHATBOT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.acm.chatbot+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.acm.chatbot+json\");"]
    #[doc = r" ```"]
    pub const VND_ACM_CHATBOT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.acm.chatbot+json",
    };
    #[doc = "\\[Dovid_Lubin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ACUCOBOL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.acucobol\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.acucobol\");"]
    #[doc = r" ```"]
    pub const VND_ACUCOBOL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.acucobol",
    };
    #[doc = "\\[Dovid_Lubin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ACUCORP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.acucorp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.acucorp\");"]
    #[doc = r" ```"]
    pub const VND_ACUCORP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.acucorp",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ADOBE_FLASH_MOVIE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.adobe.flash.movie\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.adobe.flash.movie\");"]
    #[doc = r" ```"]
    pub const VND_ADOBE_FLASH_MOVIE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.adobe.flash.movie",
    };
    #[doc = "\\[Chris_Solc\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ADOBE_FORMSCENTRAL_FCDT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.adobe.formscentral.fcdt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.adobe.formscentral.fcdt\");"]
    #[doc = r" ```"]
    pub const VND_ADOBE_FORMSCENTRAL_FCDT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.adobe.formscentral.fcdt",
    };
    #[doc = "\\[Steven_Heintz\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ADOBE_FXP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.adobe.fxp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.adobe.fxp\");"]
    #[doc = r" ```"]
    pub const VND_ADOBE_FXP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.adobe.fxp",
    };
    #[doc = "\\[Tapani_Otala\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ADOBE_PARTIAL_UPLOAD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.adobe.partial-upload\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.adobe.partial-upload\");"]
    #[doc = r" ```"]
    pub const VND_ADOBE_PARTIAL_UPLOAD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.adobe.partial-upload",
    };
    #[doc = "\\[John_Brinkman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ADOBE_XDP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.adobe.xdp+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.adobe.xdp+xml\");"]
    #[doc = r" ```"]
    pub const VND_ADOBE_XDP_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.adobe.xdp+xml",
    };
    #[doc = "\\[Jay_Moskowitz\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AETHER_IMP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.aether.imp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.aether.imp\");"]
    #[doc = r" ```"]
    pub const VND_AETHER_IMP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.aether.imp",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_AFPLINEDATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.afplinedata\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.afplinedata\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_AFPLINEDATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.afplinedata",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_AFPLINEDATA_PAGEDEF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.afplinedata-pagedef\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.afplinedata-pagedef\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_AFPLINEDATA_PAGEDEF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.afplinedata-pagedef",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_CMOCA_CMRESOURCE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.cmoca-cmresource\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.cmoca-cmresource\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_CMOCA_CMRESOURCE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.cmoca-cmresource",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_FOCA_CHARSET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.foca-charset\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.foca-charset\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_FOCA_CHARSET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.foca-charset",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_FOCA_CODEDFONT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.foca-codedfont\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.foca-codedfont\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_FOCA_CODEDFONT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.foca-codedfont",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_FOCA_CODEPAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.foca-codepage\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.foca-codepage\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_FOCA_CODEPAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.foca-codepage",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_MODCA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.modca\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.modca\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_MODCA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.modca",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_MODCA_CMTABLE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.modca-cmtable\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.modca-cmtable\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_MODCA_CMTABLE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.modca-cmtable",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_MODCA_FORMDEF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.modca-formdef\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.modca-formdef\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_MODCA_FORMDEF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.modca-formdef",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_MODCA_MEDIUMMAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.modca-mediummap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.modca-mediummap\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_MODCA_MEDIUMMAP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.modca-mediummap",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_MODCA_OBJECTCONTAINER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.modca-objectcontainer\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.modca-objectcontainer\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_MODCA_OBJECTCONTAINER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.modca-objectcontainer",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_MODCA_OVERLAY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.modca-overlay\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.modca-overlay\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_MODCA_OVERLAY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.modca-overlay",
    };
    #[doc = "\\[Jörg_Palmer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AFPC_MODCA_PAGESEGMENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.afpc.modca-pagesegment\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.afpc.modca-pagesegment\");"]
    #[doc = r" ```"]
    pub const VND_AFPC_MODCA_PAGESEGMENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.afpc.modca-pagesegment",
    };
    #[doc = "\\[Filippo_Valsorda\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.age\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.age\");"]
    #[doc = r" ```"]
    pub const VND_AGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.age",
    };
    #[doc = "\\[Katsuhiko_Ichinose\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AH_BARCODE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ah-barcode\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ah-barcode\");"]
    #[doc = r" ```"]
    pub const VND_AH_BARCODE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ah-barcode",
    };
    #[doc = "\\[Tor_Kristensen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AHEAD_SPACE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ahead.space\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ahead.space\");"]
    #[doc = r" ```"]
    pub const VND_AHEAD_SPACE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ahead.space",
    };
    #[doc = "\\[Daniel_Mould\\]\\[Gary_Clueit\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AIRZIP_FILESECURE_AZF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.airzip.filesecure.azf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.airzip.filesecure.azf\");"]
    #[doc = r" ```"]
    pub const VND_AIRZIP_FILESECURE_AZF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.airzip.filesecure.azf",
    };
    #[doc = "\\[Daniel_Mould\\]\\[Gary_Clueit\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AIRZIP_FILESECURE_AZS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.airzip.filesecure.azs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.airzip.filesecure.azs\");"]
    #[doc = r" ```"]
    pub const VND_AIRZIP_FILESECURE_AZS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.airzip.filesecure.azs",
    };
    #[doc = "\\[Patrick_Brosse\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AMADEUS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.amadeus+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.amadeus+json\");"]
    #[doc = r" ```"]
    pub const VND_AMADEUS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.amadeus+json",
    };
    #[doc = "\\[Kim_Scarborough\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AMAZON_MOBI8_EBOOK;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.amazon.mobi8-ebook\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.amazon.mobi8-ebook\");"]
    #[doc = r" ```"]
    pub const VND_AMAZON_MOBI8_EBOOK: Mime = Mime {
        ttype: "application",
        subtype: "vnd.amazon.mobi8-ebook",
    };
    #[doc = "\\[Gary_Sands\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AMERICANDYNAMICS_ACC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.americandynamics.acc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.americandynamics.acc\");"]
    #[doc = r" ```"]
    pub const VND_AMERICANDYNAMICS_ACC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.americandynamics.acc",
    };
    #[doc = "\\[Kevin_Blumberg\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AMIGA_AMI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.amiga.ami\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.amiga.ami\");"]
    #[doc = r" ```"]
    pub const VND_AMIGA_AMI: Mime = Mime {
        ttype: "application",
        subtype: "vnd.amiga.ami",
    };
    #[doc = "\\[Mike_Amundsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AMUNDSEN_MAZE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.amundsen.maze+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.amundsen.maze+xml\");"]
    #[doc = r" ```"]
    pub const VND_AMUNDSEN_MAZE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.amundsen.maze+xml",
    };
    #[doc = "\\[Greg_Kaiser\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ANDROID_OTA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.android.ota\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.android.ota\");"]
    #[doc = r" ```"]
    pub const VND_ANDROID_OTA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.android.ota",
    };
    #[doc = "\\[Kerrick_Staley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ANKI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.anki\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.anki\");"]
    #[doc = r" ```"]
    pub const VND_ANKI: Mime = Mime {
        ttype: "application",
        subtype: "vnd.anki",
    };
    #[doc = "\\[Hiroyoshi_Mori\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ANSER_WEB_CERTIFICATE_ISSUE_INITIATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.anser-web-certificate-issue-initiation\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.anser-web-certificate-issue-initiation\");"]
    #[doc = r" ```"]
    pub const VND_ANSER_WEB_CERTIFICATE_ISSUE_INITIATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.anser-web-certificate-issue-initiation",
    };
    #[doc = "\\[Daniel_Shelton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ANTIX_GAME_COMPONENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.antix.game-component\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.antix.game-component\");"]
    #[doc = r" ```"]
    pub const VND_ANTIX_GAME_COMPONENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.antix.game-component",
    };
    #[doc = "\\[Apache_Arrow_Project\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APACHE_ARROW_FILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apache.arrow.file\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apache.arrow.file\");"]
    #[doc = r" ```"]
    pub const VND_APACHE_ARROW_FILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apache.arrow.file",
    };
    #[doc = "\\[Apache_Arrow_Project\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APACHE_ARROW_STREAM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apache.arrow.stream\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apache.arrow.stream\");"]
    #[doc = r" ```"]
    pub const VND_APACHE_ARROW_STREAM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apache.arrow.stream",
    };
    #[doc = "\\[Apache_Parquet_Project\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APACHE_PARQUET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apache.parquet\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apache.parquet\");"]
    #[doc = r" ```"]
    pub const VND_APACHE_PARQUET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apache.parquet",
    };
    #[doc = "\\[Roger_Meier\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APACHE_THRIFT_BINARY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apache.thrift.binary\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apache.thrift.binary\");"]
    #[doc = r" ```"]
    pub const VND_APACHE_THRIFT_BINARY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apache.thrift.binary",
    };
    #[doc = "\\[Roger_Meier\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APACHE_THRIFT_COMPACT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apache.thrift.compact\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apache.thrift.compact\");"]
    #[doc = r" ```"]
    pub const VND_APACHE_THRIFT_COMPACT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apache.thrift.compact",
    };
    #[doc = "\\[Roger_Meier\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APACHE_THRIFT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apache.thrift.json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apache.thrift.json\");"]
    #[doc = r" ```"]
    pub const VND_APACHE_THRIFT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apache.thrift.json",
    };
    #[doc = "\\[Fawad_Shaikh\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APEXLANG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apexlang\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apexlang\");"]
    #[doc = r" ```"]
    pub const VND_APEXLANG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apexlang",
    };
    #[doc = "\\[Steve_Klabnik\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_API_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.api+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.api+json\");"]
    #[doc = r" ```"]
    pub const VND_API_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.api+json",
    };
    #[doc = "\\[Oleg_Uryutin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APLEXTOR_WARRP_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.aplextor.warrp+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.aplextor.warrp+json\");"]
    #[doc = r" ```"]
    pub const VND_APLEXTOR_WARRP_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.aplextor.warrp+json",
    };
    #[doc = "\\[Adrian_Föder\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APOTHEKENDE_RESERVATION_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apothekende.reservation+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apothekende.reservation+json\");"]
    #[doc = r" ```"]
    pub const VND_APOTHEKENDE_RESERVATION_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apothekende.reservation+json",
    };
    #[doc = "\\[Peter_Bierman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APPLE_INSTALLER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apple.installer+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apple.installer+xml\");"]
    #[doc = r" ```"]
    pub const VND_APPLE_INSTALLER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apple.installer+xml",
    };
    #[doc = "\\[Manichandra_Sajjanapu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APPLE_KEYNOTE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apple.keynote\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apple.keynote\");"]
    #[doc = r" ```"]
    pub const VND_APPLE_KEYNOTE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apple.keynote",
    };
    #[doc = "\\[RFC8216\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APPLE_MPEGURL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apple.mpegurl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apple.mpegurl\");"]
    #[doc = r" ```"]
    pub const VND_APPLE_MPEGURL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apple.mpegurl",
    };
    #[doc = "\\[Manichandra_Sajjanapu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APPLE_NUMBERS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apple.numbers\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apple.numbers\");"]
    #[doc = r" ```"]
    pub const VND_APPLE_NUMBERS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apple.numbers",
    };
    #[doc = "\\[Manichandra_Sajjanapu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_APPLE_PAGES;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.apple.pages\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.apple.pages\");"]
    #[doc = r" ```"]
    pub const VND_APPLE_PAGES: Mime = Mime {
        ttype: "application",
        subtype: "vnd.apple.pages",
    };
    #[doc = "\\[Bill_Fenner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ARASTRA_SWI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.arastra.swi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.arastra.swi\");"]
    #[doc = r" ```"]
    pub const VND_ARASTRA_SWI: Mime = Mime {
        ttype: "application",
        subtype: "vnd.arastra.swi",
    };
    #[doc = "\\[Bill_Fenner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ARISTANETWORKS_SWI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.aristanetworks.swi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.aristanetworks.swi\");"]
    #[doc = r" ```"]
    pub const VND_ARISTANETWORKS_SWI: Mime = Mime {
        ttype: "application",
        subtype: "vnd.aristanetworks.swi",
    };
    #[doc = "\\[Brad_Turner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ARTISAN_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.artisan+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.artisan+json\");"]
    #[doc = r" ```"]
    pub const VND_ARTISAN_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.artisan+json",
    };
    #[doc = "\\[Christopher_Smith\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ARTSQUARE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.artsquare\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.artsquare\");"]
    #[doc = r" ```"]
    pub const VND_ARTSQUARE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.artsquare",
    };
    #[doc = "\\[Christopher_Snazell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ASTRAEA_SOFTWARE_IOTA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.astraea-software.iota\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.astraea-software.iota\");"]
    #[doc = r" ```"]
    pub const VND_ASTRAEA_SOFTWARE_IOTA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.astraea-software.iota",
    };
    #[doc = "\\[Horia_Cristian_Slusanschi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AUDIOGRAPH;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.audiograph\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.audiograph\");"]
    #[doc = r" ```"]
    pub const VND_AUDIOGRAPH: Mime = Mime {
        ttype: "application",
        subtype: "vnd.audiograph",
    };
    #[doc = "\\[Mike_Hearn\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AUTOPACKAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.autopackage\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.autopackage\");"]
    #[doc = r" ```"]
    pub const VND_AUTOPACKAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.autopackage",
    };
    #[doc = "\\[Ben_Hinman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AVALON_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.avalon+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.avalon+json\");"]
    #[doc = r" ```"]
    pub const VND_AVALON_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.avalon+json",
    };
    #[doc = "\\[Vladimir_Vysotsky\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_AVISTAR_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.avistar+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.avistar+xml\");"]
    #[doc = r" ```"]
    pub const VND_AVISTAR_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.avistar+xml",
    };
    #[doc = "\\[Giacomo_Guilizzoni\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BALSAMIQ_BMML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.balsamiq.bmml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.balsamiq.bmml+xml\");"]
    #[doc = r" ```"]
    pub const VND_BALSAMIQ_BMML_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.balsamiq.bmml+xml",
    };
    #[doc = "\\[José_Del_Romano\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BANANA_ACCOUNTING;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.banana-accounting\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.banana-accounting\");"]
    #[doc = r" ```"]
    pub const VND_BANANA_ACCOUNTING: Mime = Mime {
        ttype: "application",
        subtype: "vnd.banana-accounting",
    };
    #[doc = "\\[Broadband_Forum\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BBF_USP_ERROR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bbf.usp.error\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bbf.usp.error\");"]
    #[doc = r" ```"]
    pub const VND_BBF_USP_ERROR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bbf.usp.error",
    };
    #[doc = "\\[Broadband_Forum\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BBF_USP_MSG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bbf.usp.msg\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bbf.usp.msg\");"]
    #[doc = r" ```"]
    pub const VND_BBF_USP_MSG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bbf.usp.msg",
    };
    #[doc = "\\[Broadband_Forum\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BBF_USP_MSG_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bbf.usp.msg+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bbf.usp.msg+json\");"]
    #[doc = r" ```"]
    pub const VND_BBF_USP_MSG_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bbf.usp.msg+json",
    };
    #[doc = "\\[Giacomo_Guilizzoni\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BALSAMIQ_BMPR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.balsamiq.bmpr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.balsamiq.bmpr\");"]
    #[doc = r" ```"]
    pub const VND_BALSAMIQ_BMPR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.balsamiq.bmpr",
    };
    #[doc = "\\[Jegulsky\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BEKITZUR_STECH_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bekitzur-stech+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bekitzur-stech+json\");"]
    #[doc = r" ```"]
    pub const VND_BEKITZUR_STECH_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bekitzur-stech+json",
    };
    #[doc = "\\[Dmytro_Yunchyk\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BELIGHTSOFT_LHZD_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.belightsoft.lhzd+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.belightsoft.lhzd+zip\");"]
    #[doc = r" ```"]
    pub const VND_BELIGHTSOFT_LHZD_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.belightsoft.lhzd+zip",
    };
    #[doc = "\\[Dmytro_Yunchyk\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BELIGHTSOFT_LHZL_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.belightsoft.lhzl+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.belightsoft.lhzl+zip\");"]
    #[doc = r" ```"]
    pub const VND_BELIGHTSOFT_LHZL_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.belightsoft.lhzl+zip",
    };
    #[doc = "\\[Heinz-Peter_Schütz\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BINT_MED_CONTENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bint.med-content\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bint.med-content\");"]
    #[doc = r" ```"]
    pub const VND_BINT_MED_CONTENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bint.med-content",
    };
    #[doc = "\\[Pathway_Commons\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BIOPAX_RDF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.biopax.rdf+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.biopax.rdf+xml\");"]
    #[doc = r" ```"]
    pub const VND_BIOPAX_RDF_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.biopax.rdf+xml",
    };
    #[doc = "\\[Victor_Costan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BLINK_IDB_VALUE_WRAPPER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.blink-idb-value-wrapper\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.blink-idb-value-wrapper\");"]
    #[doc = r" ```"]
    pub const VND_BLINK_IDB_VALUE_WRAPPER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.blink-idb-value-wrapper",
    };
    #[doc = "\\[Thomas_Holmstrom\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BLUEICE_MULTIPASS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.blueice.multipass\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.blueice.multipass\");"]
    #[doc = r" ```"]
    pub const VND_BLUEICE_MULTIPASS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.blueice.multipass",
    };
    #[doc = "\\[Mike_Foley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BLUETOOTH_EP_OOB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bluetooth.ep.oob\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bluetooth.ep.oob\");"]
    #[doc = r" ```"]
    pub const VND_BLUETOOTH_EP_OOB: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bluetooth.ep.oob",
    };
    #[doc = "\\[Mark_Powell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BLUETOOTH_LE_OOB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bluetooth.le.oob\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bluetooth.le.oob\");"]
    #[doc = r" ```"]
    pub const VND_BLUETOOTH_LE_OOB: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bluetooth.le.oob",
    };
    #[doc = "\\[Tadashi_Gotoh\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BMI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bmi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bmi\");"]
    #[doc = r" ```"]
    pub const VND_BMI: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bmi",
    };
    #[doc = "\\[NCGIS\\]\\[Bryan_Blank\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BPF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bpf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bpf\");"]
    #[doc = r" ```"]
    pub const VND_BPF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bpf",
    };
    #[doc = "\\[NCGIS\\]\\[Bryan_Blank\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BPF3;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bpf3\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bpf3\");"]
    #[doc = r" ```"]
    pub const VND_BPF3: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bpf3",
    };
    #[doc = "\\[Philippe_Imoucha\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BUSINESSOBJECTS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.businessobjects\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.businessobjects\");"]
    #[doc = r" ```"]
    pub const VND_BUSINESSOBJECTS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.businessobjects",
    };
    #[doc = "\\[Brent_Moore\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BYU_UAPI_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.byu.uapi+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.byu.uapi+json\");"]
    #[doc = r" ```"]
    pub const VND_BYU_UAPI_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.byu.uapi+json",
    };
    #[doc = "\\[Kamila_Szewczyk\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_BZIP3;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.bzip3\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.bzip3\");"]
    #[doc = r" ```"]
    pub const VND_BZIP3: Mime = Mime {
        ttype: "application",
        subtype: "vnd.bzip3",
    };
    #[doc = "\\[Andreas_Hubel\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_C3VOC_SCHEDULE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.c3voc.schedule+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.c3voc.schedule+xml\");"]
    #[doc = r" ```"]
    pub const VND_C3VOC_SCHEDULE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.c3voc.schedule+xml",
    };
    #[doc = "\\[Joerg_Falkenberg\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CAB_JSCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cab-jscript\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cab-jscript\");"]
    #[doc = r" ```"]
    pub const VND_CAB_JSCRIPT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cab-jscript",
    };
    #[doc = "\\[Shin_Muto\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CANON_CPDL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.canon-cpdl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.canon-cpdl\");"]
    #[doc = r" ```"]
    pub const VND_CANON_CPDL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.canon-cpdl",
    };
    #[doc = "\\[Shin_Muto\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CANON_LIPS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.canon-lips\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.canon-lips\");"]
    #[doc = r" ```"]
    pub const VND_CANON_LIPS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.canon-lips",
    };
    #[doc = "\\[Yüksel_Aydemir\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CAPASYSTEMS_PG_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.capasystems-pg+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.capasystems-pg+json\");"]
    #[doc = r" ```"]
    pub const VND_CAPASYSTEMS_PG_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.capasystems-pg+json",
    };
    #[doc = "\\[Peter_Astrand\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CENDIO_THINLINC_CLIENTCONF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cendio.thinlinc.clientconf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cendio.thinlinc.clientconf\");"]
    #[doc = r" ```"]
    pub const VND_CENDIO_THINLINC_CLIENTCONF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cendio.thinlinc.clientconf",
    };
    #[doc = "\\[Shuji_Fujii\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CENTURY_SYSTEMS_TCP_STREAM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.century-systems.tcp_stream\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.century-systems.tcp_stream\");"]
    #[doc = r" ```"]
    pub const VND_CENTURY_SYSTEMS_TCP_STREAM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.century-systems.tcp_stream",
    };
    #[doc = "\\[Glenn_Howes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CHEMDRAW_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.chemdraw+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.chemdraw+xml\");"]
    #[doc = r" ```"]
    pub const VND_CHEMDRAW_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.chemdraw+xml",
    };
    #[doc = "\\[Kim_Scarborough\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CHESS_PGN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.chess-pgn\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.chess-pgn\");"]
    #[doc = r" ```"]
    pub const VND_CHESS_PGN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.chess-pgn",
    };
    #[doc = "\\[Chunyun_Xiong\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CHIPNUTS_KARAOKE_MMD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.chipnuts.karaoke-mmd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.chipnuts.karaoke-mmd\");"]
    #[doc = r" ```"]
    pub const VND_CHIPNUTS_KARAOKE_MMD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.chipnuts.karaoke-mmd",
    };
    #[doc = "\\[Hidekazu_Enjo\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CIEDI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ciedi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ciedi\");"]
    #[doc = r" ```"]
    pub const VND_CIEDI: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ciedi",
    };
    #[doc = "\\[Ulrich_Kortenkamp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CINDERELLA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cinderella\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cinderella\");"]
    #[doc = r" ```"]
    pub const VND_CINDERELLA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cinderella",
    };
    #[doc = "\\[Pascal_Mayeux\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CIRPACK_ISDN_EXT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cirpack.isdn-ext\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cirpack.isdn-ext\");"]
    #[doc = r" ```"]
    pub const VND_CIRPACK_ISDN_EXT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cirpack.isdn-ext",
    };
    #[doc = "\\[Rintze_M._Zelle\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CITATIONSTYLES_STYLE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.citationstyles.style+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.citationstyles.style+xml\");"]
    #[doc = r" ```"]
    pub const VND_CITATIONSTYLES_STYLE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.citationstyles.style+xml",
    };
    #[doc = "\\[Ray_Simpson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CLAYMORE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.claymore\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.claymore\");"]
    #[doc = r" ```"]
    pub const VND_CLAYMORE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.claymore",
    };
    #[doc = "\\[Mike_Labatt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CLOANTO_RP9;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cloanto.rp9\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cloanto.rp9\");"]
    #[doc = r" ```"]
    pub const VND_CLOANTO_RP9: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cloanto.rp9",
    };
    #[doc = "\\[Guenther_Brammer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CLONK_C4GROUP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.clonk.c4group\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.clonk.c4group\");"]
    #[doc = r" ```"]
    pub const VND_CLONK_C4GROUP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.clonk.c4group",
    };
    #[doc = "\\[Gaige_Paulsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CLUETRUST_CARTOMOBILE_CONFIG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cluetrust.cartomobile-config\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cluetrust.cartomobile-config\");"]
    #[doc = r" ```"]
    pub const VND_CLUETRUST_CARTOMOBILE_CONFIG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cluetrust.cartomobile-config",
    };
    #[doc = "\\[Gaige_Paulsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CLUETRUST_CARTOMOBILE_CONFIG_PKG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cluetrust.cartomobile-config-pkg\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cluetrust.cartomobile-config-pkg\");"]
    #[doc = r" ```"]
    pub const VND_CLUETRUST_CARTOMOBILE_CONFIG_PKG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cluetrust.cartomobile-config-pkg",
    };
    #[doc = "\\[Andrew_Block\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CNCF_HELM_CHART_CONTENT_V1_TAR_GZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cncf.helm.chart.content.v1.tar+gzip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cncf.helm.chart.content.v1.tar+gzip\");"]
    #[doc = r" ```"]
    pub const VND_CNCF_HELM_CHART_CONTENT_V1_TAR_GZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cncf.helm.chart.content.v1.tar+gzip",
    };
    #[doc = "\\[Andrew_Block\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CNCF_HELM_CHART_PROVENANCE_V1_PROV;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cncf.helm.chart.provenance.v1.prov\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cncf.helm.chart.provenance.v1.prov\");"]
    #[doc = r" ```"]
    pub const VND_CNCF_HELM_CHART_PROVENANCE_V1_PROV: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cncf.helm.chart.provenance.v1.prov",
    };
    #[doc = "\\[Andrew_Block\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CNCF_HELM_CONFIG_V1_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cncf.helm.config.v1+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cncf.helm.config.v1+json\");"]
    #[doc = r" ```"]
    pub const VND_CNCF_HELM_CONFIG_V1_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cncf.helm.config.v1+json",
    };
    #[doc = "\\[Devyn_Collier_Johnson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COFFEESCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.coffeescript\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.coffeescript\");"]
    #[doc = r" ```"]
    pub const VND_COFFEESCRIPT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.coffeescript",
    };
    #[doc = "\\[Alexey_Meandrov\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COLLABIO_XODOCUMENTS_DOCUMENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.collabio.xodocuments.document\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.collabio.xodocuments.document\");"]
    #[doc = r" ```"]
    pub const VND_COLLABIO_XODOCUMENTS_DOCUMENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.collabio.xodocuments.document",
    };
    #[doc = "\\[Alexey_Meandrov\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COLLABIO_XODOCUMENTS_DOCUMENT_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.collabio.xodocuments.document-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.collabio.xodocuments.document-template\");"]
    #[doc = r" ```"]
    pub const VND_COLLABIO_XODOCUMENTS_DOCUMENT_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.collabio.xodocuments.document-template",
    };
    #[doc = "\\[Alexey_Meandrov\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COLLABIO_XODOCUMENTS_PRESENTATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.collabio.xodocuments.presentation\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.collabio.xodocuments.presentation\");"]
    #[doc = r" ```"]
    pub const VND_COLLABIO_XODOCUMENTS_PRESENTATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.collabio.xodocuments.presentation",
    };
    #[doc = "\\[Alexey_Meandrov\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COLLABIO_XODOCUMENTS_PRESENTATION_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.collabio.xodocuments.presentation-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.collabio.xodocuments.presentation-template\");"]
    #[doc = r" ```"]
    pub const VND_COLLABIO_XODOCUMENTS_PRESENTATION_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.collabio.xodocuments.presentation-template",
    };
    #[doc = "\\[Alexey_Meandrov\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COLLABIO_XODOCUMENTS_SPREADSHEET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.collabio.xodocuments.spreadsheet\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.collabio.xodocuments.spreadsheet\");"]
    #[doc = r" ```"]
    pub const VND_COLLABIO_XODOCUMENTS_SPREADSHEET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.collabio.xodocuments.spreadsheet",
    };
    #[doc = "\\[Alexey_Meandrov\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COLLABIO_XODOCUMENTS_SPREADSHEET_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.collabio.xodocuments.spreadsheet-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.collabio.xodocuments.spreadsheet-template\");"]
    #[doc = r" ```"]
    pub const VND_COLLABIO_XODOCUMENTS_SPREADSHEET_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.collabio.xodocuments.spreadsheet-template",
    };
    #[doc = "\\[Irakli_Nadareishvili\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COLLECTION_DOC_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.collection.doc+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.collection.doc+json\");"]
    #[doc = r" ```"]
    pub const VND_COLLECTION_DOC_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.collection.doc+json",
    };
    #[doc = "\\[Mike_Amundsen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COLLECTION_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.collection+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.collection+json\");"]
    #[doc = r" ```"]
    pub const VND_COLLECTION_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.collection+json",
    };
    #[doc = "\\[Ioseb_Dzmanashvili\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COLLECTION_NEXT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.collection.next+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.collection.next+json\");"]
    #[doc = r" ```"]
    pub const VND_COLLECTION_NEXT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.collection.next+json",
    };
    #[doc = "\\[Kim_Scarborough\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COMICBOOK_RAR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.comicbook-rar\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.comicbook-rar\");"]
    #[doc = r" ```"]
    pub const VND_COMICBOOK_RAR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.comicbook-rar",
    };
    #[doc = "\\[Kim_Scarborough\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COMICBOOK_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.comicbook+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.comicbook+zip\");"]
    #[doc = r" ```"]
    pub const VND_COMICBOOK_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.comicbook+zip",
    };
    #[doc = "\\[David_Applebaum\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COMMERCE_BATTELLE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.commerce-battelle\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.commerce-battelle\");"]
    #[doc = r" ```"]
    pub const VND_COMMERCE_BATTELLE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.commerce-battelle",
    };
    #[doc = "\\[Ravinder_Chandhok\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COMMONSPACE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.commonspace\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.commonspace\");"]
    #[doc = r" ```"]
    pub const VND_COMMONSPACE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.commonspace",
    };
    #[doc = "\\[Alex_Crawford\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COREOS_IGNITION_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.coreos.ignition+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.coreos.ignition+json\");"]
    #[doc = r" ```"]
    pub const VND_COREOS_IGNITION_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.coreos.ignition+json",
    };
    #[doc = "\\[Steve_Dellutri\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_COSMOCALLER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cosmocaller\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cosmocaller\");"]
    #[doc = r" ```"]
    pub const VND_COSMOCALLER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cosmocaller",
    };
    #[doc = "\\[Frank_Patz\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CONTACT_CMSG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.contact.cmsg\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.contact.cmsg\");"]
    #[doc = r" ```"]
    pub const VND_CONTACT_CMSG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.contact.cmsg",
    };
    #[doc = "\\[Andrew_Burt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CRICK_CLICKER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.crick.clicker\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.crick.clicker\");"]
    #[doc = r" ```"]
    pub const VND_CRICK_CLICKER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.crick.clicker",
    };
    #[doc = "\\[Andrew_Burt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CRICK_CLICKER_KEYBOARD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.crick.clicker.keyboard\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.crick.clicker.keyboard\");"]
    #[doc = r" ```"]
    pub const VND_CRICK_CLICKER_KEYBOARD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.crick.clicker.keyboard",
    };
    #[doc = "\\[Andrew_Burt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CRICK_CLICKER_PALETTE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.crick.clicker.palette\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.crick.clicker.palette\");"]
    #[doc = r" ```"]
    pub const VND_CRICK_CLICKER_PALETTE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.crick.clicker.palette",
    };
    #[doc = "\\[Andrew_Burt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CRICK_CLICKER_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.crick.clicker.template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.crick.clicker.template\");"]
    #[doc = r" ```"]
    pub const VND_CRICK_CLICKER_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.crick.clicker.template",
    };
    #[doc = "\\[Andrew_Burt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CRICK_CLICKER_WORDBANK;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.crick.clicker.wordbank\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.crick.clicker.wordbank\");"]
    #[doc = r" ```"]
    pub const VND_CRICK_CLICKER_WORDBANK: Mime = Mime {
        ttype: "application",
        subtype: "vnd.crick.clicker.wordbank",
    };
    #[doc = "\\[Jim_Spiller\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CRITICALTOOLS_WBS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.criticaltools.wbs+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.criticaltools.wbs+xml\");"]
    #[doc = r" ```"]
    pub const VND_CRITICALTOOLS_WBS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.criticaltools.wbs+xml",
    };
    #[doc = "\\[Fränz_Friederes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CRYPTII_PIPE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cryptii.pipe+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cryptii.pipe+json\");"]
    #[doc = r" ```"]
    pub const VND_CRYPTII_PIPE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cryptii.pipe+json",
    };
    #[doc = "\\[Connor_Horman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CRYPTO_SHADE_FILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.crypto-shade-file\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.crypto-shade-file\");"]
    #[doc = r" ```"]
    pub const VND_CRYPTO_SHADE_FILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.crypto-shade-file",
    };
    #[doc = "\\[Sebastian_Stenzel\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CRYPTOMATOR_ENCRYPTED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cryptomator.encrypted\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cryptomator.encrypted\");"]
    #[doc = r" ```"]
    pub const VND_CRYPTOMATOR_ENCRYPTED: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cryptomator.encrypted",
    };
    #[doc = "\\[Sebastian_Stenzel\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CRYPTOMATOR_VAULT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cryptomator.vault\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cryptomator.vault\");"]
    #[doc = r" ```"]
    pub const VND_CRYPTOMATOR_VAULT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cryptomator.vault",
    };
    #[doc = "\\[Bayard_Kohlhepp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CTC_POSML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ctc-posml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ctc-posml\");"]
    #[doc = r" ```"]
    pub const VND_CTC_POSML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ctc-posml",
    };
    #[doc = "\\[Jim_Ancona\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CTCT_WS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ctct.ws+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ctct.ws+xml\");"]
    #[doc = r" ```"]
    pub const VND_CTCT_WS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ctct.ws+xml",
    };
    #[doc = "\\[Michael_Sweet\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CUPS_PDF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cups-pdf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cups-pdf\");"]
    #[doc = r" ```"]
    pub const VND_CUPS_PDF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cups-pdf",
    };
    #[doc = "\\[Michael_Sweet\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CUPS_POSTSCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cups-postscript\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cups-postscript\");"]
    #[doc = r" ```"]
    pub const VND_CUPS_POSTSCRIPT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cups-postscript",
    };
    #[doc = "\\[Michael_Sweet\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CUPS_PPD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cups-ppd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cups-ppd\");"]
    #[doc = r" ```"]
    pub const VND_CUPS_PPD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cups-ppd",
    };
    #[doc = "\\[Michael_Sweet\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CUPS_RASTER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cups-raster\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cups-raster\");"]
    #[doc = r" ```"]
    pub const VND_CUPS_RASTER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cups-raster",
    };
    #[doc = "\\[Michael_Sweet\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CUPS_RAW;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cups-raw\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cups-raw\");"]
    #[doc = r" ```"]
    pub const VND_CUPS_RAW: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cups-raw",
    };
    #[doc = "\\[Robert_Byrnes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CURL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.curl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.curl\");"]
    #[doc = r" ```"]
    pub const VND_CURL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.curl",
    };
    #[doc = "\\[Matt_Kern\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CYAN_DEAN_ROOT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cyan.dean.root+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cyan.dean.root+xml\");"]
    #[doc = r" ```"]
    pub const VND_CYAN_DEAN_ROOT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cyan.dean.root+xml",
    };
    #[doc = "\\[Nor_Helmee\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CYBANK;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cybank\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cybank\");"]
    #[doc = r" ```"]
    pub const VND_CYBANK: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cybank",
    };
    #[doc = "\\[Patrick_Dwyer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CYCLONEDX_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cyclonedx+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cyclonedx+json\");"]
    #[doc = r" ```"]
    pub const VND_CYCLONEDX_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cyclonedx+json",
    };
    #[doc = "\\[Patrick_Dwyer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_CYCLONEDX_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.cyclonedx+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.cyclonedx+xml\");"]
    #[doc = r" ```"]
    pub const VND_CYCLONEDX_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.cyclonedx+xml",
    };
    #[doc = "\\[Viktor_Haag\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_D2L_COURSEPACKAGE1P0_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.d2l.coursepackage1p0+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.d2l.coursepackage1p0+zip\");"]
    #[doc = r" ```"]
    pub const VND_D2L_COURSEPACKAGE1P0_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.d2l.coursepackage1p0+zip",
    };
    #[doc = "\\[Mi_Tar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_D3M_DATASET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.d3m-dataset\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.d3m-dataset\");"]
    #[doc = r" ```"]
    pub const VND_D3M_DATASET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.d3m-dataset",
    };
    #[doc = "\\[Mi_Tar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_D3M_PROBLEM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.d3m-problem\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.d3m-problem\");"]
    #[doc = r" ```"]
    pub const VND_D3M_PROBLEM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.d3m-problem",
    };
    #[doc = "\\[Anders_Sandholm\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DART;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dart\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dart\");"]
    #[doc = r" ```"]
    pub const VND_DART: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dart",
    };
    #[doc = "\\[James_Fields\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DATA_VISION_RDZ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.data-vision.rdz\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.data-vision.rdz\");"]
    #[doc = r" ```"]
    pub const VND_DATA_VISION_RDZ: Mime = Mime {
        ttype: "application",
        subtype: "vnd.data-vision.rdz",
    };
    #[doc = "\\[Simon_Johnston\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DATALOG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.datalog\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.datalog\");"]
    #[doc = r" ```"]
    pub const VND_DATALOG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.datalog",
    };
    #[doc = "\\[Paul_Walsh\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DATAPACKAGE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.datapackage+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.datapackage+json\");"]
    #[doc = r" ```"]
    pub const VND_DATAPACKAGE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.datapackage+json",
    };
    #[doc = "\\[Paul_Walsh\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DATARESOURCE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dataresource+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dataresource+json\");"]
    #[doc = r" ```"]
    pub const VND_DATARESOURCE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dataresource+json",
    };
    #[doc = "\\[Mi_Tar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DBF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dbf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dbf\");"]
    #[doc = r" ```"]
    pub const VND_DBF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dbf",
    };
    #[doc = "\\[Debian_Policy_mailing_list\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DEBIAN_BINARY_PACKAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.debian.binary-package\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.debian.binary-package\");"]
    #[doc = r" ```"]
    pub const VND_DEBIAN_BINARY_PACKAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.debian.binary-package",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DECE_DATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.data\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dece.data\");"]
    #[doc = r" ```"]
    pub const VND_DECE_DATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dece.data",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DECE_TTML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.ttml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dece.ttml+xml\");"]
    #[doc = r" ```"]
    pub const VND_DECE_TTML_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dece.ttml+xml",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DECE_UNSPECIFIED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.unspecified\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dece.unspecified\");"]
    #[doc = r" ```"]
    pub const VND_DECE_UNSPECIFIED: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dece.unspecified",
    };
    #[doc = "\\[Michael_A_Dolan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DECE_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dece.zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dece.zip\");"]
    #[doc = r" ```"]
    pub const VND_DECE_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dece.zip",
    };
    #[doc = "\\[Michael_Dixon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DENOVO_FCSELAYOUT_LINK;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.denovo.fcselayout-link\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.denovo.fcselayout-link\");"]
    #[doc = r" ```"]
    pub const VND_DENOVO_FCSELAYOUT_LINK: Mime = Mime {
        ttype: "application",
        subtype: "vnd.denovo.fcselayout-link",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DESMUME_MOVIE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.desmume.movie\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.desmume.movie\");"]
    #[doc = r" ```"]
    pub const VND_DESMUME_MOVIE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.desmume.movie",
    };
    #[doc = "\\[Yamanaka\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DIR_BI_PLATE_DL_NOSUFFIX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dir-bi.plate-dl-nosuffix\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dir-bi.plate-dl-nosuffix\");"]
    #[doc = r" ```"]
    pub const VND_DIR_BI_PLATE_DL_NOSUFFIX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dir-bi.plate-dl-nosuffix",
    };
    #[doc = "\\[Axel_Ferrazzini\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DM_DELEGATION_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dm.delegation+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dm.delegation+xml\");"]
    #[doc = r" ```"]
    pub const VND_DM_DELEGATION_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dm.delegation+xml",
    };
    #[doc = "\\[Meredith_Searcy\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DNA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dna\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dna\");"]
    #[doc = r" ```"]
    pub const VND_DNA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dna",
    };
    #[doc = "\\[Tom_Christie\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DOCUMENT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.document+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.document+json\");"]
    #[doc = r" ```"]
    pub const VND_DOCUMENT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.document+json",
    };
    #[doc = "\\[Steve_Hattersley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DOLBY_MOBILE_1;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dolby.mobile.1\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dolby.mobile.1\");"]
    #[doc = r" ```"]
    pub const VND_DOLBY_MOBILE_1: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dolby.mobile.1",
    };
    #[doc = "\\[Steve_Hattersley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DOLBY_MOBILE_2;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dolby.mobile.2\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dolby.mobile.2\");"]
    #[doc = r" ```"]
    pub const VND_DOLBY_MOBILE_2: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dolby.mobile.2",
    };
    #[doc = "\\[Erik_Ronström\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DOREMIR_SCORECLOUD_BINARY_DOCUMENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.doremir.scorecloud-binary-document\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.doremir.scorecloud-binary-document\");"]
    #[doc = r" ```"]
    pub const VND_DOREMIR_SCORECLOUD_BINARY_DOCUMENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.doremir.scorecloud-binary-document",
    };
    #[doc = "\\[David_Parker\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DPGRAPH;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dpgraph\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dpgraph\");"]
    #[doc = r" ```"]
    pub const VND_DPGRAPH: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dpgraph",
    };
    #[doc = "\\[William_C._Appleton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DREAMFACTORY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dreamfactory\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dreamfactory\");"]
    #[doc = r" ```"]
    pub const VND_DREAMFACTORY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dreamfactory",
    };
    #[doc = "\\[Keith_Kester\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DRIVE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.drive+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.drive+json\");"]
    #[doc = r" ```"]
    pub const VND_DRIVE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.drive+json",
    };
    #[doc = "\\[Ali_Teffahi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DTG_LOCAL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dtg.local\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dtg.local\");"]
    #[doc = r" ```"]
    pub const VND_DTG_LOCAL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dtg.local",
    };
    #[doc = "\\[Ali_Teffahi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DTG_LOCAL_FLASH;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dtg.local.flash\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dtg.local.flash\");"]
    #[doc = r" ```"]
    pub const VND_DTG_LOCAL_FLASH: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dtg.local.flash",
    };
    #[doc = "\\[Ali_Teffahi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DTG_LOCAL_HTML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dtg.local.html\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dtg.local.html\");"]
    #[doc = r" ```"]
    pub const VND_DTG_LOCAL_HTML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dtg.local.html",
    };
    #[doc = "\\[Peter_Siebert\\]\\[Michael_Lagally\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_AIT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.ait\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.ait\");"]
    #[doc = r" ```"]
    pub const VND_DVB_AIT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.ait",
    };
    #[doc = "\\[Emily_DUBS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_DVBISL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.dvbisl+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.dvbisl+xml\");"]
    #[doc = r" ```"]
    pub const VND_DVB_DVBISL_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.dvbisl+xml",
    };
    #[doc = "\\[Peter_Siebert\\]\\[Michael_Lagally\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_DVBJ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.dvbj\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.dvbj\");"]
    #[doc = r" ```"]
    pub const VND_DVB_DVBJ: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.dvbj",
    };
    #[doc = "\\[Joerg_Heuer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_ESGCONTAINER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.esgcontainer\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.esgcontainer\");"]
    #[doc = r" ```"]
    pub const VND_DVB_ESGCONTAINER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.esgcontainer",
    };
    #[doc = "\\[Roy_Yue\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_IPDCDFTNOTIFACCESS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.ipdcdftnotifaccess\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.ipdcdftnotifaccess\");"]
    #[doc = r" ```"]
    pub const VND_DVB_IPDCDFTNOTIFACCESS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.ipdcdftnotifaccess",
    };
    #[doc = "\\[Joerg_Heuer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_IPDCESGACCESS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.ipdcesgaccess\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.ipdcesgaccess\");"]
    #[doc = r" ```"]
    pub const VND_DVB_IPDCESGACCESS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.ipdcesgaccess",
    };
    #[doc = "\\[Jerome_Marcon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_IPDCESGACCESS2;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.ipdcesgaccess2\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.ipdcesgaccess2\");"]
    #[doc = r" ```"]
    pub const VND_DVB_IPDCESGACCESS2: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.ipdcesgaccess2",
    };
    #[doc = "\\[Jerome_Marcon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_IPDCESGPDD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.ipdcesgpdd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.ipdcesgpdd\");"]
    #[doc = r" ```"]
    pub const VND_DVB_IPDCESGPDD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.ipdcesgpdd",
    };
    #[doc = "\\[Yiling_Xu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_IPDCROAMING;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.ipdcroaming\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.ipdcroaming\");"]
    #[doc = r" ```"]
    pub const VND_DVB_IPDCROAMING: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.ipdcroaming",
    };
    #[doc = "\\[Jean-Baptiste_Henry\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_IPTV_ALFEC_BASE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.iptv.alfec-base\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.iptv.alfec-base\");"]
    #[doc = r" ```"]
    pub const VND_DVB_IPTV_ALFEC_BASE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.iptv.alfec-base",
    };
    #[doc = "\\[Jean-Baptiste_Henry\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_IPTV_ALFEC_ENHANCEMENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.iptv.alfec-enhancement\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.iptv.alfec-enhancement\");"]
    #[doc = r" ```"]
    pub const VND_DVB_IPTV_ALFEC_ENHANCEMENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.iptv.alfec-enhancement",
    };
    #[doc = "\\[Roy_Yue\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_NOTIF_AGGREGATE_ROOT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.notif-aggregate-root+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.notif-aggregate-root+xml\");"]
    #[doc = r" ```"]
    pub const VND_DVB_NOTIF_AGGREGATE_ROOT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.notif-aggregate-root+xml",
    };
    #[doc = "\\[Roy_Yue\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_NOTIF_CONTAINER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.notif-container+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.notif-container+xml\");"]
    #[doc = r" ```"]
    pub const VND_DVB_NOTIF_CONTAINER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.notif-container+xml",
    };
    #[doc = "\\[Roy_Yue\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_NOTIF_GENERIC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.notif-generic+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.notif-generic+xml\");"]
    #[doc = r" ```"]
    pub const VND_DVB_NOTIF_GENERIC_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.notif-generic+xml",
    };
    #[doc = "\\[Roy_Yue\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_NOTIF_IA_MSGLIST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.notif-ia-msglist+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.notif-ia-msglist+xml\");"]
    #[doc = r" ```"]
    pub const VND_DVB_NOTIF_IA_MSGLIST_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.notif-ia-msglist+xml",
    };
    #[doc = "\\[Roy_Yue\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_NOTIF_IA_REGISTRATION_REQUEST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.notif-ia-registration-request+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.notif-ia-registration-request+xml\");"]
    #[doc = r" ```"]
    pub const VND_DVB_NOTIF_IA_REGISTRATION_REQUEST_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.notif-ia-registration-request+xml",
    };
    #[doc = "\\[Roy_Yue\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_NOTIF_IA_REGISTRATION_RESPONSE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.notif-ia-registration-response+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.notif-ia-registration-response+xml\");"]
    #[doc = r" ```"]
    pub const VND_DVB_NOTIF_IA_REGISTRATION_RESPONSE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.notif-ia-registration-response+xml",
    };
    #[doc = "\\[Roy_Yue\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_NOTIF_INIT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.notif-init+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.notif-init+xml\");"]
    #[doc = r" ```"]
    pub const VND_DVB_NOTIF_INIT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.notif-init+xml",
    };
    #[doc = "\\[Peter_Siebert\\]\\[Michael_Lagally\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_PFR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.pfr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.pfr\");"]
    #[doc = r" ```"]
    pub const VND_DVB_PFR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.pfr",
    };
    #[doc = "\\[Peter_Siebert\\]\\[Michael_Lagally\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DVB_SERVICE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dvb.service\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dvb.service\");"]
    #[doc = r" ```"]
    pub const VND_DVB_SERVICE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dvb.service",
    };
    #[doc = "\\[Michael_Duffy\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DXR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dxr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dxr\");"]
    #[doc = r" ```"]
    pub const VND_DXR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dxr",
    };
    #[doc = "\\[Roland_Mechling\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DYNAGEO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dynageo\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dynageo\");"]
    #[doc = r" ```"]
    pub const VND_DYNAGEO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dynageo",
    };
    #[doc = "\\[Carl_Anderson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_DZR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.dzr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.dzr\");"]
    #[doc = r" ```"]
    pub const VND_DZR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.dzr",
    };
    #[doc = "\\[Iain_Downs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EASYKARAOKE_CDGDOWNLOAD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.easykaraoke.cdgdownload\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.easykaraoke.cdgdownload\");"]
    #[doc = r" ```"]
    pub const VND_EASYKARAOKE_CDGDOWNLOAD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.easykaraoke.cdgdownload",
    };
    #[doc = "\\[Wei_Tang\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ECIP_RLP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ecip.rlp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ecip.rlp\");"]
    #[doc = r" ```"]
    pub const VND_ECIP_RLP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ecip.rlp",
    };
    #[doc = "\\[Gert_Buettgenbach\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ECDIS_UPDATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ecdis-update\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ecdis-update\");"]
    #[doc = r" ```"]
    pub const VND_ECDIS_UPDATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ecdis-update",
    };
    #[doc = "\\[Eclipse_Ditto_developers\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ECLIPSE_DITTO_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.eclipse.ditto+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.eclipse.ditto+json\");"]
    #[doc = r" ```"]
    pub const VND_ECLIPSE_DITTO_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.eclipse.ditto+json",
    };
    #[doc = "\\[Thomas_Olsson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ECOWIN_CHART;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ecowin.chart\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ecowin.chart\");"]
    #[doc = r" ```"]
    pub const VND_ECOWIN_CHART: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ecowin.chart",
    };
    #[doc = "\\[Thomas_Olsson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ECOWIN_FILEREQUEST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ecowin.filerequest\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ecowin.filerequest\");"]
    #[doc = r" ```"]
    pub const VND_ECOWIN_FILEREQUEST: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ecowin.filerequest",
    };
    #[doc = "\\[Thomas_Olsson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ECOWIN_FILEUPDATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ecowin.fileupdate\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ecowin.fileupdate\");"]
    #[doc = r" ```"]
    pub const VND_ECOWIN_FILEUPDATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ecowin.fileupdate",
    };
    #[doc = "\\[Thomas_Olsson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ECOWIN_SERIES;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ecowin.series\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ecowin.series\");"]
    #[doc = r" ```"]
    pub const VND_ECOWIN_SERIES: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ecowin.series",
    };
    #[doc = "\\[Thomas_Olsson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ECOWIN_SERIESREQUEST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ecowin.seriesrequest\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ecowin.seriesrequest\");"]
    #[doc = r" ```"]
    pub const VND_ECOWIN_SERIESREQUEST: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ecowin.seriesrequest",
    };
    #[doc = "\\[Thomas_Olsson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ECOWIN_SERIESUPDATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ecowin.seriesupdate\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ecowin.seriesupdate\");"]
    #[doc = r" ```"]
    pub const VND_ECOWIN_SERIESUPDATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ecowin.seriesupdate",
    };
    #[doc = "\\[UEFI_Forum\\]\\[Fu_Siyuan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EFI_IMG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.efi.img\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.efi.img\");"]
    #[doc = r" ```"]
    pub const VND_EFI_IMG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.efi.img",
    };
    #[doc = "\\[UEFI_Forum\\]\\[Fu_Siyuan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EFI_ISO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.efi.iso\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.efi.iso\");"]
    #[doc = r" ```"]
    pub const VND_EFI_ISO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.efi.iso",
    };
    #[doc = "\\[Nicolas_CARPI\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ELN_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.eln+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.eln+zip\");"]
    #[doc = r" ```"]
    pub const VND_ELN_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.eln+zip",
    };
    #[doc = "\\[Filip_Navara\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EMCLIENT_ACCESSREQUEST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.emclient.accessrequest+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.emclient.accessrequest+xml\");"]
    #[doc = r" ```"]
    pub const VND_EMCLIENT_ACCESSREQUEST_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.emclient.accessrequest+xml",
    };
    #[doc = "\\[Paul_Santinelli_Jr.\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ENLIVEN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.enliven\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.enliven\");"]
    #[doc = r" ```"]
    pub const VND_ENLIVEN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.enliven",
    };
    #[doc = "\\[Chris_Eich\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ENPHASE_ENVOY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.enphase.envoy\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.enphase.envoy\");"]
    #[doc = r" ```"]
    pub const VND_ENPHASE_ENVOY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.enphase.envoy",
    };
    #[doc = "\\[Tim_Brody\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EPRINTS_DATA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.eprints.data+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.eprints.data+xml\");"]
    #[doc = r" ```"]
    pub const VND_EPRINTS_DATA_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.eprints.data+xml",
    };
    #[doc = "\\[Shoji_Hoshina\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EPSON_ESF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.epson.esf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.epson.esf\");"]
    #[doc = r" ```"]
    pub const VND_EPSON_ESF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.epson.esf",
    };
    #[doc = "\\[Shoji_Hoshina\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EPSON_MSF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.epson.msf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.epson.msf\");"]
    #[doc = r" ```"]
    pub const VND_EPSON_MSF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.epson.msf",
    };
    #[doc = "\\[Yu_Gu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EPSON_QUICKANIME;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.epson.quickanime\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.epson.quickanime\");"]
    #[doc = r" ```"]
    pub const VND_EPSON_QUICKANIME: Mime = Mime {
        ttype: "application",
        subtype: "vnd.epson.quickanime",
    };
    #[doc = "\\[Yasuhito_Nagatomo\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EPSON_SALT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.epson.salt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.epson.salt\");"]
    #[doc = r" ```"]
    pub const VND_EPSON_SALT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.epson.salt",
    };
    #[doc = "\\[Shoji_Hoshina\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EPSON_SSF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.epson.ssf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.epson.ssf\");"]
    #[doc = r" ```"]
    pub const VND_EPSON_SSF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.epson.ssf",
    };
    #[doc = "\\[Paul_Tidwell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ERICSSON_QUICKCALL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ericsson.quickcall\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ericsson.quickcall\");"]
    #[doc = r" ```"]
    pub const VND_ERICSSON_QUICKCALL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ericsson.quickcall",
    };
    #[doc = "\\[Xiang_Gao\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EROFS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.erofs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.erofs\");"]
    #[doc = r" ```"]
    pub const VND_EROFS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.erofs",
    };
    #[doc = "\\[Marcus_Ligi_Büschleb\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ESPASS_ESPASS_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.espass-espass+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.espass-espass+zip\");"]
    #[doc = r" ```"]
    pub const VND_ESPASS_ESPASS_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.espass-espass+zip",
    };
    #[doc = "\\[Szilveszter_Tóth\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ESZIGNO3_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.eszigno3+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.eszigno3+xml\");"]
    #[doc = r" ```"]
    pub const VND_ESZIGNO3_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.eszigno3+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_AOC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.aoc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.aoc+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_AOC_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.aoc+xml",
    };
    #[doc = "\\[Miguel_Angel_Reina_Ortega\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_ASIC_S_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.asic-s+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.asic-s+zip\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_ASIC_S_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.asic-s+zip",
    };
    #[doc = "\\[Miguel_Angel_Reina_Ortega\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_ASIC_E_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.asic-e+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.asic-e+zip\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_ASIC_E_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.asic-e+zip",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_CUG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.cug+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.cug+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_CUG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.cug+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_IPTVCOMMAND_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.iptvcommand+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.iptvcommand+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_IPTVCOMMAND_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.iptvcommand+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_IPTVDISCOVERY_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.iptvdiscovery+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.iptvdiscovery+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_IPTVDISCOVERY_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.iptvdiscovery+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_IPTVPROFILE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.iptvprofile+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.iptvprofile+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_IPTVPROFILE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.iptvprofile+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_IPTVSAD_BC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.iptvsad-bc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.iptvsad-bc+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_IPTVSAD_BC_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.iptvsad-bc+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_IPTVSAD_COD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.iptvsad-cod+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.iptvsad-cod+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_IPTVSAD_COD_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.iptvsad-cod+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_IPTVSAD_NPVR_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.iptvsad-npvr+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.iptvsad-npvr+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_IPTVSAD_NPVR_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.iptvsad-npvr+xml",
    };
    #[doc = "\\[Miguel_Angel_Reina_Ortega\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_IPTVSERVICE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.iptvservice+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.iptvservice+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_IPTVSERVICE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.iptvservice+xml",
    };
    #[doc = "\\[Miguel_Angel_Reina_Ortega\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_IPTVSYNC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.iptvsync+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.iptvsync+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_IPTVSYNC_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.iptvsync+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_IPTVUEPROFILE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.iptvueprofile+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.iptvueprofile+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_IPTVUEPROFILE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.iptvueprofile+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_MCID_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.mcid+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.mcid+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_MCID_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.mcid+xml",
    };
    #[doc = "\\[Miguel_Angel_Reina_Ortega\\]\\[Ian_Medland\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_MHEG5;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.mheg5\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.mheg5\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_MHEG5: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.mheg5",
    };
    #[doc = "\\[Miguel_Angel_Reina_Ortega\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_OVERLOAD_CONTROL_POLICY_DATASET_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.overload-control-policy-dataset+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.overload-control-policy-dataset+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_OVERLOAD_CONTROL_POLICY_DATASET_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.overload-control-policy-dataset+xml",
    };
    #[doc = "\\[Jiwan_Han\\]\\[Thomas_Belling\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_PSTN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.pstn+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.pstn+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_PSTN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.pstn+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_SCI_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.sci+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.sci+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_SCI_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.sci+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_SIMSERVS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.simservs+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.simservs+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_SIMSERVS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.simservs+xml",
    };
    #[doc = "\\[Miguel_Angel_Reina_Ortega\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_TIMESTAMP_TOKEN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.timestamp-token\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.timestamp-token\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_TIMESTAMP_TOKEN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.timestamp-token",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_TSL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.tsl+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.tsl+xml\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_TSL_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.tsl+xml",
    };
    #[doc = "\\[Shicheng_Hu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ETSI_TSL_DER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.etsi.tsl.der\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.etsi.tsl.der\");"]
    #[doc = r" ```"]
    pub const VND_ETSI_TSL_DER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.etsi.tsl.der",
    };
    #[doc = "\\[Hervé_Kasparian\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EU_KASPARIAN_CAR_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.eu.kasparian.car+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.eu.kasparian.car+json\");"]
    #[doc = r" ```"]
    pub const VND_EU_KASPARIAN_CAR_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.eu.kasparian.car+json",
    };
    #[doc = "\\[Pete_Resnick\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EUDORA_DATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.eudora.data\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.eudora.data\");"]
    #[doc = r" ```"]
    pub const VND_EUDORA_DATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.eudora.data",
    };
    #[doc = "\\[James_Bellinger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EVOLV_ECIG_PROFILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.evolv.ecig.profile\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.evolv.ecig.profile\");"]
    #[doc = r" ```"]
    pub const VND_EVOLV_ECIG_PROFILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.evolv.ecig.profile",
    };
    #[doc = "\\[James_Bellinger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EVOLV_ECIG_SETTINGS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.evolv.ecig.settings\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.evolv.ecig.settings\");"]
    #[doc = r" ```"]
    pub const VND_EVOLV_ECIG_SETTINGS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.evolv.ecig.settings",
    };
    #[doc = "\\[James_Bellinger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EVOLV_ECIG_THEME;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.evolv.ecig.theme\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.evolv.ecig.theme\");"]
    #[doc = r" ```"]
    pub const VND_EVOLV_ECIG_THEME: Mime = Mime {
        ttype: "application",
        subtype: "vnd.evolv.ecig.theme",
    };
    #[doc = "\\[Bill_Kidwell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EXSTREAM_EMPOWER_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.exstream-empower+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.exstream-empower+zip\");"]
    #[doc = r" ```"]
    pub const VND_EXSTREAM_EMPOWER_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.exstream-empower+zip",
    };
    #[doc = "\\[Bill_Kidwell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EXSTREAM_PACKAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.exstream-package\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.exstream-package\");"]
    #[doc = r" ```"]
    pub const VND_EXSTREAM_PACKAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.exstream-package",
    };
    #[doc = "\\[ElectronicZombieCorp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EZPIX_ALBUM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ezpix-album\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ezpix-album\");"]
    #[doc = r" ```"]
    pub const VND_EZPIX_ALBUM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ezpix-album",
    };
    #[doc = "\\[ElectronicZombieCorp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_EZPIX_PACKAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ezpix-package\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ezpix-package\");"]
    #[doc = r" ```"]
    pub const VND_EZPIX_PACKAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ezpix-package",
    };
    #[doc = "\\[Samu_Sarivaara\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_F_SECURE_MOBILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.f-secure.mobile\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.f-secure.mobile\");"]
    #[doc = r" ```"]
    pub const VND_F_SECURE_MOBILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.f-secure.mobile",
    };
    #[doc = "\\[Thomas_Huth\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FASTCOPY_DISK_IMAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fastcopy-disk-image\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fastcopy-disk-image\");"]
    #[doc = r" ```"]
    pub const VND_FASTCOPY_DISK_IMAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fastcopy-disk-image",
    };
    #[doc = "\\[Gordon_Clarke\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FAMILYSEARCH_GEDCOM_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.familysearch.gedcom+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.familysearch.gedcom+zip\");"]
    #[doc = r" ```"]
    pub const VND_FAMILYSEARCH_GEDCOM_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.familysearch.gedcom+zip",
    };
    #[doc = "\\[Chad_Trabant\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FDSN_MSEED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fdsn.mseed\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fdsn.mseed\");"]
    #[doc = r" ```"]
    pub const VND_FDSN_MSEED: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fdsn.mseed",
    };
    #[doc = "\\[Chad_Trabant\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FDSN_SEED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fdsn.seed\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fdsn.seed\");"]
    #[doc = r" ```"]
    pub const VND_FDSN_SEED: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fdsn.seed",
    };
    #[doc = "\\[Holstage\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FFSNS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ffsns\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ffsns\");"]
    #[doc = r" ```"]
    pub const VND_FFSNS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ffsns",
    };
    #[doc = "\\[Steve_Gilberd\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FICLAB_FLB_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ficlab.flb+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ficlab.flb+zip\");"]
    #[doc = r" ```"]
    pub const VND_FICLAB_FLB_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ficlab.flb+zip",
    };
    #[doc = "\\[Harms_Moeller\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FILMIT_ZFC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.filmit.zfc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.filmit.zfc\");"]
    #[doc = r" ```"]
    pub const VND_FILMIT_ZFC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.filmit.zfc",
    };
    #[doc = "\\[Ingo_Hammann\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FINTS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fints\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fints\");"]
    #[doc = r" ```"]
    pub const VND_FINTS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fints",
    };
    #[doc = "\\[Alex_Dubov\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FIREMONKEYS_CLOUDCELL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.firemonkeys.cloudcell\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.firemonkeys.cloudcell\");"]
    #[doc = r" ```"]
    pub const VND_FIREMONKEYS_CLOUDCELL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.firemonkeys.cloudcell",
    };
    #[doc = "\\[Dick_Floersch\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FLOGRAPHIT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.FloGraphIt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.FloGraphIt\");"]
    #[doc = r" ```"]
    pub const VND_FLOGRAPHIT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.FloGraphIt",
    };
    #[doc = "\\[Marc_Winter\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FLUXTIME_CLIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fluxtime.clip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fluxtime.clip\");"]
    #[doc = r" ```"]
    pub const VND_FLUXTIME_CLIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fluxtime.clip",
    };
    #[doc = "\\[George_Williams\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FONT_FONTFORGE_SFD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.font-fontforge-sfd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.font-fontforge-sfd\");"]
    #[doc = r" ```"]
    pub const VND_FONT_FONTFORGE_SFD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.font-fontforge-sfd",
    };
    #[doc = "\\[Mike_Wexler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FRAMEMAKER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.framemaker\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.framemaker\");"]
    #[doc = r" ```"]
    pub const VND_FRAMEMAKER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.framemaker",
    };
    #[doc = "\\[Liu_Qiancen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FREELOG_COMIC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.freelog.comic\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.freelog.comic\");"]
    #[doc = r" ```"]
    pub const VND_FREELOG_COMIC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.freelog.comic",
    };
    #[doc = "\\[OP3FT\\]\\[Alexis_Tamas\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FROGANS_FNC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.frogans.fnc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.frogans.fnc\");"]
    #[doc = r" ```"]
    pub const VND_FROGANS_FNC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.frogans.fnc",
    };
    #[doc = "\\[OP3FT\\]\\[Alexis_Tamas\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FROGANS_LTF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.frogans.ltf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.frogans.ltf\");"]
    #[doc = r" ```"]
    pub const VND_FROGANS_LTF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.frogans.ltf",
    };
    #[doc = "\\[Derek_Smith\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FSC_WEBLAUNCH;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fsc.weblaunch\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fsc.weblaunch\");"]
    #[doc = r" ```"]
    pub const VND_FSC_WEBLAUNCH: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fsc.weblaunch",
    };
    #[doc = "\\[Kazuya_Iimura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIFILM_FB_DOCUWORKS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujifilm.fb.docuworks\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujifilm.fb.docuworks\");"]
    #[doc = r" ```"]
    pub const VND_FUJIFILM_FB_DOCUWORKS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujifilm.fb.docuworks",
    };
    #[doc = "\\[Kazuya_Iimura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIFILM_FB_DOCUWORKS_BINDER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujifilm.fb.docuworks.binder\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujifilm.fb.docuworks.binder\");"]
    #[doc = r" ```"]
    pub const VND_FUJIFILM_FB_DOCUWORKS_BINDER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujifilm.fb.docuworks.binder",
    };
    #[doc = "\\[Kazuya_Iimura\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIFILM_FB_DOCUWORKS_CONTAINER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujifilm.fb.docuworks.container\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujifilm.fb.docuworks.container\");"]
    #[doc = r" ```"]
    pub const VND_FUJIFILM_FB_DOCUWORKS_CONTAINER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujifilm.fb.docuworks.container",
    };
    #[doc = "\\[Keitaro_Ishida\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIFILM_FB_JFI_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujifilm.fb.jfi+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujifilm.fb.jfi+xml\");"]
    #[doc = r" ```"]
    pub const VND_FUJIFILM_FB_JFI_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujifilm.fb.jfi+xml",
    };
    #[doc = "\\[Nobukazu_Togashi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJITSU_OASYS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujitsu.oasys\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujitsu.oasys\");"]
    #[doc = r" ```"]
    pub const VND_FUJITSU_OASYS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujitsu.oasys",
    };
    #[doc = "\\[Nobukazu_Togashi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJITSU_OASYS2;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujitsu.oasys2\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujitsu.oasys2\");"]
    #[doc = r" ```"]
    pub const VND_FUJITSU_OASYS2: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujitsu.oasys2",
    };
    #[doc = "\\[Seiji_Okudaira\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJITSU_OASYS3;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujitsu.oasys3\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujitsu.oasys3\");"]
    #[doc = r" ```"]
    pub const VND_FUJITSU_OASYS3: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujitsu.oasys3",
    };
    #[doc = "\\[Masahiko_Sugimoto\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJITSU_OASYSGP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujitsu.oasysgp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujitsu.oasysgp\");"]
    #[doc = r" ```"]
    pub const VND_FUJITSU_OASYSGP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujitsu.oasysgp",
    };
    #[doc = "\\[Masumi_Ogita\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJITSU_OASYSPRS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujitsu.oasysprs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujitsu.oasysprs\");"]
    #[doc = r" ```"]
    pub const VND_FUJITSU_OASYSPRS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujitsu.oasysprs",
    };
    #[doc = "\\[Fumio_Tanabe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIXEROX_ART4;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujixerox.ART4\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujixerox.ART4\");"]
    #[doc = r" ```"]
    pub const VND_FUJIXEROX_ART4: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujixerox.ART4",
    };
    #[doc = "\\[Fumio_Tanabe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIXEROX_ART_EX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujixerox.ART-EX\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujixerox.ART-EX\");"]
    #[doc = r" ```"]
    pub const VND_FUJIXEROX_ART_EX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujixerox.ART-EX",
    };
    #[doc = "\\[Masanori_Onda\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIXEROX_DDD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujixerox.ddd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujixerox.ddd\");"]
    #[doc = r" ```"]
    pub const VND_FUJIXEROX_DDD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujixerox.ddd",
    };
    #[doc = "\\[Takatomo_Wakibayashi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIXEROX_DOCUWORKS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujixerox.docuworks\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujixerox.docuworks\");"]
    #[doc = r" ```"]
    pub const VND_FUJIXEROX_DOCUWORKS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujixerox.docuworks",
    };
    #[doc = "\\[Takashi_Matsumoto\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIXEROX_DOCUWORKS_BINDER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujixerox.docuworks.binder\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujixerox.docuworks.binder\");"]
    #[doc = r" ```"]
    pub const VND_FUJIXEROX_DOCUWORKS_BINDER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujixerox.docuworks.binder",
    };
    #[doc = "\\[Kiyoshi_Tashiro\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIXEROX_DOCUWORKS_CONTAINER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujixerox.docuworks.container\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujixerox.docuworks.container\");"]
    #[doc = r" ```"]
    pub const VND_FUJIXEROX_DOCUWORKS_CONTAINER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujixerox.docuworks.container",
    };
    #[doc = "\\[Fumio_Tanabe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUJIXEROX_HBPL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fujixerox.HBPL\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fujixerox.HBPL\");"]
    #[doc = r" ```"]
    pub const VND_FUJIXEROX_HBPL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fujixerox.HBPL",
    };
    #[doc = "\\[Jann_Pruulman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUT_MISNET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fut-misnet\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fut-misnet\");"]
    #[doc = r" ```"]
    pub const VND_FUT_MISNET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fut-misnet",
    };
    #[doc = "\\[Andrey_Galkin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUTOIN_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.futoin+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.futoin+cbor\");"]
    #[doc = r" ```"]
    pub const VND_FUTOIN_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.futoin+cbor",
    };
    #[doc = "\\[Andrey_Galkin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUTOIN_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.futoin+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.futoin+json\");"]
    #[doc = r" ```"]
    pub const VND_FUTOIN_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.futoin+json",
    };
    #[doc = "\\[Simon_Birtwistle\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_FUZZYSHEET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.fuzzysheet\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.fuzzysheet\");"]
    #[doc = r" ```"]
    pub const VND_FUZZYSHEET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.fuzzysheet",
    };
    #[doc = "\\[GA4GH_Secretariat\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GA4GH_PASSPORT_JWT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ga4gh.passport+jwt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ga4gh.passport+jwt\");"]
    #[doc = r" ```"]
    pub const VND_GA4GH_PASSPORT_JWT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ga4gh.passport+jwt",
    };
    #[doc = "\\[Torben_Frey\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GENOMATIX_TUXEDO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.genomatix.tuxedo\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.genomatix.tuxedo\");"]
    #[doc = r" ```"]
    pub const VND_GENOMATIX_TUXEDO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.genomatix.tuxedo",
    };
    #[doc = "\\[Divon_Lan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GENOZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.genozip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.genozip\");"]
    #[doc = r" ```"]
    pub const VND_GENOZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.genozip",
    };
    #[doc = "\\[Philipp_Gortan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GENTICS_GRD_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gentics.grd+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gentics.grd+json\");"]
    #[doc = r" ```"]
    pub const VND_GENTICS_GRD_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gentics.grd+json",
    };
    #[doc = "\\[Michał_Górny\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GENTOO_CATMETADATA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gentoo.catmetadata+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gentoo.catmetadata+xml\");"]
    #[doc = r" ```"]
    pub const VND_GENTOO_CATMETADATA_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gentoo.catmetadata+xml",
    };
    #[doc = "\\[Michał_Górny\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GENTOO_EBUILD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gentoo.ebuild\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gentoo.ebuild\");"]
    #[doc = r" ```"]
    pub const VND_GENTOO_EBUILD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gentoo.ebuild",
    };
    #[doc = "\\[Michał_Górny\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GENTOO_ECLASS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gentoo.eclass\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gentoo.eclass\");"]
    #[doc = r" ```"]
    pub const VND_GENTOO_ECLASS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gentoo.eclass",
    };
    #[doc = "\\[Michał_Górny\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GENTOO_GPKG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gentoo.gpkg\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gentoo.gpkg\");"]
    #[doc = r" ```"]
    pub const VND_GENTOO_GPKG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gentoo.gpkg",
    };
    #[doc = "\\[Michał_Górny\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GENTOO_MANIFEST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gentoo.manifest\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gentoo.manifest\");"]
    #[doc = r" ```"]
    pub const VND_GENTOO_MANIFEST: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gentoo.manifest",
    };
    #[doc = "\\[Gentoo_Portage_Project\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GENTOO_XPAK;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gentoo.xpak\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gentoo.xpak\");"]
    #[doc = r" ```"]
    pub const VND_GENTOO_XPAK: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gentoo.xpak",
    };
    #[doc = "\\[Michał_Górny\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GENTOO_PKGMETADATA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gentoo.pkgmetadata+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gentoo.pkgmetadata+xml\");"]
    #[doc = r" ```"]
    pub const VND_GENTOO_PKGMETADATA_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gentoo.pkgmetadata+xml",
    };
    #[doc = "\\[Sean_Gillies\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GEO_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.geo+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.geo+json\");"]
    #[doc = r" ```"]
    pub const VND_GEO_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.geo+json",
    };
    #[doc = "\\[Francois_Pirsch\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GEOCUBE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.geocube+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.geocube+xml\");"]
    #[doc = r" ```"]
    pub const VND_GEOCUBE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.geocube+xml",
    };
    #[doc = "\\[GeoGebra\\]\\[Yves_Kreis\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GEOGEBRA_FILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.geogebra.file\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.geogebra.file\");"]
    #[doc = r" ```"]
    pub const VND_GEOGEBRA_FILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.geogebra.file",
    };
    #[doc = "\\[GeoGebra\\]\\[Michael_Borcherds\\]\\[Markus_Hohenwarter\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GEOGEBRA_SLIDES;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.geogebra.slides\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.geogebra.slides\");"]
    #[doc = r" ```"]
    pub const VND_GEOGEBRA_SLIDES: Mime = Mime {
        ttype: "application",
        subtype: "vnd.geogebra.slides",
    };
    #[doc = "\\[GeoGebra\\]\\[Yves_Kreis\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GEOGEBRA_TOOL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.geogebra.tool\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.geogebra.tool\");"]
    #[doc = r" ```"]
    pub const VND_GEOGEBRA_TOOL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.geogebra.tool",
    };
    #[doc = "\\[Michael_Hvidsten\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GEOMETRY_EXPLORER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.geometry-explorer\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.geometry-explorer\");"]
    #[doc = r" ```"]
    pub const VND_GEOMETRY_EXPLORER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.geometry-explorer",
    };
    #[doc = "\\[Matthias_Ehmann\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GEONEXT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.geonext\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.geonext\");"]
    #[doc = r" ```"]
    pub const VND_GEONEXT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.geonext",
    };
    #[doc = "\\[Christian_Mercat\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GEOPLAN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.geoplan\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.geoplan\");"]
    #[doc = r" ```"]
    pub const VND_GEOPLAN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.geoplan",
    };
    #[doc = "\\[Christian_Mercat\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GEOSPACE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.geospace\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.geospace\");"]
    #[doc = r" ```"]
    pub const VND_GEOSPACE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.geospace",
    };
    #[doc = "\\[Thomas_Weyn\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GERBER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gerber\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gerber\");"]
    #[doc = r" ```"]
    pub const VND_GERBER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gerber",
    };
    #[doc = "\\[Gil_Bernabeu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GLOBALPLATFORM_CARD_CONTENT_MGT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.globalplatform.card-content-mgt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.globalplatform.card-content-mgt\");"]
    #[doc = r" ```"]
    pub const VND_GLOBALPLATFORM_CARD_CONTENT_MGT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.globalplatform.card-content-mgt",
    };
    #[doc = "\\[Gil_Bernabeu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GLOBALPLATFORM_CARD_CONTENT_MGT_RESPONSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.globalplatform.card-content-mgt-response\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.globalplatform.card-content-mgt-response\");"]
    #[doc = r" ```"]
    pub const VND_GLOBALPLATFORM_CARD_CONTENT_MGT_RESPONSE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.globalplatform.card-content-mgt-response",
    };
    #[doc = "\\[Christian_V._Sciberras\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GMX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gmx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gmx\");"]
    #[doc = r" ```"]
    pub const VND_GMX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gmx",
    };
    #[doc = "\\[Christian_Grothoff\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GNU_TALER_EXCHANGE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gnu.taler.exchange+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gnu.taler.exchange+json\");"]
    #[doc = r" ```"]
    pub const VND_GNU_TALER_EXCHANGE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gnu.taler.exchange+json",
    };
    #[doc = "\\[Christian_Grothoff\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GNU_TALER_MERCHANT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gnu.taler.merchant+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gnu.taler.merchant+json\");"]
    #[doc = r" ```"]
    pub const VND_GNU_TALER_MERCHANT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gnu.taler.merchant+json",
    };
    #[doc = "\\[Michael_Ashbridge\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GOOGLE_EARTH_KML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.google-earth.kml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.google-earth.kml+xml\");"]
    #[doc = r" ```"]
    pub const VND_GOOGLE_EARTH_KML_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.google-earth.kml+xml",
    };
    #[doc = "\\[Michael_Ashbridge\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GOOGLE_EARTH_KMZ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.google-earth.kmz\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.google-earth.kmz\");"]
    #[doc = r" ```"]
    pub const VND_GOOGLE_EARTH_KMZ: Mime = Mime {
        ttype: "application",
        subtype: "vnd.google-earth.kmz",
    };
    #[doc = "\\[Stefan_Szilva\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GOV_SK_E_FORM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gov.sk.e-form+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gov.sk.e-form+xml\");"]
    #[doc = r" ```"]
    pub const VND_GOV_SK_E_FORM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gov.sk.e-form+xml",
    };
    #[doc = "\\[Stefan_Szilva\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GOV_SK_E_FORM_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gov.sk.e-form+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gov.sk.e-form+zip\");"]
    #[doc = r" ```"]
    pub const VND_GOV_SK_E_FORM_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gov.sk.e-form+zip",
    };
    #[doc = "\\[Stefan_Szilva\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GOV_SK_XMLDATACONTAINER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gov.sk.xmldatacontainer+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gov.sk.xmldatacontainer+xml\");"]
    #[doc = r" ```"]
    pub const VND_GOV_SK_XMLDATACONTAINER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gov.sk.xmldatacontainer+xml",
    };
    #[doc = "\\[Martin_Tůma\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GPXSEE_MAP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gpxsee.map+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gpxsee.map+xml\");"]
    #[doc = r" ```"]
    pub const VND_GPXSEE_MAP_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gpxsee.map+xml",
    };
    #[doc = "\\[Jeff_Tupper\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GRAFEQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.grafeq\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.grafeq\");"]
    #[doc = r" ```"]
    pub const VND_GRAFEQ: Mime = Mime {
        ttype: "application",
        subtype: "vnd.grafeq",
    };
    #[doc = "\\[Jeff_Lawson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GRIDMP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.gridmp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.gridmp\");"]
    #[doc = r" ```"]
    pub const VND_GRIDMP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.gridmp",
    };
    #[doc = "\\[Todd_Joseph\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GROOVE_ACCOUNT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.groove-account\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.groove-account\");"]
    #[doc = r" ```"]
    pub const VND_GROOVE_ACCOUNT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.groove-account",
    };
    #[doc = "\\[Todd_Joseph\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GROOVE_HELP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.groove-help\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.groove-help\");"]
    #[doc = r" ```"]
    pub const VND_GROOVE_HELP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.groove-help",
    };
    #[doc = "\\[Todd_Joseph\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GROOVE_IDENTITY_MESSAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.groove-identity-message\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.groove-identity-message\");"]
    #[doc = r" ```"]
    pub const VND_GROOVE_IDENTITY_MESSAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.groove-identity-message",
    };
    #[doc = "\\[Todd_Joseph\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GROOVE_INJECTOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.groove-injector\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.groove-injector\");"]
    #[doc = r" ```"]
    pub const VND_GROOVE_INJECTOR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.groove-injector",
    };
    #[doc = "\\[Todd_Joseph\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GROOVE_TOOL_MESSAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.groove-tool-message\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.groove-tool-message\");"]
    #[doc = r" ```"]
    pub const VND_GROOVE_TOOL_MESSAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.groove-tool-message",
    };
    #[doc = "\\[Todd_Joseph\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GROOVE_TOOL_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.groove-tool-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.groove-tool-template\");"]
    #[doc = r" ```"]
    pub const VND_GROOVE_TOOL_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.groove-tool-template",
    };
    #[doc = "\\[Todd_Joseph\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_GROOVE_VCARD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.groove-vcard\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.groove-vcard\");"]
    #[doc = r" ```"]
    pub const VND_GROOVE_VCARD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.groove-vcard",
    };
    #[doc = "\\[Mike_Kelly\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HAL_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hal+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hal+json\");"]
    #[doc = r" ```"]
    pub const VND_HAL_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hal+json",
    };
    #[doc = "\\[Mike_Kelly\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HAL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hal+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hal+xml\");"]
    #[doc = r" ```"]
    pub const VND_HAL_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hal+xml",
    };
    #[doc = "\\[Eric_Hamilton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HANDHELD_ENTERTAINMENT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.HandHeld-Entertainment+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.HandHeld-Entertainment+xml\");"]
    #[doc = r" ```"]
    pub const VND_HANDHELD_ENTERTAINMENT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.HandHeld-Entertainment+xml",
    };
    #[doc = "\\[Ingo_Hammann\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HBCI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hbci\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hbci\");"]
    #[doc = r" ```"]
    pub const VND_HBCI: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hbci",
    };
    #[doc = "\\[Jan_Schütze\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HC_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hc+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hc+json\");"]
    #[doc = r" ```"]
    pub const VND_HC_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hc+json",
    };
    #[doc = "\\[Doug_R._Serres\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HCL_BIREPORTS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hcl-bireports\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hcl-bireports\");"]
    #[doc = r" ```"]
    pub const VND_HCL_BIREPORTS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hcl-bireports",
    };
    #[doc = "\\[Javier_D._Fernández\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HDT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hdt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hdt\");"]
    #[doc = r" ```"]
    pub const VND_HDT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hdt",
    };
    #[doc = "\\[Wesley_Beary\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HEROKU_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.heroku+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.heroku+json\");"]
    #[doc = r" ```"]
    pub const VND_HEROKU_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.heroku+json",
    };
    #[doc = "\\[Randy_Jones\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HHE_LESSON_PLAYER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hhe.lesson-player\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hhe.lesson-player\");"]
    #[doc = r" ```"]
    pub const VND_HHE_LESSON_PLAYER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hhe.lesson-player",
    };
    #[doc = "\\[Bob_Pentecost\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HP_HPGL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hp-HPGL\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hp-HPGL\");"]
    #[doc = r" ```"]
    pub const VND_HP_HPGL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hp-HPGL",
    };
    #[doc = "\\[Aloke_Gupta\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HP_HPID;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hp-hpid\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hp-hpid\");"]
    #[doc = r" ```"]
    pub const VND_HP_HPID: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hp-hpid",
    };
    #[doc = "\\[Steve_Aubrey\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HP_HPS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hp-hps\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hp-hps\");"]
    #[doc = r" ```"]
    pub const VND_HP_HPS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hp-hps",
    };
    #[doc = "\\[Amir_Gaash\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HP_JLYT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hp-jlyt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hp-jlyt\");"]
    #[doc = r" ```"]
    pub const VND_HP_JLYT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hp-jlyt",
    };
    #[doc = "\\[Bob_Pentecost\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HP_PCL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hp-PCL\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hp-PCL\");"]
    #[doc = r" ```"]
    pub const VND_HP_PCL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hp-PCL",
    };
    #[doc = "\\[Bob_Pentecost\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HP_PCLXL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hp-PCLXL\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hp-PCLXL\");"]
    #[doc = r" ```"]
    pub const VND_HP_PCLXL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hp-PCLXL",
    };
    #[doc = "\\[Heungsub_Lee\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HSL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hsl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hsl\");"]
    #[doc = r" ```"]
    pub const VND_HSL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hsl",
    };
    #[doc = "\\[Franck_Lefevre\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HTTPHONE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.httphone\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.httphone\");"]
    #[doc = r" ```"]
    pub const VND_HTTPHONE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.httphone",
    };
    #[doc = "\\[Allen_Gillam\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HYDROSTATIX_SOF_DATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hydrostatix.sof-data\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hydrostatix.sof-data\");"]
    #[doc = r" ```"]
    pub const VND_HYDROSTATIX_SOF_DATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hydrostatix.sof-data",
    };
    #[doc = "\\[Mario_Demuth\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HYPER_ITEM_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hyper-item+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hyper-item+json\");"]
    #[doc = r" ```"]
    pub const VND_HYPER_ITEM_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hyper-item+json",
    };
    #[doc = "\\[Irakli_Nadareishvili\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HYPER_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hyper+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hyper+json\");"]
    #[doc = r" ```"]
    pub const VND_HYPER_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hyper+json",
    };
    #[doc = "\\[Daniel_Sims\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HYPERDRIVE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hyperdrive+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hyperdrive+json\");"]
    #[doc = r" ```"]
    pub const VND_HYPERDRIVE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hyperdrive+json",
    };
    #[doc = "\\[James_Minnis\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_HZN_3D_CROSSWORD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.hzn-3d-crossword\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.hzn-3d-crossword\");"]
    #[doc = r" ```"]
    pub const VND_HZN_3D_CROSSWORD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.hzn-3d-crossword",
    };
    #[doc = "\\[Roger_Buis\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IBM_AFPLINEDATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ibm.afplinedata\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ibm.afplinedata\");"]
    #[doc = r" ```"]
    pub const VND_IBM_AFPLINEDATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ibm.afplinedata",
    };
    #[doc = "\\[Bruce_Tantlinger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IBM_ELECTRONIC_MEDIA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ibm.electronic-media\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ibm.electronic-media\");"]
    #[doc = r" ```"]
    pub const VND_IBM_ELECTRONIC_MEDIA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ibm.electronic-media",
    };
    #[doc = "\\[Amir_Herzberg\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IBM_MINIPAY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ibm.MiniPay\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ibm.MiniPay\");"]
    #[doc = r" ```"]
    pub const VND_IBM_MINIPAY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ibm.MiniPay",
    };
    #[doc = "\\[Reinhard_Hohensee\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IBM_MODCAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ibm.modcap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ibm.modcap\");"]
    #[doc = r" ```"]
    pub const VND_IBM_MODCAP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ibm.modcap",
    };
    #[doc = "\\[Bruce_Tantlinger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IBM_RIGHTS_MANAGEMENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ibm.rights-management\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ibm.rights-management\");"]
    #[doc = r" ```"]
    pub const VND_IBM_RIGHTS_MANAGEMENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ibm.rights-management",
    };
    #[doc = "\\[Bruce_Tantlinger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IBM_SECURE_CONTAINER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ibm.secure-container\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ibm.secure-container\");"]
    #[doc = r" ```"]
    pub const VND_IBM_SECURE_CONTAINER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ibm.secure-container",
    };
    #[doc = "\\[Phil_Green\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ICCPROFILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iccprofile\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.iccprofile\");"]
    #[doc = r" ```"]
    pub const VND_ICCPROFILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.iccprofile",
    };
    #[doc = "\\[Purva_R_Rajkotia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IEEE_1905;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ieee.1905\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ieee.1905\");"]
    #[doc = r" ```"]
    pub const VND_IEEE_1905: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ieee.1905",
    };
    #[doc = "\\[Tim_Fisher\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IGLOADER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.igloader\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.igloader\");"]
    #[doc = r" ```"]
    pub const VND_IGLOADER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.igloader",
    };
    #[doc = "\\[Dirk_Farin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMAGEMETER_FOLDER_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.imagemeter.folder+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.imagemeter.folder+zip\");"]
    #[doc = r" ```"]
    pub const VND_IMAGEMETER_FOLDER_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.imagemeter.folder+zip",
    };
    #[doc = "\\[Dirk_Farin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMAGEMETER_IMAGE_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.imagemeter.image+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.imagemeter.image+zip\");"]
    #[doc = r" ```"]
    pub const VND_IMAGEMETER_IMAGE_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.imagemeter.image+zip",
    };
    #[doc = "\\[Mathieu_Villegas\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMMERVISION_IVP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.immervision-ivp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.immervision-ivp\");"]
    #[doc = r" ```"]
    pub const VND_IMMERVISION_IVP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.immervision-ivp",
    };
    #[doc = "\\[Mathieu_Villegas\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMMERVISION_IVU;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.immervision-ivu\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.immervision-ivu\");"]
    #[doc = r" ```"]
    pub const VND_IMMERVISION_IVU: Mime = Mime {
        ttype: "application",
        subtype: "vnd.immervision-ivu",
    };
    #[doc = "\\[Lisa_Mattson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMS_IMSCCV1P1;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ims.imsccv1p1\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ims.imsccv1p1\");"]
    #[doc = r" ```"]
    pub const VND_IMS_IMSCCV1P1: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ims.imsccv1p1",
    };
    #[doc = "\\[Lisa_Mattson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMS_IMSCCV1P2;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ims.imsccv1p2\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ims.imsccv1p2\");"]
    #[doc = r" ```"]
    pub const VND_IMS_IMSCCV1P2: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ims.imsccv1p2",
    };
    #[doc = "\\[Lisa_Mattson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMS_IMSCCV1P3;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ims.imsccv1p3\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ims.imsccv1p3\");"]
    #[doc = r" ```"]
    pub const VND_IMS_IMSCCV1P3: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ims.imsccv1p3",
    };
    #[doc = "\\[Lisa_Mattson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMS_LIS_V2_RESULT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ims.lis.v2.result+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ims.lis.v2.result+json\");"]
    #[doc = r" ```"]
    pub const VND_IMS_LIS_V2_RESULT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ims.lis.v2.result+json",
    };
    #[doc = "\\[Lisa_Mattson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMS_LTI_V2_TOOLCONSUMERPROFILE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ims.lti.v2.toolconsumerprofile+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ims.lti.v2.toolconsumerprofile+json\");"]
    #[doc = r" ```"]
    pub const VND_IMS_LTI_V2_TOOLCONSUMERPROFILE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ims.lti.v2.toolconsumerprofile+json",
    };
    #[doc = "\\[Lisa_Mattson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMS_LTI_V2_TOOLPROXY_ID_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ims.lti.v2.toolproxy.id+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ims.lti.v2.toolproxy.id+json\");"]
    #[doc = r" ```"]
    pub const VND_IMS_LTI_V2_TOOLPROXY_ID_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ims.lti.v2.toolproxy.id+json",
    };
    #[doc = "\\[Lisa_Mattson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMS_LTI_V2_TOOLPROXY_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ims.lti.v2.toolproxy+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ims.lti.v2.toolproxy+json\");"]
    #[doc = r" ```"]
    pub const VND_IMS_LTI_V2_TOOLPROXY_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ims.lti.v2.toolproxy+json",
    };
    #[doc = "\\[Lisa_Mattson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMS_LTI_V2_TOOLSETTINGS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ims.lti.v2.toolsettings+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ims.lti.v2.toolsettings+json\");"]
    #[doc = r" ```"]
    pub const VND_IMS_LTI_V2_TOOLSETTINGS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ims.lti.v2.toolsettings+json",
    };
    #[doc = "\\[Lisa_Mattson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IMS_LTI_V2_TOOLSETTINGS_SIMPLE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ims.lti.v2.toolsettings.simple+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ims.lti.v2.toolsettings.simple+json\");"]
    #[doc = r" ```"]
    pub const VND_IMS_LTI_V2_TOOLSETTINGS_SIMPLE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ims.lti.v2.toolsettings.simple+json",
    };
    #[doc = "\\[Mark_Wahl\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INFORMEDCONTROL_RMS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.informedcontrol.rms+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.informedcontrol.rms+xml\");"]
    #[doc = r" ```"]
    pub const VND_INFORMEDCONTROL_RMS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.informedcontrol.rms+xml",
    };
    #[doc = "\\[Charles_Engelke\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INFOTECH_PROJECT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.infotech.project\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.infotech.project\");"]
    #[doc = r" ```"]
    pub const VND_INFOTECH_PROJECT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.infotech.project",
    };
    #[doc = "\\[Charles_Engelke\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INFOTECH_PROJECT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.infotech.project+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.infotech.project+xml\");"]
    #[doc = r" ```"]
    pub const VND_INFOTECH_PROJECT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.infotech.project+xml",
    };
    #[doc = "\\[Christopher_Gales\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INFORMIX_VISIONARY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.informix-visionary\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.informix-visionary\");"]
    #[doc = r" ```"]
    pub const VND_INFORMIX_VISIONARY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.informix-visionary",
    };
    #[doc = "\\[Takanori_Sudo\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INNOPATH_WAMP_NOTIFICATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.innopath.wamp.notification\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.innopath.wamp.notification\");"]
    #[doc = r" ```"]
    pub const VND_INNOPATH_WAMP_NOTIFICATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.innopath.wamp.notification",
    };
    #[doc = "\\[Jon_Swanson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INSORS_IGM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.insors.igm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.insors.igm\");"]
    #[doc = r" ```"]
    pub const VND_INSORS_IGM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.insors.igm",
    };
    #[doc = "\\[Tom_Gurak\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INTERCON_FORMNET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.intercon.formnet\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.intercon.formnet\");"]
    #[doc = r" ```"]
    pub const VND_INTERCON_FORMNET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.intercon.formnet",
    };
    #[doc = "\\[Yves_Kreis_2\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INTERGEO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.intergeo\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.intergeo\");"]
    #[doc = r" ```"]
    pub const VND_INTERGEO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.intergeo",
    };
    #[doc = "\\[Luke_Tomasello\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INTERTRUST_DIGIBOX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.intertrust.digibox\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.intertrust.digibox\");"]
    #[doc = r" ```"]
    pub const VND_INTERTRUST_DIGIBOX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.intertrust.digibox",
    };
    #[doc = "\\[Luke_Tomasello\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INTERTRUST_NNCP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.intertrust.nncp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.intertrust.nncp\");"]
    #[doc = r" ```"]
    pub const VND_INTERTRUST_NNCP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.intertrust.nncp",
    };
    #[doc = "\\[Greg_Scratchley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INTU_QBO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.intu.qbo\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.intu.qbo\");"]
    #[doc = r" ```"]
    pub const VND_INTU_QBO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.intu.qbo",
    };
    #[doc = "\\[Greg_Scratchley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_INTU_QFX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.intu.qfx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.intu.qfx\");"]
    #[doc = r" ```"]
    pub const VND_INTU_QFX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.intu.qfx",
    };
    #[doc = "\\[Marcin_Rataj\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPFS_IPNS_RECORD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ipfs.ipns-record\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ipfs.ipns-record\");"]
    #[doc = r" ```"]
    pub const VND_IPFS_IPNS_RECORD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ipfs.ipns-record",
    };
    #[doc = "\\[Marcin_Rataj\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPLD_CAR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ipld.car\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ipld.car\");"]
    #[doc = r" ```"]
    pub const VND_IPLD_CAR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ipld.car",
    };
    #[doc = "\\[Marcin_Rataj\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPLD_DAG_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ipld.dag-cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ipld.dag-cbor\");"]
    #[doc = r" ```"]
    pub const VND_IPLD_DAG_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ipld.dag-cbor",
    };
    #[doc = "\\[Marcin_Rataj\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPLD_DAG_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ipld.dag-json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ipld.dag-json\");"]
    #[doc = r" ```"]
    pub const VND_IPLD_DAG_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ipld.dag-json",
    };
    #[doc = "\\[Marcin_Rataj\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPLD_RAW;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ipld.raw\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ipld.raw\");"]
    #[doc = r" ```"]
    pub const VND_IPLD_RAW: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ipld.raw",
    };
    #[doc = "\\[Michael_Steidl\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPTC_G2_CATALOGITEM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptc.g2.catalogitem+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.iptc.g2.catalogitem+xml\");"]
    #[doc = r" ```"]
    pub const VND_IPTC_G2_CATALOGITEM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.iptc.g2.catalogitem+xml",
    };
    #[doc = "\\[Michael_Steidl\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPTC_G2_CONCEPTITEM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptc.g2.conceptitem+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.iptc.g2.conceptitem+xml\");"]
    #[doc = r" ```"]
    pub const VND_IPTC_G2_CONCEPTITEM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.iptc.g2.conceptitem+xml",
    };
    #[doc = "\\[Michael_Steidl\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPTC_G2_KNOWLEDGEITEM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptc.g2.knowledgeitem+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.iptc.g2.knowledgeitem+xml\");"]
    #[doc = r" ```"]
    pub const VND_IPTC_G2_KNOWLEDGEITEM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.iptc.g2.knowledgeitem+xml",
    };
    #[doc = "\\[Michael_Steidl\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPTC_G2_NEWSITEM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptc.g2.newsitem+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.iptc.g2.newsitem+xml\");"]
    #[doc = r" ```"]
    pub const VND_IPTC_G2_NEWSITEM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.iptc.g2.newsitem+xml",
    };
    #[doc = "\\[Michael_Steidl\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPTC_G2_NEWSMESSAGE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptc.g2.newsmessage+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.iptc.g2.newsmessage+xml\");"]
    #[doc = r" ```"]
    pub const VND_IPTC_G2_NEWSMESSAGE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.iptc.g2.newsmessage+xml",
    };
    #[doc = "\\[Michael_Steidl\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPTC_G2_PACKAGEITEM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptc.g2.packageitem+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.iptc.g2.packageitem+xml\");"]
    #[doc = r" ```"]
    pub const VND_IPTC_G2_PACKAGEITEM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.iptc.g2.packageitem+xml",
    };
    #[doc = "\\[Michael_Steidl\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPTC_G2_PLANNINGITEM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iptc.g2.planningitem+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.iptc.g2.planningitem+xml\");"]
    #[doc = r" ```"]
    pub const VND_IPTC_G2_PLANNINGITEM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.iptc.g2.planningitem+xml",
    };
    #[doc = "\\[Per_Ersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IPUNPLUGGED_RCPROFILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ipunplugged.rcprofile\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ipunplugged.rcprofile\");"]
    #[doc = r" ```"]
    pub const VND_IPUNPLUGGED_RCPROFILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ipunplugged.rcprofile",
    };
    #[doc = "\\[Martin_Knowles\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IREPOSITORY_PACKAGE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.irepository.package+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.irepository.package+xml\");"]
    #[doc = r" ```"]
    pub const VND_IREPOSITORY_PACKAGE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.irepository.package+xml",
    };
    #[doc = "\\[Satish_Navarajan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_IS_XPR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.is-xpr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.is-xpr\");"]
    #[doc = r" ```"]
    pub const VND_IS_XPR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.is-xpr",
    };
    #[doc = "\\[Ryan_Brinkman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ISAC_FCS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.isac.fcs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.isac.fcs\");"]
    #[doc = r" ```"]
    pub const VND_ISAC_FCS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.isac.fcs",
    };
    #[doc = "\\[Brijesh_Kumar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JAM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.jam\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.jam\");"]
    #[doc = r" ```"]
    pub const VND_JAM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.jam",
    };
    #[doc = "\\[Frank_Wiebeler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ISO11783_10_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.iso11783-10+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.iso11783-10+zip\");"]
    #[doc = r" ```"]
    pub const VND_ISO11783_10_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.iso11783-10+zip",
    };
    #[doc = "\\[Kiyofusa_Fujii\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JAPANNET_DIRECTORY_SERVICE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.japannet-directory-service\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.japannet-directory-service\");"]
    #[doc = r" ```"]
    pub const VND_JAPANNET_DIRECTORY_SERVICE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.japannet-directory-service",
    };
    #[doc = "\\[Jun_Yoshitake\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JAPANNET_JPNSTORE_WAKEUP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.japannet-jpnstore-wakeup\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.japannet-jpnstore-wakeup\");"]
    #[doc = r" ```"]
    pub const VND_JAPANNET_JPNSTORE_WAKEUP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.japannet-jpnstore-wakeup",
    };
    #[doc = "\\[Kiyofusa_Fujii\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JAPANNET_PAYMENT_WAKEUP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.japannet-payment-wakeup\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.japannet-payment-wakeup\");"]
    #[doc = r" ```"]
    pub const VND_JAPANNET_PAYMENT_WAKEUP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.japannet-payment-wakeup",
    };
    #[doc = "\\[Jun_Yoshitake\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JAPANNET_REGISTRATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.japannet-registration\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.japannet-registration\");"]
    #[doc = r" ```"]
    pub const VND_JAPANNET_REGISTRATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.japannet-registration",
    };
    #[doc = "\\[Kiyofusa_Fujii\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JAPANNET_REGISTRATION_WAKEUP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.japannet-registration-wakeup\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.japannet-registration-wakeup\");"]
    #[doc = r" ```"]
    pub const VND_JAPANNET_REGISTRATION_WAKEUP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.japannet-registration-wakeup",
    };
    #[doc = "\\[Jun_Yoshitake\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JAPANNET_SETSTORE_WAKEUP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.japannet-setstore-wakeup\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.japannet-setstore-wakeup\");"]
    #[doc = r" ```"]
    pub const VND_JAPANNET_SETSTORE_WAKEUP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.japannet-setstore-wakeup",
    };
    #[doc = "\\[Jun_Yoshitake\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JAPANNET_VERIFICATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.japannet-verification\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.japannet-verification\");"]
    #[doc = r" ```"]
    pub const VND_JAPANNET_VERIFICATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.japannet-verification",
    };
    #[doc = "\\[Kiyofusa_Fujii\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JAPANNET_VERIFICATION_WAKEUP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.japannet-verification-wakeup\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.japannet-verification-wakeup\");"]
    #[doc = r" ```"]
    pub const VND_JAPANNET_VERIFICATION_WAKEUP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.japannet-verification-wakeup",
    };
    #[doc = "\\[Mikhail_Gorshenev\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JCP_JAVAME_MIDLET_RMS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.jcp.javame.midlet-rms\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.jcp.javame.midlet-rms\");"]
    #[doc = r" ```"]
    pub const VND_JCP_JAVAME_MIDLET_RMS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.jcp.javame.midlet-rms",
    };
    #[doc = "\\[Sebastiaan_Deckers\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JISP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.jisp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.jisp\");"]
    #[doc = r" ```"]
    pub const VND_JISP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.jisp",
    };
    #[doc = "\\[Joost\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JOOST_JODA_ARCHIVE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.joost.joda-archive\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.joost.joda-archive\");"]
    #[doc = r" ```"]
    pub const VND_JOOST_JODA_ARCHIVE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.joost.joda-archive",
    };
    #[doc = "\\[Yokoyama_Kiyonobu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_JSK_ISDN_NGN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.jsk.isdn-ngn\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.jsk.isdn-ngn\");"]
    #[doc = r" ```"]
    pub const VND_JSK_ISDN_NGN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.jsk.isdn-ngn",
    };
    #[doc = "\\[Tim_Macdonald\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KAHOOTZ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kahootz\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kahootz\");"]
    #[doc = r" ```"]
    pub const VND_KAHOOTZ: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kahootz",
    };
    #[doc = "\\[David_Faure\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KDE_KARBON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kde.karbon\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kde.karbon\");"]
    #[doc = r" ```"]
    pub const VND_KDE_KARBON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kde.karbon",
    };
    #[doc = "\\[David_Faure\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KDE_KCHART;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kde.kchart\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kde.kchart\");"]
    #[doc = r" ```"]
    pub const VND_KDE_KCHART: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kde.kchart",
    };
    #[doc = "\\[David_Faure\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KDE_KFORMULA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kde.kformula\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kde.kformula\");"]
    #[doc = r" ```"]
    pub const VND_KDE_KFORMULA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kde.kformula",
    };
    #[doc = "\\[David_Faure\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KDE_KIVIO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kde.kivio\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kde.kivio\");"]
    #[doc = r" ```"]
    pub const VND_KDE_KIVIO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kde.kivio",
    };
    #[doc = "\\[David_Faure\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KDE_KONTOUR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kde.kontour\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kde.kontour\");"]
    #[doc = r" ```"]
    pub const VND_KDE_KONTOUR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kde.kontour",
    };
    #[doc = "\\[David_Faure\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KDE_KPRESENTER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kde.kpresenter\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kde.kpresenter\");"]
    #[doc = r" ```"]
    pub const VND_KDE_KPRESENTER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kde.kpresenter",
    };
    #[doc = "\\[David_Faure\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KDE_KSPREAD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kde.kspread\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kde.kspread\");"]
    #[doc = r" ```"]
    pub const VND_KDE_KSPREAD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kde.kspread",
    };
    #[doc = "\\[David_Faure\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KDE_KWORD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kde.kword\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kde.kword\");"]
    #[doc = r" ```"]
    pub const VND_KDE_KWORD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kde.kword",
    };
    #[doc = "\\[Dirk_DiGiorgio-Haag\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KENAMEAAPP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kenameaapp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kenameaapp\");"]
    #[doc = r" ```"]
    pub const VND_KENAMEAAPP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kenameaapp",
    };
    #[doc = "\\[Marc_Durdin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KEYMAN_KMP_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.keyman.kmp+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.keyman.kmp+zip\");"]
    #[doc = r" ```"]
    pub const VND_KEYMAN_KMP_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.keyman.kmp+zip",
    };
    #[doc = "\\[Marc_Durdin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KEYMAN_KMX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.keyman.kmx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.keyman.kmx\");"]
    #[doc = r" ```"]
    pub const VND_KEYMAN_KMX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.keyman.kmx",
    };
    #[doc = "\\[Jack_Bennett\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KIDSPIRATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kidspiration\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kidspiration\");"]
    #[doc = r" ```"]
    pub const VND_KIDSPIRATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kidspiration",
    };
    #[doc = "\\[Hemant_Thakkar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KINAR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.Kinar\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.Kinar\");"]
    #[doc = r" ```"]
    pub const VND_KINAR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.Kinar",
    };
    #[doc = "\\[Pete_Cole\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KOAN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.koan\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.koan\");"]
    #[doc = r" ```"]
    pub const VND_KOAN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.koan",
    };
    #[doc = "\\[Michael_J._Donahue\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_KODAK_DESCRIPTOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.kodak-descriptor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.kodak-descriptor\");"]
    #[doc = r" ```"]
    pub const VND_KODAK_DESCRIPTOR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.kodak-descriptor",
    };
    #[doc = "\\[NCGIS\\]\\[Bryan_Blank\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LAS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.las\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.las\");"]
    #[doc = r" ```"]
    pub const VND_LAS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.las",
    };
    #[doc = "\\[Rob_Bailey\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LAS_LAS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.las.las+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.las.las+json\");"]
    #[doc = r" ```"]
    pub const VND_LAS_LAS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.las.las+json",
    };
    #[doc = "\\[Rob_Bailey\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LAS_LAS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.las.las+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.las.las+xml\");"]
    #[doc = r" ```"]
    pub const VND_LAS_LAS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.las.las+xml",
    };
    #[doc = "\\[NCGIS\\]\\[Bryan_Blank\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LASZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.laszip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.laszip\");"]
    #[doc = r" ```"]
    pub const VND_LASZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.laszip",
    };
    #[doc = "\\[L.development_Polska\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LDEV_PRODUCTLICENSING;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ldev.productlicensing\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ldev.productlicensing\");"]
    #[doc = r" ```"]
    pub const VND_LDEV_PRODUCTLICENSING: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ldev.productlicensing",
    };
    #[doc = "\\[Mark_C_Fralick\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LEAP_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.leap+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.leap+json\");"]
    #[doc = r" ```"]
    pub const VND_LEAP_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.leap+json",
    };
    #[doc = "\\[Brett_McDowell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LIBERTY_REQUEST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.liberty-request+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.liberty-request+xml\");"]
    #[doc = r" ```"]
    pub const VND_LIBERTY_REQUEST_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.liberty-request+xml",
    };
    #[doc = "\\[Catherine_E._White\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LLAMAGRAPHICS_LIFE_BALANCE_DESKTOP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.llamagraphics.life-balance.desktop\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.llamagraphics.life-balance.desktop\");"]
    #[doc = r" ```"]
    pub const VND_LLAMAGRAPHICS_LIFE_BALANCE_DESKTOP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.llamagraphics.life-balance.desktop",
    };
    #[doc = "\\[Catherine_E._White\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LLAMAGRAPHICS_LIFE_BALANCE_EXCHANGE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.llamagraphics.life-balance.exchange+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.llamagraphics.life-balance.exchange+xml\");"]
    #[doc = r" ```"]
    pub const VND_LLAMAGRAPHICS_LIFE_BALANCE_EXCHANGE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.llamagraphics.life-balance.exchange+xml",
    };
    #[doc = "\\[Victor_Kuchynsky\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LOGIPIPE_CIRCUIT_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.logipipe.circuit+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.logipipe.circuit+zip\");"]
    #[doc = r" ```"]
    pub const VND_LOGIPIPE_CIRCUIT_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.logipipe.circuit+zip",
    };
    #[doc = "\\[Sten_Linnarsson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LOOM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.loom\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.loom\");"]
    #[doc = r" ```"]
    pub const VND_LOOM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.loom",
    };
    #[doc = "\\[Paul_Wattenberger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LOTUS_1_2_3;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.lotus-1-2-3\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.lotus-1-2-3\");"]
    #[doc = r" ```"]
    pub const VND_LOTUS_1_2_3: Mime = Mime {
        ttype: "application",
        subtype: "vnd.lotus-1-2-3",
    };
    #[doc = "\\[Paul_Wattenberger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LOTUS_APPROACH;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.lotus-approach\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.lotus-approach\");"]
    #[doc = r" ```"]
    pub const VND_LOTUS_APPROACH: Mime = Mime {
        ttype: "application",
        subtype: "vnd.lotus-approach",
    };
    #[doc = "\\[Paul_Wattenberger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LOTUS_FREELANCE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.lotus-freelance\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.lotus-freelance\");"]
    #[doc = r" ```"]
    pub const VND_LOTUS_FREELANCE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.lotus-freelance",
    };
    #[doc = "\\[Michael_Laramie\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LOTUS_NOTES;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.lotus-notes\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.lotus-notes\");"]
    #[doc = r" ```"]
    pub const VND_LOTUS_NOTES: Mime = Mime {
        ttype: "application",
        subtype: "vnd.lotus-notes",
    };
    #[doc = "\\[Paul_Wattenberger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LOTUS_ORGANIZER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.lotus-organizer\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.lotus-organizer\");"]
    #[doc = r" ```"]
    pub const VND_LOTUS_ORGANIZER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.lotus-organizer",
    };
    #[doc = "\\[Paul_Wattenberger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LOTUS_SCREENCAM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.lotus-screencam\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.lotus-screencam\");"]
    #[doc = r" ```"]
    pub const VND_LOTUS_SCREENCAM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.lotus-screencam",
    };
    #[doc = "\\[Paul_Wattenberger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_LOTUS_WORDPRO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.lotus-wordpro\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.lotus-wordpro\");"]
    #[doc = r" ```"]
    pub const VND_LOTUS_WORDPRO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.lotus-wordpro",
    };
    #[doc = "\\[James_Berry\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MACPORTS_PORTPKG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.macports.portpkg\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.macports.portpkg\");"]
    #[doc = r" ```"]
    pub const VND_MACPORTS_PORTPKG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.macports.portpkg",
    };
    #[doc = "\\[Blake_Thompson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MAPBOX_VECTOR_TILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mapbox-vector-tile\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mapbox-vector-tile\");"]
    #[doc = r" ```"]
    pub const VND_MAPBOX_VECTOR_TILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mapbox-vector-tile",
    };
    #[doc = "\\[Gary_Ellison\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MARLIN_DRM_ACTIONTOKEN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.marlin.drm.actiontoken+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.marlin.drm.actiontoken+xml\");"]
    #[doc = r" ```"]
    pub const VND_MARLIN_DRM_ACTIONTOKEN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.marlin.drm.actiontoken+xml",
    };
    #[doc = "\\[Gary_Ellison\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MARLIN_DRM_CONFTOKEN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.marlin.drm.conftoken+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.marlin.drm.conftoken+xml\");"]
    #[doc = r" ```"]
    pub const VND_MARLIN_DRM_CONFTOKEN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.marlin.drm.conftoken+xml",
    };
    #[doc = "\\[Gary_Ellison\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MARLIN_DRM_LICENSE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.marlin.drm.license+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.marlin.drm.license+xml\");"]
    #[doc = r" ```"]
    pub const VND_MARLIN_DRM_LICENSE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.marlin.drm.license+xml",
    };
    #[doc = "\\[Gary_Ellison\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MARLIN_DRM_MDCF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.marlin.drm.mdcf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.marlin.drm.mdcf\");"]
    #[doc = r" ```"]
    pub const VND_MARLIN_DRM_MDCF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.marlin.drm.mdcf",
    };
    #[doc = "\\[Jorn_Wildt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MASON_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mason+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mason+json\");"]
    #[doc = r" ```"]
    pub const VND_MASON_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mason+json",
    };
    #[doc = "\\[Erik_Dahlström\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MAXAR_ARCHIVE_3TZ_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.maxar.archive.3tz+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.maxar.archive.3tz+zip\");"]
    #[doc = r" ```"]
    pub const VND_MAXAR_ARCHIVE_3TZ_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.maxar.archive.3tz+zip",
    };
    #[doc = "\\[William_Stevenson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MAXMIND_MAXMIND_DB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.maxmind.maxmind-db\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.maxmind.maxmind-db\");"]
    #[doc = r" ```"]
    pub const VND_MAXMIND_MAXMIND_DB: Mime = Mime {
        ttype: "application",
        subtype: "vnd.maxmind.maxmind-db",
    };
    #[doc = "\\[Tadashi_Gotoh\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MCD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mcd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mcd\");"]
    #[doc = r" ```"]
    pub const VND_MCD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mcd",
    };
    #[doc = "\\[Lutz_Kettner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MDL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mdl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mdl\");"]
    #[doc = r" ```"]
    pub const VND_MDL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mdl",
    };
    #[doc = "\\[Lutz_Kettner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MDL_MBSDF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mdl-mbsdf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mdl-mbsdf\");"]
    #[doc = r" ```"]
    pub const VND_MDL_MBSDF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mdl-mbsdf",
    };
    #[doc = "\\[Frank_Schoonjans\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MEDCALCDATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.medcalcdata\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.medcalcdata\");"]
    #[doc = r" ```"]
    pub const VND_MEDCALCDATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.medcalcdata",
    };
    #[doc = "\\[Henry_Flurry\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MEDIASTATION_CDKEY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mediastation.cdkey\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mediastation.cdkey\");"]
    #[doc = r" ```"]
    pub const VND_MEDIASTATION_CDKEY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mediastation.cdkey",
    };
    #[doc = "\\[Dominique_Sandoz\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MEDICALHOLODECK_RECORDXR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.medicalholodeck.recordxr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.medicalholodeck.recordxr\");"]
    #[doc = r" ```"]
    pub const VND_MEDICALHOLODECK_RECORDXR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.medicalholodeck.recordxr",
    };
    #[doc = "\\[Eric_Wedel\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MERIDIAN_SLINGSHOT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.meridian-slingshot\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.meridian-slingshot\");"]
    #[doc = r" ```"]
    pub const VND_MERIDIAN_SLINGSHOT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.meridian-slingshot",
    };
    #[doc = "\\[Sidharth_Vinod\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MERMAID;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mermaid\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mermaid\");"]
    #[doc = r" ```"]
    pub const VND_MERMAID: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mermaid",
    };
    #[doc = "\\[Masaaki_Hirai\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MFER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.MFER\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.MFER\");"]
    #[doc = r" ```"]
    pub const VND_MFER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.MFER",
    };
    #[doc = "\\[Yukari_Ikeda\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MFMP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mfmp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mfmp\");"]
    #[doc = r" ```"]
    pub const VND_MFMP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mfmp",
    };
    #[doc = "\\[Dali_Zheng\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MICRO_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.micro+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.micro+json\");"]
    #[doc = r" ```"]
    pub const VND_MICRO_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.micro+json",
    };
    #[doc = "\\[Joe_Prevo\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MICROGRAFX_FLO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.micrografx.flo\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.micrografx.flo\");"]
    #[doc = r" ```"]
    pub const VND_MICROGRAFX_FLO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.micrografx.flo",
    };
    #[doc = "\\[Joe_Prevo\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MICROGRAFX_IGX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.micrografx.igx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.micrografx.igx\");"]
    #[doc = r" ```"]
    pub const VND_MICROGRAFX_IGX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.micrografx.igx",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MICROSOFT_PORTABLE_EXECUTABLE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.microsoft.portable-executable\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.microsoft.portable-executable\");"]
    #[doc = r" ```"]
    pub const VND_MICROSOFT_PORTABLE_EXECUTABLE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.microsoft.portable-executable",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MICROSOFT_WINDOWS_THUMBNAIL_CACHE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.microsoft.windows.thumbnail-cache\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.microsoft.windows.thumbnail-cache\");"]
    #[doc = r" ```"]
    pub const VND_MICROSOFT_WINDOWS_THUMBNAIL_CACHE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.microsoft.windows.thumbnail-cache",
    };
    #[doc = "\\[Nils_Langhammer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MIELE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.miele+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.miele+json\");"]
    #[doc = r" ```"]
    pub const VND_MIELE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.miele+json",
    };
    #[doc = "\\[Mike_Wexler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MIF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mif\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mif\");"]
    #[doc = r" ```"]
    pub const VND_MIF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mif",
    };
    #[doc = "\\[Chris_Bartram\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MINISOFT_HP3000_SAVE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.minisoft-hp3000-save\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.minisoft-hp3000-save\");"]
    #[doc = r" ```"]
    pub const VND_MINISOFT_HP3000_SAVE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.minisoft-hp3000-save",
    };
    #[doc = "\\[Tanaka\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MITSUBISHI_MISTY_GUARD_TRUSTWEB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mitsubishi.misty-guard.trustweb\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mitsubishi.misty-guard.trustweb\");"]
    #[doc = r" ```"]
    pub const VND_MITSUBISHI_MISTY_GUARD_TRUSTWEB: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mitsubishi.misty-guard.trustweb",
    };
    #[doc = "\\[Allen_K._Kabayama\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOBIUS_DAF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.Mobius.DAF\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.Mobius.DAF\");"]
    #[doc = r" ```"]
    pub const VND_MOBIUS_DAF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.Mobius.DAF",
    };
    #[doc = "\\[Allen_K._Kabayama\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOBIUS_DIS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.Mobius.DIS\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.Mobius.DIS\");"]
    #[doc = r" ```"]
    pub const VND_MOBIUS_DIS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.Mobius.DIS",
    };
    #[doc = "\\[Alex_Devasia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOBIUS_MBK;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.Mobius.MBK\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.Mobius.MBK\");"]
    #[doc = r" ```"]
    pub const VND_MOBIUS_MBK: Mime = Mime {
        ttype: "application",
        subtype: "vnd.Mobius.MBK",
    };
    #[doc = "\\[Alex_Devasia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOBIUS_MQY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.Mobius.MQY\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.Mobius.MQY\");"]
    #[doc = r" ```"]
    pub const VND_MOBIUS_MQY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.Mobius.MQY",
    };
    #[doc = "\\[Allen_K._Kabayama\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOBIUS_MSL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.Mobius.MSL\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.Mobius.MSL\");"]
    #[doc = r" ```"]
    pub const VND_MOBIUS_MSL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.Mobius.MSL",
    };
    #[doc = "\\[Allen_K._Kabayama\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOBIUS_PLC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.Mobius.PLC\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.Mobius.PLC\");"]
    #[doc = r" ```"]
    pub const VND_MOBIUS_PLC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.Mobius.PLC",
    };
    #[doc = "\\[Allen_K._Kabayama\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOBIUS_TXF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.Mobius.TXF\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.Mobius.TXF\");"]
    #[doc = r" ```"]
    pub const VND_MOBIUS_TXF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.Mobius.TXF",
    };
    #[doc = "\\[Elliott_Brown\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MODL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.modl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.modl\");"]
    #[doc = r" ```"]
    pub const VND_MODL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.modl",
    };
    #[doc = "\\[Bjorn_Wennerstrom\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOPHUN_APPLICATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mophun.application\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mophun.application\");"]
    #[doc = r" ```"]
    pub const VND_MOPHUN_APPLICATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mophun.application",
    };
    #[doc = "\\[Bjorn_Wennerstrom\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOPHUN_CERTIFICATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mophun.certificate\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mophun.certificate\");"]
    #[doc = r" ```"]
    pub const VND_MOPHUN_CERTIFICATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mophun.certificate",
    };
    #[doc = "\\[Mark_Patton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOTOROLA_FLEXSUITE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.flexsuite\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.motorola.flexsuite\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_FLEXSUITE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.motorola.flexsuite",
    };
    #[doc = "\\[Mark_Patton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOTOROLA_FLEXSUITE_ADSI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.flexsuite.adsi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.motorola.flexsuite.adsi\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_FLEXSUITE_ADSI: Mime = Mime {
        ttype: "application",
        subtype: "vnd.motorola.flexsuite.adsi",
    };
    #[doc = "\\[Mark_Patton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOTOROLA_FLEXSUITE_FIS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.flexsuite.fis\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.motorola.flexsuite.fis\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_FLEXSUITE_FIS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.motorola.flexsuite.fis",
    };
    #[doc = "\\[Mark_Patton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOTOROLA_FLEXSUITE_GOTAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.flexsuite.gotap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.motorola.flexsuite.gotap\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_FLEXSUITE_GOTAP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.motorola.flexsuite.gotap",
    };
    #[doc = "\\[Mark_Patton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOTOROLA_FLEXSUITE_KMR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.flexsuite.kmr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.motorola.flexsuite.kmr\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_FLEXSUITE_KMR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.motorola.flexsuite.kmr",
    };
    #[doc = "\\[Mark_Patton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOTOROLA_FLEXSUITE_TTC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.flexsuite.ttc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.motorola.flexsuite.ttc\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_FLEXSUITE_TTC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.motorola.flexsuite.ttc",
    };
    #[doc = "\\[Mark_Patton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOTOROLA_FLEXSUITE_WEM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.flexsuite.wem\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.motorola.flexsuite.wem\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_FLEXSUITE_WEM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.motorola.flexsuite.wem",
    };
    #[doc = "\\[Rafie_Shamsaasef\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOTOROLA_IPRM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.motorola.iprm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.motorola.iprm\");"]
    #[doc = r" ```"]
    pub const VND_MOTOROLA_IPRM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.motorola.iprm",
    };
    #[doc = "\\[Braden_N_McDaniel\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MOZILLA_XUL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mozilla.xul+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mozilla.xul+xml\");"]
    #[doc = r" ```"]
    pub const VND_MOZILLA_XUL_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mozilla.xul+xml",
    };
    #[doc = "\\[Dean_Slawson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_ARTGALRY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-artgalry\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-artgalry\");"]
    #[doc = r" ```"]
    pub const VND_MS_ARTGALRY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-artgalry",
    };
    #[doc = "\\[Eric_Fleischman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_ASF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-asf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-asf\");"]
    #[doc = r" ```"]
    pub const VND_MS_ASF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-asf",
    };
    #[doc = "\\[Kim_Scarborough\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_CAB_COMPRESSED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-cab-compressed\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-cab-compressed\");"]
    #[doc = r" ```"]
    pub const VND_MS_CAB_COMPRESSED: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-cab-compressed",
    };
    #[doc = "\\[Shawn_Maloney\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_3MFDOCUMENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-3mfdocument\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-3mfdocument\");"]
    #[doc = r" ```"]
    pub const VND_MS_3MFDOCUMENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-3mfdocument",
    };
    #[doc = "\\[Sukvinder_S._Gill\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_EXCEL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-excel\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-excel\");"]
    #[doc = r" ```"]
    pub const VND_MS_EXCEL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-excel",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_EXCEL_ADDIN_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-excel.addin.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-excel.addin.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_EXCEL_ADDIN_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-excel.addin.macroEnabled.12",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_EXCEL_SHEET_BINARY_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-excel.sheet.binary.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-excel.sheet.binary.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_EXCEL_SHEET_BINARY_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-excel.sheet.binary.macroEnabled.12",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_EXCEL_SHEET_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-excel.sheet.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-excel.sheet.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_EXCEL_SHEET_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-excel.sheet.macroEnabled.12",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_EXCEL_TEMPLATE_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-excel.template.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-excel.template.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_EXCEL_TEMPLATE_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-excel.template.macroEnabled.12",
    };
    #[doc = "\\[Kim_Scarborough\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_FONTOBJECT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-fontobject\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-fontobject\");"]
    #[doc = r" ```"]
    pub const VND_MS_FONTOBJECT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-fontobject",
    };
    #[doc = "\\[Anatoly_Techtonik\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_HTMLHELP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-htmlhelp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-htmlhelp\");"]
    #[doc = r" ```"]
    pub const VND_MS_HTMLHELP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-htmlhelp",
    };
    #[doc = "\\[Eric_Ledoux\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_IMS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-ims\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-ims\");"]
    #[doc = r" ```"]
    pub const VND_MS_IMS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-ims",
    };
    #[doc = "\\[Eric_Ledoux\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_LRM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-lrm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-lrm\");"]
    #[doc = r" ```"]
    pub const VND_MS_LRM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-lrm",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_OFFICE_ACTIVEX_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-office.activeX+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-office.activeX+xml\");"]
    #[doc = r" ```"]
    pub const VND_MS_OFFICE_ACTIVEX_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-office.activeX+xml",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_OFFICETHEME;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-officetheme\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-officetheme\");"]
    #[doc = r" ```"]
    pub const VND_MS_OFFICETHEME: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-officetheme",
    };
    #[doc = "\\[Daniel_Schneider\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_PLAYREADY_INITIATOR_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-playready.initiator+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-playready.initiator+xml\");"]
    #[doc = r" ```"]
    pub const VND_MS_PLAYREADY_INITIATOR_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-playready.initiator+xml",
    };
    #[doc = "\\[Sukvinder_S._Gill\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_POWERPOINT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-powerpoint\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-powerpoint\");"]
    #[doc = r" ```"]
    pub const VND_MS_POWERPOINT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-powerpoint",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_POWERPOINT_ADDIN_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-powerpoint.addin.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-powerpoint.addin.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_POWERPOINT_ADDIN_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-powerpoint.addin.macroEnabled.12",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_POWERPOINT_PRESENTATION_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-powerpoint.presentation.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-powerpoint.presentation.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_POWERPOINT_PRESENTATION_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-powerpoint.presentation.macroEnabled.12",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_POWERPOINT_SLIDE_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-powerpoint.slide.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-powerpoint.slide.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_POWERPOINT_SLIDE_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-powerpoint.slide.macroEnabled.12",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_POWERPOINT_SLIDESHOW_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-powerpoint.slideshow.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-powerpoint.slideshow.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_POWERPOINT_SLIDESHOW_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-powerpoint.slideshow.macroEnabled.12",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_POWERPOINT_TEMPLATE_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-powerpoint.template.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-powerpoint.template.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_POWERPOINT_TEMPLATE_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-powerpoint.template.macroEnabled.12",
    };
    #[doc = "\\[Justin_Hutchings\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_PRINTDEVICECAPABILITIES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-PrintDeviceCapabilities+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-PrintDeviceCapabilities+xml\");"]
    #[doc = r" ```"]
    pub const VND_MS_PRINTDEVICECAPABILITIES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-PrintDeviceCapabilities+xml",
    };
    #[doc = "\\[Justin_Hutchings\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_PRINTSCHEMATICKET_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-PrintSchemaTicket+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-PrintSchemaTicket+xml\");"]
    #[doc = r" ```"]
    pub const VND_MS_PRINTSCHEMATICKET_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-PrintSchemaTicket+xml",
    };
    #[doc = "\\[Sukvinder_S._Gill\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_PROJECT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-project\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-project\");"]
    #[doc = r" ```"]
    pub const VND_MS_PROJECT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-project",
    };
    #[doc = "\\[Sukvinder_S._Gill\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_TNEF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-tnef\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-tnef\");"]
    #[doc = r" ```"]
    pub const VND_MS_TNEF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-tnef",
    };
    #[doc = "\\[Justin_Hutchings\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WINDOWS_DEVICEPAIRING;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-windows.devicepairing\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-windows.devicepairing\");"]
    #[doc = r" ```"]
    pub const VND_MS_WINDOWS_DEVICEPAIRING: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-windows.devicepairing",
    };
    #[doc = "\\[Justin_Hutchings\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WINDOWS_NWPRINTING_OOB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-windows.nwprinting.oob\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-windows.nwprinting.oob\");"]
    #[doc = r" ```"]
    pub const VND_MS_WINDOWS_NWPRINTING_OOB: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-windows.nwprinting.oob",
    };
    #[doc = "\\[Justin_Hutchings\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WINDOWS_PRINTERPAIRING;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-windows.printerpairing\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-windows.printerpairing\");"]
    #[doc = r" ```"]
    pub const VND_MS_WINDOWS_PRINTERPAIRING: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-windows.printerpairing",
    };
    #[doc = "\\[Justin_Hutchings\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WINDOWS_WSD_OOB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-windows.wsd.oob\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-windows.wsd.oob\");"]
    #[doc = r" ```"]
    pub const VND_MS_WINDOWS_WSD_OOB: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-windows.wsd.oob",
    };
    #[doc = "\\[Kevin_Lau\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WMDRM_LIC_CHLG_REQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-wmdrm.lic-chlg-req\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-wmdrm.lic-chlg-req\");"]
    #[doc = r" ```"]
    pub const VND_MS_WMDRM_LIC_CHLG_REQ: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-wmdrm.lic-chlg-req",
    };
    #[doc = "\\[Kevin_Lau\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WMDRM_LIC_RESP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-wmdrm.lic-resp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-wmdrm.lic-resp\");"]
    #[doc = r" ```"]
    pub const VND_MS_WMDRM_LIC_RESP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-wmdrm.lic-resp",
    };
    #[doc = "\\[Kevin_Lau\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WMDRM_METER_CHLG_REQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-wmdrm.meter-chlg-req\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-wmdrm.meter-chlg-req\");"]
    #[doc = r" ```"]
    pub const VND_MS_WMDRM_METER_CHLG_REQ: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-wmdrm.meter-chlg-req",
    };
    #[doc = "\\[Kevin_Lau\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WMDRM_METER_RESP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-wmdrm.meter-resp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-wmdrm.meter-resp\");"]
    #[doc = r" ```"]
    pub const VND_MS_WMDRM_METER_RESP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-wmdrm.meter-resp",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WORD_DOCUMENT_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-word.document.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-word.document.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_WORD_DOCUMENT_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-word.document.macroEnabled.12",
    };
    #[doc = "\\[Chris_Rae\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WORD_TEMPLATE_MACROENABLED_12;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-word.template.macroEnabled.12\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-word.template.macroEnabled.12\");"]
    #[doc = r" ```"]
    pub const VND_MS_WORD_TEMPLATE_MACROENABLED_12: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-word.template.macroEnabled.12",
    };
    #[doc = "\\[Sukvinder_S._Gill\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WORKS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-works\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-works\");"]
    #[doc = r" ```"]
    pub const VND_MS_WORKS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-works",
    };
    #[doc = "\\[Dan_Plastina\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_WPL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-wpl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-wpl\");"]
    #[doc = r" ```"]
    pub const VND_MS_WPL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-wpl",
    };
    #[doc = "\\[Jesse_McGatha\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MS_XPSDOCUMENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ms-xpsdocument\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ms-xpsdocument\");"]
    #[doc = r" ```"]
    pub const VND_MS_XPSDOCUMENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ms-xpsdocument",
    };
    #[doc = "\\[Thomas_Huth\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MSA_DISK_IMAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.msa-disk-image\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.msa-disk-image\");"]
    #[doc = r" ```"]
    pub const VND_MSA_DISK_IMAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.msa-disk-image",
    };
    #[doc = "\\[Gwenael_Le_Bodic\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MSEQ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mseq\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mseq\");"]
    #[doc = r" ```"]
    pub const VND_MSEQ: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mseq",
    };
    #[doc = "\\[Alexander_Ivanov\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MSGPACK;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.msgpack\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.msgpack\");"]
    #[doc = r" ```"]
    pub const VND_MSGPACK: Mime = Mime {
        ttype: "application",
        subtype: "vnd.msgpack",
    };
    #[doc = "\\[Malte_Borcherding\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MSIGN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.msign\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.msign\");"]
    #[doc = r" ```"]
    pub const VND_MSIGN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.msign",
    };
    #[doc = "\\[Steve_Mills\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MULTIAD_CREATOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.multiad.creator\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.multiad.creator\");"]
    #[doc = r" ```"]
    pub const VND_MULTIAD_CREATOR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.multiad.creator",
    };
    #[doc = "\\[Steve_Mills\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MULTIAD_CREATOR_CIF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.multiad.creator.cif\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.multiad.creator.cif\");"]
    #[doc = r" ```"]
    pub const VND_MULTIAD_CREATOR_CIF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.multiad.creator.cif",
    };
    #[doc = "\\[Greg_Adams\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MUSICIAN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.musician\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.musician\");"]
    #[doc = r" ```"]
    pub const VND_MUSICIAN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.musician",
    };
    #[doc = "\\[Tim_Butler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MUSIC_NIFF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.music-niff\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.music-niff\");"]
    #[doc = r" ```"]
    pub const VND_MUSIC_NIFF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.music-niff",
    };
    #[doc = "\\[Chandrashekhara_Anantharamu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MUVEE_STYLE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.muvee.style\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.muvee.style\");"]
    #[doc = r" ```"]
    pub const VND_MUVEE_STYLE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.muvee.style",
    };
    #[doc = "\\[Franck_Lefevre\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_MYNFC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.mynfc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.mynfc\");"]
    #[doc = r" ```"]
    pub const VND_MYNFC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.mynfc",
    };
    #[doc = "\\[Sebastian_A._Weiss\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NACAMAR_YBRID_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nacamar.ybrid+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nacamar.ybrid+json\");"]
    #[doc = r" ```"]
    pub const VND_NACAMAR_YBRID_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nacamar.ybrid+json",
    };
    #[doc = "\\[Aidan_Murdock\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NATO_BINDINGDATAOBJECT_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nato.bindingdataobject+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nato.bindingdataobject+cbor\");"]
    #[doc = r" ```"]
    pub const VND_NATO_BINDINGDATAOBJECT_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nato.bindingdataobject+cbor",
    };
    #[doc = "\\[Aidan_Murdock\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NATO_BINDINGDATAOBJECT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nato.bindingdataobject+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nato.bindingdataobject+json\");"]
    #[doc = r" ```"]
    pub const VND_NATO_BINDINGDATAOBJECT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nato.bindingdataobject+json",
    };
    #[doc = "\\[Aidan_Murdock\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NATO_BINDINGDATAOBJECT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nato.bindingdataobject+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nato.bindingdataobject+xml\");"]
    #[doc = r" ```"]
    pub const VND_NATO_BINDINGDATAOBJECT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nato.bindingdataobject+xml",
    };
    #[doc = "\\[Aidan_Murdock\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NATO_OPENXMLFORMATS_PACKAGE_IEPD_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nato.openxmlformats-package.iepd+zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nato.openxmlformats-package.iepd+zip\");"]
    #[doc = r" ```"]
    pub const VND_NATO_OPENXMLFORMATS_PACKAGE_IEPD_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nato.openxmlformats-package.iepd+zip",
    };
    #[doc = "\\[Lauri_Tarkkala\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NCD_CONTROL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ncd.control\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ncd.control\");"]
    #[doc = r" ```"]
    pub const VND_NCD_CONTROL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ncd.control",
    };
    #[doc = "\\[Lauri_Tarkkala\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NCD_REFERENCE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ncd.reference\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ncd.reference\");"]
    #[doc = r" ```"]
    pub const VND_NCD_REFERENCE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ncd.reference",
    };
    #[doc = "\\[Thomas_Schoffelen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NEARST_INV_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nearst.inv+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nearst.inv+json\");"]
    #[doc = r" ```"]
    pub const VND_NEARST_INV_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nearst.inv+json",
    };
    #[doc = "\\[Andreas_Molzer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NEBUMIND_LINE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nebumind.line\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nebumind.line\");"]
    #[doc = r" ```"]
    pub const VND_NEBUMIND_LINE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nebumind.line",
    };
    #[doc = "\\[Steve_Judkins\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NERVANA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nervana\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nervana\");"]
    #[doc = r" ```"]
    pub const VND_NERVANA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nervana",
    };
    #[doc = "\\[Andy_Mutz\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NETFPX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.netfpx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.netfpx\");"]
    #[doc = r" ```"]
    pub const VND_NETFPX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.netfpx",
    };
    #[doc = "\\[Dan_DuFeu\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NEUROLANGUAGE_NLU;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.neurolanguage.nlu\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.neurolanguage.nlu\");"]
    #[doc = r" ```"]
    pub const VND_NEUROLANGUAGE_NLU: Mime = Mime {
        ttype: "application",
        subtype: "vnd.neurolanguage.nlu",
    };
    #[doc = "\\[Amit_Kumar_Gupta\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NIMN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nimn\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nimn\");"]
    #[doc = r" ```"]
    pub const VND_NIMN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nimn",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NINTENDO_SNES_ROM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nintendo.snes.rom\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nintendo.snes.rom\");"]
    #[doc = r" ```"]
    pub const VND_NINTENDO_SNES_ROM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nintendo.snes.rom",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NINTENDO_NITRO_ROM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nintendo.nitro.rom\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nintendo.nitro.rom\");"]
    #[doc = r" ```"]
    pub const VND_NINTENDO_NITRO_ROM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nintendo.nitro.rom",
    };
    #[doc = "\\[Steve_Rogan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NITF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nitf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nitf\");"]
    #[doc = r" ```"]
    pub const VND_NITF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nitf",
    };
    #[doc = "\\[Monty_Solomon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOBLENET_DIRECTORY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.noblenet-directory\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.noblenet-directory\");"]
    #[doc = r" ```"]
    pub const VND_NOBLENET_DIRECTORY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.noblenet-directory",
    };
    #[doc = "\\[Monty_Solomon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOBLENET_SEALER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.noblenet-sealer\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.noblenet-sealer\");"]
    #[doc = r" ```"]
    pub const VND_NOBLENET_SEALER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.noblenet-sealer",
    };
    #[doc = "\\[Monty_Solomon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOBLENET_WEB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.noblenet-web\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.noblenet-web\");"]
    #[doc = r" ```"]
    pub const VND_NOBLENET_WEB: Mime = Mime {
        ttype: "application",
        subtype: "vnd.noblenet-web",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_CATALOGS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.catalogs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.catalogs\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_CATALOGS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.catalogs",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_CONML_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.conml+wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.conml+wbxml\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_CONML_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.conml+wbxml",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_CONML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.conml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.conml+xml\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_CONML_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.conml+xml",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_IPTV_CONFIG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.iptv.config+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.iptv.config+xml\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_IPTV_CONFIG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.iptv.config+xml",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_ISDS_RADIO_PRESETS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.iSDS-radio-presets\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.iSDS-radio-presets\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_ISDS_RADIO_PRESETS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.iSDS-radio-presets",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_LANDMARK_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.landmark+wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.landmark+wbxml\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_LANDMARK_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.landmark+wbxml",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_LANDMARK_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.landmark+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.landmark+xml\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_LANDMARK_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.landmark+xml",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_LANDMARKCOLLECTION_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.landmarkcollection+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.landmarkcollection+xml\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_LANDMARKCOLLECTION_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.landmarkcollection+xml",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_NCD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.ncd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.ncd\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_NCD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.ncd",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_N_GAGE_AC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.n-gage.ac+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.n-gage.ac+xml\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_N_GAGE_AC_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.n-gage.ac+xml",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_N_GAGE_DATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.n-gage.data\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.n-gage.data\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_N_GAGE_DATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.n-gage.data",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_N_GAGE_SYMBIAN_INSTALL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.n-gage.symbian.install\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.n-gage.symbian.install\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_N_GAGE_SYMBIAN_INSTALL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.n-gage.symbian.install",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_PCD_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.pcd+wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.pcd+wbxml\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_PCD_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.pcd+wbxml",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_PCD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.pcd+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.pcd+xml\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_PCD_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.pcd+xml",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_RADIO_PRESET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.radio-preset\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.radio-preset\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_RADIO_PRESET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.radio-preset",
    };
    #[doc = "\\[Nokia\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOKIA_RADIO_PRESETS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.nokia.radio-presets\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.nokia.radio-presets\");"]
    #[doc = r" ```"]
    pub const VND_NOKIA_RADIO_PRESETS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.nokia.radio-presets",
    };
    #[doc = "\\[Janine_Swenson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOVADIGM_EDM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.novadigm.EDM\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.novadigm.EDM\");"]
    #[doc = r" ```"]
    pub const VND_NOVADIGM_EDM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.novadigm.EDM",
    };
    #[doc = "\\[Janine_Swenson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOVADIGM_EDX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.novadigm.EDX\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.novadigm.EDX\");"]
    #[doc = r" ```"]
    pub const VND_NOVADIGM_EDX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.novadigm.EDX",
    };
    #[doc = "\\[Janine_Swenson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NOVADIGM_EXT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.novadigm.EXT\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.novadigm.EXT\");"]
    #[doc = r" ```"]
    pub const VND_NOVADIGM_EXT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.novadigm.EXT",
    };
    #[doc = "\\[Akinori_Taya\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NTT_LOCAL_CONTENT_SHARE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ntt-local.content-share\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ntt-local.content-share\");"]
    #[doc = r" ```"]
    pub const VND_NTT_LOCAL_CONTENT_SHARE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ntt-local.content-share",
    };
    #[doc = "\\[NTT-local\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NTT_LOCAL_FILE_TRANSFER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ntt-local.file-transfer\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ntt-local.file-transfer\");"]
    #[doc = r" ```"]
    pub const VND_NTT_LOCAL_FILE_TRANSFER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ntt-local.file-transfer",
    };
    #[doc = "\\[NTT-local\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NTT_LOCAL_OGW_REMOTE_ACCESS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ntt-local.ogw_remote-access\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ntt-local.ogw_remote-access\");"]
    #[doc = r" ```"]
    pub const VND_NTT_LOCAL_OGW_REMOTE_ACCESS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ntt-local.ogw_remote-access",
    };
    #[doc = "\\[NTT-local\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NTT_LOCAL_SIP_TA_REMOTE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ntt-local.sip-ta_remote\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ntt-local.sip-ta_remote\");"]
    #[doc = r" ```"]
    pub const VND_NTT_LOCAL_SIP_TA_REMOTE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ntt-local.sip-ta_remote",
    };
    #[doc = "\\[NTT-local\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_NTT_LOCAL_SIP_TA_TCP_STREAM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ntt-local.sip-ta_tcp_stream\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ntt-local.sip-ta_tcp_stream\");"]
    #[doc = r" ```"]
    pub const VND_NTT_LOCAL_SIP_TA_TCP_STREAM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ntt-local.sip-ta_tcp_stream",
    };
    #[doc = "\\[Frank_Kilcommins\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OAI_WORKFLOWS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oai.workflows\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oai.workflows\");"]
    #[doc = r" ```"]
    pub const VND_OAI_WORKFLOWS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oai.workflows",
    };
    #[doc = "\\[Frank_Kilcommins\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OAI_WORKFLOWS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oai.workflows+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oai.workflows+json\");"]
    #[doc = r" ```"]
    pub const VND_OAI_WORKFLOWS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oai.workflows+json",
    };
    #[doc = "\\[Frank_Kilcommins\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OAI_WORKFLOWS_YAML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oai.workflows+yaml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oai.workflows+yaml\");"]
    #[doc = r" ```"]
    pub const VND_OAI_WORKFLOWS_YAML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oai.workflows+yaml",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_BASE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.base\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.base\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_BASE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.base",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_CHART;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.chart\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.chart\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_CHART: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.chart",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_CHART_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.chart-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.chart-template\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_CHART_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.chart-template",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_DATABASE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.database\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.database\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_DATABASE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.database",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_FORMULA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.formula\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.formula\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_FORMULA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.formula",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_FORMULA_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.formula-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.formula-template\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_FORMULA_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.formula-template",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_GRAPHICS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.graphics\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.graphics\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_GRAPHICS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.graphics",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_GRAPHICS_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.graphics-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.graphics-template\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_GRAPHICS_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.graphics-template",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_IMAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.image\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.image\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_IMAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.image",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_IMAGE_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.image-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.image-template\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_IMAGE_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.image-template",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_PRESENTATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.presentation\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.presentation\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_PRESENTATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.presentation",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.presentation-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.presentation-template\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_PRESENTATION_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.presentation-template",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_SPREADSHEET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.spreadsheet\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.spreadsheet\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_SPREADSHEET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.spreadsheet",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.spreadsheet-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.spreadsheet-template\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_SPREADSHEET_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.spreadsheet-template",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_TEXT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.text\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.text\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_TEXT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.text",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_TEXT_MASTER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.text-master\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.text-master\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_TEXT_MASTER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.text-master",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_TEXT_MASTER_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.text-master-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.text-master-template\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_TEXT_MASTER_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.text-master-template",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_TEXT_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.text-template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.text-template\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_TEXT_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.text-template",
    };
    #[doc = "\\[OASIS_TC_Admin\\]\\[OASIS\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OASIS_OPENDOCUMENT_TEXT_WEB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oasis.opendocument.text-web\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oasis.opendocument.text-web\");"]
    #[doc = r" ```"]
    pub const VND_OASIS_OPENDOCUMENT_TEXT_WEB: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oasis.opendocument.text-web",
    };
    #[doc = "\\[Matthias_Hessling\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OBN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.obn\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.obn\");"]
    #[doc = r" ```"]
    pub const VND_OBN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.obn",
    };
    #[doc = "\\[Michael_Koster\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OCF_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ocf+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ocf+cbor\");"]
    #[doc = r" ```"]
    pub const VND_OCF_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ocf+cbor",
    };
    #[doc = "\\[Steven_Lasker\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OCI_IMAGE_MANIFEST_V1_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oci.image.manifest.v1+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oci.image.manifest.v1+json\");"]
    #[doc = r" ```"]
    pub const VND_OCI_IMAGE_MANIFEST_V1_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oci.image.manifest.v1+json",
    };
    #[doc = "\\[Eli_Grey\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OFTN_L10N_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oftn.l10n+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oftn.l10n+json\");"]
    #[doc = r" ```"]
    pub const VND_OFTN_L10N_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oftn.l10n+json",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_CONTENTACCESSDOWNLOAD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.contentaccessdownload+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.contentaccessdownload+xml\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_CONTENTACCESSDOWNLOAD_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.contentaccessdownload+xml",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_CONTENTACCESSSTREAMING_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.contentaccessstreaming+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.contentaccessstreaming+xml\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_CONTENTACCESSSTREAMING_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.contentaccessstreaming+xml",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_CSPG_HEXBINARY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.cspg-hexbinary\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.cspg-hexbinary\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_CSPG_HEXBINARY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.cspg-hexbinary",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_DAE_SVG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.dae.svg+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.dae.svg+xml\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_DAE_SVG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.dae.svg+xml",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_DAE_XHTML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.dae.xhtml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.dae.xhtml+xml\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_DAE_XHTML_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.dae.xhtml+xml",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_MIPPVCONTROLMESSAGE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.mippvcontrolmessage+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.mippvcontrolmessage+xml\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_MIPPVCONTROLMESSAGE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.mippvcontrolmessage+xml",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_PAE_GEM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.pae.gem\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.pae.gem\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_PAE_GEM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.pae.gem",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_SPDISCOVERY_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.spdiscovery+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.spdiscovery+xml\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_SPDISCOVERY_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.spdiscovery+xml",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_SPDLIST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.spdlist+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.spdlist+xml\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_SPDLIST_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.spdlist+xml",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_UEPROFILE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.ueprofile+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.ueprofile+xml\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_UEPROFILE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.ueprofile+xml",
    };
    #[doc = "\\[Claire_DEsclercs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OIPF_USERPROFILE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oipf.userprofile+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oipf.userprofile+xml\");"]
    #[doc = r" ```"]
    pub const VND_OIPF_USERPROFILE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oipf.userprofile+xml",
    };
    #[doc = "\\[John_Palmieri\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OLPC_SUGAR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.olpc-sugar\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.olpc-sugar\");"]
    #[doc = r" ```"]
    pub const VND_OLPC_SUGAR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.olpc-sugar",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_ASSOCIATED_PROCEDURE_PARAMETER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.associated-procedure-parameter+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.associated-procedure-parameter+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_ASSOCIATED_PROCEDURE_PARAMETER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.associated-procedure-parameter+xml",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_DRM_TRIGGER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.drm-trigger+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.drm-trigger+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_DRM_TRIGGER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.drm-trigger+xml",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_IMD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.imd+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.imd+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_IMD_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.imd+xml",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_LTKM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.ltkm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.ltkm\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_LTKM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.ltkm",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_NOTIFICATION_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.notification+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.notification+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_NOTIFICATION_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.notification+xml",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_PROVISIONINGTRIGGER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.provisioningtrigger\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.provisioningtrigger\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_PROVISIONINGTRIGGER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.provisioningtrigger",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_SGBOOT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.sgboot\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.sgboot\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_SGBOOT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.sgboot",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_SGDD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.sgdd+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.sgdd+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_SGDD_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.sgdd+xml",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_SGDU;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.sgdu\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.sgdu\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_SGDU: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.sgdu",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_SIMPLE_SYMBOL_CONTAINER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.simple-symbol-container\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.simple-symbol-container\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_SIMPLE_SYMBOL_CONTAINER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.simple-symbol-container",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_SMARTCARD_TRIGGER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.smartcard-trigger+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.smartcard-trigger+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_SMARTCARD_TRIGGER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.smartcard-trigger+xml",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_SPROV_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.sprov+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.sprov+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_SPROV_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.sprov+xml",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_BCAST_STKM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.bcast.stkm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.bcast.stkm\");"]
    #[doc = r" ```"]
    pub const VND_OMA_BCAST_STKM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.bcast.stkm",
    };
    #[doc = "\\[Hao_Wang\\]\\[OMA\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_CAB_ADDRESS_BOOK_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.cab-address-book+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.cab-address-book+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_CAB_ADDRESS_BOOK_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.cab-address-book+xml",
    };
    #[doc = "\\[Hao_Wang\\]\\[OMA\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_CAB_FEATURE_HANDLER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.cab-feature-handler+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.cab-feature-handler+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_CAB_FEATURE_HANDLER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.cab-feature-handler+xml",
    };
    #[doc = "\\[Hao_Wang\\]\\[OMA\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_CAB_PCC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.cab-pcc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.cab-pcc+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_CAB_PCC_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.cab-pcc+xml",
    };
    #[doc = "\\[Hao_Wang\\]\\[OMA\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_CAB_SUBS_INVITE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.cab-subs-invite+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.cab-subs-invite+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_CAB_SUBS_INVITE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.cab-subs-invite+xml",
    };
    #[doc = "\\[Hao_Wang\\]\\[OMA\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_CAB_USER_PREFS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.cab-user-prefs+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.cab-user-prefs+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_CAB_USER_PREFS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.cab-user-prefs+xml",
    };
    #[doc = "\\[Avi_Primo\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_DCD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.dcd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.dcd\");"]
    #[doc = r" ```"]
    pub const VND_OMA_DCD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.dcd",
    };
    #[doc = "\\[Avi_Primo\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_DCDC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.dcdc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.dcdc\");"]
    #[doc = r" ```"]
    pub const VND_OMA_DCDC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.dcdc",
    };
    #[doc = "\\[Jun_Sato\\]\\[Open_Mobile_Alliance_BAC_DLDRM_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_DD2_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.dd2+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.dd2+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_DD2_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.dd2+xml",
    };
    #[doc = "\\[Uwe_Rauschenbach\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_DRM_RISD_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.drm.risd+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.drm.risd+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_DRM_RISD_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.drm.risd+xml",
    };
    #[doc = "\\[Sean_Kelley\\]\\[OMA_Presence_and_Availability_PAG_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_GROUP_USAGE_LIST_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.group-usage-list+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.group-usage-list+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_GROUP_USAGE_LIST_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.group-usage-list+xml",
    };
    #[doc = "\\[Open_Mobile_Naming_Authority\\]\\[John_Mudge\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_LWM2M_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.lwm2m+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.lwm2m+cbor\");"]
    #[doc = r" ```"]
    pub const VND_OMA_LWM2M_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.lwm2m+cbor",
    };
    #[doc = "\\[Open_Mobile_Naming_Authority\\]\\[John_Mudge\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_LWM2M_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.lwm2m+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.lwm2m+json\");"]
    #[doc = r" ```"]
    pub const VND_OMA_LWM2M_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.lwm2m+json",
    };
    #[doc = "\\[Open_Mobile_Naming_Authority\\]\\[John_Mudge\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_LWM2M_TLV;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.lwm2m+tlv\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.lwm2m+tlv\");"]
    #[doc = r" ```"]
    pub const VND_OMA_LWM2M_TLV: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.lwm2m+tlv",
    };
    #[doc = "\\[Brian_McColgan\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_PAL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.pal+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.pal+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_PAL_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.pal+xml",
    };
    #[doc = "\\[OMA_Push_to_Talk_over_Cellular_POC_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_POC_DETAILED_PROGRESS_REPORT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.poc.detailed-progress-report+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.poc.detailed-progress-report+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_POC_DETAILED_PROGRESS_REPORT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.poc.detailed-progress-report+xml",
    };
    #[doc = "\\[OMA_Push_to_Talk_over_Cellular_POC_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_POC_FINAL_REPORT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.poc.final-report+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.poc.final-report+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_POC_FINAL_REPORT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.poc.final-report+xml",
    };
    #[doc = "\\[Sean_Kelley\\]\\[OMA_Push_to_Talk_over_Cellular_POC_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_POC_GROUPS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.poc.groups+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.poc.groups+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_POC_GROUPS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.poc.groups+xml",
    };
    #[doc = "\\[OMA_Push_to_Talk_over_Cellular_POC_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_POC_INVOCATION_DESCRIPTOR_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.poc.invocation-descriptor+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.poc.invocation-descriptor+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_POC_INVOCATION_DESCRIPTOR_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.poc.invocation-descriptor+xml",
    };
    #[doc = "\\[OMA_Push_to_Talk_over_Cellular_POC_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_POC_OPTIMIZED_PROGRESS_REPORT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.poc.optimized-progress-report+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.poc.optimized-progress-report+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_POC_OPTIMIZED_PROGRESS_REPORT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.poc.optimized-progress-report+xml",
    };
    #[doc = "\\[Bryan_Sullivan\\]\\[OMA\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_PUSH;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.push\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.push\");"]
    #[doc = r" ```"]
    pub const VND_OMA_PUSH: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.push",
    };
    #[doc = "\\[Wenjun_Zeng\\]\\[Open_Mobile_Naming_Authority\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_SCIDM_MESSAGES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.scidm.messages+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.scidm.messages+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_SCIDM_MESSAGES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.scidm.messages+xml",
    };
    #[doc = "\\[Sean_Kelley\\]\\[OMA_Presence_and_Availability_PAG_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_XCAP_DIRECTORY_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma.xcap-directory+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma.xcap-directory+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMA_XCAP_DIRECTORY_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma.xcap-directory+xml",
    };
    #[doc = "\\[OMA_Data_Synchronization_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMADS_EMAIL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.omads-email+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.omads-email+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMADS_EMAIL_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.omads-email+xml",
    };
    #[doc = "\\[OMA_Data_Synchronization_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMADS_FILE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.omads-file+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.omads-file+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMADS_FILE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.omads-file+xml",
    };
    #[doc = "\\[OMA_Data_Synchronization_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMADS_FOLDER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.omads-folder+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.omads-folder+xml\");"]
    #[doc = r" ```"]
    pub const VND_OMADS_FOLDER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.omads-folder+xml",
    };
    #[doc = "\\[Julien_Grange\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMALOC_SUPL_INIT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.omaloc-supl-init\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.omaloc-supl-init\");"]
    #[doc = r" ```"]
    pub const VND_OMALOC_SUPL_INIT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.omaloc-supl-init",
    };
    #[doc = "\\[Ilan_Mahalal\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_SCWS_CONFIG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma-scws-config\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma-scws-config\");"]
    #[doc = r" ```"]
    pub const VND_OMA_SCWS_CONFIG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma-scws-config",
    };
    #[doc = "\\[Ilan_Mahalal\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_SCWS_HTTP_REQUEST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma-scws-http-request\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma-scws-http-request\");"]
    #[doc = r" ```"]
    pub const VND_OMA_SCWS_HTTP_REQUEST: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma-scws-http-request",
    };
    #[doc = "\\[Ilan_Mahalal\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OMA_SCWS_HTTP_RESPONSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oma-scws-http-response\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oma-scws-http-response\");"]
    #[doc = r" ```"]
    pub const VND_OMA_SCWS_HTTP_RESPONSE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oma-scws-http-response",
    };
    #[doc = "\\[Nathan_Black\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ONEPAGER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.onepager\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.onepager\");"]
    #[doc = r" ```"]
    pub const VND_ONEPAGER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.onepager",
    };
    #[doc = "\\[Nathan_Black\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ONEPAGERTAMP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.onepagertamp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.onepagertamp\");"]
    #[doc = r" ```"]
    pub const VND_ONEPAGERTAMP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.onepagertamp",
    };
    #[doc = "\\[Nathan_Black\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ONEPAGERTAMX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.onepagertamx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.onepagertamx\");"]
    #[doc = r" ```"]
    pub const VND_ONEPAGERTAMX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.onepagertamx",
    };
    #[doc = "\\[Nathan_Black\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ONEPAGERTAT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.onepagertat\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.onepagertat\");"]
    #[doc = r" ```"]
    pub const VND_ONEPAGERTAT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.onepagertat",
    };
    #[doc = "\\[Nathan_Black\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ONEPAGERTATP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.onepagertatp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.onepagertatp\");"]
    #[doc = r" ```"]
    pub const VND_ONEPAGERTATP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.onepagertatp",
    };
    #[doc = "\\[Nathan_Black\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ONEPAGERTATX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.onepagertatx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.onepagertatx\");"]
    #[doc = r" ```"]
    pub const VND_ONEPAGERTATX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.onepagertatx",
    };
    #[doc = "\\[Hans_Busch\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ONVIF_METADATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.onvif.metadata\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.onvif.metadata\");"]
    #[doc = r" ```"]
    pub const VND_ONVIF_METADATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.onvif.metadata",
    };
    #[doc = "\\[Mark_Otaris\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENBLOX_GAME_BINARY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openblox.game-binary\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openblox.game-binary\");"]
    #[doc = r" ```"]
    pub const VND_OPENBLOX_GAME_BINARY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openblox.game-binary",
    };
    #[doc = "\\[Mark_Otaris\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENBLOX_GAME_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openblox.game+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openblox.game+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENBLOX_GAME_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openblox.game+xml",
    };
    #[doc = "\\[Craig_Bruce\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENEYE_OEB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openeye.oeb\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openeye.oeb\");"]
    #[doc = r" ```"]
    pub const VND_OPENEYE_OEB: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openeye.oeb",
    };
    #[doc = "\\[Paul_Norman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENSTREETMAP_DATA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openstreetmap.data+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openstreetmap.data+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENSTREETMAP_DATA_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openstreetmap.data+xml",
    };
    #[doc = "\\[Peter_Todd\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENTIMESTAMPS_OTS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.opentimestamps.ots\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.opentimestamps.ots\");"]
    #[doc = r" ```"]
    pub const VND_OPENTIMESTAMPS_OTS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.opentimestamps.ots",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_CUSTOM_PROPERTIES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.custom-properties+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.custom-properties+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_CUSTOM_PROPERTIES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.custom-properties+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_CUSTOMXMLPROPERTIES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.customXmlProperties+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.customXmlProperties+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_CUSTOMXMLPROPERTIES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.customXmlProperties+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWING_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.drawing+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.drawing+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWING_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.drawing+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_CHART_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.drawingml.chart+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.drawingml.chart+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_CHART_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.drawingml.chart+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_CHARTSHAPES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.drawingml.chartshapes+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.drawingml.chartshapes+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_CHARTSHAPES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.drawingml.chartshapes+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMCOLORS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.drawingml.diagramColors+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.drawingml.diagramColors+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMCOLORS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.drawingml.diagramColors+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMDATA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.drawingml.diagramData+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.drawingml.diagramData+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMDATA_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.drawingml.diagramData+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMLAYOUT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMLAYOUT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.drawingml.diagramLayout+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMSTYLE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_DRAWINGML_DIAGRAMSTYLE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.drawingml.diagramStyle+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_EXTENDED_PROPERTIES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.extended-properties+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.extended-properties+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_EXTENDED_PROPERTIES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.extended-properties+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_COMMENTAUTHORS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_COMMENTAUTHORS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.commentAuthors+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_COMMENTS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.comments+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.comments+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_COMMENTS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.comments+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_HANDOUTMASTER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_HANDOUTMASTER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.handoutMaster+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_NOTESMASTER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.notesMaster+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.notesMaster+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_NOTESMASTER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.notesMaster+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_NOTESSLIDE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.notesSlide+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.notesSlide+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_NOTESSLIDE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.notesSlide+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.presentation\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.presentation\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.presentation",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION_MAIN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.presentation.main+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.presentation.main+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESENTATION_MAIN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.presentation.main+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESPROPS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.presProps+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.presProps+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_PRESPROPS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.presProps+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.slide\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.slide\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.slide",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.slide+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.slide+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.slide+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDELAYOUT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.slideLayout+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.slideLayout+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDELAYOUT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.slideLayout+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDEMASTER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.slideMaster+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.slideMaster+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDEMASTER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.slideMaster+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDESHOW;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.slideshow\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.slideshow\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDESHOW: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.slideshow",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDESHOW_MAIN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDESHOW_MAIN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.slideshow.main+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDEUPDATEINFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_SLIDEUPDATEINFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.slideUpdateInfo+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TABLESTYLES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.tableStyles+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.tableStyles+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TABLESTYLES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.tableStyles+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TAGS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.tags+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.tags+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TAGS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.tags+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.template\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.template",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TEMPLATE_MAIN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.template.main+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.template.main+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_TEMPLATE_MAIN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.template.main+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_VIEWPROPS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.presentationml.viewProps+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.presentationml.viewProps+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_PRESENTATIONML_VIEWPROPS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.presentationml.viewProps+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CALCCHAIN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CALCCHAIN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.calcChain+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CHARTSHEET_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CHARTSHEET_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.chartsheet+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_COMMENTS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.comments+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.comments+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_COMMENTS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.comments+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CONNECTIONS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.connections+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.connections+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_CONNECTIONS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.connections+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_DIALOGSHEET_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_DIALOGSHEET_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.dialogsheet+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_EXTERNALLINK_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_EXTERNALLINK_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.externalLink+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOTCACHEDEFINITION_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOTCACHEDEFINITION_XML: Mime =
        Mime {
            ttype: "application",
            subtype: "vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheDefinition+xml",
        };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOTCACHERECORDS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOTCACHERECORDS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.pivotCacheRecords+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOTTABLE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_PIVOTTABLE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.pivotTable+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_QUERYTABLE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_QUERYTABLE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.queryTable+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_REVISIONHEADERS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_REVISIONHEADERS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.revisionHeaders+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_REVISIONLOG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_REVISIONLOG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.revisionLog+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHAREDSTRINGS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHAREDSTRINGS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.sharedStrings+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.sheet\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.sheet",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEET_MAIN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEET_MAIN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.sheet.main+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEETMETADATA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_SHEETMETADATA_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.sheetMetadata+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_STYLES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.styles+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.styles+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_STYLES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.styles+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TABLE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.table+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.table+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TABLE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.table+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TABLESINGLECELLS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TABLESINGLECELLS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.tableSingleCells+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.template\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.template",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TEMPLATE_MAIN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_TEMPLATE_MAIN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.template.main+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_USERNAMES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_USERNAMES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.userNames+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_VOLATILEDEPENDENCIES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_VOLATILEDEPENDENCIES_XML: Mime =
        Mime {
            ttype: "application",
            subtype: "vnd.openxmlformats-officedocument.spreadsheetml.volatileDependencies+xml",
        };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_WORKSHEET_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_SPREADSHEETML_WORKSHEET_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.spreadsheetml.worksheet+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_THEME_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.theme+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.theme+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_THEME_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.theme+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_THEMEOVERRIDE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.themeOverride+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.themeOverride+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_THEMEOVERRIDE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.themeOverride+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_VMLDRAWING;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.vmlDrawing\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.vmlDrawing\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_VMLDRAWING: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.vmlDrawing",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_COMMENTS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.comments+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.comments+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_COMMENTS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.comments+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.document\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.document\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.document",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT_GLOSSARY_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT_GLOSSARY_XML: Mime =
        Mime {
            ttype: "application",
            subtype: "vnd.openxmlformats-officedocument.wordprocessingml.document.glossary+xml",
        };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT_MAIN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_DOCUMENT_MAIN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.document.main+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_ENDNOTES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_ENDNOTES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.endnotes+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FONTTABLE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FONTTABLE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.fontTable+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FOOTER_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.footer+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.footer+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FOOTER_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.footer+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FOOTNOTES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_FOOTNOTES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.footnotes+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_NUMBERING_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_NUMBERING_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.numbering+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_SETTINGS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.settings+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.settings+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_SETTINGS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.settings+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_STYLES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.styles+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.styles+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_STYLES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.styles+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_TEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.template\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.template\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_TEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.template",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_TEMPLATE_MAIN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_TEMPLATE_MAIN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.template.main+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_WEBSETTINGS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_OFFICEDOCUMENT_WORDPROCESSINGML_WEBSETTINGS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-officedocument.wordprocessingml.webSettings+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_PACKAGE_CORE_PROPERTIES_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-package.core-properties+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-package.core-properties+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_PACKAGE_CORE_PROPERTIES_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-package.core-properties+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_PACKAGE_DIGITAL_SIGNATURE_XMLSIGNATURE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-package.digital-signature-xmlsignature+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-package.digital-signature-xmlsignature+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_PACKAGE_DIGITAL_SIGNATURE_XMLSIGNATURE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-package.digital-signature-xmlsignature+xml",
    };
    #[doc = "\\[Makoto_Murata\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OPENXMLFORMATS_PACKAGE_RELATIONSHIPS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.openxmlformats-package.relationships+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.openxmlformats-package.relationships+xml\");"]
    #[doc = r" ```"]
    pub const VND_OPENXMLFORMATS_PACKAGE_RELATIONSHIPS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.openxmlformats-package.relationships+xml",
    };
    #[doc = "\\[Ning_Dong\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ORACLE_RESOURCE_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oracle.resource+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oracle.resource+json\");"]
    #[doc = r" ```"]
    pub const VND_ORACLE_RESOURCE_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oracle.resource+json",
    };
    #[doc = "\\[CHATRAS_Bruno\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ORANGE_INDATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.orange.indata\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.orange.indata\");"]
    #[doc = r" ```"]
    pub const VND_ORANGE_INDATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.orange.indata",
    };
    #[doc = "\\[Steven_Klos\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OSA_NETDEPLOY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.osa.netdeploy\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.osa.netdeploy\");"]
    #[doc = r" ```"]
    pub const VND_OSA_NETDEPLOY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.osa.netdeploy",
    };
    #[doc = "\\[Jason_Birch\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OSGEO_MAPGUIDE_PACKAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.osgeo.mapguide.package\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.osgeo.mapguide.package\");"]
    #[doc = r" ```"]
    pub const VND_OSGEO_MAPGUIDE_PACKAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.osgeo.mapguide.package",
    };
    #[doc = "\\[Peter_Kriens\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OSGI_BUNDLE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.osgi.bundle\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.osgi.bundle\");"]
    #[doc = r" ```"]
    pub const VND_OSGI_BUNDLE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.osgi.bundle",
    };
    #[doc = "\\[Peter_Kriens\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OSGI_DP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.osgi.dp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.osgi.dp\");"]
    #[doc = r" ```"]
    pub const VND_OSGI_DP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.osgi.dp",
    };
    #[doc = "\\[Peter_Kriens\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OSGI_SUBSYSTEM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.osgi.subsystem\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.osgi.subsystem\");"]
    #[doc = r" ```"]
    pub const VND_OSGI_SUBSYSTEM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.osgi.subsystem",
    };
    #[doc = "\\[Magnus_Nystrom\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OTPS_CT_KIP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.otps.ct-kip+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.otps.ct-kip+xml\");"]
    #[doc = r" ```"]
    pub const VND_OTPS_CT_KIP_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.otps.ct-kip+xml",
    };
    #[doc = "\\[C._Titus_Brown\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_OXLI_COUNTGRAPH;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.oxli.countgraph\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.oxli.countgraph\");"]
    #[doc = r" ```"]
    pub const VND_OXLI_COUNTGRAPH: Mime = Mime {
        ttype: "application",
        subtype: "vnd.oxli.countgraph",
    };
    #[doc = "\\[Steve_Rice\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PAGERDUTY_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pagerduty+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pagerduty+json\");"]
    #[doc = r" ```"]
    pub const VND_PAGERDUTY_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pagerduty+json",
    };
    #[doc = "\\[Gavin_Peacock\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PALM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.palm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.palm\");"]
    #[doc = r" ```"]
    pub const VND_PALM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.palm",
    };
    #[doc = "\\[Natarajan_Balasundara\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PANOPLY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.panoply\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.panoply\");"]
    #[doc = r" ```"]
    pub const VND_PANOPLY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.panoply",
    };
    #[doc = "\\[John_Kemp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PAOS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.paos.xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.paos.xml\");"]
    #[doc = r" ```"]
    pub const VND_PAOS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.paos.xml",
    };
    #[doc = "\\[Christian_Trosclair\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PATENTDIVE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.patentdive\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.patentdive\");"]
    #[doc = r" ```"]
    pub const VND_PATENTDIVE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.patentdive",
    };
    #[doc = "\\[Andrew_David_Kendall\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PATIENTECOMMSDOC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.patientecommsdoc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.patientecommsdoc\");"]
    #[doc = r" ```"]
    pub const VND_PATIENTECOMMSDOC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.patientecommsdoc",
    };
    #[doc = "\\[Prakash_Baskaran\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PAWAAFILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pawaafile\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pawaafile\");"]
    #[doc = r" ```"]
    pub const VND_PAWAAFILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pawaafile",
    };
    #[doc = "\\[Slawomir_Lisznianski\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PCOS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pcos\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pcos\");"]
    #[doc = r" ```"]
    pub const VND_PCOS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pcos",
    };
    #[doc = "\\[April_Gandert\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PG_FORMAT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pg.format\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pg.format\");"]
    #[doc = r" ```"]
    pub const VND_PG_FORMAT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pg.format",
    };
    #[doc = "\\[April_Gandert\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PG_OSASLI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pg.osasli\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pg.osasli\");"]
    #[doc = r" ```"]
    pub const VND_PG_OSASLI: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pg.osasli",
    };
    #[doc = "\\[Lucas_Maneos\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PIACCESS_APPLICATION_LICENCE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.piaccess.application-licence\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.piaccess.application-licence\");"]
    #[doc = r" ```"]
    pub const VND_PIACCESS_APPLICATION_LICENCE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.piaccess.application-licence",
    };
    #[doc = "\\[Giuseppe_Naccarato\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PICSEL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.picsel\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.picsel\");"]
    #[doc = r" ```"]
    pub const VND_PICSEL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.picsel",
    };
    #[doc = "\\[Rhys_Lewis\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PMI_WIDGET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pmi.widget\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pmi.widget\");"]
    #[doc = r" ```"]
    pub const VND_PMI_WIDGET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pmi.widget",
    };
    #[doc = "\\[Sean_Kelley\\]\\[OMA_Push_to_Talk_over_Cellular_POC_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_POC_GROUP_ADVERTISEMENT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.poc.group-advertisement+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.poc.group-advertisement+xml\");"]
    #[doc = r" ```"]
    pub const VND_POC_GROUP_ADVERTISEMENT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.poc.group-advertisement+xml",
    };
    #[doc = "\\[Jorge_Pando\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_POCKETLEARN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pocketlearn\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pocketlearn\");"]
    #[doc = r" ```"]
    pub const VND_POCKETLEARN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pocketlearn",
    };
    #[doc = "\\[David_Guy\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_POWERBUILDER6;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.powerbuilder6\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.powerbuilder6\");"]
    #[doc = r" ```"]
    pub const VND_POWERBUILDER6: Mime = Mime {
        ttype: "application",
        subtype: "vnd.powerbuilder6",
    };
    #[doc = "\\[David_Guy\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_POWERBUILDER6_S;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.powerbuilder6-s\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.powerbuilder6-s\");"]
    #[doc = r" ```"]
    pub const VND_POWERBUILDER6_S: Mime = Mime {
        ttype: "application",
        subtype: "vnd.powerbuilder6-s",
    };
    #[doc = "\\[Reed_Shilts\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_POWERBUILDER7;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.powerbuilder7\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.powerbuilder7\");"]
    #[doc = r" ```"]
    pub const VND_POWERBUILDER7: Mime = Mime {
        ttype: "application",
        subtype: "vnd.powerbuilder7",
    };
    #[doc = "\\[Reed_Shilts\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_POWERBUILDER75;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.powerbuilder75\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.powerbuilder75\");"]
    #[doc = r" ```"]
    pub const VND_POWERBUILDER75: Mime = Mime {
        ttype: "application",
        subtype: "vnd.powerbuilder75",
    };
    #[doc = "\\[Reed_Shilts\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_POWERBUILDER75_S;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.powerbuilder75-s\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.powerbuilder75-s\");"]
    #[doc = r" ```"]
    pub const VND_POWERBUILDER75_S: Mime = Mime {
        ttype: "application",
        subtype: "vnd.powerbuilder75-s",
    };
    #[doc = "\\[Reed_Shilts\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_POWERBUILDER7_S;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.powerbuilder7-s\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.powerbuilder7-s\");"]
    #[doc = r" ```"]
    pub const VND_POWERBUILDER7_S: Mime = Mime {
        ttype: "application",
        subtype: "vnd.powerbuilder7-s",
    };
    #[doc = "\\[Juoko_Tenhunen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PREMINET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.preminet\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.preminet\");"]
    #[doc = r" ```"]
    pub const VND_PREMINET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.preminet",
    };
    #[doc = "\\[Roman_Smolgovsky\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PREVIEWSYSTEMS_BOX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.previewsystems.box\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.previewsystems.box\");"]
    #[doc = r" ```"]
    pub const VND_PREVIEWSYSTEMS_BOX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.previewsystems.box",
    };
    #[doc = "\\[Pete_Hoch\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PROTEUS_MAGAZINE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.proteus.magazine\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.proteus.magazine\");"]
    #[doc = r" ```"]
    pub const VND_PROTEUS_MAGAZINE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.proteus.magazine",
    };
    #[doc = "\\[Kristopher_Durski\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PSFS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.psfs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.psfs\");"]
    #[doc = r" ```"]
    pub const VND_PSFS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.psfs",
    };
    #[doc = "\\[Igor_Lima_Bolacha_Severino\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PT_MUNDUSMUNDI;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pt.mundusmundi\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pt.mundusmundi\");"]
    #[doc = r" ```"]
    pub const VND_PT_MUNDUSMUNDI: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pt.mundusmundi",
    };
    #[doc = "\\[Oren_Ben-Kiki\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PUBLISHARE_DELTA_TREE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.publishare-delta-tree\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.publishare-delta-tree\");"]
    #[doc = r" ```"]
    pub const VND_PUBLISHARE_DELTA_TREE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.publishare-delta-tree",
    };
    #[doc = "\\[Charles_P._Lamb\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PVI_PTID1;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pvi.ptid1\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pvi.ptid1\");"]
    #[doc = r" ```"]
    pub const VND_PVI_PTID1: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pvi.ptid1",
    };
    #[doc = "\\[RFC3391\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PWG_MULTIPLEXED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pwg-multiplexed\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pwg-multiplexed\");"]
    #[doc = r" ```"]
    pub const VND_PWG_MULTIPLEXED: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pwg-multiplexed",
    };
    #[doc = "\\[Don_Wright\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_PWG_XHTML_PRINT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.pwg-xhtml-print+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.pwg-xhtml-print+xml\");"]
    #[doc = r" ```"]
    pub const VND_PWG_XHTML_PRINT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.pwg-xhtml-print+xml",
    };
    #[doc = "\\[Glenn_Forrester\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_QUALCOMM_BREW_APP_RES;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.qualcomm.brew-app-res\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.qualcomm.brew-app-res\");"]
    #[doc = r" ```"]
    pub const VND_QUALCOMM_BREW_APP_RES: Mime = Mime {
        ttype: "application",
        subtype: "vnd.qualcomm.brew-app-res",
    };
    #[doc = "\\[Casper_Joost_Eyckelhof\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_QUARANTAINENET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.quarantainenet\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.quarantainenet\");"]
    #[doc = r" ```"]
    pub const VND_QUARANTAINENET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.quarantainenet",
    };
    #[doc = "\\[Hannes_Scheidler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_QUARK_QUARKXPRESS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.Quark.QuarkXPress\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.Quark.QuarkXPress\");"]
    #[doc = r" ```"]
    pub const VND_QUARK_QUARKXPRESS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.Quark.QuarkXPress",
    };
    #[doc = "\\[Matthias_Ludwig\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_QUOBJECT_QUOXDOCUMENT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.quobject-quoxdocument\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.quobject-quoxdocument\");"]
    #[doc = r" ```"]
    pub const VND_QUOBJECT_QUOXDOCUMENT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.quobject-quoxdocument",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MOML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.moml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.moml+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MOML_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.moml+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_AUDIT_CONF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-audit-conf+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-audit-conf+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_AUDIT_CONF_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-audit-conf+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_AUDIT_CONN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-audit-conn+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-audit-conn+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_AUDIT_CONN_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-audit-conn+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_AUDIT_DIALOG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-audit-dialog+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-audit-dialog+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_AUDIT_DIALOG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-audit-dialog+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_AUDIT_STREAM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-audit-stream+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-audit-stream+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_AUDIT_STREAM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-audit-stream+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_AUDIT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-audit+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-audit+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_AUDIT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-audit+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_CONF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-conf+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-conf+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_CONF_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-conf+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_DIALOG_BASE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-dialog-base+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-dialog-base+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_DIALOG_BASE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-dialog-base+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_DIALOG_FAX_DETECT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-dialog-fax-detect+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-dialog-fax-detect+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_DIALOG_FAX_DETECT_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-dialog-fax-detect+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_DIALOG_FAX_SENDRECV_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-dialog-fax-sendrecv+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-dialog-fax-sendrecv+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_DIALOG_FAX_SENDRECV_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-dialog-fax-sendrecv+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_DIALOG_GROUP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-dialog-group+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-dialog-group+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_DIALOG_GROUP_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-dialog-group+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_DIALOG_SPEECH_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-dialog-speech+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-dialog-speech+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_DIALOG_SPEECH_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-dialog-speech+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_DIALOG_TRANSFORM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-dialog-transform+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-dialog-transform+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_DIALOG_TRANSFORM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-dialog-transform+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_DIALOG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml-dialog+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml-dialog+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_DIALOG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml-dialog+xml",
    };
    #[doc = "\\[RFC5707\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RADISYS_MSML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.radisys.msml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.radisys.msml+xml\");"]
    #[doc = r" ```"]
    pub const VND_RADISYS_MSML_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.radisys.msml+xml",
    };
    #[doc = "\\[Kevin_Crook\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RAINSTOR_DATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.rainstor.data\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.rainstor.data\");"]
    #[doc = r" ```"]
    pub const VND_RAINSTOR_DATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.rainstor.data",
    };
    #[doc = "\\[Etay_Szekely\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RAPID;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.rapid\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.rapid\");"]
    #[doc = r" ```"]
    pub const VND_RAPID: Mime = Mime {
        ttype: "application",
        subtype: "vnd.rapid",
    };
    #[doc = "\\[Kim_Scarborough\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RAR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.rar\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.rar\");"]
    #[doc = r" ```"]
    pub const VND_RAR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.rar",
    };
    #[doc = "\\[Nick_Reeves\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_REALVNC_BED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.realvnc.bed\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.realvnc.bed\");"]
    #[doc = r" ```"]
    pub const VND_REALVNC_BED: Mime = Mime {
        ttype: "application",
        subtype: "vnd.realvnc.bed",
    };
    #[doc = "\\[W3C_Music_Notation_Community_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RECORDARE_MUSICXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.recordare.musicxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.recordare.musicxml\");"]
    #[doc = r" ```"]
    pub const VND_RECORDARE_MUSICXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.recordare.musicxml",
    };
    #[doc = "\\[W3C_Music_Notation_Community_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RECORDARE_MUSICXML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.recordare.musicxml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.recordare.musicxml+xml\");"]
    #[doc = r" ```"]
    pub const VND_RECORDARE_MUSICXML_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.recordare.musicxml+xml",
    };
    #[doc = "\\[Frantisek_Kucera\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RELPIPE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.relpipe\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.relpipe\");"]
    #[doc = r" ```"]
    pub const VND_RELPIPE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.relpipe",
    };
    #[doc = "\\[James_Wick\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RENLEARN_RLPRINT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.RenLearn.rlprint\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.RenLearn.rlprint\");"]
    #[doc = r" ```"]
    pub const VND_RENLEARN_RLPRINT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.RenLearn.rlprint",
    };
    #[doc = "\\[Benedikt_Muessig\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RESILIENT_LOGIC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.resilient.logic\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.resilient.logic\");"]
    #[doc = r" ```"]
    pub const VND_RESILIENT_LOGIC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.resilient.logic",
    };
    #[doc = "\\[Stephen_Mizell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RESTFUL_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.restful+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.restful+json\");"]
    #[doc = r" ```"]
    pub const VND_RESTFUL_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.restful+json",
    };
    #[doc = "\\[Ken_Jibiki\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RIG_CRYPTONOTE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.rig.cryptonote\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.rig.cryptonote\");"]
    #[doc = r" ```"]
    pub const VND_RIG_CRYPTONOTE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.rig.cryptonote",
    };
    #[doc = "\\[Sybren_Kikstra\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ROUTE66_LINK66_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.route66.link66+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.route66.link66+xml\");"]
    #[doc = r" ```"]
    pub const VND_ROUTE66_LINK66_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.route66.link66+xml",
    };
    #[doc = "\\[Lee_Harding\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RS_274X;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.rs-274x\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.rs-274x\");"]
    #[doc = r" ```"]
    pub const VND_RS_274X: Mime = Mime {
        ttype: "application",
        subtype: "vnd.rs-274x",
    };
    #[doc = "\\[Jerry_Harris\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_RUCKUS_DOWNLOAD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ruckus.download\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ruckus.download\");"]
    #[doc = r" ```"]
    pub const VND_RUCKUS_DOWNLOAD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ruckus.download",
    };
    #[doc = "\\[Lauri_Tarkkala\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_S3SMS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.s3sms\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.s3sms\");"]
    #[doc = r" ```"]
    pub const VND_S3SMS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.s3sms",
    };
    #[doc = "\\[Heikki_Vesalainen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SAILINGTRACKER_TRACK;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sailingtracker.track\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sailingtracker.track\");"]
    #[doc = r" ```"]
    pub const VND_SAILINGTRACKER_TRACK: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sailingtracker.track",
    };
    #[doc = "\\[Markus_Strehle\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SAR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sar\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sar\");"]
    #[doc = r" ```"]
    pub const VND_SAR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sar",
    };
    #[doc = "\\[Shinji_Kusakari\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SBM_CID;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sbm.cid\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sbm.cid\");"]
    #[doc = r" ```"]
    pub const VND_SBM_CID: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sbm.cid",
    };
    #[doc = "\\[Masanori_Murai\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SBM_MID2;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sbm.mid2\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sbm.mid2\");"]
    #[doc = r" ```"]
    pub const VND_SBM_MID2: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sbm.mid2",
    };
    #[doc = "\\[Craig_Bradney\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SCRIBUS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.scribus\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.scribus\");"]
    #[doc = r" ```"]
    pub const VND_SCRIBUS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.scribus",
    };
    #[doc = "\\[John_Kwan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALED_3DF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.3df\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealed.3df\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_3DF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealed.3df",
    };
    #[doc = "\\[John_Kwan\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALED_CSF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.csf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealed.csf\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_CSF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealed.csf",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALED_DOC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.doc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealed.doc\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_DOC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealed.doc",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALED_EML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.eml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealed.eml\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_EML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealed.eml",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALED_MHT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.mht\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealed.mht\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_MHT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealed.mht",
    };
    #[doc = "\\[Martin_Lambert\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALED_NET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.net\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealed.net\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_NET: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealed.net",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALED_PPT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.ppt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealed.ppt\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_PPT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealed.ppt",
    };
    #[doc = "\\[John_Kwan\\]\\[Martin_Lambert\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALED_TIFF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.tiff\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealed.tiff\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_TIFF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealed.tiff",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALED_XLS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealed.xls\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealed.xls\");"]
    #[doc = r" ```"]
    pub const VND_SEALED_XLS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealed.xls",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALEDMEDIA_SOFTSEAL_HTML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealedmedia.softseal.html\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealedmedia.softseal.html\");"]
    #[doc = r" ```"]
    pub const VND_SEALEDMEDIA_SOFTSEAL_HTML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealedmedia.softseal.html",
    };
    #[doc = "\\[David_Petersen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEALEDMEDIA_SOFTSEAL_PDF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sealedmedia.softseal.pdf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sealedmedia.softseal.pdf\");"]
    #[doc = r" ```"]
    pub const VND_SEALEDMEDIA_SOFTSEAL_PDF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sealedmedia.softseal.pdf",
    };
    #[doc = "\\[Steve_Webb\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEEMAIL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.seemail\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.seemail\");"]
    #[doc = r" ```"]
    pub const VND_SEEMAIL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.seemail",
    };
    #[doc = "\\[ICT_Manager\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEIS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.seis+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.seis+json\");"]
    #[doc = r" ```"]
    pub const VND_SEIS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.seis+json",
    };
    #[doc = "\\[Anders_Hansson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEMA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sema\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sema\");"]
    #[doc = r" ```"]
    pub const VND_SEMA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sema",
    };
    #[doc = "\\[Anders_Hansson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEMD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.semd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.semd\");"]
    #[doc = r" ```"]
    pub const VND_SEMD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.semd",
    };
    #[doc = "\\[Anders_Hansson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SEMF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.semf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.semf\");"]
    #[doc = r" ```"]
    pub const VND_SEMF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.semf",
    };
    #[doc = "\\[Connor_Horman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SHADE_SAVE_FILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.shade-save-file\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.shade-save-file\");"]
    #[doc = r" ```"]
    pub const VND_SHADE_SAVE_FILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.shade-save-file",
    };
    #[doc = "\\[Guy_Selzler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SHANA_INFORMED_FORMDATA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.shana.informed.formdata\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.shana.informed.formdata\");"]
    #[doc = r" ```"]
    pub const VND_SHANA_INFORMED_FORMDATA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.shana.informed.formdata",
    };
    #[doc = "\\[Guy_Selzler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SHANA_INFORMED_FORMTEMPLATE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.shana.informed.formtemplate\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.shana.informed.formtemplate\");"]
    #[doc = r" ```"]
    pub const VND_SHANA_INFORMED_FORMTEMPLATE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.shana.informed.formtemplate",
    };
    #[doc = "\\[Guy_Selzler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SHANA_INFORMED_INTERCHANGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.shana.informed.interchange\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.shana.informed.interchange\");"]
    #[doc = r" ```"]
    pub const VND_SHANA_INFORMED_INTERCHANGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.shana.informed.interchange",
    };
    #[doc = "\\[Guy_Selzler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SHANA_INFORMED_PACKAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.shana.informed.package\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.shana.informed.package\");"]
    #[doc = r" ```"]
    pub const VND_SHANA_INFORMED_PACKAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.shana.informed.package",
    };
    #[doc = "\\[Ben_Ramsey\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SHOOTPROOF_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.shootproof+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.shootproof+json\");"]
    #[doc = r" ```"]
    pub const VND_SHOOTPROOF_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.shootproof+json",
    };
    #[doc = "\\[Ronald_Jacobs\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SHOPKICK_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.shopkick+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.shopkick+json\");"]
    #[doc = r" ```"]
    pub const VND_SHOPKICK_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.shopkick+json",
    };
    #[doc = "\\[Mi_Tar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SHP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.shp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.shp\");"]
    #[doc = r" ```"]
    pub const VND_SHP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.shp",
    };
    #[doc = "\\[Mi_Tar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SHX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.shx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.shx\");"]
    #[doc = r" ```"]
    pub const VND_SHX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.shx",
    };
    #[doc = "\\[Uwe_Hermann\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SIGROK_SESSION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sigrok.session\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sigrok.session\");"]
    #[doc = r" ```"]
    pub const VND_SIGROK_SESSION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sigrok.session",
    };
    #[doc = "\\[Patrick_Koh\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SIMTECH_MINDMAPPER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.SimTech-MindMapper\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.SimTech-MindMapper\");"]
    #[doc = r" ```"]
    pub const VND_SIMTECH_MINDMAPPER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.SimTech-MindMapper",
    };
    #[doc = "\\[Kevin_Swiber\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SIREN_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.siren+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.siren+json\");"]
    #[doc = r" ```"]
    pub const VND_SIREN_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.siren+json",
    };
    #[doc = "\\[Hiroaki_Takahashi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SMAF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.smaf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.smaf\");"]
    #[doc = r" ```"]
    pub const VND_SMAF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.smaf",
    };
    #[doc = "\\[Jonathan_Neitz\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SMART_NOTEBOOK;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.smart.notebook\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.smart.notebook\");"]
    #[doc = r" ```"]
    pub const VND_SMART_NOTEBOOK: Mime = Mime {
        ttype: "application",
        subtype: "vnd.smart.notebook",
    };
    #[doc = "\\[Michael_Boyle\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SMART_TEACHER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.smart.teacher\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.smart.teacher\");"]
    #[doc = r" ```"]
    pub const VND_SMART_TEACHER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.smart.teacher",
    };
    #[doc = "\\[Reinhard_Holzner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SMINTIO_PORTALS_ARCHIVE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.smintio.portals.archive\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.smintio.portals.archive\");"]
    #[doc = r" ```"]
    pub const VND_SMINTIO_PORTALS_ARCHIVE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.smintio.portals.archive",
    };
    #[doc = "\\[Connor_Horman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SNESDEV_PAGE_TABLE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.snesdev-page-table\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.snesdev-page-table\");"]
    #[doc = r" ```"]
    pub const VND_SNESDEV_PAGE_TABLE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.snesdev-page-table",
    };
    #[doc = "\\[Jakub_Hytka\\]\\[Martin_Vondrous\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SOFTWARE602_FILLER_FORM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.software602.filler.form+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.software602.filler.form+xml\");"]
    #[doc = r" ```"]
    pub const VND_SOFTWARE602_FILLER_FORM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.software602.filler.form+xml",
    };
    #[doc = "\\[Jakub_Hytka\\]\\[Martin_Vondrous\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SOFTWARE602_FILLER_FORM_XML_ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.software602.filler.form-xml-zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.software602.filler.form-xml-zip\");"]
    #[doc = r" ```"]
    pub const VND_SOFTWARE602_FILLER_FORM_XML_ZIP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.software602.filler.form-xml-zip",
    };
    #[doc = "\\[Cliff_Gauntlett\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SOLENT_SDKM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.solent.sdkm+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.solent.sdkm+xml\");"]
    #[doc = r" ```"]
    pub const VND_SOLENT_SDKM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.solent.sdkm+xml",
    };
    #[doc = "\\[Stefan_Jernberg\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SPOTFIRE_DXP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.spotfire.dxp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.spotfire.dxp\");"]
    #[doc = r" ```"]
    pub const VND_SPOTFIRE_DXP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.spotfire.dxp",
    };
    #[doc = "\\[Stefan_Jernberg\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SPOTFIRE_SFS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.spotfire.sfs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.spotfire.sfs\");"]
    #[doc = r" ```"]
    pub const VND_SPOTFIRE_SFS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.spotfire.sfs",
    };
    #[doc = "\\[Clemens_Ladisch\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SQLITE3;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sqlite3\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sqlite3\");"]
    #[doc = r" ```"]
    pub const VND_SQLITE3: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sqlite3",
    };
    #[doc = "\\[Asang_Dani\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SSS_COD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sss-cod\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sss-cod\");"]
    #[doc = r" ```"]
    pub const VND_SSS_COD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sss-cod",
    };
    #[doc = "\\[Eric_Bruno\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SSS_DTF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sss-dtf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sss-dtf\");"]
    #[doc = r" ```"]
    pub const VND_SSS_DTF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sss-dtf",
    };
    #[doc = "\\[Eric_Bruno\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SSS_NTF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sss-ntf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sss-ntf\");"]
    #[doc = r" ```"]
    pub const VND_SSS_NTF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sss-ntf",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_STEPMANIA_PACKAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.stepmania.package\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.stepmania.package\");"]
    #[doc = r" ```"]
    pub const VND_STEPMANIA_PACKAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.stepmania.package",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_STEPMANIA_STEPCHART;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.stepmania.stepchart\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.stepmania.stepchart\");"]
    #[doc = r" ```"]
    pub const VND_STEPMANIA_STEPCHART: Mime = Mime {
        ttype: "application",
        subtype: "vnd.stepmania.stepchart",
    };
    #[doc = "\\[Glenn_Levitt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_STREET_STREAM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.street-stream\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.street-stream\");"]
    #[doc = r" ```"]
    pub const VND_STREET_STREAM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.street-stream",
    };
    #[doc = "\\[Marc_Hadley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SUN_WADL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sun.wadl+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sun.wadl+xml\");"]
    #[doc = r" ```"]
    pub const VND_SUN_WADL_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sun.wadl+xml",
    };
    #[doc = "\\[Jonathan_Niedfeldt\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SUS_CALENDAR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sus-calendar\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sus-calendar\");"]
    #[doc = r" ```"]
    pub const VND_SUS_CALENDAR: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sus-calendar",
    };
    #[doc = "\\[Scott_Becker\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SVD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.svd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.svd\");"]
    #[doc = r" ```"]
    pub const VND_SVD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.svd",
    };
    #[doc = "\\[Glenn_Widener\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SWIFTVIEW_ICS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.swiftview-ics\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.swiftview-ics\");"]
    #[doc = r" ```"]
    pub const VND_SWIFTVIEW_ICS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.swiftview-ics",
    };
    #[doc = "\\[Finn_Rayk_Gärtner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYBYL_MOL2;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sybyl.mol2\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sybyl.mol2\");"]
    #[doc = r" ```"]
    pub const VND_SYBYL_MOL2: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sybyl.mol2",
    };
    #[doc = "\\[Johann_Terblanche\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYCLE_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.sycle+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.sycle+xml\");"]
    #[doc = r" ```"]
    pub const VND_SYCLE_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.sycle+xml",
    };
    #[doc = "\\[Dan_Luhring\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYFT_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.syft+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.syft+json\");"]
    #[doc = r" ```"]
    pub const VND_SYFT_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.syft+json",
    };
    #[doc = "\\[Peter_Thompson\\]\\[OMA-DM_Work_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYNCML_DM_NOTIFICATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.syncml.dm.notification\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.syncml.dm.notification\");"]
    #[doc = r" ```"]
    pub const VND_SYNCML_DM_NOTIFICATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.syncml.dm.notification",
    };
    #[doc = "\\[OMA-DM_Work_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYNCML_DMDDF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.syncml.dmddf+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.syncml.dmddf+xml\");"]
    #[doc = r" ```"]
    pub const VND_SYNCML_DMDDF_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.syncml.dmddf+xml",
    };
    #[doc = "\\[OMA-DM_Work_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYNCML_DMTNDS_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.syncml.dmtnds+wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.syncml.dmtnds+wbxml\");"]
    #[doc = r" ```"]
    pub const VND_SYNCML_DMTNDS_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.syncml.dmtnds+wbxml",
    };
    #[doc = "\\[OMA-DM_Work_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYNCML_DMTNDS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.syncml.dmtnds+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.syncml.dmtnds+xml\");"]
    #[doc = r" ```"]
    pub const VND_SYNCML_DMTNDS_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.syncml.dmtnds+xml",
    };
    #[doc = "\\[OMA-DM_Work_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYNCML_DMDDF_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.syncml.dmddf+wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.syncml.dmddf+wbxml\");"]
    #[doc = r" ```"]
    pub const VND_SYNCML_DMDDF_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.syncml.dmddf+wbxml",
    };
    #[doc = "\\[OMA-DM_Work_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYNCML_DM_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.syncml.dm+wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.syncml.dm+wbxml\");"]
    #[doc = r" ```"]
    pub const VND_SYNCML_DM_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.syncml.dm+wbxml",
    };
    #[doc = "\\[Bindu_Rama_Rao\\]\\[OMA-DM_Work_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYNCML_DM_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.syncml.dm+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.syncml.dm+xml\");"]
    #[doc = r" ```"]
    pub const VND_SYNCML_DM_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.syncml.dm+xml",
    };
    #[doc = "\\[OMA_Data_Synchronization_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYNCML_DS_NOTIFICATION;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.syncml.ds.notification\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.syncml.ds.notification\");"]
    #[doc = r" ```"]
    pub const VND_SYNCML_DS_NOTIFICATION: Mime = Mime {
        ttype: "application",
        subtype: "vnd.syncml.ds.notification",
    };
    #[doc = "\\[OMA_Data_Synchronization_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_SYNCML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.syncml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.syncml+xml\");"]
    #[doc = r" ```"]
    pub const VND_SYNCML_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.syncml+xml",
    };
    #[doc = "\\[Paul_Walsh\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TABLESCHEMA_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.tableschema+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.tableschema+json\");"]
    #[doc = r" ```"]
    pub const VND_TABLESCHEMA_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.tableschema+json",
    };
    #[doc = "\\[Daniel_Shelton\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TAO_INTENT_MODULE_ARCHIVE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.tao.intent-module-archive\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.tao.intent-module-archive\");"]
    #[doc = r" ```"]
    pub const VND_TAO_INTENT_MODULE_ARCHIVE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.tao.intent-module-archive",
    };
    #[doc = "\\[Guy_Harris\\]\\[Glen_Turner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TCPDUMP_PCAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.tcpdump.pcap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.tcpdump.pcap\");"]
    #[doc = r" ```"]
    pub const VND_TCPDUMP_PCAP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.tcpdump.pcap",
    };
    #[doc = "\\[Arno_Schoedl\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_THINK_CELL_PPTTC_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.think-cell.ppttc+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.think-cell.ppttc+json\");"]
    #[doc = r" ```"]
    pub const VND_THINK_CELL_PPTTC_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.think-cell.ppttc+json",
    };
    #[doc = "\\[Joey_Smith\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.tml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.tml\");"]
    #[doc = r" ```"]
    pub const VND_TML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.tml",
    };
    #[doc = "\\[Alex_Sibilev\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TMD_MEDIAFLEX_API_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.tmd.mediaflex.api+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.tmd.mediaflex.api+xml\");"]
    #[doc = r" ```"]
    pub const VND_TMD_MEDIAFLEX_API_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.tmd.mediaflex.api+xml",
    };
    #[doc = "\\[Nicolas_Helin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TMOBILE_LIVETV;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.tmobile-livetv\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.tmobile-livetv\");"]
    #[doc = r" ```"]
    pub const VND_TMOBILE_LIVETV: Mime = Mime {
        ttype: "application",
        subtype: "vnd.tmobile-livetv",
    };
    #[doc = "\\[Rick_Rupp\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TRI_ONESOURCE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.tri.onesource\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.tri.onesource\");"]
    #[doc = r" ```"]
    pub const VND_TRI_ONESOURCE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.tri.onesource",
    };
    #[doc = "\\[Frank_Cusack\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TRID_TPT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.trid.tpt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.trid.tpt\");"]
    #[doc = r" ```"]
    pub const VND_TRID_TPT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.trid.tpt",
    };
    #[doc = "\\[Steven_Simonoff\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TRISCAPE_MXS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.triscape.mxs\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.triscape.mxs\");"]
    #[doc = r" ```"]
    pub const VND_TRISCAPE_MXS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.triscape.mxs",
    };
    #[doc = "\\[J._Scott_Hepler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TRUEAPP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.trueapp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.trueapp\");"]
    #[doc = r" ```"]
    pub const VND_TRUEAPP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.trueapp",
    };
    #[doc = "\\[Brad_Chase\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_TRUEDOC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.truedoc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.truedoc\");"]
    #[doc = r" ```"]
    pub const VND_TRUEDOC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.truedoc",
    };
    #[doc = "\\[Martin_Talbot\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UBISOFT_WEBPLAYER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ubisoft.webplayer\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ubisoft.webplayer\");"]
    #[doc = r" ```"]
    pub const VND_UBISOFT_WEBPLAYER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ubisoft.webplayer",
    };
    #[doc = "\\[Dave_Manning\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UFDL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ufdl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ufdl\");"]
    #[doc = r" ```"]
    pub const VND_UFDL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ufdl",
    };
    #[doc = "\\[David_Sarfatti\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UIC_OSDM_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uic.osdm+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uic.osdm+json\");"]
    #[doc = r" ```"]
    pub const VND_UIC_OSDM_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uic.osdm+json",
    };
    #[doc = "\\[Tim_Ocock\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UIQ_THEME;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uiq.theme\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uiq.theme\");"]
    #[doc = r" ```"]
    pub const VND_UIQ_THEME: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uiq.theme",
    };
    #[doc = "\\[Jamie_Riden\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UMAJIN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.umajin\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.umajin\");"]
    #[doc = r" ```"]
    pub const VND_UMAJIN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.umajin",
    };
    #[doc = "\\[Unity3d\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UNITY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.unity\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.unity\");"]
    #[doc = r" ```"]
    pub const VND_UNITY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.unity",
    };
    #[doc = "\\[Arne_Gerdes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UOML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uoml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uoml+xml\");"]
    #[doc = r" ```"]
    pub const VND_UOML_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uoml+xml",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_ALERT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.alert\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.alert\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_ALERT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.alert",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_ALERT_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.alert-wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.alert-wbxml\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_ALERT_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.alert-wbxml",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_BEARER_CHOICE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.bearer-choice\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.bearer-choice\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_BEARER_CHOICE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.bearer-choice",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_BEARER_CHOICE_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.bearer-choice-wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.bearer-choice-wbxml\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_BEARER_CHOICE_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.bearer-choice-wbxml",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_CACHEOP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.cacheop\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.cacheop\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_CACHEOP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.cacheop",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_CACHEOP_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.cacheop-wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.cacheop-wbxml\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_CACHEOP_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.cacheop-wbxml",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_CHANNEL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.channel\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.channel\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_CHANNEL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.channel",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_CHANNEL_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.channel-wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.channel-wbxml\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_CHANNEL_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.channel-wbxml",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_LIST;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.list\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.list\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_LIST: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.list",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_LISTCMD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.listcmd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.listcmd\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_LISTCMD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.listcmd",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_LISTCMD_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.listcmd-wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.listcmd-wbxml\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_LISTCMD_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.listcmd-wbxml",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_LIST_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.list-wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.list-wbxml\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_LIST_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.list-wbxml",
    };
    #[doc = "\\[Sebastian_Baer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_URI_MAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uri-map\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uri-map\");"]
    #[doc = r" ```"]
    pub const VND_URI_MAP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uri-map",
    };
    #[doc = "\\[Bruce_Martin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_UPLANET_SIGNAL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.uplanet.signal\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.uplanet.signal\");"]
    #[doc = r" ```"]
    pub const VND_UPLANET_SIGNAL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.uplanet.signal",
    };
    #[doc = "\\[Henrik_Andersson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VALVE_SOURCE_MATERIAL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.valve.source.material\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.valve.source.material\");"]
    #[doc = r" ```"]
    pub const VND_VALVE_SOURCE_MATERIAL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.valve.source.material",
    };
    #[doc = "\\[Taisuke_Sugimoto\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VCX;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.vcx\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.vcx\");"]
    #[doc = r" ```"]
    pub const VND_VCX: Mime = Mime {
        ttype: "application",
        subtype: "vnd.vcx",
    };
    #[doc = "\\[Luc_Rogge\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VD_STUDY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.vd-study\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.vd-study\");"]
    #[doc = r" ```"]
    pub const VND_VD_STUDY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.vd-study",
    };
    #[doc = "\\[Lyndsey_Ferguson\\]\\[Biplab_Sarkar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VECTORWORKS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.vectorworks\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.vectorworks\");"]
    #[doc = r" ```"]
    pub const VND_VECTORWORKS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.vectorworks",
    };
    #[doc = "\\[James_Wigger\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VEL_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.vel+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.vel+json\");"]
    #[doc = r" ```"]
    pub const VND_VEL_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.vel+json",
    };
    #[doc = "\\[Petr_Peterka\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VERIMATRIX_VCAS;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.verimatrix.vcas\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.verimatrix.vcas\");"]
    #[doc = r" ```"]
    pub const VND_VERIMATRIX_VCAS: Mime = Mime {
        ttype: "application",
        subtype: "vnd.verimatrix.vcas",
    };
    #[doc = "\\[Al_Brown\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VERITONE_AION_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.veritone.aion+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.veritone.aion+json\");"]
    #[doc = r" ```"]
    pub const VND_VERITONE_AION_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.veritone.aion+json",
    };
    #[doc = "\\[Massimo_Bertoli\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VERYANT_THIN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.veryant.thin\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.veryant.thin\");"]
    #[doc = r" ```"]
    pub const VND_VERYANT_THIN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.veryant.thin",
    };
    #[doc = "\\[Jim_Zubov\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VES_ENCRYPTED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.ves.encrypted\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.ves.encrypted\");"]
    #[doc = r" ```"]
    pub const VND_VES_ENCRYPTED: Mime = Mime {
        ttype: "application",
        subtype: "vnd.ves.encrypted",
    };
    #[doc = "\\[Robert_Hess\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VIDSOFT_VIDCONFERENCE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.vidsoft.vidconference\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.vidsoft.vidconference\");"]
    #[doc = r" ```"]
    pub const VND_VIDSOFT_VIDCONFERENCE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.vidsoft.vidconference",
    };
    #[doc = "\\[Troy_Sandal\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VISIO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.visio\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.visio\");"]
    #[doc = r" ```"]
    pub const VND_VISIO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.visio",
    };
    #[doc = "\\[Gayatri_Aravindakumar\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VISIONARY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.visionary\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.visionary\");"]
    #[doc = r" ```"]
    pub const VND_VISIONARY: Mime = Mime {
        ttype: "application",
        subtype: "vnd.visionary",
    };
    #[doc = "\\[Mark_Risher\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VIVIDENCE_SCRIPTFILE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.vividence.scriptfile\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.vividence.scriptfile\");"]
    #[doc = r" ```"]
    pub const VND_VIVIDENCE_SCRIPTFILE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.vividence.scriptfile",
    };
    #[doc = "\\[Delton_Rowe\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_VSF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.vsf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.vsf\");"]
    #[doc = r" ```"]
    pub const VND_VSF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.vsf",
    };
    #[doc = "\\[WAP-Forum\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WAP_SIC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wap.sic\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wap.sic\");"]
    #[doc = r" ```"]
    pub const VND_WAP_SIC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wap.sic",
    };
    #[doc = "\\[WAP-Forum\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WAP_SLC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wap.slc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wap.slc\");"]
    #[doc = r" ```"]
    pub const VND_WAP_SLC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wap.slc",
    };
    #[doc = "\\[Peter_Stark\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WAP_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wap.wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wap.wbxml\");"]
    #[doc = r" ```"]
    pub const VND_WAP_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wap.wbxml",
    };
    #[doc = "\\[Peter_Stark\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WAP_WMLC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wap.wmlc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wap.wmlc\");"]
    #[doc = r" ```"]
    pub const VND_WAP_WMLC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wap.wmlc",
    };
    #[doc = "\\[Peter_Stark\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WAP_WMLSCRIPTC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wap.wmlscriptc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wap.wmlscriptc\");"]
    #[doc = r" ```"]
    pub const VND_WAP_WMLSCRIPTC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wap.wmlscriptc",
    };
    #[doc = "\\[Fawad_Shaikh\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WASMFLOW_WAFL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wasmflow.wafl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wasmflow.wafl\");"]
    #[doc = r" ```"]
    pub const VND_WASMFLOW_WAFL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wasmflow.wafl",
    };
    #[doc = "\\[Yaser_Rehem\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WEBTURBO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.webturbo\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.webturbo\");"]
    #[doc = r" ```"]
    pub const VND_WEBTURBO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.webturbo",
    };
    #[doc = "\\[Wi-Fi_Alliance\\]\\[Dr._Jun_Tian\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WFA_DPP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wfa.dpp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wfa.dpp\");"]
    #[doc = r" ```"]
    pub const VND_WFA_DPP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wfa.dpp",
    };
    #[doc = "\\[Mick_Conley\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WFA_P2P;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wfa.p2p\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wfa.p2p\");"]
    #[doc = r" ```"]
    pub const VND_WFA_P2P: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wfa.p2p",
    };
    #[doc = "\\[Wi-Fi_Alliance\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WFA_WSC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wfa.wsc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wfa.wsc\");"]
    #[doc = r" ```"]
    pub const VND_WFA_WSC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wfa.wsc",
    };
    #[doc = "\\[Priya_Dandawate\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WINDOWS_DEVICEPAIRING;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.windows.devicepairing\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.windows.devicepairing\");"]
    #[doc = r" ```"]
    pub const VND_WINDOWS_DEVICEPAIRING: Mime = Mime {
        ttype: "application",
        subtype: "vnd.windows.devicepairing",
    };
    #[doc = "\\[Thomas_Kjornes\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WMC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wmc\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wmc\");"]
    #[doc = r" ```"]
    pub const VND_WMC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wmc",
    };
    #[doc = "\\[Thinh_Nguyenphu\\]\\[Prakash_Iyer\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WMF_BOOTSTRAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wmf.bootstrap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wmf.bootstrap\");"]
    #[doc = r" ```"]
    pub const VND_WMF_BOOTSTRAP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wmf.bootstrap",
    };
    #[doc = "\\[Wolfram\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WOLFRAM_MATHEMATICA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wolfram.mathematica\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wolfram.mathematica\");"]
    #[doc = r" ```"]
    pub const VND_WOLFRAM_MATHEMATICA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wolfram.mathematica",
    };
    #[doc = "\\[Wolfram\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WOLFRAM_MATHEMATICA_PACKAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wolfram.mathematica.package\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wolfram.mathematica.package\");"]
    #[doc = r" ```"]
    pub const VND_WOLFRAM_MATHEMATICA_PACKAGE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wolfram.mathematica.package",
    };
    #[doc = "\\[Wolfram\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WOLFRAM_PLAYER;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wolfram.player\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wolfram.player\");"]
    #[doc = r" ```"]
    pub const VND_WOLFRAM_PLAYER: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wolfram.player",
    };
    #[doc = "\\[David_Riccitelli\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WORDLIFT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wordlift\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wordlift\");"]
    #[doc = r" ```"]
    pub const VND_WORDLIFT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wordlift",
    };
    #[doc = "\\[Kim_Scarborough\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WORDPERFECT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wordperfect\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wordperfect\");"]
    #[doc = r" ```"]
    pub const VND_WORDPERFECT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wordperfect",
    };
    #[doc = "\\[Jan_Bostrom\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WQD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wqd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wqd\");"]
    #[doc = r" ```"]
    pub const VND_WQD: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wqd",
    };
    #[doc = "\\[Chris_Bartram\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WRQ_HP3000_LABELLED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wrq-hp3000-labelled\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wrq-hp3000-labelled\");"]
    #[doc = r" ```"]
    pub const VND_WRQ_HP3000_LABELLED: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wrq-hp3000-labelled",
    };
    #[doc = "\\[Bill_Wohler\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WT_STF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wt.stf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wt.stf\");"]
    #[doc = r" ```"]
    pub const VND_WT_STF: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wt.stf",
    };
    #[doc = "\\[John_Ingi_Ingimundarson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WV_CSP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wv.csp+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wv.csp+xml\");"]
    #[doc = r" ```"]
    pub const VND_WV_CSP_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wv.csp+xml",
    };
    #[doc = "\\[Matti_Salmi\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WV_CSP_WBXML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wv.csp+wbxml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wv.csp+wbxml\");"]
    #[doc = r" ```"]
    pub const VND_WV_CSP_WBXML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wv.csp+wbxml",
    };
    #[doc = "\\[John_Ingi_Ingimundarson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_WV_SSP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.wv.ssp+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.wv.ssp+xml\");"]
    #[doc = r" ```"]
    pub const VND_WV_SSP_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.wv.ssp+xml",
    };
    #[doc = "\\[David_Brossard\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XACML_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xacml+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xacml+json\");"]
    #[doc = r" ```"]
    pub const VND_XACML_JSON: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xacml+json",
    };
    #[doc = "\\[David_Matthewman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XARA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xara\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xara\");"]
    #[doc = r" ```"]
    pub const VND_XARA: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xara",
    };
    #[doc = "\\[Ben_Brown\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XARIN_CPJ;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xarin.cpj\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xarin.cpj\");"]
    #[doc = r" ```"]
    pub const VND_XARIN_CPJ: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xarin.cpj",
    };
    #[doc = "\\[Svante_Seleborg\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XECRETS_ENCRYPTED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xecrets-encrypted\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xecrets-encrypted\");"]
    #[doc = r" ```"]
    pub const VND_XECRETS_ENCRYPTED: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xecrets-encrypted",
    };
    #[doc = "\\[Dave_Manning\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XFDL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xfdl\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xfdl\");"]
    #[doc = r" ```"]
    pub const VND_XFDL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xfdl",
    };
    #[doc = "\\[Michael_Mansell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XFDL_WEBFORM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xfdl.webform\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xfdl.webform\");"]
    #[doc = r" ```"]
    pub const VND_XFDL_WEBFORM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xfdl.webform",
    };
    #[doc = "\\[Fred_Waskiewicz\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XMI_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xmi+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xmi+xml\");"]
    #[doc = r" ```"]
    pub const VND_XMI_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xmi+xml",
    };
    #[doc = "\\[Reuven_Sherwin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XMPIE_CPKG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xmpie.cpkg\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xmpie.cpkg\");"]
    #[doc = r" ```"]
    pub const VND_XMPIE_CPKG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xmpie.cpkg",
    };
    #[doc = "\\[Reuven_Sherwin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XMPIE_DPKG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xmpie.dpkg\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xmpie.dpkg\");"]
    #[doc = r" ```"]
    pub const VND_XMPIE_DPKG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xmpie.dpkg",
    };
    #[doc = "\\[Reuven_Sherwin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XMPIE_PLAN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xmpie.plan\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xmpie.plan\");"]
    #[doc = r" ```"]
    pub const VND_XMPIE_PLAN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xmpie.plan",
    };
    #[doc = "\\[Reuven_Sherwin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XMPIE_PPKG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xmpie.ppkg\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xmpie.ppkg\");"]
    #[doc = r" ```"]
    pub const VND_XMPIE_PPKG: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xmpie.ppkg",
    };
    #[doc = "\\[Reuven_Sherwin\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_XMPIE_XLIM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.xmpie.xlim\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.xmpie.xlim\");"]
    #[doc = r" ```"]
    pub const VND_XMPIE_XLIM: Mime = Mime {
        ttype: "application",
        subtype: "vnd.xmpie.xlim",
    };
    #[doc = "\\[Tomohiro_Yamamoto\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAMAHA_HV_DIC;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yamaha.hv-dic\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yamaha.hv-dic\");"]
    #[doc = r" ```"]
    pub const VND_YAMAHA_HV_DIC: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yamaha.hv-dic",
    };
    #[doc = "\\[Tomohiro_Yamamoto\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAMAHA_HV_SCRIPT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yamaha.hv-script\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yamaha.hv-script\");"]
    #[doc = r" ```"]
    pub const VND_YAMAHA_HV_SCRIPT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yamaha.hv-script",
    };
    #[doc = "\\[Tomohiro_Yamamoto\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAMAHA_HV_VOICE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yamaha.hv-voice\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yamaha.hv-voice\");"]
    #[doc = r" ```"]
    pub const VND_YAMAHA_HV_VOICE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yamaha.hv-voice",
    };
    #[doc = "\\[Mark_Olleson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAMAHA_OPENSCOREFORMAT_OSFPVG_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yamaha.openscoreformat.osfpvg+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yamaha.openscoreformat.osfpvg+xml\");"]
    #[doc = r" ```"]
    pub const VND_YAMAHA_OPENSCOREFORMAT_OSFPVG_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yamaha.openscoreformat.osfpvg+xml",
    };
    #[doc = "\\[Mark_Olleson\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAMAHA_OPENSCOREFORMAT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yamaha.openscoreformat\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yamaha.openscoreformat\");"]
    #[doc = r" ```"]
    pub const VND_YAMAHA_OPENSCOREFORMAT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yamaha.openscoreformat",
    };
    #[doc = "\\[Takehiro_Sukizaki\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAMAHA_REMOTE_SETUP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yamaha.remote-setup\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yamaha.remote-setup\");"]
    #[doc = r" ```"]
    pub const VND_YAMAHA_REMOTE_SETUP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yamaha.remote-setup",
    };
    #[doc = "\\[Keiichi_Shinoda\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAMAHA_SMAF_AUDIO;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yamaha.smaf-audio\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yamaha.smaf-audio\");"]
    #[doc = r" ```"]
    pub const VND_YAMAHA_SMAF_AUDIO: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yamaha.smaf-audio",
    };
    #[doc = "\\[Keiichi_Shinoda\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAMAHA_SMAF_PHRASE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yamaha.smaf-phrase\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yamaha.smaf-phrase\");"]
    #[doc = r" ```"]
    pub const VND_YAMAHA_SMAF_PHRASE: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yamaha.smaf-phrase",
    };
    #[doc = "\\[Takehiro_Sukizaki\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAMAHA_THROUGH_NGN;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yamaha.through-ngn\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yamaha.through-ngn\");"]
    #[doc = r" ```"]
    pub const VND_YAMAHA_THROUGH_NGN: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yamaha.through-ngn",
    };
    #[doc = "\\[Takehiro_Sukizaki\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAMAHA_TUNNEL_UDPENCAP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yamaha.tunnel-udpencap\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yamaha.tunnel-udpencap\");"]
    #[doc = r" ```"]
    pub const VND_YAMAHA_TUNNEL_UDPENCAP: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yamaha.tunnel-udpencap",
    };
    #[doc = "\\[Jens_Jorgensen\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YAOWEME;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yaoweme\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yaoweme\");"]
    #[doc = r" ```"]
    pub const VND_YAOWEME: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yaoweme",
    };
    #[doc = "\\[Mr._Yellow\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YELLOWRIVER_CUSTOM_MENU;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.yellowriver-custom-menu\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.yellowriver-custom-menu\");"]
    #[doc = r" ```"]
    pub const VND_YELLOWRIVER_CUSTOM_MENU: Mime = Mime {
        ttype: "application",
        subtype: "vnd.yellowriver-custom-menu",
    };
    #[doc = "\\[Laura_Wood\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_YOUTUBE_YT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.youtube.yt\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.youtube.yt\");"]
    #[doc = r" ```"]
    pub const VND_YOUTUBE_YT: Mime = Mime {
        ttype: "application",
        subtype: "vnd.youtube.yt",
    };
    #[doc = "\\[Rene_Grothmann\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ZUL;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.zul\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.zul\");"]
    #[doc = r" ```"]
    pub const VND_ZUL: Mime = Mime {
        ttype: "application",
        subtype: "vnd.zul",
    };
    #[doc = "\\[Micheal_Hewett\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VND_ZZAZZ_DECK_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vnd.zzazz.deck+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vnd.zzazz.deck+xml\");"]
    #[doc = r" ```"]
    pub const VND_ZZAZZ_DECK_XML: Mime = Mime {
        ttype: "application",
        subtype: "vnd.zzazz.deck+xml",
    };
    #[doc = "\\[RFC4267\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VOICEXML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"voicexml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/voicexml+xml\");"]
    #[doc = r" ```"]
    pub const VOICEXML_XML: Mime = Mime {
        ttype: "application",
        subtype: "voicexml+xml",
    };
    #[doc = "\\[RFC8366\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VOUCHER_CMS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"voucher-cms+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/voucher-cms+json\");"]
    #[doc = r" ```"]
    pub const VOUCHER_CMS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "voucher-cms+json",
    };
    #[doc = "\\[W3C\\]\\[Ivan_Herman\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vp\");"]
    #[doc = r" ```"]
    pub const VP: Mime = Mime {
        ttype: "application",
        subtype: "vp",
    };
    #[doc = "\\[RFC6035\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::VQ_RTCPXR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"vq-rtcpxr\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/vq-rtcpxr\");"]
    #[doc = r" ```"]
    pub const VQ_RTCPXR: Mime = Mime {
        ttype: "application",
        subtype: "vq-rtcpxr",
    };
    #[doc = "\\[W3C\\]\\[Eric_Prudhommeaux\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::WASM;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"wasm\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/wasm\");"]
    #[doc = r" ```"]
    pub const WASM: Mime = Mime {
        ttype: "application",
        subtype: "wasm",
    };
    #[doc = "\\[RFC3858\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::WATCHERINFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"watcherinfo+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/watcherinfo+xml\");"]
    #[doc = r" ```"]
    pub const WATCHERINFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "watcherinfo+xml",
    };
    #[doc = "\\[RFC8292\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::WEBPUSH_OPTIONS_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"webpush-options+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/webpush-options+json\");"]
    #[doc = r" ```"]
    pub const WEBPUSH_OPTIONS_JSON: Mime = Mime {
        ttype: "application",
        subtype: "webpush-options+json",
    };
    #[doc = "\\[RFC2957\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::WHOISPP_QUERY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"whoispp-query\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/whoispp-query\");"]
    #[doc = r" ```"]
    pub const WHOISPP_QUERY: Mime = Mime {
        ttype: "application",
        subtype: "whoispp-query",
    };
    #[doc = "\\[RFC2958\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::WHOISPP_RESPONSE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"whoispp-response\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/whoispp-response\");"]
    #[doc = r" ```"]
    pub const WHOISPP_RESPONSE: Mime = Mime {
        ttype: "application",
        subtype: "whoispp-response",
    };
    #[doc = "\\[W3C\\]\\[Steven_Pemberton\\]\\[W3C-Widgets-2012\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::WIDGET;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"widget\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/widget\");"]
    #[doc = r" ```"]
    pub const WIDGET: Mime = Mime {
        ttype: "application",
        subtype: "widget",
    };
    #[doc = "\\[Larry_Campbell\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::WITA;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"wita\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/wita\");"]
    #[doc = r" ```"]
    pub const WITA: Mime = Mime {
        ttype: "application",
        subtype: "wita",
    };
    #[doc = "\\[Paul_Lindner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::WORDPERFECT5_1;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"wordperfect5.1\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/wordperfect5.1\");"]
    #[doc = r" ```"]
    pub const WORDPERFECT5_1: Mime = Mime {
        ttype: "application",
        subtype: "wordperfect5.1",
    };
    #[doc = "\\[W3C\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::WSDL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"wsdl+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/wsdl+xml\");"]
    #[doc = r" ```"]
    pub const WSDL_XML: Mime = Mime {
        ttype: "application",
        subtype: "wsdl+xml",
    };
    #[doc = "\\[W3C\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::WSPOLICY_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"wspolicy+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/wspolicy+xml\");"]
    #[doc = r" ```"]
    pub const WSPOLICY_XML: Mime = Mime {
        ttype: "application",
        subtype: "wspolicy+xml",
    };
    #[doc = "\\[RFC8894\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::X_PKI_MESSAGE;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"x-pki-message\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/x-pki-message\");"]
    #[doc = r" ```"]
    pub const X_PKI_MESSAGE: Mime = Mime {
        ttype: "application",
        subtype: "x-pki-message",
    };
    #[doc = "\\[WHATWG\\]\\[Anne_van_Kesteren\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::X_WWW_FORM_URLENCODED;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"x-www-form-urlencoded\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/x-www-form-urlencoded\");"]
    #[doc = r" ```"]
    pub const X_WWW_FORM_URLENCODED: Mime = Mime {
        ttype: "application",
        subtype: "x-www-form-urlencoded",
    };
    #[doc = "\\[RFC8894\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::X_X509_CA_CERT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"x-x509-ca-cert\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/x-x509-ca-cert\");"]
    #[doc = r" ```"]
    pub const X_X509_CA_CERT: Mime = Mime {
        ttype: "application",
        subtype: "x-x509-ca-cert",
    };
    #[doc = "\\[RFC8894\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::X_X509_CA_RA_CERT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"x-x509-ca-ra-cert\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/x-x509-ca-ra-cert\");"]
    #[doc = r" ```"]
    pub const X_X509_CA_RA_CERT: Mime = Mime {
        ttype: "application",
        subtype: "x-x509-ca-ra-cert",
    };
    #[doc = "\\[RFC8894\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::X_X509_NEXT_CA_CERT;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"x-x509-next-ca-cert\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/x-x509-next-ca-cert\");"]
    #[doc = r" ```"]
    pub const X_X509_NEXT_CA_CERT: Mime = Mime {
        ttype: "application",
        subtype: "x-x509-next-ca-cert",
    };
    #[doc = "\\[RFC1494\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::X400_BP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"x400-bp\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/x400-bp\");"]
    #[doc = r" ```"]
    pub const X400_BP: Mime = Mime {
        ttype: "application",
        subtype: "x400-bp",
    };
    #[doc = "\\[RFC7061\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XACML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xacml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xacml+xml\");"]
    #[doc = r" ```"]
    pub const XACML_XML: Mime = Mime {
        ttype: "application",
        subtype: "xacml+xml",
    };
    #[doc = "\\[RFC4825\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XCAP_ATT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xcap-att+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xcap-att+xml\");"]
    #[doc = r" ```"]
    pub const XCAP_ATT_XML: Mime = Mime {
        ttype: "application",
        subtype: "xcap-att+xml",
    };
    #[doc = "\\[RFC4825\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XCAP_CAPS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xcap-caps+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xcap-caps+xml\");"]
    #[doc = r" ```"]
    pub const XCAP_CAPS_XML: Mime = Mime {
        ttype: "application",
        subtype: "xcap-caps+xml",
    };
    #[doc = "\\[RFC5874\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XCAP_DIFF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xcap-diff+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xcap-diff+xml\");"]
    #[doc = r" ```"]
    pub const XCAP_DIFF_XML: Mime = Mime {
        ttype: "application",
        subtype: "xcap-diff+xml",
    };
    #[doc = "\\[RFC4825\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XCAP_EL_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xcap-el+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xcap-el+xml\");"]
    #[doc = r" ```"]
    pub const XCAP_EL_XML: Mime = Mime {
        ttype: "application",
        subtype: "xcap-el+xml",
    };
    #[doc = "\\[RFC4825\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XCAP_ERROR_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xcap-error+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xcap-error+xml\");"]
    #[doc = r" ```"]
    pub const XCAP_ERROR_XML: Mime = Mime {
        ttype: "application",
        subtype: "xcap-error+xml",
    };
    #[doc = "\\[RFC4825\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XCAP_NS_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xcap-ns+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xcap-ns+xml\");"]
    #[doc = r" ```"]
    pub const XCAP_NS_XML: Mime = Mime {
        ttype: "application",
        subtype: "xcap-ns+xml",
    };
    #[doc = "\\[RFC6502\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XCON_CONFERENCE_INFO_DIFF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xcon-conference-info-diff+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xcon-conference-info-diff+xml\");"]
    #[doc = r" ```"]
    pub const XCON_CONFERENCE_INFO_DIFF_XML: Mime = Mime {
        ttype: "application",
        subtype: "xcon-conference-info-diff+xml",
    };
    #[doc = "\\[RFC6502\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XCON_CONFERENCE_INFO_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xcon-conference-info+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xcon-conference-info+xml\");"]
    #[doc = r" ```"]
    pub const XCON_CONFERENCE_INFO_XML: Mime = Mime {
        ttype: "application",
        subtype: "xcon-conference-info+xml",
    };
    #[doc = "\\[Joseph_Reagle\\]\\[XENC_Working_Group\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XENC_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xenc+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xenc+xml\");"]
    #[doc = r" ```"]
    pub const XENC_XML: Mime = Mime {
        ttype: "application",
        subtype: "xenc+xml",
    };
    #[doc = "\\[ISO-TC_171-SC_2\\]\\[Betsy_Fanning\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XFDF;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xfdf\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xfdf\");"]
    #[doc = r" ```"]
    pub const XFDF: Mime = Mime {
        ttype: "application",
        subtype: "xfdf",
    };
    #[doc = "\\[W3C\\]\\[Robin_Berjon\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XHTML_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xhtml+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xhtml+xml\");"]
    #[doc = r" ```"]
    pub const XHTML_XML: Mime = Mime {
        ttype: "application",
        subtype: "xhtml+xml",
    };
    #[doc = "\\[OASIS\\]\\[Chet_Ensign\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XLIFF_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xliff+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xliff+xml\");"]
    #[doc = r" ```"]
    pub const XLIFF_XML: Mime = Mime {
        ttype: "application",
        subtype: "xliff+xml",
    };
    #[doc = "\\[RFC7303\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xml\");"]
    #[doc = r" ```"]
    pub const XML: Mime = Mime {
        ttype: "application",
        subtype: "xml",
    };
    #[doc = "\\[RFC7303\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XML_DTD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xml-dtd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xml-dtd\");"]
    #[doc = r" ```"]
    pub const XML_DTD: Mime = Mime {
        ttype: "application",
        subtype: "xml-dtd",
    };
    #[doc = "\\[RFC7303\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XML_EXTERNAL_PARSED_ENTITY;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xml-external-parsed-entity\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xml-external-parsed-entity\");"]
    #[doc = r" ```"]
    pub const XML_EXTERNAL_PARSED_ENTITY: Mime = Mime {
        ttype: "application",
        subtype: "xml-external-parsed-entity",
    };
    #[doc = "\\[RFC7351\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XML_PATCH_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xml-patch+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xml-patch+xml\");"]
    #[doc = r" ```"]
    pub const XML_PATCH_XML: Mime = Mime {
        ttype: "application",
        subtype: "xml-patch+xml",
    };
    #[doc = "\\[RFC3923\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XMPP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xmpp+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xmpp+xml\");"]
    #[doc = r" ```"]
    pub const XMPP_XML: Mime = Mime {
        ttype: "application",
        subtype: "xmpp+xml",
    };
    #[doc = "\\[Mark_Nottingham\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XOP_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xop+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xop+xml\");"]
    #[doc = r" ```"]
    pub const XOP_XML: Mime = Mime {
        ttype: "application",
        subtype: "xop+xml",
    };
    #[doc = "\\[W3C\\]\\[http://www.w3.org/TR/2007/REC-xslt20-20070123/#media-type-registration\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XSLT_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xslt+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xslt+xml\");"]
    #[doc = r" ```"]
    pub const XSLT_XML: Mime = Mime {
        ttype: "application",
        subtype: "xslt+xml",
    };
    #[doc = "\\[RFC4374\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::XV_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"xv+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/xv+xml\");"]
    #[doc = r" ```"]
    pub const XV_XML: Mime = Mime {
        ttype: "application",
        subtype: "xv+xml",
    };
    #[doc = "\\[YAML\\]\\[RFC9512\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::YAML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"yaml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/yaml\");"]
    #[doc = r" ```"]
    pub const YAML: Mime = Mime {
        ttype: "application",
        subtype: "yaml",
    };
    #[doc = "\\[RFC6020\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::YANG;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"yang\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/yang\");"]
    #[doc = r" ```"]
    pub const YANG: Mime = Mime {
        ttype: "application",
        subtype: "yang",
    };
    #[doc = "\\[RFC9254\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::YANG_DATA_CBOR;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"yang-data+cbor\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/yang-data+cbor\");"]
    #[doc = r" ```"]
    pub const YANG_DATA_CBOR: Mime = Mime {
        ttype: "application",
        subtype: "yang-data+cbor",
    };
    #[doc = "\\[RFC8040\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::YANG_DATA_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"yang-data+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/yang-data+json\");"]
    #[doc = r" ```"]
    pub const YANG_DATA_JSON: Mime = Mime {
        ttype: "application",
        subtype: "yang-data+json",
    };
    #[doc = "\\[RFC8040\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::YANG_DATA_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"yang-data+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/yang-data+xml\");"]
    #[doc = r" ```"]
    pub const YANG_DATA_XML: Mime = Mime {
        ttype: "application",
        subtype: "yang-data+xml",
    };
    #[doc = "\\[RFC8072\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::YANG_PATCH_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"yang-patch+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/yang-patch+json\");"]
    #[doc = r" ```"]
    pub const YANG_PATCH_JSON: Mime = Mime {
        ttype: "application",
        subtype: "yang-patch+json",
    };
    #[doc = "\\[RFC8072\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::YANG_PATCH_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"yang-patch+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/yang-patch+xml\");"]
    #[doc = r" ```"]
    pub const YANG_PATCH_XML: Mime = Mime {
        ttype: "application",
        subtype: "yang-patch+xml",
    };
    #[doc = "\\[RFC9595\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::YANG_SID_JSON;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"yang-sid+json\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/yang-sid+json\");"]
    #[doc = r" ```"]
    pub const YANG_SID_JSON: Mime = Mime {
        ttype: "application",
        subtype: "yang-sid+json",
    };
    #[doc = "\\[RFC6020\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::YIN_XML;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"yin+xml\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/yin+xml\");"]
    #[doc = r" ```"]
    pub const YIN_XML: Mime = Mime {
        ttype: "application",
        subtype: "yin+xml",
    };
    #[doc = "\\[Paul_Lindner\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ZIP;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"zip\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/zip\");"]
    #[doc = r" ```"]
    pub const ZIP: Mime = Mime {
        ttype: "application",
        subtype: "zip",
    };
    #[doc = "\\[RFC6713\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ZLIB;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"zlib\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/zlib\");"]
    #[doc = r" ```"]
    pub const ZLIB: Mime = Mime {
        ttype: "application",
        subtype: "zlib",
    };
    #[doc = "\\[RFC8878\\]"]
    #[doc = r" ```no_run"]
    #[doc = "let media = mime2::application::ZSTD;"]
    #[doc = "assert_eq!(media.ttype, \"application\");"]
    #[doc = "assert_eq!(media.subtype, \"zstd\");"]
    #[doc = "assert_eq!(media.to_string(), \"application/zstd\");"]
    #[doc = r" ```"]
    pub const ZSTD: Mime = Mime {
        ttype: "application",
        subtype: "zstd",
    };
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
