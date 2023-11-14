static GstVideoFormat
gst_core_media_buffer_get_video_format (OSType format)
{
  switch (format) {
    case kCVPixelFormatType_420YpCbCr8Planar:
      return GST_VIDEO_FORMAT_I420;
    case kCVPixelFormatType_420YpCbCr8BiPlanarVideoRange:
      return GST_VIDEO_FORMAT_NV12;
    case kCVPixelFormatType_422YpCbCr8_yuvs:
      return GST_VIDEO_FORMAT_YUY2;
    case kCVPixelFormatType_422YpCbCr8:
      return GST_VIDEO_FORMAT_UYVY;
    case kCVPixelFormatType_32BGRA:
      return GST_VIDEO_FORMAT_BGRA;
    case kCVPixelFormatType_32RGBA:
      return GST_VIDEO_FORMAT_RGBA;
    default:
      GST_WARNING ("Unknown OSType format: %d", (gint) format);
      return GST_VIDEO_FORMAT_UNKNOWN;
  }
}
static gboolean
gst_video_info_init_from_pixel_buffer(GstVideoInfo *info,
                                      CVPixelBufferRef pixel_buf) {
  size_t width, height;
  OSType format_type;
  GstVideoFormat video_format;

  width = CVPixelBufferGetWidth(pixel_buf);
  height = CVPixelBufferGetHeight(pixel_buf);
  format_type = CVPixelBufferGetPixelFormatType(pixel_buf);
  video_format = gst_core_media_buffer_get_video_format(format_type);

  if (video_format == GST_VIDEO_FORMAT_UNKNOWN) {
    return FALSE;
  }

  gst_video_info_init(info);
  gst_video_info_set_format(info, video_format, width, height);

  return TRUE;
}

GstBuffer *gst_core_media_buffer_new(CMSampleBufferRef sample_buf,
                                     gboolean use_video_meta,
                                     GstVideoTextureCache *cache) {
  CVImageBufferRef image_buf;
  CMBlockBufferRef block_buf;
  GstBuffer *buf;

  image_buf = CMSampleBufferGetImageBuffer(sample_buf);
  block_buf = CMSampleBufferGetDataBuffer(sample_buf);

  buf = gst_buffer_new();

  gst_core_media_meta_add(buf, sample_buf, image_buf, block_buf);

  if (image_buf != NULL && CFGetTypeID(image_buf) == CVPixelBufferGetTypeID()) {
    GstVideoInfo info;
    gboolean has_padding = FALSE;
    CVPixelBufferRef pixel_buf = (CVPixelBufferRef)image_buf;

    if (!gst_video_info_init_from_pixel_buffer(&info, pixel_buf)) {
      goto error;
    }

    gst_core_video_wrap_pixel_buffer(buf, &info, pixel_buf, cache,
                                     &has_padding);

  } else {
    goto error;
  }

  return buf;

error:
  if (buf) {
    gst_buffer_unref(buf);
  }
  return NULL;
}

CVPixelBufferRef gst_core_media_buffer_get_pixel_buffer(GstBuffer *buf) {
  GstCoreMediaMeta *meta = (GstCoreMediaMeta *)gst_buffer_get_meta(
      buf, GST_CORE_MEDIA_META_API_TYPE);
  g_return_val_if_fail(meta != NULL, NULL);

  return CVPixelBufferRetain(meta->pixel_buf);
}
