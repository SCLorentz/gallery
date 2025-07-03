import Cocoa
import Metal
import MetalKit

class SwiftWindowController: NSWindowController {
    init() {
        let window = NSWindow(contentRect: NSMakeRect(0, 0, 800, 600),
            styleMask: [.titled, .closable, .resizable, .fullSizeContentView],
            backing: .buffered,
            defer: false)
        
        window.titleVisibility = .hidden
        window.titlebarAppearsTransparent = true
        window.styleMask.insert(.fullSizeContentView)
        super.init(window: window)
    }

    required init?(coder: NSCoder) {
        fatalError("init(coder:) has not been implemented")
    }

    func show() {
        self.window?.makeKeyAndOrderFront(nil)
    }
}

var controller: SwiftWindowController?

@_cdecl("init_window")
public func init_window() {
    DispatchQueue.main.async {
        controller = SwiftWindowController()
        controller?.show()
    }
}