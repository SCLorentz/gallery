use std::path::PathBuf;

use druid::{
    commands::{OPEN_FILE, SHOW_OPEN_PANEL}, menu::MenuEventCtx, widget::{Button, Flex, Label}, AppLauncher, Data, Env, Event, EventCtx, FileDialogOptions, ImageBuf, Lens, Widget, WidgetExt, WindowDesc
};

use if_empty::*;

mod mac;
use mac::*;

mod settings;

mod image;

use image::{
    encoder::{decode, encode}, load_image, load_image_compressed, DynamicImage
};

#[derive(Clone, Data, Lens)]
struct AppState
{
    selected_file: String,
    image: Option<ImageBuf>,
    mode: String,
}

pub fn import_file(ctx: &mut MenuEventCtx)
{
    let options = FileDialogOptions::new()
        .name_label("Arquivo")
        .title("Selecione um arquivo")
        .multi_selection()
        .button_text("Import");

    ctx.submit_command(SHOW_OPEN_PANEL.with(options));
}

fn main()
{
    let main_window = WindowDesc::new(build_ui())
        .title("BHGallery")
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
    )
    {
        if let Event::Command(cmd) = event
        {
            assert!(cmd.get(OPEN_FILE).is_some(), "couldn't open file");
            let file_info = cmd.get(OPEN_FILE).unwrap();

            let path = match file_info.path().to_str()
            {
                Some(path) => path,
                None => return
            };

            data.selected_file = path.to_string();

            let path = PathBuf::from(&data.selected_file);
            let extension = path.extension().and_then(|ext| ext.to_str()).unwrap_or("");

            if extension != "gz"
            {
                assert!(encode(path.clone()).is_ok(), "Erro ao comprimir");

                match load_image(path)
                {
                    Ok(image_buf) => data.image = Some(image_buf),
                    Err(err) => eprintln!("Erro ao carregar imagem: {}", err),
                }

                return
            }

            let buffer = decode(path).unwrap_or_else(|err| {
                eprintln!("Erro ao descompactar arquivo: {}", err);
                Vec::new()
            });

            if buffer.is_empty() { return }

            let Ok(image_buf) = load_image_compressed(&buffer) else
            {
                eprintln!("Erro ao carregar imagem do buffer");
                return;
            };

            data.image = Some(image_buf);
        }

        child.event(ctx, event, data, env);
    }
}