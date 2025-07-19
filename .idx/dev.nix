# To learn more about how to use Nix to configure your environment
# see: https://developers.google.com/idx/guides/customize-idx-env
{ pkgs, ... }: {
  # Which nixpkgs channel to use.
  channel = "stable-24.05"; # or "unstable"
  # Use https://search.nixos.org/packages to find packages
  packages = [
    pkgs.zig
    # *** THE FIX: Use rustup to manage the Rust toolchain. ***
    # This is the standard and most reliable method.
    pkgs.rustup
    pkgs.busybox
    pkgs.gcc
  ];
  # Sets environment variables in the workspace
  env = { };
  idx = {
    # Search for the extensions you want on https://open-vsx.org/ and use "publisher.id"
    extensions = [
      # "vscodevim.vim"
      "pkief.material-icon-theme"
      "ziglang.vscode-zig"
      "rust-lang.rust"
    ];
    workspace = {
      # Runs when a workspace is first created with this `dev.nix` file
      onCreate = {
        # Use rustup to install the latest stable toolchain and make it the default.
        install = "rustup default stable && rustup update";
        # Open editors for the following files by default, if they exist:
        default.openFiles = [
          "README.md"
        ];
      };
      # To run something each time the workspace is (re)started, use the `onStart` hook
    };
  };
}
