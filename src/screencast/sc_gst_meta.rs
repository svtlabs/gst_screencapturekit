// This example demonstrates how sc_gst_metaeta can be defined and used on buffers.
//
// It simply attaches a GstMeta with a Rust String to buffers that are passed into
// an appsrc and retrieves them again from an appsink.

use gst::{glib, prelude::*};
use screencapturekit::cm_sample_buffer::CMSampleBuffer;

use std::{fmt, mem};

#[repr(transparent)]
pub struct SCGstMeta(imp::SCGstMeta);

// Metas must be Send+Sync.
unsafe impl Send for SCGstMeta {}
unsafe impl Sync for SCGstMeta {}

impl SCGstMeta {
    // Add a new sc_gst_meta to the buffer with the given label.
    pub fn add(
        buffer: &mut gst::BufferRef,
        sample_buffer: CMSampleBuffer,
    ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
        unsafe {
            // Manually dropping because gst_buffer_add_meta() takes ownership of the
            // content of the struct.
            let mut params = mem::ManuallyDrop::new(imp::SCGstMetaParams { sample_buffer });

            // The label is passed through via the params to sc_gst_meta_init().
            let meta = gst::ffi::gst_buffer_add_meta(
                buffer.as_mut_ptr(),
                imp::sc_gst_meta_get_info(),
                &mut *params as *mut imp::SCGstMetaParams as glib::ffi::gpointer,
            ) as *mut imp::SCGstMeta;

            Self::from_mut_ptr(buffer, meta)
        }
    }

    // Retrieve the stored label.
    pub fn sample_buffer(&self) -> &CMSampleBuffer {
        &self.0.sample_buffer
    }
}

// Trait to allow using the gst::Buffer API with this meta.
unsafe impl MetaAPI for SCGstMeta {
    type GstType = imp::SCGstMeta;

    fn meta_api() -> glib::Type {
        imp::sc_gst_meta_api_get_type()
    }
}

impl fmt::Debug for SCGstMeta {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.debug_struct("SCGstMeta")
            .field("sample_buffer", &self.sample_buffer())
            .finish()
    }
}

// Actual unsafe implementation of the meta.
mod imp {
    use std::{mem, ptr};

    use glib::once_cell::sync::Lazy;
    use glib::translate::*;
    use gst::glib;
    use screencapturekit::cm_sample_buffer::CMSampleBuffer;

    pub(super) struct SCGstMetaParams {
        pub sample_buffer: CMSampleBuffer,
    }

    // This is the C type that is actually stored as meta inside the buffers.
    #[repr(C)]
    pub struct SCGstMeta {
        parent: gst::ffi::GstMeta,
        pub(super) sample_buffer: CMSampleBuffer,
    }

    // Function to register the meta API and get a type back.
    pub(super) fn sc_gst_meta_api_get_type() -> glib::Type {
        static TYPE: Lazy<glib::Type> = Lazy::new(|| unsafe {
            let t = from_glib(gst::ffi::gst_meta_api_type_register(
                b"MySCGstMetaAPI\0".as_ptr() as *const _,
                // We provide no tags here as our meta is just a label and does
                // not refer to any specific aspect of the buffer.
                [ptr::null::<std::os::raw::c_char>()].as_ptr() as *mut *const _,
            ));

            assert_ne!(t, glib::Type::INVALID);

            t
        });

        *TYPE
    }

    // Initialization function for our meta. This needs to ensure all fields are correctly
    // initialized. They will contain random memory before.
    unsafe extern "C" fn sc_gst_meta_init(
        meta: *mut gst::ffi::GstMeta,
        params: glib::ffi::gpointer,
        _buffer: *mut gst::ffi::GstBuffer,
    ) -> glib::ffi::gboolean {
        assert!(!params.is_null());

        let meta = &mut *(meta as *mut SCGstMeta);
        let params = ptr::read(params as *const SCGstMetaParams);

        // Need to initialize all our fields correctly here.
        ptr::write(&mut meta.sample_buffer, params.sample_buffer);

        true.into_glib()
    }

    // Free function for our meta. This needs to free/drop all memory we allocated.
    unsafe extern "C" fn sc_gst_meta_free(
        meta: *mut gst::ffi::GstMeta,
        _buffer: *mut gst::ffi::GstBuffer,
    ) {
        let meta = &mut *(meta as *mut SCGstMeta);

        // Need to free/drop all our fields here.
        ptr::drop_in_place(&mut meta.sample_buffer);
    }

    // Transform function for our meta. This needs to get it from the old buffer to the new one
    // in a way that is compatible with the transformation type. In this case we just always
    // copy it over.
    unsafe extern "C" fn sc_gst_meta_transform(
        dest: *mut gst::ffi::GstBuffer,
        meta: *mut gst::ffi::GstMeta,
        _buffer: *mut gst::ffi::GstBuffer,
        _type_: glib::ffi::GQuark,
        _data: glib::ffi::gpointer,
    ) -> glib::ffi::gboolean {
        let meta = &*(meta as *mut SCGstMeta);

        // We simply copy over our meta here. Other metas might have to look at the type
        // and do things conditional on that, or even just drop the meta.
        super::SCGstMeta::add(
            gst::BufferRef::from_mut_ptr(dest),
            meta.sample_buffer.clone(),
        );

        true.into_glib()
    }

    // Register the meta itself with its functions.
    pub(super) fn sc_gst_meta_get_info() -> *const gst::ffi::GstMetaInfo {
        struct MetaInfo(ptr::NonNull<gst::ffi::GstMetaInfo>);
        unsafe impl Send for MetaInfo {}
        unsafe impl Sync for MetaInfo {}

        static META_INFO: Lazy<MetaInfo> = Lazy::new(|| unsafe {
            MetaInfo(
                ptr::NonNull::new(gst::ffi::gst_meta_register(
                    sc_gst_meta_api_get_type().into_glib(),
                    b"MySCGstMeta\0".as_ptr() as *const _,
                    mem::size_of::<SCGstMeta>(),
                    Some(sc_gst_meta_init),
                    Some(sc_gst_meta_free),
                    Some(sc_gst_meta_transform),
                ) as *mut gst::ffi::GstMetaInfo)
                .expect("Failed to register meta API"),
            )
        });

        META_INFO.0.as_ptr()
    }
}
