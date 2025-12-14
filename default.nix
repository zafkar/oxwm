{
  lib,
  rustPlatform,
  pkg-config,
  libX11,
  libXft,
  libXrender,
  freetype,
  fontconfig,
  gitRev ? "unkown",
}:
rustPlatform.buildRustPackage (finalAttrs: {
  pname = "oxwm";
  version = "${lib.substring 0 8 gitRev}";

  src = ./.;

  cargoLock.lockFile = ./Cargo.lock;

  nativeBuildInputs = [pkg-config];

  buildInputs = [
    libX11
    libXft
    libXrender
    freetype
    fontconfig
  ];

  # tests require a running X server
  doCheck = false;

  postInstall = ''
    install resources/oxwm.desktop -Dt $out/share/xsessions
    install -Dm644 resources/oxwm.1 -t $out/share/man/man1
    install -Dm644 templates/oxwm.lua -t $out/share/oxwm
  '';

  passthru.providedSessions = ["oxwm"];

  meta = {
    description = "Dynamic window manager written in Rust, inspired by dwm";
    homepage = "https://github.com/tonybanters/oxwm";
    license = lib.licenses.gpl3Only;
    platforms = lib.platforms.linux;
    mainProgram = "oxwm";
  };
})
