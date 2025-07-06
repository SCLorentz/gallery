{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell {
    buildInputs = [
        pkgs.libheif
        pkgs.libheif.dev
    ];
    shellHook = ''
        export CPLUS_INCLUDE_PATH=${pkgs.libheif.dev}/include:$CPLUS_INCLUDE_PATH
        export CPLUS_INCLUDE_PATH=$CPLUS_INCLUDE_PATH:${toString ./src/image}
        export LIBRARY_PATH=${pkgs.libheif.dev}/lib:$LIBRARY_PATH
        export LD_LIBRARY_PATH=${pkgs.libheif.dev}/lib:$LD_LIBRARY_PATH
        export PKG_CONFIG_PATH=${pkgs.libheif.dev}/lib/pkgconfig:$PKG_CONFIG_PATH
    '';
    nativeBuildInputs = [
        pkgs.pkg-config
    ];
}