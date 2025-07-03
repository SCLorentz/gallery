use std::sync::Arc;
use druid::{Widget, WindowDesc, WindowId};
use crate::AppState;

pub struct WindowManager
{
    #[allow(unused)]
    pub id: Arc<WindowId>,
    pub window: Option<WindowDesc<AppState>>,
}

impl WindowManager
{
    #[allow(unused)]
    pub fn get_id(&self) -> &Arc<WindowId>
    {
        &self.id
    }

    pub fn new<F, W>(title: &str, height: f64, width: f64, ui_fn: F) -> Self
    where
        F: Fn() -> W,
        W: Widget<AppState> + 'static,
    {
        let window_id = Arc::new(WindowId::next());
        let mut window = WindowDesc::new(ui_fn())
            .title(title)
            .window_size((width, height));

        #[cfg(target_os = "macos")]
        {
            window = window.menu(|id, data, env| crate::mac::make_menu(id, data, env));
        }

        WindowManager {
            id: window_id,
            window: Some(window),
        }
    }
}