
static const GstMetaInfo *gst_core_video_meta_get_info (void);

static void
gst_core_video_meta_add (GstBuffer * buffer, CVBufferRef cvbuf)
{
  GstCoreVideoMeta *meta;

  meta = (GstCoreVideoMeta *) gst_buffer_add_meta (buffer,
      gst_core_video_meta_get_info (), NULL);
  meta->cvbuf = CVBufferRetain (cvbuf);
  meta->pixbuf = (CVPixelBufferRef) cvbuf;
}

static gboolean
gst_core_video_meta_init (GstCoreVideoMeta * meta, gpointer params,
    GstBuffer * buf)
{
  meta->cvbuf = NULL;
  meta->pixbuf = NULL;

  return TRUE;
}

static void
gst_core_video_meta_free (GstCoreVideoMeta * meta, GstBuffer * buf)
{
  CVBufferRelease (meta->cvbuf);
}

static gboolean
gst_core_video_meta_transform (GstBuffer * transbuf, GstCoreVideoMeta * meta,
    GstBuffer * buffer, GQuark type, GstMetaTransformCopy * data)
{
  if (!data->region) {
    /* only copy if the complete data is copied as well */
    gst_core_video_meta_add (transbuf, meta->cvbuf);
  } else {
    GST_WARNING_OBJECT (transbuf,
        "dropping Core Video metadata due to partial buffer");
  }

  return TRUE;                  /* retval unused */
}

GType
gst_core_video_meta_api_get_type (void)
{
  static GType type;
  static const gchar *tags[] = { "memory", NULL };

  if (g_once_init_enter (&type)) {
    GType _type = gst_meta_api_type_register ("GstCoreVideoMetaAPI", tags);
    g_once_init_leave (&type, _type);
  }
  return type;
}

static const GstMetaInfo *
gst_core_video_meta_get_info (void)
{
  static const GstMetaInfo *core_video_meta_info = NULL;

  if (g_once_init_enter (&core_video_meta_info)) {
    const GstMetaInfo *meta = gst_meta_register (GST_CORE_VIDEO_META_API_TYPE,
        "GstCoreVideoMeta", sizeof (GstCoreVideoMeta),
        (GstMetaInitFunction) gst_core_video_meta_init,
        (GstMetaFreeFunction) gst_core_video_meta_free,
        (GstMetaTransformFunction) gst_core_video_meta_transform);
    g_once_init_leave (&core_video_meta_info, meta);
  }
  return core_video_meta_info;
}

