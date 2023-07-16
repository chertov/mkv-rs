#[macro_use] extern crate log;

use std::io::Seek;
use anyhow::Context;

use mkv::{ElementReadBlocking, ElementWriteBlocking};

fn main() -> Result<(), anyhow::Error> {
    let env = env_logger::Env::default()
        .filter_or("MY_LOG_LEVEL", "debug")
        .write_style_or("MY_LOG_STYLE", "always");
    env_logger::init_from_env(env);

    let input = "./movies/sample1.mkv";
    let output = "./out.mkv";

    remux(input, output).context("Failed remux")?;

    Ok(())
}

fn read(input: &str) -> Result<(), anyhow::Error> {
    let meta = std::fs::metadata(input).context(format!("Failed metadata '{input}'"))?;
    let size = meta.len();
    let mut input = std::fs::File::open(input).context(format!("Failed open '{input}'"))?;
    // let mut f = std::io::BufReader::new(input);
    // let mut f = std::io::BufReader::new(input);

    // loop {
    //     let pos = input.stream_position().context("Failed stream_position")?;
    //     // debug!("pos {pos}, remain {}", size - pos);
    //     if size - pos == 0 { break; }
    //     match mkv::parser::Element::parse(&mut input) {
    //         Ok(el) => {
    //             debug!("el {:?}", el);
    //             el.skip(&mut input).context("Failed skip")?;
    //             // input.seek(std::io::SeekFrom::Current(el.header_len as i64)).context("Failed seek")?;
    //             // if el.type_ == mkv::ElementType::Struct {
    //             //     continue;
    //             // } else {
    //             //
    //             // };
    //             // let new_pos = input.seek(std::io::SeekFrom::Current(el.size as i64)).context("Failed seek")?;
    //             // debug!("new_pos {new_pos}, seek: {}, remain {}", el.size as i64, size - new_pos);
    //         }
    //         Err(e) => {
    //             error!("err {:?}", e);
    //             break;
    //         }
    //     }
    // }

    let (header, _) = mkv::structs::EbmlHeader::read(&mut input).context("Failed EbmlHeader::read")?;
    debug!("header {:#?}", header);
    let (size, _) = mkv::structs::Segment::read_header(&mut input).context("Failed Segment::read_header")?;
    debug!("Segment size {size:?}");
    let (seek_head, _) = mkv::structs::SeekHead::read(&mut input).context("Failed SeekHead::read")?;
    debug!("seek_head {seek_head:#?}");
    let void = mkv::element::Element::read_header_blocking(&mut input).context("Failed void Element::read")?;
    void.skip_blocking(&mut input).context("Failed skip void")?;
    debug!("void {void:#?}");
    let (info, _) = mkv::structs::Info::read(&mut input).context("Failed Info::read")?;
    debug!("info {info:#?}");
    let (tracks, _) = mkv::structs::Tracks::read(&mut input).context("Failed Tracks::read")?;
    debug!("tracks {tracks:?}");
    let void = mkv::element::Element::read_header_blocking(&mut input).context("Failed void Element::read")?;
    void.skip_blocking(&mut input).context("Failed skip void")?;
    debug!("void {void:?}");
    let (attachments, _) = mkv::structs::Attachments::read(&mut input).context("Failed Attachments::read")?;
    debug!("attachments {:?}", attachments.attached_file.len());
    for _ in 0..1000 {
        let pos = input.stream_position().context("Failed stream_position")?;
        if size.unwrap() - pos == 0 { break; }

        let (cluster, _) = mkv::structs::Cluster::read(&mut input).context("Failed Cluster::read")?;
        debug!("cluster {:?}", cluster.timestamp);
        debug!("cluster {:?}", cluster.elements().len());
        // for simple_block in cluster.simple_block {
        //     debug!("    simple_block {} {:0x?}", simple_block.v.len(), &simple_block.v[0..10]);
        // }
        // for block_group in cluster.block_group {
        //     debug!("    block_group {:?}", block_group);
        // }
    }

    Ok(())
}


