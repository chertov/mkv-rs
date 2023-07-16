use crate::parser::ebml;

pub fn generate(ebml_matroska: &ebml::EBMLMatroska) -> String {
    let mut str = format!("");
    str += "\
use super::ElementType;

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
#[repr(u64)]
pub enum EbmlId {
";
    for element in &ebml_matroska.sorted_elements() {
        str += &format!("    {} = 0x{:02X?},\n", element.id_enum(), element.id);
    }
    str += "
}
impl EbmlId {
    pub fn from_u64(id: u64) -> Result<Self, anyhow::Error> {
        Ok(match id {
";
    for element in &ebml_matroska.sorted_elements() {
        // str += &format!("            {} => Self::{},\n", element.id_const(), element.id_enum());
        str += &format!("            0x{:02X?} => Self::{},\n", element.id, element.id_enum());
    }
    str += "
            val => return Err(anyhow::anyhow!(\"unknown id: 0x{:02X?}\", val)),
        })
    }
    pub fn type_(&self) -> ElementType {
        match self {
";
    for element in &ebml_matroska.sorted_elements() {
        str += &format!("            Self::{} => ElementType::{},\n", element.id_enum(), element.rust_type());
        // str += &format!("pub const {}_INFO: ElementInfo = ElementInfo {{ id: {}, type_: ElementType::Binary }};\n", element.id_const(), element.id_const());
    }
    str += "
        }
    }
    pub fn unknown_size_allowed(&self) -> bool {
        match self {
";
    for element in &ebml_matroska.sorted_elements() {
        str += &format!("            Self::{} => {},\n", element.id_enum(), element.unknown_size_allowed);
        // str += &format!("pub const {}_INFO: ElementInfo = ElementInfo {{ id: {}, type_: ElementType::Binary }};\n", element.id_const(), element.id_const());
    }
    str += "
        }
    }
}
";
    // for element in &ebml_matroska.sorted_elements() {
    //     str += &format!("pub const {}: u64 = 0x{:02X?};\n", element.id_const(), element.id);
    //     // str += &format!("pub const {}_INFO: ElementInfo = ElementInfo {{ id: {}, type_: ElementType::Binary }};\n", element.id_const(), element.id_const());
    // }
    str += "\n";

    str
}