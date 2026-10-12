use serde_derive::{Deserialize, Serialize};

/// GPT Image background sent by the desktop UI.
/// Omitted requests default to transparent for GPT Image models.
#[derive(Copy, Clone, Debug, Serialize, Deserialize, PartialEq, Eq)]
#[serde(rename_all = "snake_case")]
pub enum TauriImageBackground {
  Auto,
  Transparent,
  Opaque,
}
