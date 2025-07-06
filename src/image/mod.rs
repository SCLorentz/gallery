use std::path::PathBuf;

use druid::{
    piet::ImageFormat, BoxConstraints, Data, Env, Event, EventCtx, ImageBuf, LayoutCtx,
    LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Widget,
};

use image::{ImageBuffer, Rgba};
use crate::AppState;
//pub mod encoder;

pub fn load_image_compressed(buffer: &[u8]) -> Result<ImageBuf, String>
{
    buffer.is_empty()
        .then(|| Err::<(), _>(format!("Empty Buffer!")))
        .transpose()?;

    let dynamic_image = image::load_from_memory(buffer)
        .map_err(|err| format!("Error loading image: {}", err))?
        .to_rgba8();

    let (width, height) = dynamic_image.dimensions();

    if width == 0 || height == 0 { return Err(format!("Invalid image size")) }

    Ok(druid_buff(dynamic_image, width, height))
}

pub fn load_image(path: PathBuf) -> Result<druid::ImageBuf, String>
{
    let dynamic_image = image::open(path)
        .map_err(|err| format!("Erro ao abrir a imagem: {}", err))?
        .to_rgba8();

    let (width, height) = dynamic_image.dimensions();
    Ok(druid_buff(dynamic_image, width, height))
}

fn druid_buff(img: ImageBuffer<Rgba<u8>, Vec<u8>>, w: u32, h: u32) -> druid::ImageBuf
{
    druid::ImageBuf::from_raw(img.into_raw(), ImageFormat::RgbaSeparate, w as usize, h as usize)
}

pub struct DynamicImage { image: Option<ImageBuf> }

impl DynamicImage
{
    pub fn new() -> Self
    {
        Self { image: None }
    }
}

// I used AI assistence in here
impl Widget<AppState> for DynamicImage
{
    fn event(&mut self, _ctx: &mut EventCtx, _event: &Event, _data: &mut AppState, _env: &Env) {}

    fn lifecycle(&mut self, ctx: &mut LifeCycleCtx, event: &LifeCycle, data: &AppState, _env: &Env)
    {
        if let LifeCycle::WidgetAdded = event
        {
            self.image = data.image.clone();
            ctx.request_paint();
        }
    }

    fn update(&mut self, ctx: &mut UpdateCtx, old_data: &AppState, data: &AppState, _env: &Env)
    {
        if old_data.image.same(&data.image) { return; }

        self.image = data.image.clone();
        ctx.request_paint();
    }

    fn layout(&mut self, _ctx: &mut LayoutCtx, bc: &BoxConstraints, _data: &AppState, _env: &Env) -> Size { bc.constrain((300.0, 300.0)) }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppState, _env: &Env)
    {
        use druid::{kurbo::Rect, piet::InterpolationMode};
        let Some(image) = &self.image else { return };

        let (size, image_size) = (ctx.size(), image.size());
        let ratio = image_size.width / image_size.height;

        let (new_width, new_height) =
        if size.width / ratio <= size.height {(size.width, size.width / ratio)}
        else {(size.height * ratio, size.height)};

        let rect = Rect::from_origin_size((0.0, 0.0), (new_width, new_height));

        let Ok(core_graphics_image) = ctx.make_image(image.width(), image.height(), &image.raw_pixels(), image.format()) else { return };
        
        ctx.draw_image(&core_graphics_image, rect, InterpolationMode::Bilinear);
    }
}