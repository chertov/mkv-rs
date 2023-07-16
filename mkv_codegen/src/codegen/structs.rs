use crate::parser::ebml;

pub fn define_structs(ebml_matroska: &ebml::EBMLMatroska) -> String {
    let mut str = format!("");
    str += "use super::Ebml;\n";
    str += "use super::ids::EbmlId;\n";
    str += "use super::ElementSize;\n";
    str += "\n";

    for struct_ in &ebml_matroska.sorted_strcuts() {
        str += "#[derive(Debug, Clone, Default)]\n";
        str += &format!("pub struct {} {{\n", struct_.type_name());
        str += &format!("    pub size: {},\n", struct_.size_type());
        str += "\n";

        for child in &struct_.children {
            str += &format!("    pub {}: ", child.element.var_name());
            let type_name = child.element.type_name();
            let type_name = child.element.type_.to_native(type_name);
            str += &match child.element.attr {
                ebml::TypeAttr::Required => format!("Ebml<{type_name}>"),
                ebml::TypeAttr::Optional => format!("Option<Ebml<{type_name}>>"),
                ebml::TypeAttr::Repeated => format!("Vec<Ebml<{type_name}>>"),
            };
            str += ",\n";
        }
        str += "}\n";

        if !struct_.children.is_empty() {

            str += &format!("impl {} {{\n", struct_.type_name());
            str += &format!("    pub fn elements(&self) -> std::collections::BTreeSet<{}Fields> {{\n", struct_.type_name());
            str += &format!("        let mut elements = std::collections::BTreeSet::new();\n");
            for child in &struct_.children {
                let name = child.element.var_name();
                str += &format!("        ");
                str += &match child.element.attr {
                    ebml::TypeAttr::Required => format!("elements.insert({}Fields::{}(self.{name}.clone()));", struct_.type_name(), child.element.type_name()),
                    ebml::TypeAttr::Optional => format!("if let Some(el) = &self.{name} {{ elements.insert({}Fields::{}(el.clone())); }}", struct_.type_name(), child.element.type_name()),
                    ebml::TypeAttr::Repeated => format!("for el in &self.{name} {{ elements.insert({}Fields::{}(el.clone())); }}", struct_.type_name(), child.element.type_name()),
                };
                str += &format!("\n");
            }
            str += &format!("        elements\n");
            str += &format!("    }}\n");
            str += &format!("}}\n");

            str += &format!("#[derive(Debug)]\n");
            str += &format!("pub enum {}Fields {{\n", struct_.type_name());
            for child in &struct_.children {
                let type_name = child.element.type_name();
                let type_name = child.element.type_.to_native(type_name);
                str += &format!("    {}(Ebml<{type_name}>),\n", child.element.type_name());
            }
            str += "}\n";
            str += &format!("impl {}Fields {{\n", struct_.type_name());
            str += &format!("    pub fn index(&self) -> (EbmlId, Option<u64>, u64) {{\n");
            str += &format!("        match self {{\n");
            for child in &struct_.children {
                str += &format!("            Self::{}(val) => (EbmlId::{}, val.index, val.id),\n", child.element.type_name(), child.element.id_enum());
            }
            str += &format!("        }}\n");
            str += &format!("    }}\n");
            str += "}\n";

            str += &format!("crate::impl_ord!({}Fields);\n", struct_.type_name());
        }
        str += "\n";
    }
    str
}