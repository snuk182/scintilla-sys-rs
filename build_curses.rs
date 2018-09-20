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
        .file("../scintilla/src/AutoComplete.cxx")
        .file("../scintilla/src/CallTip.cxx")
        .file("../scintilla/src/CaseConvert.cxx")
        .file("../scintilla/src/CaseFolder.cxx")
        .file("../scintilla/src/Catalogue.cxx")
        .file("../scintilla/src/CellBuffer.cxx")
        .file("../scintilla/src/CharClassify.cxx")
        .file("../scintilla/src/ContractionState.cxx")
        .file("../scintilla/src/Decoration.cxx")
        .file("../scintilla/src/Document.cxx")
        .file("../scintilla/src/EditModel.cxx")
        .file("../scintilla/src/Editor.cxx")
        .file("../scintilla/src/EditView.cxx")
        .file("../scintilla/src/ExternalLexer.cxx")
        .file("../scintilla/src/Indicator.cxx")
        .file("../scintilla/src/KeyMap.cxx")
        .file("../scintilla/src/LineMarker.cxx")
        .file("../scintilla/src/MarginView.cxx")
        .file("../scintilla/src/PerLine.cxx")
        .file("../scintilla/src/PositionCache.cxx")
        .file("../scintilla/src/RESearch.cxx")
        .file("../scintilla/src/RunStyles.cxx")
        .file("../scintilla/src/ScintillaBase.cxx")
        .file("../scintilla/src/Selection.cxx")
        .file("../scintilla/src/Style.cxx")
        .file("../scintilla/src/UniConversion.cxx")
        .file("../scintilla/src/ViewStyle.cxx")
        .file("../scintilla/src/XPM.cxx")
        .file("../scintilla/lexlib/Accessor.cxx")
        .file("../scintilla/lexlib/CharacterCategory.cxx")
        .file("../scintilla/lexlib/CharacterSet.cxx")
        .file("../scintilla/lexlib/LexerBase.cxx")
        .file("../scintilla/lexlib/LexerModule.cxx")
        .file("../scintilla/lexlib/LexerNoExceptions.cxx")
        .file("../scintilla/lexlib/LexerSimple.cxx")
        .file("../scintilla/lexlib/PropSetSimple.cxx")
        .file("../scintilla/lexlib/StyleContext.cxx")
        .file("../scintilla/lexlib/WordList.cxx")
        .file("../scintilla/lexers/LexA68k.cxx")
        .file("../scintilla/lexers/LexAbaqus.cxx")
        .file("../scintilla/lexers/LexAda.cxx")
        .file("../scintilla/lexers/LexAPDL.cxx")
        .file("../scintilla/lexers/LexAsm.cxx")
        .file("../scintilla/lexers/LexAsn1.cxx")
        .file("../scintilla/lexers/LexASY.cxx")
        .file("../scintilla/lexers/LexAU3.cxx")
        .file("../scintilla/lexers/LexAVE.cxx")
        .file("../scintilla/lexers/LexAVS.cxx")
        .file("../scintilla/lexers/LexBaan.cxx")
        .file("../scintilla/lexers/LexBash.cxx")
        .file("../scintilla/lexers/LexBasic.cxx")
        .file("../scintilla/lexers/LexBatch.cxx")
        .file("../scintilla/lexers/LexBibTeX.cxx")
        .file("../scintilla/lexers/LexBullant.cxx")
        .file("../scintilla/lexers/LexCaml.cxx")
        .file("../scintilla/lexers/LexCLW.cxx")
        .file("../scintilla/lexers/LexCmake.cxx")
        .file("../scintilla/lexers/LexCOBOL.cxx")
        .file("../scintilla/lexers/LexCoffeeScript.cxx")
        .file("../scintilla/lexers/LexConf.cxx")
        .file("../scintilla/lexers/LexCPP.cxx")
        .file("../scintilla/lexers/LexCrontab.cxx")
        .file("../scintilla/lexers/LexCsound.cxx")
        .file("../scintilla/lexers/LexCSS.cxx")
        .file("../scintilla/lexers/LexD.cxx")
        .file("../scintilla/lexers/LexDiff.cxx")
        .file("../scintilla/lexers/LexDMAP.cxx")
        .file("../scintilla/lexers/LexDMIS.cxx")
        .file("../scintilla/lexers/LexECL.cxx")
        .file("../scintilla/lexers/LexEiffel.cxx")
        .file("../scintilla/lexers/LexErlang.cxx")
        .file("../scintilla/lexers/LexErrorList.cxx")
        .file("../scintilla/lexers/LexEScript.cxx")
        .file("../scintilla/lexers/LexFlagship.cxx")
        .file("../scintilla/lexers/LexForth.cxx")
        .file("../scintilla/lexers/LexFortran.cxx")
        .file("../scintilla/lexers/LexGAP.cxx")
        .file("../scintilla/lexers/LexGui4Cli.cxx")
        .file("../scintilla/lexers/LexHaskell.cxx")
        .file("../scintilla/lexers/LexHex.cxx")
        .file("../scintilla/lexers/LexHTML.cxx")
        .file("../scintilla/lexers/LexInno.cxx")
        .file("../scintilla/lexers/LexKix.cxx")
        .file("../scintilla/lexers/LexKVIrc.cxx")
        .file("../scintilla/lexers/LexLaTeX.cxx")
        .file("../scintilla/lexers/LexLisp.cxx")
        .file("../scintilla/lexers/LexLout.cxx")
        .file("../scintilla/lexers/LexLua.cxx")
        .file("../scintilla/lexers/LexMagik.cxx")
        .file("../scintilla/lexers/LexMake.cxx")
        .file("../scintilla/lexers/LexMarkdown.cxx")
        .file("../scintilla/lexers/LexMatlab.cxx")
        .file("../scintilla/lexers/LexMetapost.cxx")
        .file("../scintilla/lexers/LexMMIXAL.cxx")
        .file("../scintilla/lexers/LexModula.cxx")
        .file("../scintilla/lexers/LexMPT.cxx")
        .file("../scintilla/lexers/LexMSSQL.cxx")
        .file("../scintilla/lexers/LexMySQL.cxx")
        .file("../scintilla/lexers/LexNimrod.cxx")
        .file("../scintilla/lexers/LexNsis.cxx")
        .file("../scintilla/lexers/LexNull.cxx")
        .file("../scintilla/lexers/LexOpal.cxx")
        .file("../scintilla/lexers/LexOScript.cxx")
        .file("../scintilla/lexers/LexPascal.cxx")
        .file("../scintilla/lexers/LexPB.cxx")
        .file("../scintilla/lexers/LexPerl.cxx")
        .file("../scintilla/lexers/LexPLM.cxx")
        .file("../scintilla/lexers/LexPO.cxx")
        .file("../scintilla/lexers/LexPOV.cxx")
        .file("../scintilla/lexers/LexPowerPro.cxx")
        .file("../scintilla/lexers/LexPowerShell.cxx")
        .file("../scintilla/lexers/LexProgress.cxx")
        .file("../scintilla/lexers/LexProps.cxx")
        .file("../scintilla/lexers/LexPS.cxx")
        .file("../scintilla/lexers/LexPython.cxx")
        .file("../scintilla/lexers/LexR.cxx")
        .file("../scintilla/lexers/LexRebol.cxx")
        .file("../scintilla/lexers/LexRegistry.cxx")
        .file("../scintilla/lexers/LexRuby.cxx")
        .file("../scintilla/lexers/LexRust.cxx")
        .file("../scintilla/lexers/LexScriptol.cxx")
        .file("../scintilla/lexers/LexSmalltalk.cxx")
        .file("../scintilla/lexers/LexSML.cxx")
        .file("../scintilla/lexers/LexSorcus.cxx")
        .file("../scintilla/lexers/LexSpecman.cxx")
        .file("../scintilla/lexers/LexSpice.cxx")
        .file("../scintilla/lexers/LexSQL.cxx")
        .file("../scintilla/lexers/LexSTTXT.cxx")
        .file("../scintilla/lexers/LexTACL.cxx")
        .file("../scintilla/lexers/LexTADS3.cxx")
        .file("../scintilla/lexers/LexTAL.cxx")
        .file("../scintilla/lexers/LexTCL.cxx")
        .file("../scintilla/lexers/LexTCMD.cxx")
        .file("../scintilla/lexers/LexTeX.cxx")
        .file("../scintilla/lexers/LexTxt2tags.cxx")
        .file("../scintilla/lexers/LexVB.cxx")
        .file("../scintilla/lexers/LexVerilog.cxx")
        .file("../scintilla/lexers/LexVHDL.cxx")
        .file("../scintilla/lexers/LexVisualProlog.cxx")
        .file("../scintilla/lexers/LexYAML.cxx");

    cc_build.include("../pdcurses").define("CURSES", None).define("SCI_LEXER", None).flag("-pedantic").flag("-std=c++11").file("ScintillaTerm.cxx");

    cc_build.compile("scinterm");
}
