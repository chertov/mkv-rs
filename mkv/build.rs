
fn main() -> Result<(), anyhow::Error> {
    let dir = std::env::var("CARGO_MANIFEST_DIR")?;
    let mut path = std::path::Path::new(&dir)
        .join("src")
        .join("gen");

    mkv_codegen::run(&path)?;
    Ok(())
}
