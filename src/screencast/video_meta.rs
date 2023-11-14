mod media_meta {
    use std::{fmt, mem};

    use gst::prelude::*;

    #[repr(transparent)]
    pub struct CoreVideoMeta(imp::CoreVideoMeta);

    unsafe impl Send for CoreVideoMeta {}
    unsafe impl Sync for CoreVideoMeta {}

    impl CoreVideoMeta {
        // Add a new custom meta to the buffer with the given label.
        pub fn add(
            buffer: &mut gst::BufferRef,
            cvbuf: gpointer,
        ) -> gst::MetaRefMut<Self, gst::meta::Standalone> {
            unsafe {
                // Manually dropping because gst_buffer_add_meta() takes ownership of the
                // content of the struct.
                let mut params = mem::ManuallyDrop::new(imp::CoreVideoMetaParams { cvbuf });

                let meta = gst::ffi::gst_buffer_add_meta(
                    buffer.as_mut_ptr(),
                    imp::core_media_meta_get_info(),
                    &mut *params as *mut imp::CoreVideoMetaParams as glib::ffi::gpointer,
                ) as *mut imp::CoreVideoMeta;

                Self::from_mut_ptr(buffer, meta)
            }
        }
    }

    // Trait to allow using the gst::Buffer API with this meta.
    unsafe impl MetaAPI for CoreVideoMeta {
        type GstType = imp::CoreVideoMeta;

        fn meta_api() -> glib::Type {
            imp::core_media_meta_api_get_type()
        }
    }

    impl fmt::Debug for CoreVideoMeta {
        fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
            f.debug_struct("CoreVideoMeta").finish()
        }
    }

    // Actual unsafe implementation of the meta.
    mod imp {
        use std::{mem, ptr};

        use glib::once_cell::sync::Lazy;
        use glib::translate::*;
        use gst::ffi::GstMetaTransformCopy;
        use gst::glib::ffi::gpointer;

        pub(super) struct CoreVideoMetaParams {
            pub cvbuf: gpointer,
        }

        // This is the C type that is actually stored as meta inside the buffers.
        #[repr(C)]
        #[no_mangle]
        pub struct CoreVideoMeta {
            parent: gst::ffi::GstMeta,
            pub cvbuf: gpointer,
            pub pixbuf: gpointer,
        }

        // Function to register the meta API and get a type back.
        pub(super) fn core_media_meta_api_get_type() -> glib::Type {
            static TYPE: Lazy<glib::Type> = Lazy::new(|| unsafe {
                let t = from_glib(gst::ffi::gst_meta_api_type_register(
                    b"GstCoreVideoMetaAPI\0".as_ptr() as *const _,
                    [b"memory\0", ptr::null::<std::os::raw::c_char>()].as_ptr() as *mut *const _,
                ));

                assert_ne!(t, glib::Type::INVALID);

                t
            });

            *TYPE
        }

        // Initialization function for our meta. This needs to ensure all fields are correctly
        // initialized. They will contain random memory before.
        unsafe extern "C" fn core_media_meta_init(
            meta: *mut gst::ffi::GstMeta,
            params: glib::ffi::gpointer,
            _buffer: *mut gst::ffi::GstBuffer,
        ) -> glib::ffi::gboolean {
            assert!(!params.is_null());

            let meta = &mut *(meta as *mut CoreVideoMeta);
            let params = ptr::read(params as *const CoreVideoMetaParams);

            // Need to initialize all our fields correctly here.
            ptr::write(&mut meta.cvbuf, params.cvbuf);
            ptr::write(&mut meta.pixbuf, params.cvbuf);
            true.into_glib()
        }

        // Free function for our meta. This needs to free/drop all memory we allocated.
        unsafe extern "C" fn core_media_meta_free(
            meta: *mut gst::ffi::GstMeta,
            _buffer: *mut gst::ffi::GstBuffer,
        ) {
            let meta = &mut *(meta as *mut CoreVideoMeta);

            // Need to free/drop all our fields here.
            ptr::drop_in_place(&mut meta.label);
        }

        // Transform function for our meta. This needs to get it from the old buffer to the new one
        // in a way that is compatible with the transformation type. In this case we just always
        // copy it over.
        unsafe extern "C" fn core_media_meta_transform(
            dest: *mut gst::ffi::GstBuffer,
            meta: *mut gst::ffi::GstMeta,
            _buffer: *mut gst::ffi::GstBuffer,
            _type_: glib::ffi::GQuark,
            data: *mut GstMetaTransformCopy,
        ) -> glib::ffi::gboolean {
            let meta = &*(meta as *mut CoreVideoMeta);

            if (!*data.region) {
                // We simply copy over our meta here. Other metas might have to look at the type
                // and do things conditional on that, or even just drop the meta.
                super::CoreVideoMeta::add(gst::BufferRef::from_mut_ptr(dest), meta.cvbuf);
            } else {
            }

            true.into_glib()
        }

        // Register the meta itself with its functions.
        pub(super) fn core_media_meta_get_info() -> *const gst::ffi::GstMetaInfo {
            struct MetaInfo(ptr::NonNull<gst::ffi::GstMetaInfo>);
            unsafe impl Send for MetaInfo {}
            unsafe impl Sync for MetaInfo {}

            static META_INFO: Lazy<MetaInfo> = Lazy::new(|| unsafe {
                MetaInfo(
                    ptr::NonNull::new(gst::ffi::gst_meta_register(
                        core_media_meta_api_get_type().into_glib(),
                        b"GstCoreVideoMeta\0".as_ptr() as *const _,
                        mem::size_of::<CoreVideoMeta>(),
                        Some(core_media_meta_init),
                        Some(core_media_meta_free),
                        Some(core_media_meta_transform),
                    ) as *mut gst::ffi::GstMetaInfo)
                    .expect("Failed to register meta API"),
                )
            });

            META_INFO.0.as_ptr()
        }
    }
}
