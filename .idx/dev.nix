{ pkgs, ... }: {
  channel = "stable-24.05";
  packages = [
    pkgs.zig
    pkgs.rustup
    pkgs.busybox
    pkgs.gcc
  ];
  env = { };
  idx = {
    extensions = [
      "pkief.material-icon-theme"
      "ziglang.vscode-zig"
      "rust-lang.rust"
    ];
    workspace = {
      onCreate = {
        install = "rustup default stable && rustup update && cargo run";
        default.openFiles = [
          "README.md"
        ];
      };
    };
  };
}