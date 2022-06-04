{
  description = "Chicky Chicky!";

  inputs = {
    nixpkgs.url = "github:nixos/nixpkgs/nixos-unstable";
    flake-utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
  };

  outputs = { nixpkgs, rust-overlay, flake-utils, ... }:
    flake-utils.lib.eachDefaultSystem
      (system:
        let
          overlays = [ (import rust-overlay) ];
          pkgs = import nixpkgs {
            inherit system overlays;
            config = { allowUnfree = true; };
          };
        in
        {
          devShell = with pkgs; mkShell
            {
              buildInputs = [
                cargo
                clang
                glibc
                glfw-wayland
                lld
                pkgconfig
                udev
                alsaLib
                lutris
                xlibsWrapper
                xorg.libXcursor
                xorg.libXrandr
                xorg.libXi
                vulkan-tools
                vulkan-headers
                vulkan-loader
                vulkan-validation-layers
              ];
            };
        }
      );
}
