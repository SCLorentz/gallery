use druid::piet::ImageFormat;

pub fn load_image(path: &str) -> Result<druid::ImageBuf, String>
{
    let dynamic_image = image::open(path)
        .map_err(|err| format!("Erro ao abrir a imagem: {}", err))?
        .to_rgba8();

    let (width, height) = dynamic_image.dimensions();
    Ok(druid::ImageBuf::from_raw(
        dynamic_image.into_raw(),
        ImageFormat::RgbaSeparate,
        width as usize,
        height as usize,
    ))
}