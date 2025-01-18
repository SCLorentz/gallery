use std::path::PathBuf;

use druid::{
    commands::{OPEN_FILE, SHOW_OPEN_PANEL}, widget::{Button, Flex, Label}, AppLauncher, BoxConstraints, Data, Env, Event, EventCtx, FileDialogOptions, ImageBuf, LayoutCtx, Lens, LifeCycle, LifeCycleCtx, PaintCtx, RenderContext, Size, UpdateCtx, Widget, WidgetExt, WindowDesc
};

use if_empty::*;

mod mac;
use mac::*;

mod encoder;
use encoder::*;

mod settings;
//use settings::*;

mod image_handler;
use image_handler::*;

#[derive(Clone, Data, Lens)]
struct AppState
{
    // TODO: Add support for multiple files
    selected_file: String,
    image: Option<ImageBuf>,
    mode: String,
}

fn main()
{
    let main_window = WindowDesc::new(build_ui())
        .title("Gallery")
        .window_size((400.0, 400.0))
        .with_min_size((400.0, 400.0))
        .menu(make_menu);

    let initial_state = AppState
    {
        selected_file: "".to_string(),
        mode: "".to_string(),
        image: None,
    };

    AppLauncher::with_window(main_window)
        .launch(initial_state)
        .expect("Falha ao lançar a aplicação");
}

fn build_ui() -> impl Widget<AppState>
{
    let open_file_button = Button::new("Import File(s)").on_click(|ctx, _data: &mut AppState, _env|
    {
        let options = FileDialogOptions::new()
            .name_label("Arquivo")
            .title("Selecione um arquivo")
            .multi_selection()
            .button_text("Import");

        ctx.submit_command(SHOW_OPEN_PANEL.with(options));
    });

    let file_label = Label::new(|data: &AppState, _env: &_|
    {
        format!(
            "Import File: {}",
            data.selected_file.clone().if_empty("Nenhum arquivo".to_string())
        )
    });

    let image_widget = DynamicImage::new().center();

    Flex::column()
        .with_child(open_file_button.padding(10.0))
        .with_child(file_label.padding(10.0))
        .with_child(image_widget.padding(10.0))
        .controller(FileSelectionController)
}

struct FileSelectionController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for FileSelectionController
{
    fn event(
        &mut self,
        child: &mut W,
        ctx: &mut EventCtx,
        event: &Event,
        data: &mut AppState,
        env: &Env,
    ) {
        if let Event::Command(cmd) = event
        {
            let Some(file_info) = cmd.get(OPEN_FILE) else
            {
                eprintln!("couldn't open file");
                return;
            };

            let Some(path) = file_info.path().to_str() else { return; };

            data.selected_file = path.to_string();

            let path = PathBuf::from(&data.selected_file);

            match path.extension().and_then(|ext| ext.to_str()).unwrap_or("")
            {
                "gz" =>
                {
                    let Ok(buffer) = decode(path) else
                    {
                        eprintln!("Erro ao descompactar arquivo");
                        return;
                    };

                    if buffer.is_empty()
                    {
                        eprintln!("Buffer descompactado está vazio!");
                        return;
                    }

                    let Ok(image_buf) = load_image_compressed(&buffer) else
                    {
                        eprintln!("Erro ao carregar imagem do buffer");
                        return;
                    };

                    data.image = Some(image_buf);
                },
                _ => encode(path).unwrap_or_else(|err| println!("Erro ao comprimir: {}", err)),
            }
        }

        child.event(ctx, event, data, env);
    }
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
        if let Some(image) = &self.image
        {
            let size = image.size();
            let rect = size.to_rect();

            if let Ok(core_graphics_image) = ctx.make_image(image.width(), image.height(), &image.raw_pixels(), image.format())
            {
                ctx.draw_image(&core_graphics_image, rect, druid::piet::InterpolationMode::Bilinear);
            }
        }
    }
}
