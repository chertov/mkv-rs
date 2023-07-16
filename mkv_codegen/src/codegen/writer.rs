use crate::parser::ebml;
use ebml::ElementType;
use super::to_async;

pub fn generate(ebml_matroska: &ebml::EBMLMatroska) -> Result<String, anyhow::Error> {
    let mut str = format!("");
    str += "
use std::collections::VecDeque;

use super::io::*;
use super::structs::*;
use super::ids::EbmlId;
use tokio::io::AsyncWriteExt;

";
    for struct_ in &ebml_matroska.sorted_strcuts() {
        str += &format!("impl {} {{\n", struct_.type_name());
        str += &impl_write(struct_, false);
        str += &impl_write_header(struct_, false);
        str += &impl_write_body(struct_, false);
        str += "}\n";
        str += &format!("impl {} {{\n", struct_.type_name());
        str += &impl_write(struct_, true);
        str += &impl_write_header(struct_, true);
        str += &impl_write_body(struct_, true);
        str += "}\n";
        str += "\n";
    }
    Ok(str)
}

fn impl_write(_struct_: &Box<ebml::EBMLStruct>, async_: bool) -> String {
    let mut str = format!("");
    let mut await_ = ".await";
    let mut blocking = "";
    if async_ {
        str += &format!("    pub async fn write<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {{\n");
    } else {
        str += &format!("    pub fn write_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {{\n");
        await_ = "";
        blocking = "_blocking";
    }
    str += &format!("        let mut buf = vec![]; self.write_body{blocking}(&mut buf){await_}?;\n");
    str += &format!("        let mut size = self.write_header{blocking}(w, buf.len() as u64){await_}?;\n");
    str += &format!("        w.write_all(&buf){await_}?; size += buf.len();\n");
    str += &format!("        Ok(size)\n");
    str += &format!("    }}\n");
    str
}

fn impl_write_header(struct_: &Box<ebml::EBMLStruct>, async_: bool) -> String {
    let mut str = format!("");
    let mut await_ = ".await";
    let mut blocking = "";
    if async_ {
        str += &format!("    pub async fn write_header<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {{\n");
    } else {
        str += &format!("    pub fn write_header_blocking<W: std::io::Write>(&self, w: &mut W, size: u64) -> Result<usize, anyhow::Error> {{\n");
        await_ = "";
        blocking = "_blocking";
    }
    str += &format!("        Ok({}?)\n", to_async(async_, &format!("write_element_id_size(w, EbmlId::{} as u64, size)", struct_.type_name())));
    str += &format!("    }}\n");
    str
}

