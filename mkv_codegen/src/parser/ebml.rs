
use anyhow::{ anyhow, Context };

use serde_derive::Deserialize;

use super::{from_bool, var_name, type_name, id_enum};

const EBML_HEADER_TAG: &'static str = "EbmlHeader";

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct EnumSrc {
    value: String,
    label: String,

    #[serde(default)]
    documentation: Vec<DocumentationSrc>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct RestrictionSrc {
    #[serde(default, rename = "enum")]
    enums: Vec<EnumSrc>,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
enum ExtensionType {
    #[serde(rename = "libmatroska")]
    LibMatroska,
    #[serde(rename = "webmproject.org")]
    WebmProjectOrg,
    #[serde(rename = "stream copy")]
    StreamCopy,
    #[serde(rename = "other document")]
    OtherDocument,
    #[serde(rename = "divx.com")]
    DivXCom
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct ExtensionSrc {
    #[serde(rename="type")]
    type_: ExtensionType,

    cppname: Option<String>,
    spec: Option<String>,

    #[serde(default, deserialize_with = "from_bool")]
    keep: bool,
    #[serde(default, deserialize_with = "from_bool")]
    webm: bool,
    #[serde(default, deserialize_with = "from_bool")]
    divx: bool,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
pub struct DocumentationSrc {
    lang: String,
    purpose: String,

    #[serde(rename="$value")]
    text: String,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct ImplementationNoteSrc {
    note_attribute: String,
    #[serde(rename="$value")]
    text: String,
}

#[derive(Debug, PartialEq, Clone)]
struct PathSegmentSrc {
    pub type_: String,
    pub plus: bool,
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct ElementSrc {
    #[serde(skip)]
    path_: Vec<PathSegmentSrc>,

    #[serde(rename = "name")]
    name_: String,

    path: String,
    id: String,
    #[serde(rename = "type")]
    type_: String,
    default: Option<String>,

    minver: Option<u32>,
    maxver: Option<u32>,
    range: Option<String>,
    length: Option<u32>,
    #[serde(rename = "minOccurs")]
    min_occurs: Option<u32>,
    #[serde(rename = "maxOccurs")]
    max_occurs: Option<u32>,
    recurring: Option<u32>,
    #[serde(rename = "unknownsizeallowed")]
    unknown_size_allowed: Option<u32>,

    #[serde(default)]
    documentation: Vec<DocumentationSrc>,
    #[serde(default, rename = "implementation_note")]
    implementation_notes: Vec<ImplementationNoteSrc>,
    #[serde(default, rename = "extension")]
    extensions: Vec<ExtensionSrc>,

    restriction: Option<RestrictionSrc>,

    #[serde(default, deserialize_with = "from_bool")]
    recursive: bool,
}
impl ElementSrc {
    pub fn name(&self) -> String { type_name(&self.name_) }
}

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct EBMLMatroskaSrc {
    #[serde(rename = "docType")]
    doc_type: String,
    version: String,

    #[serde(rename = "element")]
    elements: Vec<ElementSrc>
}

#[derive(Debug, Clone, PartialEq)]
pub enum ElementType {
    UInteger,
    Integer,
    Float,
    String,
    Utf8,
    Binary,
    Date,
    Struct,
}
impl ElementType {
    pub fn to_native(&self, type_name: String) -> String {
        match self {
            ElementType::UInteger => "u64",
            ElementType::Integer => "i64",
            ElementType::Float => "f64",
            ElementType::String | ElementType::Utf8 => "String",
            ElementType::Date => "time::OffsetDateTime",
            ElementType::Binary => "Vec<u8>",
            ElementType::Struct => type_name.as_str(),
        }.to_string()
    }
    fn to_rust(&self) -> &'static str {
        match self {
            ElementType::UInteger => "UInteger",
            ElementType::Integer => "Integer",
            ElementType::Float => "Float",
            ElementType::String => "String",
            ElementType::Utf8 => "Utf8",
            ElementType::Binary => "Binary",
            ElementType::Date => "Date",
            ElementType::Struct => "Struct",
        }
    }
    fn from_xml_str(str: &str) -> Result<Self, anyhow::Error> {
        Ok(match str {
            "uinteger" => ElementType::UInteger,
            "integer" => ElementType::Integer,
            "float" => ElementType::Float,
            "string" => ElementType::String,
            "utf-8" => ElementType::Utf8,
            "binary" => ElementType::Binary,
            "date" => ElementType::Date,
            "master" => ElementType::Struct,
            str => { return Err(anyhow::format_err!("Unknown element type sting '{}'", str)); }
        })
    }
    fn to_xml_str(&self) -> String {
        match self {
            ElementType::UInteger => "uinteger",
            ElementType::Integer => "integer",
            ElementType::Float => "float",
            ElementType::String => "string",
            ElementType::Utf8 => "utf-8",
            ElementType::Binary => "binary",
            ElementType::Date => "date",
            ElementType::Struct => "master",
        }.to_string()
    }
}


#[derive(Debug, Clone)]
pub struct PathSegment {
    pub element: Box<Element>,
    pub plus: bool,
}

#[derive(Debug, Clone)]
pub enum TypeAttr {
    Required,
    Optional,
    Repeated,
}

#[derive(Debug, Clone)]
pub struct Element {
    index: usize,
    name: String,
    pub path_src: String,
    pub path: Vec<PathSegment>,
    pub id: u32,
    pub type_: ElementType,
    pub default: Option<String>,
    pub range: Option<String>,

    pub min_occurs: u32,
    pub max_occurs: Option<u32>,

    pub attr: TypeAttr,


    pub recurring: bool,
    pub unknown_size_allowed: bool,

    pub minver: Option<u32>,
    pub maxver: Option<u32>,

    pub recursive: bool,

    pub documentation: Vec<DocumentationSrc>,
}
impl Element {
    // pub fn paths(&self) -> Vec<String> {
    //     self.path.iter().map(|el| el.element.name.clone()).collect()
    // }
    pub fn var_name(&self) -> String { var_name(&self.name) }
    pub fn type_name(&self) -> String { type_name(&self.name) }
    pub fn rust_type(&self) -> &'static str { self.type_.to_rust() }
    pub fn id_enum(&self) -> String { id_enum(&self.name) }
}


#[derive(Debug, Clone)]
pub struct EBMLMatroska {
    pub elements: std::collections::BTreeMap<String, Box<Element>>,
    pub structs: std::collections::BTreeMap<String, Box<EBMLStruct>>
}
impl EBMLMatroska {
    pub fn sorted_strcuts(&self) -> Vec<Box<EBMLStruct>> {
        let mut structs: Vec<Box<EBMLStruct>> = self.structs.values().map(|b| b.clone()).collect();
        structs.sort_by(|a, b| a.element.index.cmp(&b.element.index));
        structs
    }
    pub fn sorted_elements(&self) -> Vec<Box<Element>> {
        let mut elements: Vec<Box<Element>> = self.elements.values().map(|b| b.clone()).collect();
        elements.sort_by(|a, b| a.index.cmp(&b.index));
        elements
    }
}

#[derive(Debug, Clone)]
pub struct Child {
    pub element: Box<Element>,
}

#[derive(Debug, Clone)]
pub struct EBMLStruct {
    pub(crate) element: Box<Element>,
    pub children: Vec<Child>,
}
impl EBMLStruct {
    // pub fn childs(&self) -> Vec<String> {
    //     self.children.iter().map(|k| k.element.type_name()).collect()
    // }
    pub fn type_name(&self) -> String { self.element.type_name() }
    pub fn size_type(&self) -> String { if self.element.unknown_size_allowed { "ElementSize" } else { "u64" }.to_string() }
}








pub fn parse() -> Result<EBMLMatroska, anyhow::Error> {

    // let ebml_matroska_xml = reqwest::get("https://raw.githubusercontent.com/ietf-wg-cellar/matroska-specification/master/ebml_matroska.xml").await?
    //     .text().await?;
    // https://raw.githubusercontent.com/ietf-wg-cellar/matroska-specification/master/ebml_matroska.xml
    // let path = "./ebml_matroska.xml";
    // let ebml_matroska_xml = std::fs::read_to_string(path).context(format!("Failed to read to string file '{path}'"))?;

    let ebml_matroska_xml = include_str!("./../../ebml_matroska.xml");
    let ebml_matroska_xml = ebml_matroska_xml.replace("Mastering", "Structing");
    let ebml_matroska_xml = ebml_matroska_xml.replace("Master", "Struct");
    if ebml_matroska_xml.contains("Master") {
        panic!("Master is alive!!!!!")
    }
    let mut ebml_matroska_src: EBMLMatroskaSrc = serde_xml_rs::from_str(&ebml_matroska_xml).context("Failed to parse xml")?;
    {
        let mut elements = vec![];
        insert_elements(&mut elements);
        insert_void_crc32(&mut elements)?;
        for element in ebml_matroska_src.elements {
            elements.push(element);
        }
        ebml_matroska_src.elements = elements;
    }

    let mut elements = std::collections::BTreeMap::new();

    let last = elements.len();
    for (index, element_src) in ebml_matroska_src.elements.iter_mut().enumerate() {
        let index = index + last;
        let path_src = element_src.path.clone();
        let path = {
            let path = element_src.path.replace("\\", "/");
            // let path = path.replace("/+", "/");
            let path = path.trim_matches('/');
            let path : Vec<PathSegmentSrc> = path.split("/").map(|s| {
                let plus = s.contains("+");
                let s = s.replace("+", "");
                let s = if s == "EBML" { EBML_HEADER_TAG } else { s.as_str() };
                let segment = type_name(s);
                PathSegmentSrc { type_: segment, plus }
            }).collect();
            match path.split_last() {
                Some((last, path)) => {
                    if last.type_.as_str() != element_src.name().as_str() {
                        return Err(anyhow!("path last element is not equals name '{}'", last.type_));
                    }
                    path.to_vec()
                },
                None => path,
            }
        };
        element_src.path_ = path.clone();
        let range = element_src.range.clone().map(|v| {
            // let params : Vec<&str> = v.split("-").collect();
            // if params.len() != 2 { panic!("range value '{}' is incorrect", v); }
            // (u32::from_str(params[0]).unwrap(), u32::from_str(params[1]).unwrap())
            v
        });

        let min_occurs = element_src.min_occurs.unwrap_or(0);

        let attr = match element_src.max_occurs {
            Some(max_occurs) => {
                if max_occurs > 1 { TypeAttr::Repeated } else {
                    if min_occurs == 0 { TypeAttr::Optional } else { TypeAttr::Required }
                }
            }
            None => TypeAttr::Repeated
        };

        if let Some(element) = elements.insert(element_src.name(), Box::new(Element {
            index,
            name: element_src.name(),
            path_src, path: vec![],
            id: u32::from_str_radix(&element_src.id.replace("0x", ""), 16).expect(&format!("id value '{}' is not u32", element_src.id)),
            type_: ElementType::from_xml_str(&element_src.type_)?,
            // default: element_src.default.map(|v| u32::from_str(&v).expect(&format!("value '{}' is not error", v))),
            default: element_src.default.clone(),
            minver: element_src.minver,
            maxver: element_src.maxver,
            range,
            min_occurs,
            max_occurs: element_src.max_occurs,
            attr,

            recurring: element_src.recurring.map(|v| v == 1).unwrap_or(false),
            unknown_size_allowed: element_src.unknown_size_allowed.map(|v| v == 1).unwrap_or(false),
            documentation: element_src.documentation.clone(),
            recursive: element_src.recursive,
        })) {
            return Err(anyhow!("Duplicate element with name '{}'", element.name))
        };
    }

    for element_src in &ebml_matroska_src.elements {
        let mut path = vec![];
        for segment in &element_src.path_ {
            match elements.get(&segment.type_.clone()) {
                Some(el) => path.push(PathSegment { element: el.clone(), plus: segment.plus }),
                None => return Err(anyhow!("Can't find element with name '{}'", segment.type_)),
            }
        }
        match elements.get_mut(&element_src.name()) {
            Some(element) => element.path = path,
            None => return Err(anyhow!("Can't find element with name '{}'", element_src.name())),
        }
    }

    let mut structs = std::collections::BTreeMap::new();

    for (_, element) in &elements {
        if element.type_ == ElementType::Struct {
            let struct_ = EBMLStruct {
                element: element.clone(),
                children: Default::default()
            };
            structs.insert(struct_.type_name(), Box::new(struct_));
        }
    }

    let mut children = std::collections::BTreeMap::new();
    for (_, element) in &elements {
        if let Some(last) = element.path.last() {
            match structs.get_mut(last.element.name.as_str()) {
                Some(struct_) => {
                    let child = Child {
                        element: element.clone(),
                    };
                    if let Some(_) = children.insert(element.name.clone(), child.clone()) {
                        return Err(anyhow!("child '{}' already exists in struct '{}'", element.name, last.element.name));
                    }
                    struct_.children.push(child);
                },
                None => {}
            }
        }
    }

    for (_, struct_) in structs.iter_mut() {
        struct_.children.sort_by(|a, b| a.element.index.cmp(&b.element.index));
    }

    let ebml_matroska = EBMLMatroska { elements, structs };

    Ok(ebml_matroska)
}


fn insert_void_crc32(elements: &mut Vec<ElementSrc>) -> Result<(), anyhow::Error> {
    let segment = "Segment".to_string();
    {
        let name = "Void".to_string();
        let element = ElementSrc {
            name_: name.clone(), path: format!("\\{segment}\\{name}"), path_: vec![],
            id: "0xEC".to_string(),
            type_: ElementType::Binary.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(0), max_occurs: None,
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    }
    {
        let name = "Crc32".to_string();
        let element = ElementSrc {
            name_: name.clone(), path: format!("\\{segment}\\{name}"), path_: vec![],
            id: "0xBF".to_string(),
            type_: ElementType::Binary.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(0), max_occurs: Some(1),
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    }
    Ok(())
}

fn insert_elements(elements: &mut Vec<ElementSrc>) {
    {
        let element = ElementSrc {
            name_: EBML_HEADER_TAG.to_string(), path: format!("\\{EBML_HEADER_TAG}"), path_: vec![],
            id: "0x1A45DFA3".to_string(),
            type_: ElementType::Struct.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(1), max_occurs: Some(1),
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    };
    {
        let name = "Version".to_string();
        let element = ElementSrc {
            name_: name.clone(), path: format!("\\{EBML_HEADER_TAG}\\{name}"), path_: vec![],
            id: "0x4286".to_string(),
            type_: ElementType::UInteger.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(1), max_occurs: Some(1),
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    }
    {
        let name = "ReadVersion".to_string();
        let element = ElementSrc {
            name_: name.clone(), path: format!("\\{EBML_HEADER_TAG}\\{name}"), path_: vec![],
            id: "0x42F7".to_string(),
            type_: ElementType::UInteger.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(1), max_occurs: Some(1),
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    }
    {
        let name = "DocType".to_string();
        let element = ElementSrc {
            name_: name.clone(), path: format!("\\{EBML_HEADER_TAG}\\{name}"), path_: vec![],
            id: "0x4282".to_string(),
            type_: ElementType::Utf8.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(1), max_occurs: Some(1),
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    }
    {
        let name = "DocTypeVersion".to_string();
        let element = ElementSrc {
            name_: name.clone(), path: format!("\\{EBML_HEADER_TAG}\\{name}"), path_: vec![],
            id: "0x4287".to_string(),
            type_: ElementType::UInteger.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(1), max_occurs: Some(1),
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    }
    {
        let name = "DocTypeReadVersion".to_string();
        let element = ElementSrc {
            name_: name.clone(), path: format!("\\{EBML_HEADER_TAG}\\{name}"), path_: vec![],
            id: "0x4285".to_string(),
            type_: ElementType::UInteger.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(1), max_occurs: Some(1),
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    }
    {
        let name = "DocTypeExtension".to_string();
        let element = ElementSrc {
            name_: name.clone(), path: format!("\\{EBML_HEADER_TAG}\\{name}"), path_: vec![],
            id: "0x4281".to_string(),
            type_: ElementType::Struct.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(0), max_occurs: Some(1),
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    }
    {
        let name = "DocTypeExtensionName".to_string();
        let element = ElementSrc {
            name_: name.clone(), path: format!("\\{EBML_HEADER_TAG}\\{name}"), path_: vec![],
            id: "0x4283".to_string(),
            type_: ElementType::Utf8.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(0), max_occurs: Some(1),
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    }
    {
        let name = "DocTypeExtensionVersion".to_string();
        let element = ElementSrc {
            name_: name.clone(), path: format!("\\{EBML_HEADER_TAG}\\{name}"), path_: vec![],
            id: "0x4284".to_string(),
            type_: ElementType::UInteger.to_xml_str(),
            default: None, range: None,
            length: None,
            min_occurs: Some(0), max_occurs: Some(1),
            recurring: None,
            unknown_size_allowed: Some(0),
            minver: None,
            maxver: None,
            recursive: false,
            documentation: vec![],
            implementation_notes: vec![],
            extensions: vec![],
            restriction: None,
        };
        elements.push( element);
    }
}