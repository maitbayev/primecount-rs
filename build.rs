extern crate cmake;
use cmake::Config;
use std::env;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let dst = Config::new("libprimecount")
        .define("BUILD_PRIMECOUNT", "OFF")
        .build();
    let target_os = env::var("CARGO_CFG_TARGET_OS").unwrap();
    let target_env = env::var("CARGO_CFG_TARGET_ENV").unwrap();
    // now - emitting some cargo commands to build and link the lib
    println!("cargo:rustc-link-search=native={}/lib", dst.display());
    match (target_os.as_str(), target_env.as_str()) {
        ("linux", _) => println!("cargo:rustc-link-lib=dylib=stdc++"),
        ("macos", _) | ("ios", _) => println!("cargo:rustc-link-lib=dylib=c++"),
        ("windows", "msvc") => (),
        ("windows", "gnu") => println!("cargo:rustc-link-lib=dylib=stdc++"),
        _ => unimplemented!(),
    }
    println!("cargo:rustc-link-lib=static=primecount");
    println!("cargo:rustc-link-lib=static=primesieve");

    Ok(())
}
