use std::{fmt::Result, fs, path::PathBuf};

use druid::{
    commands::{OPEN_FILE, SHOW_OPEN_PANEL}, menu::MenuEventCtx, platform_menus::mac::file, widget::{Button, Flex, Label}, AppLauncher, Data, Env, Event, EventCtx, FileDialogOptions, ImageBuf, Lens, Widget, WidgetExt, WindowDesc
};

use infer;

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

pub fn import_file(ctx: (Option<&mut MenuEventCtx>, Option<&mut EventCtx>))
{
    let options = FileDialogOptions::new()
        .name_label("Files")
        .title("Import files")
        .multi_selection()
        .button_text("Import");

    match ctx
    {
        (Some(menu_ctx), None) => menu_ctx.submit_command(SHOW_OPEN_PANEL.with(options)),
        (None, Some(event_ctx)) => event_ctx.submit_command(SHOW_OPEN_PANEL.with(options)),
        _ => eprintln!("unexpected arguments passed to `import_file`"),
    }
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
        .expect("Error while launching the application");
}

fn build_ui() -> impl Widget<AppState>
{
    let open_file_button = Button::new("Import File(s)").on_click(|ctx, _data: &mut AppState, _env| import_file((None, Some(ctx))));

    let file_label = Label::new(|data: &AppState, _env: &_|
        format!(
            "Import: {}",
            data.selected_file.clone().if_empty("None".to_string())
        )
    );

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
            if let Err(err) = (|| -> Result<(), std::io::Error>
            {
                let file_info = cmd.get(OPEN_FILE)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "couldn't open file"))?;

                let path = file_info.path().to_str()
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "invalid path"))?;

                data.selected_file = path.to_string();

                let path = PathBuf::from(&data.selected_file);

                let content = fs::read(path.clone())?;

                let kind = infer::get(&content)
                    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "file type not detected"))?;
                
                println!("Tipo detectado: {}", kind.mime_type());

                match kind.mime_type()
                {
                    "image/jpeg" | "image/png" | "image/gif" | "image/bmp" | "image/webp" => 
                    {
                        let image_buf = load_image(path.clone())
                            .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                        data.image = Some(image_buf);

                        assert!(encode(path, content).is_ok(), "Erro ao comprimir");
                    },
                    "application/gzip" =>
                    {
                        let image_buf = handle_gzip(content).map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

                        data.image = Some(image_buf);
                    },
                    _ => todo!(),
                }

                Ok(())
            })() {
                eprintln!("Erro: {}", err);
            }
        }
    }
        //child.event(ctx, event, data, env);
}

fn handle_gzip(content: Vec<u8>) -> std::result::Result<ImageBuf, std::io::Error>
{
    let buffer = decode(content)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    let image_buf = load_image_compressed(&buffer)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;

    return Ok(image_buf);
}