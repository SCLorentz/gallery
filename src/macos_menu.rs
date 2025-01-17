use druid::{Data, Env, LocalizedString, Menu, MenuItem};

pub fn make_menu<T: Data>(_window_id: Option<druid::WindowId>, _data: &T, _env: &Env) -> Menu<T>
{
    let base_menu = Menu::new(LocalizedString::new("Application Menu"))
        .entry(druid::platform_menus::mac::application::default())
        .separator();

    let edit_menu = Menu::new(LocalizedString::new("Edit"))
        .entry(MenuItem::new(LocalizedString::new("Undo")).command(druid::commands::UNDO))
        .entry(MenuItem::new(LocalizedString::new("Redo")).command(druid::commands::REDO))
        .separator()
        .entry(MenuItem::new(LocalizedString::new("Cut")).command(druid::commands::CUT))
        .entry(MenuItem::new(LocalizedString::new("Copy")).command(druid::commands::COPY))
        .entry(MenuItem::new(LocalizedString::new("Paste")).command(druid::commands::PASTE));

    let custom_menu = Menu::new(LocalizedString::new("Custom"))
        .entry(MenuItem::new(LocalizedString::new("Quit").with_placeholder("Sair do App"))
            .command(druid::commands::QUIT_APP));

    base_menu.entry(edit_menu).entry(custom_menu)
}