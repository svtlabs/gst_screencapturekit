
fn main() {
    gst_plugin_version_helper::info();
      cc::Build::new()
        .include("/Library/Frameworks/GStreamer.framework/Versions/1.0/Headers")
        .file("src/applemedia/coremediabuffer.c")
        .compile("applemedia");
}

