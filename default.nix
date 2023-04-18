with import <nixpkgs> {};

stdenv.mkDerivation
{
    pname = "clippy";
    version = "1.0.0";

    buildInputs = [
        openssl
        pkg-config
    ];
    nativeBuildInputs = [
        rustup
    ];
}
