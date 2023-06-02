{ pkgs ? import <nixpkgs> {} }:
pkgs.mkShell rec {
    buildInputs = with pkgs; [
        rustup
        gcc
        cmake
        pkg-config
    ];
    nativeBuildInputs = with pkgs; [
        openssl
    ];
    HISTFILE = toString ./.history;
    shellHook = ''
        rustup override set nightly
        export LD_LIBRARY_PATH="$LD_LIBRARY_PATH:${pkgs.vulkan-loader}/lib:${pkgs.wayland}/lib:${pkgs.libxkbcommon}/lib"
    '';
}