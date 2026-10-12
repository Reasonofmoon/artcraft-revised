use serde_derive::{Deserialize, Serialize};

/// GPT Image background. Fal's schema default is `Auto` when the field is
/// omitted. Callers that leave this unset get `Transparent` so generated
/// images have no background unless Auto or Opaque is chosen.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum RouterImageBackground {
  Auto,
  Transparent,
  Opaque,
}

/// `None` means the product default, transparent.
pub fn resolve_router_image_background(
  background: Option<RouterImageBackground>,
) -> RouterImageBackground {
  background.unwrap_or(RouterImageBackground::Transparent)
}
