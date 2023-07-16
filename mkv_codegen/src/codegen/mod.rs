
mod ids;
mod structs;

mod reader;
mod writer;

use crate::parser::ebml;
use crate::parser::matroska;

pub fn run(path: &std::path::Path) -> Result<(), anyhow::Error> {
    let ebml_matroska = ebml::parse()?;
    // debug!("ebml_matroska {:#?}", ebml_matroska);
    // for element in &ebml_matroska.sorted_elements() {
    //     debug!("el: {}, type: {:?}, {:?}", element.type_name(), element.type_, element.paths());
    // }
    // for struct_ in &ebml_matroska.sorted_strcuts() {
    //     debug!("struct: {}               childs: {:?}", struct_.type_name(), struct_.childs());
    // }

    std::fs::write(path.join("mod.rs"), generate_mod(&ebml_matroska))?;
    std::fs::write(path.join("structs.rs"), structs::define_structs(&ebml_matroska))?;
    std::fs::write(path.join("reader.rs"), reader::generate(&ebml_matroska)?)?;
    std::fs::write(path.join("writer.rs"), writer::generate(&ebml_matroska)?)?;


    std::fs::write(path.join("ids.rs"), ids::generate(&ebml_matroska))?;

    let _matroska = matroska::parse()?;
    // debug!("matroska {:#?}", matroska);
    // for element in ebml_matroska.elements {
    //     debug!("el: {}, id: {}", element.name, element.id);
    // }

    Ok(())
}

pub fn generate_mod(_ebml_matroska: &ebml::EBMLMatroska) -> String {
    let mut str = format!("");
    str += "\
pub mod ids;
pub mod reader;
pub mod writer;
pub mod structs;

use super::*;
";
    str
}

pub(super) fn to_async(async_: bool, str: &str) -> String {
    if async_ {
        format!("async_::{str}.await")
    } else {
        format!("blocking::{str}")
    }
}