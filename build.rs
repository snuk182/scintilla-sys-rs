use std::env;
use std::path::Path;
use std::process::Command;

#[cfg(all(feature = "win32", target_os = "windows"))]
mod build_win32;
#[cfg(all(feature = "win32", target_os = "windows"))]
use build_win32 as inner;

#[cfg(all(feature = "cocoa", target_os = "macos"))]
mod build_cocoa;
#[cfg(all(feature = "cocoa", target_os = "macos"))]
use build_cocoa as inner;

#[cfg(feature = "qt")]
mod build_qt;
#[cfg(feature = "qt")]
use build_qt as inner;

#[cfg(feature = "gtk")]
mod build_gtk;
#[cfg(feature = "gtk")]
use build_gtk as inner;

#[cfg(feature = "curses")]
mod build_curses;
#[cfg(feature = "curses")]
use build_curses as inner;

#[allow(unreachable_code)]
fn main() {
    #[cfg(not(any(feature = "win32", feature = "cocoa", feature = "qt", feature = "gtk", feature = "curses")))]
    {
        panic!("You have to pick a backend feature explicitly. For now available are \"win32\" and \"curses\".");
    }

    if !Path::new("sys/scintilla/.git").exists() {
        Command::new("git")
            .args(&["submodule", "update", "--init", "sys/scintilla"])
            .status()
            .unwrap();
    }
    env::set_current_dir("sys/scintilla").expect("Could not change dir");

    inner::main();

    //println!("cargo:rustc-link-lib=scilexer");
}

#[cfg(not(any(feature = "win32", feature = "cocoa", feature = "qt", feature = "gtk", feature = "curses")))]
mod inner {
    pub fn main() {}
}
