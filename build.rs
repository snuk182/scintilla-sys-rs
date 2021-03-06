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

#[cfg(feature = "qt5")]
mod build_qt;
#[cfg(feature = "qt5")]
use build_qt as inner;

#[cfg(feature = "gtk3")]
mod build_gtk;
#[cfg(feature = "gtk3")]
use build_gtk as inner;

#[cfg(feature = "curses")]
mod build_curses;
#[cfg(feature = "curses")]
use build_curses as inner;

#[allow(unreachable_code)]
fn main() {
    if !Path::new("sys/scintilla/.git").exists() {
        Command::new("git").args(&["submodule", "update", "--init", "--recursive"]).status().unwrap();
    }
    env::set_current_dir("sys/scintilla").expect("Could not change dir");

    inner::main();
}

#[cfg(not(any(feature = "win32", feature = "cocoa", feature = "qt5", feature = "gtk3", feature = "curses")))]
mod inner {
    pub fn main() {
	     println!("You have to pick a backend feature explicitly. For now available are \"win32\", \"cocoa\", \"gtk3\" and \"qt5\".");
    }
}
