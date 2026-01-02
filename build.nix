{
  pkgs,
  stdenv,
  rustPlatform,
}:
let
  targetName = "wasm32-unknown-unknown";
  pname = "css-typing-gen";
  version = "0.2.5";

  wasm-build = rustPlatform.buildRustPackage {
    inherit pname version;

    cargoLock.lockFile = ./Cargo.lock;

    src = ./.;

    nativeBuildInputs = with pkgs; [
      wasm-bindgen-cli_0_2_106
      pkg-config
      llvmPackages.lld
    ];

    buildInputs = with pkgs; [
      openssl
      gnumake
    ];

    doCheck = false;

    buildPhase = ''
      runHook preBuild

      cargo build --target ${targetName} --release

      mkdir -p $out/pkg
      wasm-bindgen target/${targetName}/release/css_typing_gen.wasm --out-dir=$out/pkg

      runHook postBuild
    '';

    installPhase = "echo 'Skipping installPhase'";
  };
in
stdenv.mkDerivation (finalAttrs: {
  inherit pname version;

  src = ./www;

  nativeBuildInputs = with pkgs; [
    nodejs
    pnpmConfigHook
    pnpm_10
  ];

  buildPhase = ''
    runHook preBuild

    ln -s ${wasm-build}/pkg ../pkg
    pnpm build
    cp -r dist $out

    runHook postBuild
  '';

  pnpmDeps = pkgs.fetchPnpmDeps {
    inherit (finalAttrs) pname version src;
    fetcherVersion = 2;
    hash = "sha256-jb89wL3gZGObd6qrCQe1dOudllZQ2q5dcObeMoKHwV0=";
  };
})
