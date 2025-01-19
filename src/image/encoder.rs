use std::{
    fs::{self, File},
    io::prelude::*,
    path::PathBuf,
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};

pub fn encode(path: PathBuf) -> Result<(), std::io::Error>
{
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());

    let content = fs::read(path.clone())?;
    encoder.write_all(&content)?;

    let compressed_bytes = encoder.finish()?;

    let new_path = path.with_extension(format!("{}.gz", path.extension().unwrap().to_str().unwrap()));

    let mut output = File::create(new_path)?;
    output.write_all(&compressed_bytes)?;

    Ok(())
}

pub fn decode(path: PathBuf) -> Result<Vec<u8>, std::io::Error>
{
    let content = fs::read(path.clone())?;

    let mut decoder = GzDecoder::new(content.as_slice());
    let mut buff = Vec::new();

    decoder.read_to_end(&mut buff)?;

    Ok(buff)
}

// * decompresses and export the file to the selected path
#[allow(unused)]
pub fn export_to_file(path: PathBuf) -> Result<(), std::io::Error>
{
    let content = fs::read(path.clone())?;

    let mut decoder = GzDecoder::new(content.as_slice());
    let mut buff = Vec::new();

    decoder.read_to_end(&mut buff)?;

    let new_path = path.with_extension("");

    let mut output = File::create(new_path)?;
    output.write_all(&buff)?;

    Ok(())
}