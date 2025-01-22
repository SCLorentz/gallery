use std::path::PathBuf;

use druid::{piet::ImageFormat, BoxConstraints, Data, Env, Event, EventCtx, ImageBuf, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Widget};

use image::{ImageBuffer, Rgba};

use crate::AppState;

pub mod encoder;

pub fn load_image_compressed(buffer: &[u8]) -> Result<ImageBuf, String>
{
    if buffer.is_empty() { return Err("O buffer está vazio!".to_string()) }

    let dynamic_image = image::load_from_memory(buffer)
        .map_err(|err| format!("Erro ao carregar a imagem: {}", err))?
        .to_rgba8();

    let (width, height) = dynamic_image.dimensions();

    if width == 0 || height == 0 { return Err("invalid image size".to_string()) }

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

fn druid_buff(dynamic_image: ImageBuffer<Rgba<u8>, Vec<u8>>, width: u32, height: u32) -> druid::ImageBuf
{
    druid::ImageBuf::from_raw(
        dynamic_image.into_raw(),
        ImageFormat::RgbaSeparate,
        width as usize,
        height as usize,
    )
}

pub struct DynamicImage
{
    image: Option<ImageBuf>,
}

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

    fn lifecycle(
        &mut self,
        ctx: &mut LifeCycleCtx,
        event: &LifeCycle,
        data: &AppState,
        _env: &Env,
    ) {
        if let LifeCycle::WidgetAdded = event
        {
            self.image = data.image.clone();
            ctx.request_paint();
        }
    }

    fn update(
        &mut self,
        ctx: &mut UpdateCtx,
        old_data: &AppState,
        data: &AppState,
        _env: &Env,
    ) {
        if !old_data.image.same(&data.image)
        {
            self.image = data.image.clone();
            ctx.request_paint();
        }
    }

    fn layout(
        &mut self,
        _ctx: &mut LayoutCtx,
        bc: &BoxConstraints,
        _data: &AppState,
        _env: &Env,
    ) -> Size {
        bc.constrain((300.0, 300.0))
    }

    fn paint(&mut self, ctx: &mut PaintCtx, _data: &AppState, _env: &Env)
    {
        let Some(image) = &self.image else { return };

        let size = ctx.size();
        let image_size = image.size();
        let aspect_ratio = image_size.width / image_size.height;
        
        let new_width;
        let new_height;

        if size.width / aspect_ratio <= size.height
        {
            new_width = size.width;
            new_height = size.width / aspect_ratio;
        }
        else
        {
            new_width = size.height * aspect_ratio;
            new_height = size.height;
        }

        let rect = druid::kurbo::Rect::from_origin_size((0.0, 0.0), (new_width, new_height));

        let Ok(core_graphics_image) = ctx.make_image(image.width(), image.height(), &image.raw_pixels(), image.format()) else { return };
        
        ctx.draw_image(&core_graphics_image, rect, druid::piet::InterpolationMode::Bilinear);
    }
}