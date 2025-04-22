extern crate cc;

use std::env;
use std::process::Command;

pub fn main() {
    Command::new("git").args(&["checkout", "rel-5-5-2"]).status().unwrap();

    let mut qt_location = String::from_utf8(Command::new("qmake").args(&["-query", "QT_INSTALL_HEADERS"]).output().unwrap().stdout).unwrap();
    let len = qt_location.len();
    let _ = qt_location.split_off(len - 1);

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
        .flag("-std=c++17")
        .file("ScintillaEditBase/moc_ScintillaQt.cpp")
        .file("ScintillaEditBase/moc_ScintillaEditBase.cpp")
        .file("ScintillaEditBase/ScintillaEditBase.cpp")
        .file("ScintillaEditBase/ScintillaQt.cpp")
        .file("ScintillaEditBase/PlatQt.cpp")
        .file("../../qt/qt_widgets_c_scintilla_ScintillaEditBase.cpp")
        .file("../../qt/moc_qt_widgets_c_scintilla_ScintillaEditBase.cpp");

    cc_build.compile("scilexer");
}
