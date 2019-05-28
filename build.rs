extern crate curl;
extern crate gl_generator;

use curl::easy::Easy;
use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::path::Path;
use std::process::Command;
use std::fs;
use std::fs::File;
use std::io::Write;
use std::io;

fn main() {
    let profile = env::var("PROFILE").unwrap_or("Debug".to_string());
    let current_dir = std::env::current_dir().unwrap();
    let target;
    let target_os = env::var("TARGET").unwrap();
    const SDL2_FILENAME: &'static str = "SDL2-2.0.9.zip";
    const SDL2_URL: &'static str = "https://www.libsdl.org/release/SDL2-2.0.9.zip";
    const SDL2_PATH: &'static str = "SDL2-2.0.9";

    if profile == "Release" {
        target = Path::new(&current_dir).join("target/release");
    } else {
        target = Path::new(&current_dir).join("target/debug");
    }

    let dest = env::var("OUT_DIR").unwrap();
    let mut file = File::create(&Path::new(&dest).join("bindings.rs")).unwrap();

    Registry::new(Api::Gles2, (2, 0), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();

    // Download SDL if needed
    if !Path::new(SDL2_FILENAME).exists() {
        download_from_url(SDL2_URL, SDL2_FILENAME);
    }

    if !Path::new(SDL2_PATH).exists() {
        unzip_file(SDL2_FILENAME);
    }
}

fn download_from_url(url: &str, dst_file: &str) {
    File::create(dst_file).and_then(|mut file| {
        let mut curl = Easy::new();
        curl.url(url).expect("Error setting url");
        curl.write_function(move |data| Ok(file.write(data).expect("Error writing data")))
            .expect("Error setting write function");
        curl.perform().expect("Error downloading archive");
        Ok(())
    }).expect("Could not open output file");
}

fn unzip_file(filename: &str) {
    Command::new("unzip")
    .args(&[filename])
    .status()
    .expect("Error unzipping SDL2");
}