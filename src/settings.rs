use druid::{widget::{Flex, Label}, Widget, WidgetExt, WindowDesc};

use crate::{make_menu, AppState};

#[allow(unused)]
pub fn generate_window()
{
    let settings_window = WindowDesc::new(build_settings())
        .title("Configurações")
        .window_size((400.0, 250.0))
        .resizable(false)
        .menu(make_menu);
}

pub fn build_settings() -> impl Widget<AppState>
{
    Flex::column()
        .with_child(Label::new("Settings").padding(10.0))
}