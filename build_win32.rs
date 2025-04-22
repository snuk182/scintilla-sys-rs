extern crate cc;
extern crate embed_resource;

use std::env;
use std::process::Command;

pub fn main() {
    #[cfg(all(feature = "win32", not(target_os = "windows")))]
    {
        panic!("The feature `win32` is available on the Windows platforms only.");
    }

    Command::new("git").args(&["checkout", "rel-5-5-2"]).status().unwrap();

    let mut cc_build = cc::Build::new();
    cc_build
        .include("../include")
        .include("../lexlib")
        .include("../src")
        //.cpp_link_stdlib("stdc++")
        .define("STATIC_BUILD", None)
        .opt_level(3)
        .cpp(true)
        //.debug(true)
        //.flag("-fkeep-inline-functions")
        .flag("-std=c++17")
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

    cc_build.file("PlatWin.cxx").file("HanjaDic.cxx").file("ScintillaWin.cxx");

    env::set_current_dir("win32").expect("Could not change dir");
    cc_build.compile("scilexer");

    embed_resource::compile("ScintRes.rc");

	println!("cargo:rustc-link-lib=dylib=imm32");
	println!("cargo:rustc-link-lib=dylib=ole32");
	println!("cargo:rustc-link-lib=dylib=oleaut32");
    println!("cargo:rustc-link-lib=dylib=uuid");
}
