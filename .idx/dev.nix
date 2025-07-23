{ pkgs, ... }: {
  channel = "stable-24.05";
  packages = [
    pkgs.zig
    pkgs.rustup
    pkgs.busybox
    pkgs.gcc
    pkgs.neovim
    pkgs.cargo-cross
    # No need for pkgs.docker here, it's included by the option below
  ];

  # Add this line to start the Docker daemon automatically
  virtualisation.docker.enable = true;

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
        # It's good practice to wait for the Docker daemon to be ready
        # However, for a simple build command it should be fine.
        install = "rustup default stable && rustup update && cargo run";
        default.openFiles = [
          "README.md"
        ];
      };
    };
  };
}