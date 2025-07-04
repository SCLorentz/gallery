use std::{fs, path::PathBuf, sync::Arc};

use druid::{
    commands::{OPEN_FILE, SHOW_OPEN_PANEL},
    menu::MenuEventCtx,
    widget::{Button, Flex, Label},
    AppLauncher, Command, Data, Env, Event, EventCtx, FileDialogOptions,
    Handled, ImageBuf, Lens, Target, Widget, WidgetExt, WindowDesc, WindowId,
};

use infer;
use if_empty::*;

#[cfg(target_os = "macos")]
mod mac;

mod app;
use app::{WindowManager};

mod settings;
use settings::build_settings_ui;

mod image;
use image::{
    encoder::{decode, encode},
    load_image,
    load_image_compressed,
    DynamicImage,
};

mod init;
use init::init_bhg;

/*#[cfg(target_os = "macos")]
extern "C"
{
    fn init_window();
}*/

#[derive(Clone, Data, Lens)]
struct AppState
{
    selected_file: String,
    image: Option<ImageBuf>,
    mode: String,
    settings_window_id: Option<Arc<druid::WindowId>>,
}

pub const SHOW_SETTINGS: druid::Selector = druid::Selector::new("app.show-settings");


fn main()
{
    let img_path = "bhg.img";
    if !PathBuf::from(img_path).exists() { init_bhg(); }

    let mut main_window = WindowManager::new("BHGallery", 400.0, 400.0, build_ui);

    let initial_state = AppState {
        selected_file: "".to_string(),
        mode: "".to_string(),
        image: None,
        settings_window_id: None,
    };

    AppLauncher::with_window(main_window.window.take().unwrap())
        .delegate(Delegate {})
        .launch(initial_state)
        .expect("Error while launching the application");
}

fn build_ui() -> impl Widget<AppState>
{
    let open_file_button = Button::new("Import File(s)")
        .on_click(|ctx, _data: &mut AppState, _env| import_file((None, Some(ctx))));

    let file_label = Label::new(|data: &AppState, _env: &_|
        format!(
            "Import: {}",
            data.selected_file.clone().if_empty("None".to_string())
        )
    );

    let image_widget = DynamicImage::new().center();

    let open_settings = Button::new("Settings")
        .on_click(|ctx, _data: &mut AppState, _env| ctx.submit_command(SHOW_SETTINGS));

    Flex::column()
        .with_child(open_settings.padding(10.0))
        .with_child(open_file_button.padding(10.0))
        .with_child(file_label.padding(10.0))
        .with_child(image_widget.padding(10.0))
        .controller(FileSelectionController)
}

pub fn import_file(ctx: (Option<&mut MenuEventCtx>, Option<&mut EventCtx>))
{
    let opt = FileDialogOptions::new()
        .name_label("Files")
        .title("Import files")
        .multi_selection()
        .button_text("Import");

    match ctx
    {
        (Some(m), None) => m.submit_command(SHOW_OPEN_PANEL.with(opt)),
        (None, Some(e)) => e.submit_command(SHOW_OPEN_PANEL.with(opt)),
        _ => eprintln!("unexpected arguments passed to `import_file`"),
    }
}

// TODO: this is fuking terrible to read
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
            let Some(file_info) = cmd.get(OPEN_FILE) else
            {
                eprintln!("couldn't open file");
                return;
            };

            let Some(path) = file_info.path().to_str() else { return };

            data.selected_file = path.to_string();

            let path = PathBuf::from(&data.selected_file);

            let content = match fs::read(path.clone())
            {
                Ok(content) => content,
                Err(err) => {
                    eprintln!("Erro ao ler o arquivo: {}", err);
                    return;
                },
            };

            let Some(kind) = infer::get(&content) else
            {
                eprintln!("Tipo de arquivo não detectado");
                return;
            };

            println!("Tipo detectado: {}", kind.mime_type());

            match kind.mime_type()
            {
                c if c.starts_with("image/") =>
                {
                    let Ok(image_buf) = load_image(path.clone()) else
                    {
                        eprintln!("Erro ao carregar imagem");
                        return;
                    };

                    data.image = Some(image_buf);

                    assert!(encode(path, content).is_ok(), "Erro ao comprimir");
                },
                "application/gzip" => data.image = Some(handle_gzip(content).unwrap_or_else(|err|
                {
                    eprintln!("Erro ao carregar imagem: {}", err);
                    ImageBuf::empty()
                })),
                file_type =>
                {
                    let svg = content.iter().take(5).eq(b"<?xml");

                    if svg
                    {
                        eprintln!("Not supported (yet)");
                        return;
                    }

                    eprintln!("Not supported type of file: {}", file_type);
                },
            }
        }

        child.event(ctx, event, data, env);
    }
}

fn handle_gzip(content: Vec<u8>) -> Result<ImageBuf, std::io::Error>
{
    let buffer = decode(content)?;
    let image_buf = load_image_compressed(&buffer)
        .map_err(|e| std::io::Error::new(std::io::ErrorKind::Other, e))?;
    Ok(image_buf)
}

struct Delegate;

impl druid::AppDelegate<AppState> for Delegate
{
    fn command(
        &mut self,
        ctx: &mut druid::DelegateCtx,
        _target: Target,
        cmd: &Command,
        data: &mut AppState,
        _env: &Env,
    ) -> Handled
    {
        if cmd.is(SHOW_SETTINGS)
        {
            if data.settings_window_id.is_none()
            {
                let settings_window = WindowDesc::new(build_settings_ui())
                    .title("Configurações")
                    .window_size((500.0, 400.0));
                ctx.new_window(settings_window);

                data.settings_window_id = Some(Arc::new(WindowId::next()));
            }
            return Handled::Yes;
        }
        Handled::No
    }

    // TODO: make the settings window close whenever the main window is closed
}