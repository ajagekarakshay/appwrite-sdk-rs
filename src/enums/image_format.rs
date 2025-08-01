//! Image format enum

use serde::{Deserialize, Serialize};

/// Image formats for image operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageFormat {
    #[serde(rename = "jpg")]
    Jpg,
    #[serde(rename = "jpeg")]
    Jpeg,
    #[serde(rename = "gif")]
    Gif,
    #[serde(rename = "png")]
    Png,
    #[serde(rename = "webp")]
    Webp,
}

impl AsRef<str> for ImageFormat {
    fn as_ref(&self) -> &str {
        match self {
            ImageFormat::Jpg => "jpg",
            ImageFormat::Jpeg => "jpeg",
            ImageFormat::Gif => "gif",
            ImageFormat::Png => "png",
            ImageFormat::Webp => "webp",
        }
    }
}