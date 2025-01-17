use std::path::PathBuf;

use druid::{
    commands::{OPEN_FILE, SHOW_OPEN_PANEL}, widget::{Button, Flex, Image, Label}, AppLauncher, Data, Env, Event, EventCtx, FileDialogOptions, Lens, Widget, WidgetExt, WindowDesc
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
    selected_file: String,
    mode: String,
}

fn main()
{
    let main_window = WindowDesc::new(build_ui())
        .title("Vizualizador de Imagens")
        .window_size((400.0, 250.0))
        .with_min_size((400.0, 400.0))
        .menu(make_menu);

    let initial_state = AppState
    {
        selected_file: "".to_string(),
        mode: "".to_string(),
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
            .button_text("Abrir");

        ctx.submit_command(SHOW_OPEN_PANEL.with(options));
    });

    let file_label = Label::new(|data: &AppState, _env: &_|
    {
        format!(
            "Importar Arquivo: {}",
            data.selected_file.clone().if_empty("Nenhum arquivo".to_string())
        )
    });

    let image_buf = load_image("./default.jpg").unwrap_or_else(|err|
    {
        eprintln!("{}", err);
        druid::ImageBuf::empty()
    });

    let image_widget = Image::new(image_buf)
        .fix_width(300.0)
        .fix_height(300.0)
        .center();

    Flex::column()
        .with_child(open_file_button.padding(10.0))
        .with_child(file_label.padding(10.0))
        .with_child(image_widget.padding(10.0))
        .controller(FileSelectionController)
}

struct FileSelectionController;

impl<W: Widget<AppState>> druid::widget::Controller<AppState, W> for FileSelectionController {
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
            let Some(file_info) = cmd.get(OPEN_FILE) else { panic!("couldn't open file") };

            if let Some(path) = file_info.path().to_str()
            {
                data.selected_file = path.to_string();

                let path = PathBuf::from(&data.selected_file);

                match path.extension().and_then(|ext| ext.to_str()).unwrap_or("")
                {
                    "gz" => decode(path).unwrap_or_else(|err| println!("Erro ao descomprimir: {}", err)),
                    _ => encode(path).unwrap_or_else(|err| println!("Erro ao comprimir: {}", err)),
                }
            }
        }

        child.event(ctx, event, data, env);
    }
}