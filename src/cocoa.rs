use cocoa::appkit::{NSApp, NSApplication, NSButton, NSView, NSWindow, NSWindowStyleMask};
use cocoa::base::{id, nil};
use cocoa::foundation::{NSAutoreleasePool, NSPoint, NSRect, NSSize, NSString};

pub fn create_custom_window()
{
    unsafe
    {
        // Criar um pool de autorelease para gerenciar memória
        let _pool = NSAutoreleasePool::new(nil);

        // Inicializar o NSApplication
        let app = NSApp();
        app.setActivationPolicy_(cocoa::appkit::NSApplicationActivationPolicyRegular);

        // Criar a janela
        let window = NSWindow::alloc(nil).initWithContentRect_styleMask_backing_defer_(
            NSRect::new(NSPoint::new(0.0, 0.0), NSSize::new(400.0, 400.0)),
            NSWindowStyleMask::NSTitledWindowMask | NSWindowStyleMask::NSClosableWindowMask | NSWindowStyleMask::NSResizableWindowMask,
            cocoa::appkit::NSBackingStoreBuffered,
            false,
        );
        NSButton::setTitle_(window, NSString::alloc(nil).init_str("Janela Customizada"));
        window.center();
        window.makeKeyAndOrderFront_(nil);

        // Adicionar um botão à janela
        let content_view: id = window.contentView();
        let button = NSButton::initWithFrame_(NSButton::alloc(nil), NSRect::new(NSPoint::new(100.0, 150.0), NSSize::new(200.0, 50.0)));
        NSButton::setTitle_(button, NSString::alloc(nil).init_str("Clique Aqui"));
        button.setBezelStyle_(cocoa::appkit::NSBezelStyle::NSRoundedBezelStyle); // Estilo de botão padrão
        content_view.addSubview_(button);

        // Ativar o aplicativo
        app.activateIgnoringOtherApps_(true);
        app.run();
    }
}