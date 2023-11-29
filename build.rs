fn main() {
    // gst_plugin_version_helper::info();
    println!("cargo:rustc-link-lib=framework=CoreFoundation");
    println!("cargo:rustc-link-lib=framework=CoreMedia");
    println!("cargo:rustc-link-lib=framework=CoreVideo");
    println!("cargo:rustc-link-lib=framework=Foundation");
    println!("cargo:rustc-link-lib=framework=CoreGraphics");
    println!("cargo:rustc-link-lib=framework=CoreImage");
    println!("cargo:rustc-link-lib=framework=ImageIO");
    cc::Build::new()
        .include("/Library/Frameworks/GStreamer.framework/Versions/1.0/Headers")
        .file("src/applemedia/coremediabuffer.c")
        .compile("applemedia");
}
