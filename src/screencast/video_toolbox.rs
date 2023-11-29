use objc::{class, runtime::Object, *};
use objc_foundation::{
    INSDictionary, INSObject, INSValue, NSData, NSDictionary, NSObject, NSString, NSValue,
};
use objc_id::{Id, ShareId};
use std::ptr;

use screencapturekit::sc_types::four_char_code::FourCharCode;

use crate::screencast::video_toolbox::ffi::CVPixelBufferGetBaseAddress;

use self::ffi::{
    kCFAllocatorDefault, kCGImageDestinationLossyCompressionQuality, CFNumberCreate,
    CGColorSpaceCreateDeviceRGB, CVPixelBufferCreate, CVPixelBufferGetHeight,
    CVPixelBufferGetWidth, CVPixelBufferLockBaseAddress, CVPixelBufferRef,
    CVPixelBufferUnlockBaseAddress,
};

/// .
///
/// # Safety
///
/// .
pub unsafe fn create_picture_of_buffer(pixel_buffer: CVPixelBufferRef) -> ShareId<NSData> {
    let ci_image_class = class!(CIImage);
    let ci_context_class = class!(CIContext);
    let ci_context: *mut Object = msg_send![ci_context_class, alloc];
    let ci_context: *mut Object = msg_send![ci_context, init];
    let ci_image: *mut Object = msg_send![ci_image_class, alloc];
    let ci_image: *mut Object = msg_send![ci_image, initWithCVPixelBuffer: pixel_buffer ];
    let id_val = CFNumberCreate(kCFAllocatorDefault, 12, &0.99f32);
    let options = NSDictionary::from_keys_and_objects(
        &[kCGImageDestinationLossyCompressionQuality],
        vec![id_val],
    );
    let color_space = CGColorSpaceCreateDeviceRGB();
    let jpeg_data: *mut NSData = msg_send![ci_context, JPEGRepresentationOfImage: ci_image colorSpace: color_space options: options ];
    ShareId::from_ptr(jpeg_data)
}

/// .
///
/// # Safety
///
/// .
pub unsafe fn create_pixel_buffer() -> CVPixelBufferRef {
    let mut pixel_buffer: CVPixelBufferRef = ptr::null_mut();

    CVPixelBufferCreate(
        kCFAllocatorDefault,
        1280,
        720,
        FourCharCode::from_chars(b"BGRA".to_owned()),
        ptr::null_mut(),
        &mut pixel_buffer,
    );
    CVPixelBufferLockBaseAddress(pixel_buffer, 0);
    let width = CVPixelBufferGetWidth(pixel_buffer);
    let height = CVPixelBufferGetHeight(pixel_buffer);
    println!("width: {width} height: {height}");
    let buffer = CVPixelBufferGetBaseAddress(pixel_buffer);
    let pixel_size = 10;
    for i in 0..(width * height - 1) / pixel_size {
        for s in 0..pixel_size {
            buffer.offset(i * s).write( (s + 0xff00ff) as u32);
        }
    }
    CVPixelBufferUnlockBaseAddress(pixel_buffer, 0);
    pixel_buffer
}

mod ffi {
    // pub type VTDecompressionOutputCallback = extern "C" fn(
    //     decompressionOutputRefCon: *mut c_void,
    //     sourceFrameRefCon: *mut c_void,
    //     status: OSStatus,
    //     infoFlags: VTDecodeInfoFlags,
    //     imageBuffer: CVImageBufferRef,
    //     presentationTimeStamp: CMTime,
    //     presentationDuration: CMTime,
    // );
    //

    pub type CVPixelBufferRef = *mut Object;

    use objc_foundation::{NSObject, NSString};
    use objc_id::Id;
    use screencapturekit::sc_types::{four_char_code::FourCharCode, rc::Object};
    #[link(name = "CoreVideo", kind = "framework")]
    extern "C" {
        pub static kCFAllocatorDefault: *mut Object;
    }
    #[link(name = "VideoToolbox", kind = "framework")]
    extern "C" {
        #[allow(improper_ctypes)]
        pub static kCGImageDestinationLossyCompressionQuality: &'static NSString;
        pub fn CFNumberCreate(allocator: *mut Object, cftype: usize, ptr: &f32) -> Id<NSObject>;

        pub fn CGColorSpaceCreateDeviceRGB() -> *mut Object;
        pub fn CVPixelBufferGetWidth(pixel_buffer: CVPixelBufferRef) -> isize;
        pub fn CVPixelBufferGetHeight(pixel_buffer: CVPixelBufferRef) -> isize;
        pub fn CVPixelBufferGetBaseAddress(pixel_buffer: CVPixelBufferRef) -> *mut u32;
        pub fn CVPixelBufferUnlockBaseAddress(
            pixel_buffer: CVPixelBufferRef,
            lock_flags: u32,
        ) -> u32;
        pub fn CVPixelBufferLockBaseAddress(pixel_buffer: CVPixelBufferRef, lock_flags: u32)
            -> u32;
        pub fn CVPixelBufferCreate(
            allocator: *mut Object,
            width: u32,
            height: u32,
            pixel_format: FourCharCode,
            attributes: *mut Object,
            pixel_buffer_out: &mut CVPixelBufferRef,
        ) -> u32;
        // pub fn VTDecompressionSessionGetTypeID() -> *mut Object;
        // pub fn VTDecompressionSessionCreate(
        //     allocator: CFAllocatorRef,
        //     videoFormatDescription: CMVideoFormatDescriptionRef,
        //     videoDecoderSpecification: CFDictionaryRef,
        //     destinationImageBufferAttributes: CFDictionaryRef,
        //     outputCallback: *const VTDecompressionOutputCallbackRecord,
        //     decompressionSessionOut: *mut VTDecompressionSessionRef,
        // ) -> OSStatus;
        // pub fn VTDecompressionSessionDecodeFrame(
        //     session: VTDecompressionSessionRef,
        //     sampleBuffer: CMSampleBufferRef,
        //     decodeFlags: VTDecodeFrameFlags,
        //     sourceFrameRefCon: *mut c_void,
        //     infoFlagsOut: *mut VTDecodeInfoFlags,
        // ) -> OSStatus;
    }
}
