extern crate cmake;
use cmake::Config;

fn main() {
    let dst = Config::new("libprimecount").build();

    // now - emitting some cargo commands to build and link the lib
    println!("cargo:rustc-link-search=native={}", dst.display());
    // Phase `foo` here stands for the library name (without lib prefix and without .a suffix)
    println!("cargo:rustc-link-lib=static=primecount");
}
