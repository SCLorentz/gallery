//use std::process::Command;

fn main()
{
    /*let status = Command::new("swiftc")
        .args(&[
            "src/mac/ui.swift",
            "-emit-library",
            "-emit-module",
            "-o", "target/debug/libmacgui.dylib",
            "-Xlinker", "-install_name",
            "-Xlinker", "@rpath/libmacgui.dylib",
        ])
        .status()
        .expect("Falha ao compilar Swift");

    assert!(status.success());*/

    cc::Build::new()
        .cpp(true)
        .file("src/main.cpp")
        .file("src/image/heif.cpp")
        .flag("-std=c++17")
        .flag("-mmacosx-version-min=11.3")
        .compile("cppbridge");

    println!("cargo:rustc-link-search=native=target/debug");
    println!("cargo:rustc-link-lib=dylib=macgui");
    println!("cargo:rustc-link-lib=heif");
}