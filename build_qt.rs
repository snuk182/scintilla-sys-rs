extern crate cc;

use std::env;
use std::process::Command;

pub fn main() {
    Command::new("git").args(&["checkout", "rel-4-0-2"]).status().unwrap();

    let mut qt_location = String::from_utf8(Command::new("qmake").args(&["-query", "QT_INSTALL_HEADERS"]).output().unwrap().stdout).unwrap();
    let len = qt_location.len();
    qt_location.split_off(len - 1);

    env::set_current_dir("qt").expect("Could not change dir to 'qt'");

    Command::new("moc")
        .args(&[
            "-I../include",
            "-I../lexlib",
            "-I../src",
            "-IScintillaEditBase",
            "-I../../qt",
            &format!("-I{}", qt_location),
            &format!("-I{}/QtCore", qt_location),
            &format!("-I{}/QtGui", qt_location),
            &format!("-I{}/QtWidgets", qt_location),
            "ScintillaEditBase/ScintillaQt.h",
            "-o",
            "ScintillaEditBase/moc_ScintillaQt.cpp",
        ]).status()
        .unwrap();

    Command::new("moc")
        .args(&[
            "-I../include",
            "-I../lexlib",
            "-I../src",
            "-IScintillaEditBase",
            "-I../../qt",
            &format!("-I{}", qt_location),
            &format!("-I{}/QtCore", qt_location),
            &format!("-I{}/QtGui", qt_location),
            &format!("-I{}/QtWidgets", qt_location),
            "ScintillaEditBase/ScintillaEditBase.h",
            "-o",
            "ScintillaEditBase/moc_ScintillaEditBase.cpp",
        ]).status()
        .unwrap();

    Command::new("moc")
        .args(&[
            "-I../include",
            "-I../lexlib",
            "-I../src",
            "-IScintillaEditBase",
            "-I../../qt",
            &format!("-I{}", qt_location),
            &format!("-I{}/QtCore", qt_location),
            &format!("-I{}/QtGui", qt_location),
            &format!("-I{}/QtWidgets", qt_location),
            "../../qt/qt_widgets_c_scintilla_ScintillaEditBase.h",
            "-o",
            "../../qt/moc_qt_widgets_c_scintilla_ScintillaEditBase.cpp",
        ]).status()
        .unwrap();

    let mut cc_build = cc::Build::new();
    cc_build
        .include("../include")
        .include("../lexlib")
        .include("../src")
        .opt_level(3)
        .cpp(true)
        .define("STATIC_BUILD", None)
        //.debug(true)
        .flag("-fkeep-inline-functions")
        .file("../src/AutoComplete.cxx")
        .file("../src/CallTip.cxx")
        .file("../src/CaseConvert.cxx")
        .file("../src/CaseFolder.cxx")
        .file("../src/Catalogue.cxx")
        .file("../src/CellBuffer.cxx")
        .file("../src/CharClassify.cxx")
        .file("../src/ContractionState.cxx")
        .file("../src/DBCS.cxx")
        .file("../src/Decoration.cxx")
        .file("../src/Document.cxx")
        .file("../src/EditModel.cxx")
        .file("../src/Editor.cxx")
        .file("../src/EditView.cxx")
        .file("../src/ExternalLexer.cxx")
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
        .file("../src/ViewStyle.cxx")
        .file("../src/XPM.cxx")
        .file("../lexlib/Accessor.cxx")
        .file("../lexlib/CharacterCategory.cxx")
        .file("../lexlib/CharacterSet.cxx")
        .file("../lexlib/DefaultLexer.cxx")
        .file("../lexlib/LexerBase.cxx")
        .file("../lexlib/LexerModule.cxx")
        .file("../lexlib/LexerNoExceptions.cxx")
        .file("../lexlib/LexerSimple.cxx")
        .file("../lexlib/PropSetSimple.cxx")
        .file("../lexlib/StyleContext.cxx")
        .file("../lexlib/WordList.cxx")
        .file("../lexers/LexA68k.cxx")
        .file("../lexers/LexAbaqus.cxx")
        .file("../lexers/LexAda.cxx")
        .file("../lexers/LexAPDL.cxx")
        .file("../lexers/LexAsm.cxx")
        .file("../lexers/LexAsn1.cxx")
        .file("../lexers/LexASY.cxx")
        .file("../lexers/LexAU3.cxx")
        .file("../lexers/LexAVE.cxx")
        .file("../lexers/LexAVS.cxx")
        .file("../lexers/LexBaan.cxx")
        .file("../lexers/LexBash.cxx")
        .file("../lexers/LexBasic.cxx")
        .file("../lexers/LexBatch.cxx")
        .file("../lexers/LexBibTeX.cxx")
        .file("../lexers/LexBullant.cxx")
        .file("../lexers/LexCaml.cxx")
        .file("../lexers/LexCLW.cxx")
        .file("../lexers/LexCmake.cxx")
        .file("../lexers/LexCOBOL.cxx")
        .file("../lexers/LexCoffeeScript.cxx")
        .file("../lexers/LexConf.cxx")
        .file("../lexers/LexCPP.cxx")
        .file("../lexers/LexCrontab.cxx")
        .file("../lexers/LexCsound.cxx")
        .file("../lexers/LexCSS.cxx")
        .file("../lexers/LexD.cxx")
        .file("../lexers/LexDiff.cxx")
        .file("../lexers/LexDMAP.cxx")
        .file("../lexers/LexDMIS.cxx")
        .file("../lexers/LexECL.cxx")
        .file("../lexers/LexEDIFACT.cxx")
        .file("../lexers/LexEiffel.cxx")
        .file("../lexers/LexErlang.cxx")
        .file("../lexers/LexErrorList.cxx")
        .file("../lexers/LexEScript.cxx")
        .file("../lexers/LexFlagship.cxx")
        .file("../lexers/LexForth.cxx")
        .file("../lexers/LexFortran.cxx")
        .file("../lexers/LexGAP.cxx")
        .file("../lexers/LexGui4Cli.cxx")
        .file("../lexers/LexHaskell.cxx")
        .file("../lexers/LexHex.cxx")
        .file("../lexers/LexHTML.cxx")
        .file("../lexers/LexIndent.cxx")
        .file("../lexers/LexInno.cxx")
        .file("../lexers/LexJSON.cxx")
        .file("../lexers/LexKix.cxx")
        .file("../lexers/LexKVIrc.cxx")
        .file("../lexers/LexLaTeX.cxx")
        .file("../lexers/LexLisp.cxx")
        .file("../lexers/LexLout.cxx")
        .file("../lexers/LexLua.cxx")
        .file("../lexers/LexMagik.cxx")
        .file("../lexers/LexMake.cxx")
        .file("../lexers/LexMarkdown.cxx")
        .file("../lexers/LexMatlab.cxx")
        .file("../lexers/LexMetapost.cxx")
        .file("../lexers/LexMMIXAL.cxx")
        .file("../lexers/LexModula.cxx")
        .file("../lexers/LexMPT.cxx")
        .file("../lexers/LexMSSQL.cxx")
        .file("../lexers/LexMySQL.cxx")
        .file("../lexers/LexNimrod.cxx")
        .file("../lexers/LexNsis.cxx")
        .file("../lexers/LexNull.cxx")
        .file("../lexers/LexOpal.cxx")
        .file("../lexers/LexOScript.cxx")
        .file("../lexers/LexPascal.cxx")
        .file("../lexers/LexPB.cxx")
        .file("../lexers/LexPerl.cxx")
        .file("../lexers/LexPLM.cxx")
        .file("../lexers/LexPO.cxx")
        .file("../lexers/LexPOV.cxx")
        .file("../lexers/LexPowerPro.cxx")
        .file("../lexers/LexPowerShell.cxx")
        .file("../lexers/LexProgress.cxx")
        .file("../lexers/LexProps.cxx")
        .file("../lexers/LexPS.cxx")
        .file("../lexers/LexPython.cxx")
        .file("../lexers/LexR.cxx")
        .file("../lexers/LexRebol.cxx")
        .file("../lexers/LexRegistry.cxx")
        .file("../lexers/LexRuby.cxx")
        .file("../lexers/LexRust.cxx")
        .file("../lexers/LexScriptol.cxx")
        .file("../lexers/LexSmalltalk.cxx")
        .file("../lexers/LexSML.cxx")
        .file("../lexers/LexSorcus.cxx")
        .file("../lexers/LexSpecman.cxx")
        .file("../lexers/LexSpice.cxx")
        .file("../lexers/LexSQL.cxx")
        .file("../lexers/LexSTTXT.cxx")
        .file("../lexers/LexTACL.cxx")
        .file("../lexers/LexTADS3.cxx")
        .file("../lexers/LexTAL.cxx")
        .file("../lexers/LexTCL.cxx")
        .file("../lexers/LexTCMD.cxx")
        .file("../lexers/LexTeX.cxx")
        .file("../lexers/LexTxt2tags.cxx")
        .file("../lexers/LexVB.cxx")
        .file("../lexers/LexVerilog.cxx")
        .file("../lexers/LexVHDL.cxx")
        .file("../lexers/LexVisualProlog.cxx")
        .file("../lexers/LexYAML.cxx");

    cc_build
        .include("ScintillaEditBase")
        .include("../../qt")
        .flag("-isystem")
        .flag(&format!("{}", qt_location))
        .flag("-isystem")
        .flag(&format!("{}/QtCore", qt_location))
        .flag("-isystem")
        .flag(&format!("{}/QtGui", qt_location))
        .flag("-isystem")
        .flag(&format!("{}/QtWidgets", qt_location))
        .flag("-isystem")
        .flag(&format!("{}/mkspecs/linux-g++-64", qt_location))
        .define("SCINTILLA_QT", Some("1"))
        .define("MAKING_LIBRARY", Some("1"))
        .define("SCI_LEXER", Some("1"))
        .define("_CRT_SECURE_NO_DEPRECATE", Some("1"))
        //.define("QT_NO_DEBUG", None)
        .define("QT_WIDGETS_LIB", None)
        .define("QT_GUI_LIB", None)
        .define("QT_CORE_LIB", None)
        .flag("-std=c++14")
        .file("ScintillaEditBase/moc_ScintillaQt.cpp")
        .file("ScintillaEditBase/moc_ScintillaEditBase.cpp")
        .file("ScintillaEditBase/ScintillaEditBase.cpp")
        .file("ScintillaEditBase/ScintillaQt.cpp")
        .file("ScintillaEditBase/PlatQt.cpp")
        .file("../../qt/qt_widgets_c_scintilla_ScintillaEditBase.cpp")
        .file("../../qt/moc_qt_widgets_c_scintilla_ScintillaEditBase.cpp");

    cc_build.compile("scilexer");
}
