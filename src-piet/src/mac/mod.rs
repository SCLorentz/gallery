use druid::{Data, Env, LocalizedString, Menu, MenuItem};

use crate::import_file;
use crate::SHOW_SETTINGS;

//use crate::settings;

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
        .entry(MenuItem::new("Configurações").command(SHOW_SETTINGS))
        .entry(MenuItem::new(LocalizedString::new("Quit").with_placeholder("Sair do App"))
            .command(druid::commands::QUIT_APP))
        .entry(MenuItem::new(LocalizedString::new("Import File"))
            .on_activate(|ctx, _data, _env| import_file((Some(ctx), None))))
        .entry(MenuItem::new(LocalizedString::new("Decompress file"))
            .on_activate(|_ctx, _data, _env| todo!()));

    // TODO: add export button here (export_to_file(path))
    // TODO: add share button
    // TODO: add settings button

    base_menu.entry(edit_menu).entry(custom_menu)
}