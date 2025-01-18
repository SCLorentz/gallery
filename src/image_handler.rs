use druid::{piet::ImageFormat, BoxConstraints, Data, Env, Event, EventCtx, ImageBuf, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Widget};

use crate::AppState;

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

        let size = image.size();
        let rect = size.to_rect();

        let Ok(core_graphics_image) = ctx.make_image(image.width(), image.height(), &image.raw_pixels(), image.format()) else { return };
        
        ctx.draw_image(&core_graphics_image, rect, druid::piet::InterpolationMode::Bilinear);
    }
}