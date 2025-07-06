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
    //encoder::{decode, encode},
    load_image,
    load_image_compressed,
    DynamicImage,
};

mod init;
use init::init_bhg;

extern "C"
{
    #[cfg(target_os = "macos")]
    fn init_window();

    fn to_heif(input_file: *const i8) -> i32;
}

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
    fn event(&mut self, child: &mut W, ctx: &mut EventCtx, event: &Event, data: &mut AppState, env: &Env)
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

                    unsafe {
                        use std::ffi::CString;
                        let c_path = CString::new(path.to_string_lossy().as_bytes()).unwrap();
                        let result = to_heif(c_path.as_ptr());
                        if result != 0 {
                            eprintln!("Falha ao converter para HEIF");
                        }
                    };
                },
                "application/gzip" => todo!("review older version of this code"),
                _ => {
                    eprintln!("Tipo de arquivo não suportado: {}", kind.mime_type());
                    return;
                },
            }
        }

        child.event(ctx, event, data, env);
    }
}

struct Delegate;

impl druid::AppDelegate<AppState> for Delegate
{
    fn command(&mut self, ctx: &mut druid::DelegateCtx, _target: Target, cmd: &Command, data: &mut AppState, _env: &Env) -> Handled
    {
        if !cmd.is(SHOW_SETTINGS) { return Handled::No; }

        if data.settings_window_id.is_none()
        {
            let settings_window = WindowDesc::new(build_settings_ui())
                .title("Configurações")
                .window_size((500.0, 400.0));
            ctx.new_window(settings_window);

            data.settings_window_id = Some(Arc::new(WindowId::next()));
        }
        Handled::Yes
    }

    // TODO: make the settings window close whenever the main window is closed
}