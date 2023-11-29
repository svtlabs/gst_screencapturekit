// use gst::glib;
// use gst::prelude::*;

// mod sc_gst_meta;
// mod sc_gst_src;
// mod sc_gst_enc;
pub mod video_toolbox;
// The public Rust wrapper type for our element
// glib::wrapper! {
//     pub struct ScreenCaptureSrc(ObjectSubclass<sc_gst_src::ScreenCaptureSrc>) @extends gst_base::PushSrc, gst_base::BaseSrc, gst::Element, gst::Object;
// }
//
// // glib::wrapper! {
// //     pub struct ScreenCaptureEnc(ObjectSubclass<sc_gst_enc::SCGstEnc>) @extends gst_video::VideoEncoder, gst::Element, gst::Object;
// // }
//
//
// pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
//     // gst::Element::register( Some(plugin),
//     //     "screencapture-enc",
//     //     gst::Rank::None,
//     //     ScreenCaptureEnc::static_type(),
//     // );
//     gst::Element::register(
//         Some(plugin),
//         "screencapture-src",
//         gst::Rank::Primary,
//         ScreenCaptureSrc::static_type(),
//     )
//   }
