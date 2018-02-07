extern crate cc;
extern crate embed_resource;

use std::env;
use std::process::Command;

pub fn main() {
    #[cfg(all(feature = "win32", not(target_os = "windows")))]
    {
        panic!("The feature `win32` is available on the Windows platforms only.");
    }

    Command::new("git")
        .args(&["checkout", "rel-4-0-2"])
        .status()
        .unwrap();

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
        .warnings(false)
        
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
        .file("PlatWin.cxx")
        .file("HanjaDic.cxx")
        .file("ScintillaWin.cxx");
        
    env::set_current_dir("win32").expect("Could not change dir");
    cc_build.compile("scilexer");

    embed_resource::compile("ScintRes.rc");
    
    println!("cargo:rustc-link-lib=dylib=imm32");
    println!("cargo:rustc-link-lib=dylib=uuid");
}
