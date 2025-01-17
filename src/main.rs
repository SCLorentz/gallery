use std::{
    fs::{self, File},
    io::prelude::*,
    path::PathBuf,
};

use druid::{
    commands::{OPEN_FILE, SHOW_OPEN_PANEL},
    widget::{Button, Flex, Label, TextBox},
    AppLauncher, Data, Env, Event, EventCtx, FileDialogOptions, Lens, LensExt, Selector, Widget,
    WidgetExt, WindowDesc,
};
use flate2::{read::GzDecoder, write::GzEncoder, Compression};

use if_empty::*;

#[derive(Clone, Data, Lens)]
struct AppState
{
    selected_file: String,
    mode: String,
}

fn main()
{
    let main_window = WindowDesc::new(build_ui)
        .title("Compressor de Arquivos")
        .window_size((400.0, 250.0));

    let initial_state = AppState
    {
        selected_file: "".to_string(),
        mode: "".to_string(),
    };

    AppLauncher::with_window(main_window)
        .use_simple_logger()
        .launch(initial_state)
        .expect("Falha ao lançar a aplicação");
}

fn build_ui() -> impl Widget<AppState>
{
    let input = TextBox::new()
        .with_placeholder("Digite 'encode' ou 'decode'")
        .fix_width(200.0)
        .lens(AppState::mode);

    let execute_button = Button::new("Executar").on_click(|_ctx, data: &mut AppState, _env|
    {
        if data.selected_file.is_empty()
        {
            println!("Nenhum arquivo selecionado!");
            return;
        }

        let path = PathBuf::from(&data.selected_file);

        match data.mode.as_str()
        {
            "encode" => encode(path).unwrap_or_else(|err|
            {
                println!("Erro ao comprimir: {}", err);
            }),
            "decode" => decode(path).unwrap_or_else(|err|
            {
                println!("Erro ao descomprimir: {}", err);
            }),
            _ => eprintln!("Modo inválido! Use 'encode' ou 'decode'."),
        }
    });

    let open_file_button = Button::new("Selecionar Arquivo").on_click(|ctx, _data, _env|
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
            "Arquivo selecionado: {}",
            data.selected_file.clone().if_empty("Nenhum arquivo".to_string())
        )
    });

    Flex::column()
        .with_child(Label::new("Compressor de Arquivos").padding(10.0))
        .with_child(input.padding(10.0))
        .with_child(open_file_button.padding(10.0))
        .with_child(file_label.padding(10.0))
        .with_child(execute_button.padding(10.0))
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
            let Some(file_info) = cmd.get(OPEN_FILE) else {
                panic!("couldn't open file")
            };

            if let Some(path) = file_info.path().to_str()
            {
                data.selected_file = path.to_string();
            }
        }

        child.event(ctx, event, data, env);
    }
}

fn encode(path: PathBuf) -> Result<(), std::io::Error>
{
    let mut encoder = GzEncoder::new(Vec::new(), Compression::default());

    let content = fs::read(path.clone())?;
    encoder.write_all(&content)?;

    let compressed_bytes = encoder.finish()?;

    let new_path = path.with_extension(format!("{}.gz", path.extension().unwrap().to_str().unwrap()));

    let mut output = File::create(new_path)?;
    output.write_all(&compressed_bytes)?;

    Ok(())
}

fn decode(path: PathBuf) -> Result<(), std::io::Error>
{
    let content = fs::read(path.clone())?;

    let mut decoder = GzDecoder::new(content.as_slice());
    let mut buff = Vec::new();

    decoder.read_to_end(&mut buff)?;

    let new_path = path.with_extension("");

    let mut output = File::create(new_path)?;
    output.write_all(&buff)?;

    Ok(())
}