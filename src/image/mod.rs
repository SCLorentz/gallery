use std::path::PathBuf;

use druid::{piet::ImageFormat, BoxConstraints, Data, Env, Event, EventCtx, ImageBuf, LayoutCtx, LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Widget};

use resvg::usvg::{Options, Path, Tree};
use tiny_skia::Pixmap;

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

    Ok(ImageBuf::from_raw(
        dynamic_image.into_raw(),
        ImageFormat::RgbaSeparate,
        width as usize,
        height as usize,
    ))
}

// TODO: Test if this works
#[allow(unused)]
pub fn render_svg_to_imagebuf(svg_path: &str, width: u32, height: u32) -> Result<ImageBuf, String>
{
    let opt = Options::default();
    let svg_data = std::fs::read(svg_path).map_err(|err| format!("Erro ao ler o arquivo SVG: {}", err))?;
    let tree = Tree::from_data(&svg_data, &opt).map_err(|err| format!("Erro ao carregar SVG: {}", err))?;

    let mut pixmap = Pixmap::new(width, height).ok_or("Falha ao criar o Pixmap")?;

    resvg::render(
        &tree,
        resvg::usvg::FitTo::Size(width, height),
        tiny_skia::Transform::default(),
        pixmap.as_mut()
    )
    .ok_or("Falha ao rasterizar o SVG")?;

    Ok(ImageBuf::from_raw(
        pixmap.data().to_vec(),
        ImageFormat::RgbaSeparate,
        width as usize,
        height as usize,
    ))
}

pub fn load_image(path: PathBuf) -> Result<druid::ImageBuf, String>
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