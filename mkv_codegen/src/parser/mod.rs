pub mod ebml;
pub mod matroska;

pub fn from_bool<'de, D>(deserializer: D) -> Result<bool, D::Error>
    where
        D: serde::Deserializer<'de>,
{
    use serde::Deserialize;
    use serde::de::Error;
    let val: Option<u32> = Deserialize::deserialize(deserializer)?;
    Ok(match val.unwrap_or(0) {
        0 => false,
        1 => true,
        val => return Err(D::Error::custom(format!("unknown value '{val}', must be \"0\" or \"1\""))),
    })
}

use convert_case::{Case, Casing};
pub fn var_name(name: &str) -> String { name.to_case(Case::Snake) }
pub fn type_name(name: &str) -> String { name.to_case(Case::UpperCamel) }
pub fn id_enum(name: &str) -> String { format!("{}", name.to_case(Case::Pascal)) }


