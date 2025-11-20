{ pkgs
, makeRustPlatform
}:
let
  targetName = "wasm32-unknown-unknown";

  wasm-rust = pkgs.rust-bin.stable.latest.default.override {
    extensions = [ "rust-src" ];
    targets = [ targetName ];
  };

  rustPlatformWasm = makeRustPlatform {
    cargo = wasm-rust;
    rustc = wasm-rust;
  };

  wasm-build = rustPlatformWasm.buildRustPackage {
    name = "css-typing-gen";
    cargoLock.lockFile = ./Cargo.lock;

    src = ./.;

    nativeBuildInputs = with pkgs; [
      wasm-bindgen-cli
    ];

    buildInputs = with pkgs; [
      openssl
      pkg-config
      gnumake
    ];

    buildPhase = ''
      cargo build --target ${targetName} --release
      wasm-bindgen target/${targetName}/release/css_typing_gen.wasm --out-dir=$out/pkg
    '';

    installPhase = "echo 'Skipping installPhase'";
  };
in
pkgs.stdenv.mkDerivation (finalAttrs: {
  pname = "css-typing-gen";
  version = "0.2.5";

  src = ./www;

  nativeBuildInputs = with pkgs; [
    nodejs
    pnpm_10.configHook
  ];

  buildPhase = ''
    runHook preBuild

    ln -s ${wasm-build}/pkg ../pkg
    pnpm build
    cp -r dist $out

    runHook postBuild
  '';

  pnpmDeps = pkgs.pnpm_10.fetchDeps {
    inherit (finalAttrs) pname version src;
    fetcherVersion = 2;
    hash = "sha256-wYA2lTxsaSh8Zmp6FV+4l9bMaj4jA9rVvNmaJT3Eg2E=";
  };
})