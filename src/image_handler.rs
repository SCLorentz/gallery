use druid::{piet::ImageFormat, ImageBuf};

pub fn load_image_compressed(buffer: &[u8]) -> Result<ImageBuf, String>
{
    if buffer.is_empty() {
        return Err("O buffer está vazio!".to_string());
    }

    let dynamic_image = image::load_from_memory(buffer)
        .map_err(|err| format!("Erro ao carregar a imagem: {}", err))?
        .to_rgba8();

    let (width, height) = dynamic_image.dimensions();

    if width == 0 || height == 0 {
        return Err("Imagem carregada tem dimensões inválidas!".to_string());
    }

    Ok(ImageBuf::from_raw(
        dynamic_image.into_raw(),
        ImageFormat::RgbaSeparate,
        width as usize,
        height as usize,
    ))
}

/*pub fn load_image(path: &str) -> Result<druid::ImageBuf, String>
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
}*/