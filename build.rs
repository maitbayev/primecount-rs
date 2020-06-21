extern crate cmake;
use cmake::Config;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let dst = Config::new("libprimecount")
        .define("BUILD_PRIMECOUNT", "OFF")
        .build();

    // now - emitting some cargo commands to build and link the lib
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    match env::var("TARGET")? {
        t if t.contains("apple") => println!("cargo:rustc-link-lib=dylib=c++"),
        t if t.contains("linux") => println!("cargo:rustc-link-lib=dylib=stdc++"),
        _ => unimplemented!(),
    }
    println!("cargo:rustc-link-lib=static=primecount");
    println!("cargo:rustc-link-lib=static=primesieve");

    Ok(())
}
