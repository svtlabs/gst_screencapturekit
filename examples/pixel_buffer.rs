use std::{fs::File, io::Write, process::Command};

use gst_screencapturekit::{
    self,
    screencast::video_toolbox::{create_picture_of_buffer, create_pixel_buffer},
};
use objc_foundation::INSData;
fn main() {
    unsafe {
        let px = create_pixel_buffer();
        let jpeg = create_picture_of_buffer(px);
        let mut buffer = File::create("picture.jpg").unwrap();

        buffer.write_all(jpeg.bytes()).unwrap();
        Command::new("open")
            .args(["picture.jpg"])
            .output()
            .expect("failedto execute process");
    };
}
