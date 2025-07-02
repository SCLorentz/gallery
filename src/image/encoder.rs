use std::{
    fs::{self, File},
    io::{self, Read, Write},
    path::PathBuf,
};

use flate2::{read::GzDecoder, write::GzEncoder, Compression};

pub fn encode(path: PathBuf, content: Vec<u8>) -> io::Result<()>
{
    let mut encoder = GzEncoder::new(Vec::new(), Compression::best());
    encoder.write_all(&content)?;
    let compressed_bytes = encoder.finish()?;

    let ext = path.extension()
        .and_then(|e| e.to_str())
        .unwrap_or("");
    let new_path = path.with_extension(format!("{ext}z"));

    let mut output = File::create(new_path)?;
    output.write_all(&compressed_bytes)?;

    Ok(())
}

pub fn decode(content: Vec<u8>) -> io::Result<Vec<u8>> 
{
    let mut decoder = GzDecoder::new(content.as_slice());
    let mut buff = Vec::new();
    decoder.read_to_end(&mut buff)?;
    Ok(buff)
}

#[allow(unused)]
pub fn export_to_file(path: PathBuf) -> io::Result<()> 
{
    let content = fs::read(&path)?;

    let mut decoder = GzDecoder::new(content.as_slice());
    let mut buff = Vec::new();
    decoder.read_to_end(&mut buff)?;

    let new_path = path.with_extension("");

    let mut output = File::create(new_path)?;
    output.write_all(&buff)?;

    Ok(())
}