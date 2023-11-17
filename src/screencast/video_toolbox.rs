mod ffi {
    pub type VTDecompressionOutputCallback = extern "C" fn(
        decompressionOutputRefCon: *mut c_void,
        sourceFrameRefCon: *mut c_void,
        status: OSStatus,
        infoFlags: VTDecodeInfoFlags,
        imageBuffer: CVImageBufferRef,
        presentationTimeStamp: CMTime,
        presentationDuration: CMTime,
    );

    #[link(name = "VideoToolbox", kind = "framework")]
    extern "C" {
        pub fn VTDecompressionSessionGetTypeID() -> CFTypeID;
        pub fn VTDecompressionSessionCreate(
            allocator: CFAllocatorRef,
            videoFormatDescription: CMVideoFormatDescriptionRef,
            videoDecoderSpecification: CFDictionaryRef,
            destinationImageBufferAttributes: CFDictionaryRef,
            outputCallback: *const VTDecompressionOutputCallbackRecord,
            decompressionSessionOut: *mut VTDecompressionSessionRef,
        ) -> OSStatus;
        pub fn VTDecompressionSessionDecodeFrame(
            session: VTDecompressionSessionRef,
            sampleBuffer: CMSampleBufferRef,
            decodeFlags: VTDecodeFrameFlags,
            sourceFrameRefCon: *mut c_void,
            infoFlagsOut: *mut VTDecodeInfoFlags,
        ) -> OSStatus;
    }
}