fn remux(input: &str, output: &str) -> Result<(), anyhow::Error> {
    // let mut output = std::fs::File::create(output).context(format!("Failed to create '{output}'"))?;

    let meta = std::fs::metadata(input).context(format!("Failed metadata '{input}'"))?;
    let size = meta.len();
    debug!("size {size}");
    let mut input = std::fs::File::open(input).context(format!("Failed open '{input}'"))?;
    // let mut f = std::io::BufReader::new(input);
    // let mut f = std::io::BufReader::new(input);

    // loop {
    //     let pos = input.stream_position().context("Failed stream_position")?;
    //     // debug!("pos {pos}, remain {}", size - pos);
    //     if size - pos == 0 { break; }
    //     match mkv::readr::Element::read(&mut input) {
    //         Ok(el) => {
    //             debug!("el {:?}", el);
    //             el.skip(&mut input).context("Failed skip")?;
    //             // input.seek(std::io::SeekFrom::Current(el.header_len as i64)).context("Failed seek")?;
    //             // if el.type_ == mkv::ElementType::Struct {
    //             //     continue;
    //             // } else {
    //             //
    //             // };
    //             // let new_pos = input.seek(std::io::SeekFrom::Current(el.size as i64)).context("Failed seek")?;
    //             // debug!("new_pos {new_pos}, seek: {}, remain {}", el.size as i64, size - new_pos);
    //         }
    //         Err(e) => {
    //             error!("err {:?}", e);
    //             break;
    //         }
    //     }
    // }

    let (header, _) = mkv::structs::EbmlHeader::read(&mut input).context("Failed EbmlHeader::read")?;
    debug!("header {:#?}", header);
    // debug!("header write {len} bytes");

    let size = mkv::structs::Segment::read_header(&mut input).context("Failed Segment::read_header")?;
    debug!("Segment size {size:?}");

    let (seek_head, _) = mkv::structs::SeekHead::read(&mut input).context("Failed SeekHead::read")?;
    // debug!("seek_head {seek_head:#?}");
    // debug!("seek_head write {len} bytes");

    let void = mkv::element::Element::read_header_blocking(&mut input).context("Failed void Element::read")?;
    void.skip_blocking(&mut input).context("Failed skip void")?;
    // debug!("void {void:#?}");
    let (info, _) = mkv::structs::Info::read(&mut input).context("Failed Info::read")?;
    // debug!("info {info:#?}");

    let (tracks, _) = mkv::structs::Tracks::read(&mut input).context("Failed Tracks::read")?;
    // debug!("tracks {tracks:?}");

    let void = mkv::element::Element::read_header_blocking(&mut input).context("Failed void Element::read")?;
    void.skip_blocking(&mut input).context("Failed skip void")?;
    // debug!("void {void:?}");
    // attachments.write(&mut output).context("Failed Attachments::write")?;

    let (attachments, _) = mkv::structs::Attachments::read(&mut input).context("Failed Attachments::read")?;
    debug!("attachments {:?}", attachments.attached_file.len());
    // attachments.write(&mut output).context("Failed Attachments::write")?;

    let mut output = std::fs::File::create(output).context(format!("Failed to create '{output}'"))?;
    header.write_blocking(&mut output).context("Failed EbmlHeader::write")?;

    for _ in 0..100 {
        let mut index = 0;
        let mut segment = mkv::structs::Segment::default();
        segment.seek_head.push(mkv::Ebml::new_index(index, seek_head.clone())); index+=1;
        segment.info = mkv::Ebml::new_index(index, info.clone()); index+=1;
        segment.tracks.replace(mkv::Ebml::new_index(index, tracks.clone())); index+=1;
        // let mut last_timestamp = 0;
        for _ in 0..10 {
            let (mut cluster, _) = mkv::structs::Cluster::read(&mut input).context("Failed Cluster::read")?;
            // *cluster.timestamp.v = 0;
            segment.cluster.push(mkv::Ebml::new_index(index, cluster)); index+=1;
            // last_timestamp = *cluster.timestamp.v;
            // debug!("cluster {:?} {}", cluster.timestamp, cluster.elements().len());
            // debug!("cluster {:?}", cluster.timestamp.v);

            // let len = cluster.write(&mut buf).context("Failed Cluster::write")?;
            // debug!("cluster {} write {len} bytes", cluster.timestamp.v);

            // for el in cluster.elements() {
            //     debug!("    el {:?}", el);
            // }


            // for simple_block in cluster.simple_block {
            //     debug!("    simple_block {} {:0x?}", simple_block.v.len(), &simple_block.v[0..10]);
            // }
            // for block_group in cluster.block_group {
            //     debug!("    block_group {:?}", block_group);
            // }
        }
        // segment.seek_head.push(mkv::Ebml::new_index(0, seek_head.clone()));
        // segment.seek_head.push(mkv::Ebml::new_index(34, seek_head.clone()));
        //
        // let elements = segment.elements();
        // for el in elements {
        //     debug!("el {:?}", el.index());
        // }

        segment.write_blocking(&mut output).context("Failed Segment::write")?;

        // let len = segment.write_header(&mut output, buf.len() as u64).context("Failed Segment::write_header")?;
        // output.write_all(&buf).context("Failed write_all")?;
        // debug!("segment write_header {len} bytes");
    }

    Ok(())
}
