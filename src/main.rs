use std::{
    io::{self, Read, Write, prelude::*},
    fs::File,
    env
};

use flate2::Compression;
use flate2::write::ZlibEncoder;
use flate2::read::GzDecoder;

fn main()
{
    //let _ = compress();
    let _ = uncompress();
}

fn compress() -> Result<(), io::Error>
{    
    let mut input = File::open("./file.svg").unwrap();
    let mut output = File::create("./file.svgz").unwrap();

    let mut encoder = ZlibEncoder::new(Vec::new(), Compression::default());

    io::copy(&mut input, &mut encoder)?;
    let compressed_data = encoder.finish()?;

    output.write_all(&compressed_data)?;

    Ok(())
}

fn uncompress() -> Result<(), io::Error>
{
    let compressed_data = std::fs::read("./file.svgz")?;

    let mut decoder = GzDecoder::new(compressed_data.as_slice());

    let mut output = File::create("./file.test.svg")?;

    io::copy(&mut decoder, &mut output)?;

    Ok(())
}
