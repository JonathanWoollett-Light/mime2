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
