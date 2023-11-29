// // Copyright (C) 2019 Sebastian Dröge <sebastian@centricular.com>
// //
// // Licensed under the Apache License, Version 2.0 <LICENSE-APACHE or
// // http://www.apache.org/licenses/LICENSE-2.0> or the MIT license
// // <LICENSE-MIT or http://opensource.org/licenses/MIT>, at your
// // option. This file may not be copied, modified, or distributed
// // except according to those terms.
// //
// // SPDX-License-Identifier: MIT OR Apache-2.0
//
// use atomic_refcell::AtomicRefCell;
// use gst::glib;
// use gst::glib::once_cell::sync::Lazy;
// use gst::subclass::prelude::*;
// use gst_video::prelude::*;
// use gst_video::subclass::prelude::*;
// use std::sync::Mutex;
//
// const DEFAULT_BITRATE: u32 = 0;
// const DEFAULT_FRAME_REORDERING: bool = false;
// const DEFAULT_REALTIME: bool = false;
// const DEFAULT_QUALITY: f32 = 0.5;
// const DEFAULT_MAX_KEYFRAME_INTERVAL: u32 = 0;
// const DEFAULT_MAX_KEYFRAME_INTERVAL_DURATION: u32 = 0;
// const DEFAULT_PRESERVE_ALPHA: bool = true;
// const DEFAULT_OUTPUT_QUEUE_SIZE: u32 = 3;
//
// #[derive(Debug, Clone, Copy)]
// struct Settings {
//     bitrate: u32,
//     allow_frame_reordering: bool,
//     realtime: bool,
//     quality: f32,
//     max_keyframe_interval: u32,
//     max_keyframe_interval_duration: u32,
//     preserve_alpha: bool,
//     queue_size: u32,
// }
//
// impl Default for Settings {
//     fn default() -> Self {
//         Settings {
//             bitrate: DEFAULT_BITRATE,
//             allow_frame_reordering: DEFAULT_FRAME_REORDERING,
//             realtime: DEFAULT_REALTIME,
//             quality: DEFAULT_QUALITY,
//             max_keyframe_interval: DEFAULT_MAX_KEYFRAME_INTERVAL,
//             max_keyframe_interval_duration: DEFAULT_MAX_KEYFRAME_INTERVAL_DURATION,
//             preserve_alpha: DEFAULT_PRESERVE_ALPHA,
//             queue_size: DEFAULT_OUTPUT_QUEUE_SIZE,
//         }
//     }
// }
//
// struct Context {}
//
// impl Context {
//     fn receive_packet(&mut self) {
//         // Todo
//         todo!()
//     }
//
//     fn send_frame(
//         &mut self,
//         frame_number: u32,
//         in_frame: &gst_video::VideoFrameRef<&gst::BufferRef>,
//         force_keyframe: bool,
//     ) {
//         todo!()
//     }
//
//     fn flush(&mut self) {
//         todo!()
//     }
// }
//
// struct State {
//     context: Context,
//     video_info: gst_video::VideoInfo,
// }
//
// #[derive(Default)]
// pub struct SCGstEnc {
//     state: AtomicRefCell<Option<State>>,
//     settings: Mutex<Settings>,
// }
//
// static CAT: Lazy<gst::DebugCategory> = Lazy::new(|| {
//     gst::DebugCategory::new(
//         "screencapture-enc",
//         gst::DebugColorFlags::empty(),
//         Some("ScreenCaptureKit Encode using VideoToolbox"),
//     )
// });
//
// #[glib::object_subclass]
// impl ObjectSubclass for SCGstEnc {
//     const NAME: &'static str = "ScreenCaptureKitEnc";
//     type Type = super::ScreenCaptureEnc;
//     type ParentType = gst_video::VideoEncoder;
// }
//
// impl ObjectImpl for SCGstEnc {
//     fn properties() -> &'static [glib::ParamSpec] {
//         static PROPERTIES: Lazy<Vec<glib::ParamSpec>> = Lazy::new(|| vec![]);
//
//         PROPERTIES.as_ref()
//     }
//
//     fn set_property(&self, _id: usize, value: &glib::Value, pspec: &glib::ParamSpec) {}
//
//     fn property(&self, _id: usize, pspec: &glib::ParamSpec) -> glib::Value {}
// }
//
// impl GstObjectImpl for SCGstEnc {}
//
// impl ElementImpl for SCGstEnc {
//     fn metadata() -> Option<&'static gst::subclass::ElementMetadata> {
//         static ELEMENT_METADATA: Lazy<gst::subclass::ElementMetadata> = Lazy::new(|| {
//             gst::subclass::ElementMetadata::new(
//                 "ScreenCaptureKit Source",
//                 "Encoder/ScreenCapture",
//                 "Encodes media from ScreenCaptureKit",
//                 "Per Johansson <per@doom.fish>",
//             )
//         });
//         Some(&*ELEMENT_METADATA)
//     }
//
//     fn pad_templates() -> &'static [gst::PadTemplate] {
//         static PAD_TEMPLATES: Lazy<Vec<gst::PadTemplate>> = Lazy::new(|| {
//             let sink_caps = gst_video::VideoCapsBuilder::new()
//                 .format_list([gst_video::VideoFormat::Nv12])
//                 .build();
//             let sink_pad_template = gst::PadTemplate::new(
//                 "sink",
//                 gst::PadDirection::Sink,
//                 gst::PadPresence::Always,
//                 &sink_caps,
//             )
//             .unwrap();
//
//             let src_caps = gst::Caps::builder("video/x-av1")
//                 .field("stream-format", "obu-stream")
//                 .field("alignment", "tu")
//                 .build();
//             let src_pad_template = gst::PadTemplate::new(
//                 "src",
//                 gst::PadDirection::Src,
//                 gst::PadPresence::Always,
//                 &src_caps,
//             )
//             .unwrap();
//
//             vec![src_pad_template, sink_pad_template]
//         });
//
//         PAD_TEMPLATES.as_ref()
//     }
// }
//
// impl VideoEncoderImpl for SCGstEnc {
//     fn stop(&self) -> Result<(), gst::ErrorMessage> {
//         *self.state.borrow_mut() = None;
//         Ok(())
//     }
//
//     fn propose_allocation(
//         &self,
//         query: &mut gst::query::Allocation,
//     ) -> Result<(), gst::LoggableError> {
//         query.add_allocation_meta::<gst_video::VideoMeta>(None);
//         self.parent_propose_allocation(query)
//     }
//
//     // For the colorimetry mapping below
//     #[allow(clippy::wildcard_in_or_patterns)]
//     fn set_format(
//         &self,
//         state: &gst_video::VideoCodecState<'static, gst_video::video_codec_state::Readable>,
//     ) -> Result<(), gst::LoggableError> {
//         self.finish()
//             .map_err(|_| gst::loggable_error!(CAT, "Failed to drain"))?;
//
//         let video_info = state.info();
//         gst::debug!(CAT, imp: self, "Setting format {:?}", video_info);
//
//         let settings = self.settings.lock().unwrap();
//
//         // let container_sequence_header =
//         //     gst::Buffer::from_mut_slice(context.container_sequence_header());
//         //
//         // *self.state.borrow_mut() = Some(State {
//             // context,
//             // video_info,
//         // });
//
//         let instance = self.obj();
//         let output_state = instance
//             .set_output_state(
//                 gst::Caps::builder("video/x-av1")
//                     .field("stream-format", "obu-stream")
//                     .field("alignment", "tu")
//                     // .field("codec_data", container_sequence_header)
//                     .build(),
//                 Some(state),
//             )
//             .map_err(|_| gst::loggable_error!(CAT, "Failed to set output state"))?;
//         instance
//             .negotiate(output_state)
//             .map_err(|_| gst::loggable_error!(CAT, "Failed to negotiate"))?;
//
//         self.parent_set_format(state)
//     }
//
//     fn flush(&self) -> bool {
//         gst::debug!(CAT, imp: self, "Flushing");
//
//         let mut state_guard = self.state.borrow_mut();
//         // if let Some(ref mut state) = *state_guard {
//         //     state.context.flush();
//         //     while let Ok(_) | Err(data::EncoderStatus::Encoded) = state.context.receive_packet() {
//         //         gst::debug!(CAT, imp: self, "Dropping packet on flush",);
//         //     }
//         // }
//
//         true
//     }
//
//     fn finish(&self) -> Result<gst::FlowSuccess, gst::FlowError> {
//         gst::debug!(CAT, imp: self, "Finishing");
//
//         let mut state_guard = self.state.borrow_mut();
//         if let Some(ref mut state) = *state_guard {
//             state.context.flush();
//             self.output_frames(state)?;
//         }
//
//         Ok(gst::FlowSuccess::Ok)
//     }
//
//     fn handle_frame(
//         &self,
//         frame: gst_video::VideoCodecFrame,
//     ) -> Result<gst::FlowSuccess, gst::FlowError> {
//         let mut state_guard = self.state.borrow_mut();
//         let state = state_guard.as_mut().ok_or(gst::FlowError::NotNegotiated)?;
//
//         self.output_frames(state)?;
//
//         gst::debug!(
//             CAT,
//             imp: self,
//             "Sending frame {}",
//             frame.system_frame_number()
//         );
//
//         let input_buffer = frame.input_buffer().expect("frame without input buffer");
//
//         let in_frame =
//             gst_video::VideoFrameRef::from_buffer_ref_readable(input_buffer, &state.video_info)
//                 .map_err(|_| {
//                     gst::element_imp_error!(
//                         self,
//                         gst::CoreError::Failed,
//                         ["Failed to map output buffer readable"]
//                     );
//                     gst::FlowError::Error
//                 })?;
//
//         // match state.context.send_frame(
//         //     frame.system_frame_number(),
//         //     &in_frame,
//         //     frame
//         //         .flags()
//         //         .contains(gst_video::VideoCodecFrameFlags::FORCE_KEYFRAME),
//         // ) {
//         //     Ok(_) => {
//         //         gst::debug!(CAT, imp: self, "Sent frame {}", frame.system_frame_number());
//         //     }
//         //     Err(data::EncoderStatus::Failure) => {
//         //         gst::element_imp_error!(self, gst::CoreError::Failed, ["Failed to send frame"]);
//         //         return Err(gst::FlowError::Error);
//         //     }
//         //     Err(_) => (),
//         // }
//
//         self.output_frames(state)
//     }
// }
//
// impl SCGstEnc {
//     fn output_frames(&self, state: &mut State) -> Result<gst::FlowSuccess, gst::FlowError> {
//         loop {
//             match state.context.receive_packet() {
//                 Ok((packet_type, packet_number, frame_number, packet_data)) => {
//                     gst::debug!(
//                         CAT,
//                         imp: self,
//                         "Received packet {} of size {}, frame type {:?}",
//                         packet_number,
//                         packet_data.len(),
//                         packet_type
//                     );
//
//                     let instance = self.obj();
//                     let mut frame = instance
//                         .frame(frame_number as i32)
//                         .expect("frame not found");
//
//                     if packet_type == data::FrameType::KEY {
//                         frame.set_flags(gst_video::VideoCodecFrameFlags::SYNC_POINT);
//                     }
//                     let output_buffer = gst::Buffer::from_mut_slice(packet_data);
//                     frame.set_output_buffer(output_buffer);
//                     instance.finish_frame(frame)?;
//                 }
//                 // Err(data::EncoderStatus::Encoded) => {
//                 //     gst::debug!(CAT, imp: self, "Encoded but not output frame yet",);
//                 // }
//                 // Err(data::EncoderStatus::NeedMoreData) => {
//                 //     gst::debug!(CAT, imp: self, "Encoded but need more data",);
//                 //     return Ok(gst::FlowSuccess::Ok);
//                 // }
//                 // Err(data::EncoderStatus::Failure) => {
//                 //     gst::element_imp_error!(
//                 //         self,
//                 //         gst::CoreError::Failed,
//                 //         ["Failed to receive frame"]
//                 //     );
//                 //     return Err(gst::FlowError::Error);
//                 // }
//                 // Err(err) => {
//                 //     gst::debug!(CAT, imp: self, "Soft error when receiving frame: {:?}", err);
//                 //     return Ok(gst::FlowSuccess::Ok);
//                 // }
//             }
//         }
//     }
// }
