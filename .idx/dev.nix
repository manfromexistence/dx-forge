{ pkgs, ... }: {
  channel = "stable-24.05";
  packages = [
    pkgs.zig
    pkgs.rustup
    pkgs.busybox
    pkgs.gcc
    pkgs.neovim
    pkgs.mingw-w64
  ];

  env = { };
  idx = {
    extensions = [
      "pkief.material-icon-theme"
      "ziglang.vscode-zig"
      "rust-lang.rust"
      "tamasfe.even-better-toml"
    ];
    workspace = {
      onCreate = {
        install = ''
                    echo "Setting up native cross-compilation for Windows..."

                    # 1. Add the rust target for Windows
                    rustup target add x86_64-pc-windows-gnu

                    # 2. Create the Cargo config file to specify the Windows C linker
                    mkdir -p .cargo
                    cat <<EOT > .cargo/config.toml
          [target.x86_64-pc-windows-gnu]
          linker = "x86_64-w64-mingw32-gcc"
          ar = "x86_64-w64-mingw32-ar"
          EOT

                    echo "Setup complete. You can now build using 'cargo build --target...'"
        '';
        default.openFiles = [
          "README.md"
          ".cargo/config.toml"
        ];
      };
    };
  };
}
