use druid::{Data, Env, LocalizedString, platform_menus::mac};

use crate::import_file;
use crate::SHOW_SETTINGS;

//use crate::settings;

fn dstring<T>(c: &'static str) -> LocalizedString<T> { LocalizedString::new(c) }

pub fn make_menu<T: Data>(_window_id: Option<druid::WindowId>, _data: &T, _env: &Env) -> druid::Menu<T>
{
    use druid::{
        commands::{UNDO, REDO, CUT, COPY, PASTE},
        menu::{Menu as M, MenuItem as I},
    };

    let base_menu = M::new(dstring("Application Menu"))
        .entry(mac::application::default())
        .separator();

    let edit_menu = M::new(dstring("Edit"))
        .entry(I::new(dstring("Undo")).command(UNDO))
        .entry(I::new(dstring("Redo")).command(REDO))
        .separator()
        .entry(I::new(dstring("Cut")).command(CUT))
        .entry(I::new(dstring("Copy")).command(COPY))
        .entry(I::new(dstring("Paste")).command(PASTE));

    let custom_menu = M::new(dstring("File"))
        .entry(I::new(dstring("Configurações")).command(SHOW_SETTINGS))
        .entry(I::new(dstring("Import File"))
            .on_activate(|ctx, _, _| import_file((Some(ctx), None))))
        .entry(I::new(dstring("Decompress file"))
            .on_activate(|_, _, _| todo!()));

    // TODO: add export button here (export_to_file(path))
    // TODO: add share button
    // TODO: add settings button

    base_menu.entry(edit_menu).entry(custom_menu)
}