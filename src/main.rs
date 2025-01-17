use std::{
    io::prelude::*,
    fs::{self, File}
};

use flate2::Compression;
use flate2::read::GzDecoder;
use flate2::write::GzEncoder;

fn main()
{
    match std::env::args().nth(1)
    {
        Some(val) if val == "encode".to_string() => encode().unwrap_or_else(|err| eprintln!("{}", err)),
        _ => decode().unwrap_or_else(|err| eprintln!("{}", err)),
    }
}

fn encode() -> Result<(), std::io::Error>
{
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    let content = fs::read("./file.svg")?;
    encoder.write_all(&content)?;

    let compressed_bytes = encoder.finish()?;

    let mut output = File::create("./file.svgz")?;
    output.write_all(&compressed_bytes)?;

    Ok(())
}

fn decode() -> Result<(), std::io::Error>
{
    let content = fs::read("./file.svgz")?;

    let mut d = GzDecoder::new(content.as_slice());

    let mut buff = Vec::new();

    d.read_to_end(&mut buff)?;

    let mut output = File::create("./file.svg")?;
    output.write_all(&buff)?;

    Ok(())
}