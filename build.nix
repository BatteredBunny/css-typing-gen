{
  pkgs,
  stdenv,
  rustPlatform,
  pnpm_11,
  fetchPnpmDeps,
  pnpmConfigHook,
  nodejs,
  llvmPackages,
  pkg-config,
  wasm-bindgen-cli_0_2_121,
  openssl,
}:
let
  targetName = "wasm32-unknown-unknown";
  pname = "css-typing-gen";
  version = "0.2.5";
  pnpm = pnpm_11;

  wasm-build = rustPlatform.buildRustPackage {
    inherit pname version;

    cargoLock.lockFile = ./Cargo.lock;

    src = ./.;

    nativeBuildInputs = [
      wasm-bindgen-cli_0_2_121
      pkg-config
      llvmPackages.lld
    ];

    buildInputs = [
      openssl
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

  nativeBuildInputs = [
    nodejs
    pnpmConfigHook
    pnpm
  ];

  buildPhase = ''
    runHook preBuild

    ln -s ${wasm-build}/pkg ../pkg
    pnpm build
    cp -r dist $out

    runHook postBuild
  '';

  pnpmDeps = fetchPnpmDeps {
    pname = "css-typing-gen-frontend";
    inherit (finalAttrs) version src;
    inherit pnpm;
    fetcherVersion = 3;
    hash = "sha256-3/diT0SXP4UeGsBvICEf300HunS8yZ3R7rwWiUVDzUc=";
  };
})
