use std::{
    fs::{self, File},
    io::{Read, Write, Result},
    path::PathBuf,
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};

pub fn encode(path: PathBuf, content: Vec<u8>) -> Result<()>
{
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&content)?;

    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");

    Ok(save_on_path(
        path.with_extension(format!("{ext}z")),
        encoder.finish()?
    )?)
}

fn save_on_path(path: PathBuf, content: Vec<u8>) -> Result<()>
{
    let mut output = File::create(path)?;
    output.write_all(&content)?;
    Ok(())
}

pub fn decode(content: Vec<u8>) -> Result<Vec<u8>> 
{
    let mut decoder = GzDecoder::new(content.as_slice());
    let mut buff = Vec::new();
    decoder.read_to_end(&mut buff)?;
    Ok(buff)
}

#[allow(unused)]
pub fn export_to_file(path: PathBuf) -> Result<()> 
{
    let content = fs::read(&path)?;

    let mut decoder = GzDecoder::new(content.as_slice());
    let mut buff = Vec::new();
    decoder.read_to_end(&mut buff)?;
    save_on_path(path.with_extension(""), buff)?;

    Ok(())
}