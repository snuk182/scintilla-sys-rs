

pub fn main() {
    #[cfg(all(feature = "cocoa", not(target_os = "macos")))]
    {
        panic!("The feature `cocoa` is available on the OSX/MacOS platforms only.");
    }

    env::set_current_dir("cocoa").expect("Could not change dir");

    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=Appkit");
}
