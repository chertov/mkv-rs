#[macro_use] extern crate log;

use anyhow::Context;
use std::io::Seek;

use mkv::ElementType;

fn dump<R: std::io::Read + std::io::Seek>(tabs: u32, r: &mut R, el: &mkv::element::Element) -> Result<(), anyhow::Error> {
    let indent = indent(tabs);

    // if el.id == EbmlId::SimpleBlock { std::process::exit(0); }
    match el.type_ {
        ElementType::UInteger |
        ElementType::Integer |
        ElementType::Float |
        ElementType::String |
        ElementType::Utf8 |
        ElementType::Date => {
            let content = el.read_body_blocking(r).context(format!("Failed to read body of {el:?}"))?;
            debug!("{indent}{:?}: {:?}", el.id, content);
        }
        ElementType::Binary => {
            el.skip_blocking(r).context("Failed skip")?;
            debug!("{indent}{:?}({:?}): len: {:?}", el.id, el.type_, el.size);
        }
        ElementType::Struct => {
            debug!("{indent}{:?}:", el.id);
            let start = r.stream_position()?;
            loop {
                let pos = r.stream_position()?;
                if start + el.size.unwrap() <= pos { break; }
                let el = mkv::element::Element::read_header_blocking(r)?;
                dump(tabs + 1, r, &el)?;
            }
        }
    }
    Ok(())
}


fn main() -> Result<(), anyhow::Error> {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let input = "./movies/sample1.mkv";

    let meta = std::fs::metadata(input).context(format!("Failed metadata '{input}'"))?;
    let size = meta.len();

    let mut input = std::fs::File::open(input).context(format!("Failed open '{input}'"))?;

    loop {
        let pos = input.stream_position().context("Failed stream_position")?;
        // debug!("pos {pos}, remain {}", size - pos);
        if size - pos == 0 { break; }
        let el = mkv::element::Element::read_header_blocking(&mut input)?;
        dump(0, &mut input, &el)?;
    }

    Ok(())
}

fn indent(tabs: u32) -> String {
    let mut str = format!("");
    for _ in 0 .. tabs { str += "    "; }
    str
}

