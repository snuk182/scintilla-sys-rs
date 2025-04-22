extern crate cc;

#[cfg(target_os = "windows")]
extern crate embed_resource;

use std::env;
use std::path::Path;
use std::process::Command;

pub fn main() {
    Command::new("git").args(&["checkout", "rel-3-6-3"]).status().unwrap();

    build_pdcurses();
    build_scinterm();
}

fn build_pdcurses() {
    env::set_current_dir("../..").expect("Could not change dir");

    if !Path::new("sys/pdcurses/.git").exists() {
        Command::new("git").args(&["submodule", "update", "--init", "sys/pdcurses"]).status().unwrap();
    }
    env::set_current_dir("sys/pdcurses").expect("Could not change dir");

    let mut cc_build = cc::Build::new();
    cc_build
        .include(".")
        .flag("-Wno-unused-parameter")
        .flag("-std=c++17")
        .define("PDC_WIDE", None)
        .file("pdcurses/addch.c")
        .file("pdcurses/addchstr.c")
        .file("pdcurses/addstr.c")
        .file("pdcurses/attr.c")
        .file("pdcurses/beep.c")
        .file("pdcurses/bkgd.c")
        .file("pdcurses/border.c")
        .file("pdcurses/clear.c")
        .file("pdcurses/color.c")
        .file("pdcurses/debug.c")
        .file("pdcurses/delch.c")
        .file("pdcurses/deleteln.c")
        .file("pdcurses/deprec.c")
        .file("pdcurses/getch.c")
        .file("pdcurses/getstr.c")
        .file("pdcurses/getyx.c")
        .file("pdcurses/inch.c")
        .file("pdcurses/inchstr.c")
        .file("pdcurses/initscr.c")
        .file("pdcurses/inopts.c")
        .file("pdcurses/insch.c")
        .file("pdcurses/insstr.c")
        .file("pdcurses/instr.c")
        .file("pdcurses/kernel.c")
        .file("pdcurses/keyname.c")
        .file("pdcurses/mouse.c")
        .file("pdcurses/move.c")
        .file("pdcurses/outopts.c")
        .file("pdcurses/overlay.c")
        .file("pdcurses/pad.c")
        .file("pdcurses/panel.c")
        .file("pdcurses/printw.c")
        .file("pdcurses/refresh.c")
        .file("pdcurses/scanw.c")
        .file("pdcurses/scroll.c")
        .file("pdcurses/scr_dump.c")
        .file("pdcurses/slk.c")
        .file("pdcurses/termattr.c")
        .file("pdcurses/terminfo.c")
        .file("pdcurses/touch.c")
        .file("pdcurses/util.c")
        .file("pdcurses/window.c");

    #[cfg(target_os = "windows")]
    cc_build
        .include("wincon")
        .file("wincon/pdcclip.c")
        .file("wincon/pdcdisp.c")
        .file("wincon/pdcgetsc.c")
        .file("wincon/pdckbd.c")
        .file("wincon/pdcscrn.c")
        .file("wincon/pdcsetsc.c")
        .file("wincon/pdcutil.c");

    #[cfg(not(target_os = "windows"))]
    cc_build
        .include("x11")
        .flag("--enable-widec")
        .flag("--enable-xim")
        .file("x11/pdcclip.c")
        .file("x11/pdcdisp.c")
        .file("x11/pdcgetsc.c")
        .file("x11/pdckbd.c")
        .file("x11/pdcscrn.c")
        .file("x11/pdcsetsc.c")
        .file("x11/pdcutil.c")
        .file("x11/pdcx11.c")
        .file("x11/sb.c")
        .file("x11/ScrollBox.c")
        .file("x11/x11.c");

    cc_build.compile("pdcurses");

    #[cfg(target_os = "windows")]
    embed_resource::compile("wincon/pdcurses.rc");
}

fn build_scinterm() {
    env::set_current_dir("../..").expect("Could not change dir");

    if !Path::new("sys/scinterm/.git").exists() {
        Command::new("git").args(&["submodule", "update", "--init", "sys/scinterm"]).status().unwrap();
    }
    env::set_current_dir("sys/scinterm").expect("Could not change dir");

    let mut cc_build = cc::Build::new();

    cc_build
        .include("../scintilla/include")
        .include("../scintilla/src")
        .include("../scintilla/lexlib")
        .cpp_link_stdlib("stdc++")
        .define("STATIC_BUILD", None)
        .opt_level(3)
        .cpp(true)
        .warnings(false)
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

    cc_build.include("../pdcurses").define("CURSES", None).define("SCI_LEXER", None).flag("-pedantic").flag("-std=c++11").file("ScintillaTerm.cxx");

    cc_build.compile("scinterm");
}
