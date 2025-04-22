extern crate cc;
extern crate fs_extra;

use std::env;
use std::path;
use std::process::Command;

pub fn main() {
    #[cfg(all(feature = "cocoa", not(target_os = "macos")))]
    {
        panic!("The feature `cocoa` is available on the OSX/MacOS platforms only.");
    }
    Command::new("git").args(&["checkout", "rel-5-5-2"]).status().unwrap();

    env::set_current_dir("cocoa/ScintillaFramework").expect("Could not change dir");
    Command::new("xcodebuild").status().unwrap();

    let mut framework_dir = env::current_dir().unwrap();
    framework_dir.push("build/Release");
    println!("cargo:rustc-link-search=framework={}", framework_dir.display());
    framework_dir.push("Scintilla.framework");

    let mut out_dir = path::PathBuf::from(env::var("OUT_DIR").unwrap());
    out_dir.push("../../../../Frameworks");

    fs_extra::dir::create_all(out_dir.clone(), true).unwrap();

    let mut opts = fs_extra::dir::CopyOptions::new();
    opts.overwrite = true;
    fs_extra::dir::copy(framework_dir, out_dir, &opts).unwrap();

    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=Appkit");
    println!("cargo:rustc-link-lib=framework=Scintilla");
}