fn impl_write_body(struct_: &Box<ebml::EBMLStruct>, async_: bool) -> String {
    let mut str = format!("");
    let mut await_ = ".await";
    let mut blocking = "";
    if async_ {
        str += &format!("    pub async fn write_body<W: tokio::io::AsyncWrite + Send + Unpin>(&self, w: &mut W) -> Result<usize, anyhow::Error> {{\n");
    } else {
        str += &format!("    pub fn write_body_blocking<W: std::io::Write>(&self, w: &mut W) -> Result<usize, anyhow::Error> {{\n");
        await_ = "";
        blocking = "_blocking";
    }
    str += &format!("        let mut size = 0usize;\n");
    if !struct_.children.is_empty() {
    str += &format!("        for el in self.elements() {{\n");
    str += &format!("            size += match el {{\n");
    for child in &struct_.children {
        let id = format!("EbmlId::{} as u64", child.element.id_enum());
    str += &format!("                {}Fields::{}(val) => {}?,\n", struct_.type_name(), child.element.type_name(), write_code(async_, &child.element.type_, &id, &format!("val.v")));
    }
    str += &format!("            }}\n");
    str += &format!("        }}\n");
    }
    // for child in &struct_.children {
    //     let name = child.element.var_name();
    //     match child.element.attr {
    //         TypeAttr::Optional => {
    //             str += &format!("        if let Some(val) = &self.{name} {{ size += {}?; }}\n", write_code(&child.element.type_, "val.v"));
    //         },
    //         TypeAttr::Required => {
    //             str += &format!("        size += {}?;\n", write_code(&child.element.type_, &format!("self.{name}.v")));
    //         },
    //         TypeAttr::Repeated => {
    //             str += &format!("        for val in &self.{name} {{ size += {}?; }};\n", write_code(&child.element.type_, &format!("val.v")));
    //         }
    //     }
    //     // str += &format!("        let mut {}: VecDeque<{}> = VecDeque::new();\n", child.element.var_name(), type_name);
    // }
    // str += &format!("\n");
    // str += &format!("        let mut all_size = 0;\n");
    // str += &format!("        while all_size < size {{\n");
    // str += &format!("            let (id, size, header_len) = read_element_id_size(r)?;\n");
    // str += &format!("            all_size += size + header_len;\n");
    // str += &format!("            match id {{\n");
    // for child in &struct_.children {
    //     let type_name = child.element.type_name();
    //     let read_code = child.element.type_.read_native(type_name);
    //     str += &format!("                EbmlId::{} => {}.push_back({read_code}),\n", child.element.type_name(), child.element.var_name());
    // }
    // str += &format!("                id => Err(anyhow::anyhow!(\"unexpected element id '{{:?}}' in '{}'\", id))?,\n", struct_.type_name());
    // str += &format!("            }}\n");
    // str += &format!("        }}\n");
    // str += &format!("\n");
    // for child in &struct_.children {
    //     match child.element.attr {
    //         TypeAttr::Optional => {
    //             str += &format!("        if {}.len() > 1 {{ Err(anyhow::anyhow!(\"Only zero or one element '{}' in '{}' possible. Found {{}}\", {}.len()))? }}\n",
    //                             child.element.var_name(),
    //                             child.element.type_name(),
    //                             struct_.type_name(),
    //                             child.element.var_name()
    //             );
    //             str += &format!("        let {} = {}.pop_front();\n", child.element.var_name(), child.element.var_name());
    //         },
    //         TypeAttr::Required => {
    //             if let Some(default) = &child.element.default {
    //                 let default = match child.element.type_ {
    //                     ElementType::UInteger => format!("{default}"),
    //                     ElementType::Integer => format!("{default}"),
    //                     ElementType::Float => format!("hexf::hexf64!(\"{default}\")"),
    //                     ElementType::String | ElementType::Utf8 => format!("\"{default}\".to_string()"),
    //                     ElementType::Binary => format!("{default}"),
    //                     ElementType::Date => format!("{default}"),
    //                     ElementType::Struct => format!("{default}"),
    //                 };
    //                 str += &format!("        if {}.len() == 0 {{ {}.push_back({default}); }}\n", child.element.var_name(), child.element.var_name());
    //             }
    //             str += &format!("        if {}.len() != 1 {{ Err(anyhow::anyhow!(\"One element '{}' must be in '{}'. Found {{}}\", {}.len()))? }}\n",
    //                             child.element.var_name(),
    //                             child.element.type_name(),
    //                             struct_.type_name(),
    //                             child.element.var_name()
    //             );
    //             str += &format!("        let {} = {}.pop_front().ok_or_else(|| anyhow::anyhow!(\"Required element '{}' doesn't exist in '{}'\"))?;\n",
    //                             child.element.var_name(), child.element.var_name(),
    //                             child.element.type_name(),
    //                             struct_.type_name(),
    //             );
    //         },
    //         TypeAttr::Repeated => {
    //             str += &format!("        let {} = Vec::from({});\n", child.element.var_name(), child.element.var_name());
    //         },
    //     }
    // }
    str += &format!("        Ok(size)\n");
    str += &format!("    }}\n");
    str
}

fn write_code(async_: bool, type_: &ElementType, id: &str, name: &str) -> String {
    if async_ {
        match type_ {
            ElementType::UInteger => format!("async_::write_el_uint(w, {id}, &*{name}).await"),
            ElementType::Integer => format!("async_::write_el_int(w, {id}, &*{name}).await"),
            ElementType::Float => format!("async_::write_el_float64(w, {id}, &*{name}).await"),
            ElementType::String => format!("async_::write_el_string(w, {id}, &{name}).await"),
            ElementType::Utf8 => format!("async_::write_el_utf8(w, {id}, &{name}).await"),
            ElementType::Binary => format!("async_::write_el_bin(w, {id}, &{name}).await"),
            ElementType::Date => format!("async_::write_el_date(w, {id}, &{name}).await"),
            ElementType::Struct => format!("{name}.write(w).await"),
        }
    } else {
        match type_ {
            ElementType::UInteger => format!("blocking::write_el_uint(w, {id}, &*{name})"),
            ElementType::Integer => format!("blocking::write_el_int(w, {id}, &*{name})"),
            ElementType::Float => format!("blocking::write_el_float64(w, {id}, &*{name})"),
            ElementType::String => format!("blocking::write_el_string(w, {id}, &{name})"),
            ElementType::Utf8 => format!("blocking::write_el_utf8(w, {id}, &{name})"),
            ElementType::Binary => format!("blocking::write_el_bin(w, {id}, &{name})"),
            ElementType::Date => format!("blocking::write_el_date(w, {id}, &{name})"),
            ElementType::Struct => format!("{name}.write_blocking(w)"),
        }
    }
}
