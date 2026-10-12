/// Fal's own schema default for GPT Image `background` is `"auto"`.
/// We send `"transparent"` when the caller does not pick one, so generated
/// images have no background unless Auto or Opaque is chosen explicitly.
pub const DEFAULT_GPT_IMAGE_BACKGROUND: &str = "transparent";

/// PNG is the transparency-capable default. WebP also keeps an alpha channel;
/// JPEG does not, so a transparent background cannot use it.
pub const DEFAULT_GPT_IMAGE_OUTPUT_FORMAT: &str = "png";

/// Resolve the wire `background` and `output_format` for a GPT Image request.
///
/// `None` background becomes `"transparent"`. `"auto"` and `"opaque"` pass
/// through. A transparent background forces a format that can store alpha:
/// an omitted format or `"jpeg"` becomes `"png"`; `"png"` and `"webp"` stay.
pub fn resolve_gpt_image_background_and_format(
  background: Option<&str>,
  output_format: Option<&str>,
) -> (String, String) {
  let background = background.unwrap_or(DEFAULT_GPT_IMAGE_BACKGROUND).to_string();
  let mut output_format = output_format.unwrap_or(DEFAULT_GPT_IMAGE_OUTPUT_FORMAT).to_string();
  if background == "transparent" && output_format == "jpeg" {
    output_format = DEFAULT_GPT_IMAGE_OUTPUT_FORMAT.to_string();
  }
  (background, output_format)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn omitted_background_is_transparent_png() {
    assert_eq!(
      resolve_gpt_image_background_and_format(None, None),
      ("transparent".to_string(), "png".to_string()),
    );
  }

  #[test]
  fn auto_and_opaque_keep_an_explicit_jpeg() {
    assert_eq!(
      resolve_gpt_image_background_and_format(Some("auto"), Some("jpeg")),
      ("auto".to_string(), "jpeg".to_string()),
    );
    assert_eq!(
      resolve_gpt_image_background_and_format(Some("opaque"), Some("jpeg")),
      ("opaque".to_string(), "jpeg".to_string()),
    );
  }

  #[test]
  fn transparent_jpeg_is_promoted_to_png_and_webp_is_kept() {
    assert_eq!(
      resolve_gpt_image_background_and_format(Some("transparent"), Some("jpeg")),
      ("transparent".to_string(), "png".to_string()),
    );
    assert_eq!(
      resolve_gpt_image_background_and_format(None, Some("webp")),
      ("transparent".to_string(), "webp".to_string()),
    );
  }
}
