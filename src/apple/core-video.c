
GstAppleCoreVideoPixelBuffer *
gst_apple_core_video_pixel_buffer_new (CVPixelBufferRef buf)
{
  GstAppleCoreVideoPixelBuffer *gpixbuf =
      g_slice_new (GstAppleCoreVideoPixelBuffer);
  gpixbuf->refcount = 1;
  g_mutex_init (&gpixbuf->mutex);
  gpixbuf->buf = CVPixelBufferRetain (buf);
  gpixbuf->lock_state = GST_APPLE_CORE_VIDEO_MEMORY_UNLOCKED;
  gpixbuf->lock_count = 0;
  return gpixbuf;
}


void

gst_core_video_wrap_pixel_buffer (GstBuffer * buf,
    GstVideoInfo * info,
    CVPixelBufferRef pixel_buf,
    GstVideoTextureCache * cache, gboolean * has_padding)
{
  guint n_planes;
  gsize offset[GST_VIDEO_MAX_PLANES] = { 0 };
  gint stride[GST_VIDEO_MAX_PLANES] = { 0 };
  UInt32 size;
  GstAppleCoreVideoPixelBuffer *gpixbuf;
  GstMemory *mem = NULL;

  gpixbuf = gst_apple_core_video_pixel_buffer_new (pixel_buf);

  if (has_padding)
    *has_padding = FALSE;

  if (CVPixelBufferIsPlanar (pixel_buf)) {
    gint i, size = 0, plane_offset = 0;

    n_planes = CVPixelBufferGetPlaneCount (pixel_buf);
    for (i = 0; i < n_planes; i++) {
      stride[i] = CVPixelBufferGetBytesPerRowOfPlane (pixel_buf, i);

      if (stride[i] != GST_VIDEO_INFO_PLANE_STRIDE (info, i) && has_padding)
        *has_padding = TRUE;

      size = stride[i] * CVPixelBufferGetHeightOfPlane (pixel_buf, i);
      offset[i] = plane_offset;
      plane_offset += size;

        mem =
            GST_MEMORY_CAST (gst_apple_core_video_memory_new_wrapped (gpixbuf,
                i, size));
      gst_buffer_append_memory (buf, mem);
    }
  } else {
    n_planes = 1;
    stride[0] = CVPixelBufferGetBytesPerRow (pixel_buf);
    offset[0] = 0;
    size = stride[0] * CVPixelBufferGetHeight (pixel_buf);

      mem =
          GST_MEMORY_CAST (gst_apple_core_video_memory_new_wrapped (gpixbuf, 0,
              size));
    gst_buffer_append_memory (buf, mem);
  }

