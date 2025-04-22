extern crate cc;
extern crate pkg_config;

use std::env;
use std::process::Command;

pub fn main() {
    Command::new("git").args(&["checkout", "rel-5-5-2"]).status().unwrap();

    env::set_current_dir("gtk").expect("Could not change dir to 'gtk'");

    let mut cc_build = cc::Build::new();
    cc_build
        .include("../include")
        .include("../lexlib")
        .include("../src")
        .opt_level(3)
        .cpp(true)
        .define("STATIC_BUILD", None)
        .flag("-fkeep-inline-functions")
        .file("../src/AutoComplete.cxx")
        .file("../src/CallTip.cxx")
        .file("../src/CaseConvert.cxx")
        .file("../src/CaseFolder.cxx")
        .file("../src/CellBuffer.cxx")
        .file("../src/ChangeHistory.cxx")
        .file("../src/CharacterCategoryMap.cxx")
        .file("../src/CharacterType.cxx")
        .file("../src/CharClassify.cxx")
        .file("../src/ContractionState.cxx")
        .file("../src/DBCS.cxx")
        .file("../src/Decoration.cxx")
        .file("../src/Document.cxx")
        .file("../src/EditModel.cxx")
        .file("../src/Editor.cxx")
        .file("../src/EditView.cxx")
        .file("../src/Geometry.cxx")
        .file("../src/Indicator.cxx")
        .file("../src/KeyMap.cxx")
        .file("../src/LineMarker.cxx")
        .file("../src/MarginView.cxx")
        .file("../src/PerLine.cxx")
        .file("../src/PositionCache.cxx")
        .file("../src/RESearch.cxx")
        .file("../src/RunStyles.cxx")
        .file("../src/ScintillaBase.cxx")
        .file("../src/Selection.cxx")
        .file("../src/Style.cxx")
        .file("../src/UniConversion.cxx")
        .file("../src/UniqueString.cxx")
        .file("../src/UndoHistory.cxx")
        .file("../src/ViewStyle.cxx")
        .file("../src/XPM.cxx");

    let gtk_probe = pkg_config::Config::new().atleast_version("3.0").probe("gtk+-3.0").unwrap();
    let glib_probe = pkg_config::Config::new().atleast_version("2.0").probe("glib-2.0").unwrap();

    for lib in gtk_probe.include_paths.as_slice() {
        cc_build.include(lib.to_str().unwrap());
    }
    for lib in glib_probe.include_paths.as_slice() {
        cc_build.include(lib.to_str().unwrap());
    }

    let mut cc_build_c = cc::Build::new();

    for lib in glib_probe.include_paths.as_slice() {
        cc_build_c.include(lib.to_str().unwrap());
    }
    cc_build_c.include(".").file("scintilla-marshal.c");

    cc_build_c.compile("scintilla_marshal");

    cc_build
        .include(".")
        .define("GTK", None)
        .define("SCI_LEXER", None)
        //.define("CHECK_DEPRECATED", None)
        //.define("NOTHREADS", None)
        .flag("--std=c++17")
        .file("PlatGTK.cxx")
        .file("ScintillaGTKAccessible.cxx")
        .file("ScintillaGTK.cxx");

    cc_build.compile("scilexer");

    println!("cargo:rustc-link-lib=dylib=gmodule-2.0");
    println!("cargo:rustc-link-lib=dylib=wayland-client");
}
