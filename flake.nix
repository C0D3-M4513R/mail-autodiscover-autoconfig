{
  inputs = {
    # This must be the stable nixpkgs if you're running the app on a
    # stable NixOS install.  Mixing EGL library versions doesn't work.
    nixpkgs.url = "github:NixOS/nixpkgs/nixos-24.11";
    utils.url = "github:numtide/flake-utils";
    rust-overlay.url = "github:oxalica/rust-overlay";
    flake-compat = {
      url = github:edolstra/flake-compat;
      flake = true;
    };
  };

  outputs = { self, nixpkgs, utils, rust-overlay, ... }:
    utils.lib.eachSystem ["x86_64-linux" "aarch64-linux"] (system:
      let
        arch = {
            "x86_64-linux" = "amd64";
            "aarch64-linux" = "arm64";
        };

        overlays = [ (import rust-overlay) ];
        pkgs = import nixpkgs {
            inherit system overlays;
            config.allowUnfree = true;
        };

        rustVersion = pkgs.rust-bin.stable.latest.default;

        rustPlatform = pkgs.makeRustPlatform {
          cargo = rustVersion;
          rustc = rustVersion;
        };

        manifest = (builtins.fromTOML (builtins.readFile ./Cargo.toml)).package;

        commonBuildInputs = with pkgs; [
          pkg-config
          openssl
        ];

        package = pkgs.rust.packages.stable.rustPlatform.buildRustPackage rec{
          pname = manifest.name;
          version = manifest.version;
          src = pkgs.lib.cleanSource ./.;
          cargoBuildFlags = "";

          cargoLock = {
            lockFile = ./Cargo.lock;
          };

          nativeBuildInputs = [
            pkgs.autoPatchelfHook
          ];

          buildInputs = with pkgs; [
            pkgs.rust-bin.stable.latest.default
          ] ++ commonBuildInputs;

          # Certain Rust tools won't work without this
          # This can also be fixed by using oxalica/rust-overlay and specifying the rust-src extension
          # See https://discourse.nixos.org/t/rust-src-not-found-and-other-misadventures-of-developing-rust-on-nixos/11570/3?u=samuela. for more details.
          RUST_SRC_PATH = pkgs.rust.packages.stable.rustPlatform.rustLibSrc;
          PKG_CONFIG_PATH = "${pkgs.openssl.dev}/lib/pkgconfig";
          #LD_LIBRARY_PATH = libPath;
          OPENSSL_LIB_DIR = pkgs.openssl.out + "/lib";

          meta = {
            description = "Autmatically provide configuration for new email clients";
            homepage = "https://github.com/c0d3-m4513r/mail-autodiscover-autoconfig/";
            license = nixpkgs.lib.licenses.mpl20;
            maintainers = [];
            mainProgram = manifest.name;
          };
        };
        templates = pkgs.stdenv.mkDerivation {
            name = "templates";
            src = ./templates;
            postInstall = ''
                mkdir $out
                cp -r . $out
            '';
        };
        docker = pkgs.dockerTools.buildLayeredImage {
            name = manifest.name;
            tag = "${manifest.version}-${arch."${system}"}";

            contents = [
                templates
                package
            ];

            config = {
                Env = [
                    "ROCKET_TEMPLATE_DIR=${templates.outPath}"
                ];
                Expose = {
                    "8000" = {};
                };
                Cmd = [ "${nixpkgs.lib.getExe package}" ];
            };

            created = "${builtins.substring 0 4 self.lastModifiedDate}-${builtins.substring 4 2 self.lastModifiedDate}-${builtins.substring 6 2 self.lastModifiedDate}T${builtins.substring 8 2 self.lastModifiedDate}:${builtins.substring 10 2 self.lastModifiedDate}:${builtins.substring 12 2 self.lastModifiedDate}Z";
        };
      in
      {
        packages = {
            "${manifest.name}" = package;
            templates = templates;
            "${manifest.name}-docker" = docker;
        };

        defaultApp = utils.lib.mkApp {
          drv = self.defaultPackage."${system}";
        };

        devShell = with pkgs; mkShell {
          buildInputs = [
            #cargo
            cargo-insta
            docker-compose
            pre-commit
            #rust-analyzer
            #rustPackages.clippy
            #rustc
            #rustfmt
            tokei
          ] ++ commonBuildInputs;
          RUST_SRC_PATH = pkgs.rust.packages.stable.rustPlatform.rustLibSrc;
          LD_LIBRARY_PATH = lib.makeLibraryPath commonBuildInputs;
          GIT_EXTERNAL_DIFF = "${difftastic}/bin/difft";
        };
      });
}
