
static const GstMetaInfo *gst_core_media_meta_get_info(void);

static void gst_core_media_meta_add(GstBuffer *buffer,
                                    CMSampleBufferRef sample_buf,
                                    CVImageBufferRef image_buf,
                                    CMBlockBufferRef block_buf) {
  GstCoreMediaMeta *meta;

  meta = (GstCoreMediaMeta *)gst_buffer_add_meta(
      buffer, gst_core_media_meta_get_info(), NULL);
  CFRetain(sample_buf);
  if (image_buf)
    CVBufferRetain(image_buf);
  if (block_buf)
    CFRetain(block_buf);
  meta->sample_buf = sample_buf;
  meta->image_buf = image_buf;
  meta->block_buf = block_buf;
  if (image_buf != NULL && CFGetTypeID(image_buf) == CVPixelBufferGetTypeID())
    meta->pixel_buf = (CVPixelBufferRef)image_buf;
  else
    meta->pixel_buf = NULL;
}

static gboolean gst_core_media_meta_init(GstCoreMediaMeta *meta,
                                         gpointer params, GstBuffer *buf) {
  meta->sample_buf = NULL;
  meta->image_buf = NULL;
  meta->pixel_buf = NULL;
  meta->block_buf = NULL;

  return TRUE;
}

static void gst_core_media_meta_free(GstCoreMediaMeta *meta, GstBuffer *buf) {
  if (meta->image_buf != NULL) {
    CVBufferRelease(meta->image_buf);
  }

  if (meta->block_buf != NULL) {
    CFRelease(meta->block_buf);
  }

  CFRelease(meta->sample_buf);
}

static gboolean gst_core_media_meta_transform(GstBuffer *transbuf,
                                              GstCoreMediaMeta *meta,
                                              GstBuffer *buffer, GQuark type,
                                              GstMetaTransformCopy *data) {
  if (!data->region) {
    /* only copy if the complete data is copied as well */
    gst_core_media_meta_add(transbuf, meta->sample_buf, meta->image_buf,
                            meta->block_buf);
  } else {
    GST_WARNING_OBJECT(transbuf,
                       "dropping Core Media metadata due to partial buffer");
  }

  return TRUE; /* retval unused */
}

GType gst_core_media_meta_api_get_type(void) {
  static GType type;
  static const gchar *tags[] = {"memory", NULL};

  if (g_once_init_enter(&type)) {
    GType _type = gst_meta_api_type_register("GstCoreMediaMetaAPI", tags);
    g_once_init_leave(&type, _type);
  }
  return type;
}

static const GstMetaInfo *gst_core_media_meta_get_info(void) {
  static const GstMetaInfo *core_media_meta_info = NULL;

  if (g_once_init_enter(&core_media_meta_info)) {
    const GstMetaInfo *meta = gst_meta_register(
        GST_CORE_MEDIA_META_API_TYPE, "GstCoreMediaMeta",
        sizeof(GstCoreMediaMeta), (GstMetaInitFunction)gst_core_media_meta_init,
        (GstMetaFreeFunction)gst_core_media_meta_free,
        (GstMetaTransformFunction)gst_core_media_meta_transform);
    g_once_init_leave(&core_media_meta_info, meta);
  }
  return core_media_meta_info;
}
