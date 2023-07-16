use std::convert::TryInto;
use anyhow::anyhow;
use serde_derive::Deserialize;

#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct DescriptionSrc {
    lang: String,

    #[serde(rename = "$value")]
    description: String,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct ClassSrc {
    name: String,

    #[serde(default)]
    description: Vec<DescriptionSrc>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct ClassesSrc {
    #[serde(rename = "class")]
    classes: Vec<ClassSrc>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
enum TagType {
    #[serde(rename = "binary")]
    Binary,
    #[serde(rename = "UTF-8")]
    UTF8,
    #[serde(rename = "-")]
    Unknown,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct TagSrc {
    name: String,
    class: String,
    #[serde(rename = "type")]
    type_: TagType,

    #[serde(default)]
    description: Vec<DescriptionSrc>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct TagsSrc {
    #[serde(rename = "tag")]
    tags: Vec<TagSrc>,
}
#[derive(Debug, Deserialize, PartialEq, Clone)]
#[serde(deny_unknown_fields)]
struct MatroskaSrc {
    version: String,

    classes: ClassesSrc,
    tags: TagsSrc,
}





#[derive(Debug, Clone)]
pub struct Class {
    name: String,
    descriptions: std::collections::BTreeMap<String, String>,
}
impl From<ClassSrc> for Class {
    fn from(val: ClassSrc) -> Self {
        let mut descriptions = std::collections::BTreeMap::new();
        for description in val.description {
            descriptions.insert(description.lang, description.description);
        }
        Self { name: val.name, descriptions }
    }
}

#[derive(Debug, Clone)]
pub struct Tag {
    name: String,
    class: Box<Class>,
    type_: TagType,
    descriptions: std::collections::BTreeMap<String, String>,
}
impl Tag {
    fn try_from(val: TagSrc, classes: &std::collections::BTreeMap<String, Box<Class>>) -> Result<Self, anyhow::Error> {
        let mut descriptions = std::collections::BTreeMap::new();
        for description in val.description {
            descriptions.insert(description.lang, description.description);
        }
        let class = val.class;
        let class = classes.get(&class).ok_or_else(move || anyhow!("Can't find class '{}'", class))?.clone();
        Ok(Self { name: val.name, class, type_: val.type_, descriptions })
    }
}

#[derive(Debug, Clone)]
pub struct Matroska {
    classes: std::collections::BTreeMap<String, Box<Class>>,
    tags: std::collections::BTreeMap<String, Box<Tag>>,
}
impl std::convert::TryFrom<MatroskaSrc> for Matroska {
    type Error = anyhow::Error;
    fn try_from(val: MatroskaSrc) -> Result<Self, Self::Error> {

        let mut classes = std::collections::BTreeMap::new();
        for class in val.classes.classes {
            let class: Class = class.into();
            if let Some(class) = classes.insert(class.name.clone(), Box::new(class)) {
                return Err(anyhow!("Duplicate class '{}'", class.name))
            };
        }

        let mut tags = std::collections::BTreeMap::new();
        for tag_src in val.tags.tags {
            let tag: Tag = Tag::try_from(tag_src, &classes)?;
            if let Some(tag) = tags.insert(tag.name.clone(), Box::new(tag)) {
                return Err(anyhow!("Duplicate tag '{}'", tag.name))
            };
        }

        Ok(Self { classes, tags })
    }
}

pub fn parse() -> Result<Matroska, anyhow::Error> {

    let matroska_tags_xml = include_str!("./../../matroska_tags.xml");
    // let path = "./matroska_tags.xml";
    // let matroska_tags_xml = std::fs::read_to_string(path).context(format!("Failed to read to string file '{path}'"))?;
    // let matroska_tags_xml = reqwest::get("https://raw.githubusercontent.com/ietf-wg-cellar/matroska-specification/master/matroska_tags.xml").await?.text().await?;

    // debug!("matroska_tags_xml\n{}", matroska_tags_xml);
    let matroska_tags_src: MatroskaSrc = serde_xml_rs::from_str(&matroska_tags_xml)?;
    // debug!("matroska_tags_src\n{:#?}", matroska_tags_src);

    Ok(matroska_tags_src.try_into()?)
}