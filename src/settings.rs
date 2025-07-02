use druid::{widget::{Button, Flex, Label}, Widget};

use crate::AppState;

pub fn build_settings_ui() -> impl Widget<AppState>
{
    Flex::column()
        .with_child(Label::new("Configurações"))
        .with_spacer(20.0)
        .with_child(Button::new("Fechar").on_click(|ctx, _data: &mut AppState, _env| ctx.window().close()))
}