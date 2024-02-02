let
  rust_overlay = import (builtins.fetchTarball
    "https://github.com/oxalica/rust-overlay/archive/master.tar.gz");
  pkgs = import <nixpkgs> { overlays = [ rust_overlay ]; };
  rustVersion = "1.73.0";
  rust = pkgs.rust-bin.stable.${rustVersion}.default.override {
    extensions = [
      "rust-src" # for rust-analyzer
      "llvm-tools-preview"
    ];
    targets = [ "thumbv6m-none-eabi" "thumbv7em-none-eabihf" ];
  };
in pkgs.mkShell {
  buildInputs = [ rust ] ++ (with pkgs; [
    rust-analyzer
    pkg-config
    minicom
    gcc-arm-embedded-11
    cargo
    cargo-binutils
    probe-run
    flip-link
  ]);
  RUST_BACKTRACE = 1;
}
