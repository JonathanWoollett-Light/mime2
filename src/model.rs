use super::Mime;
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
