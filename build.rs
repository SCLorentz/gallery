fn main()
{
    cc::Build::new()
        .cpp(true)
        .file("src/main.cpp")
        .flag("-std=c++17")
        .flag("-mmacosx-version-min=11.0")
        .compile("cppbridge");
}