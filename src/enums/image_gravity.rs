//! Image gravity enum

use serde::{Deserialize, Serialize};

/// Image gravity options for image operations
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ImageGravity {
    #[serde(rename = "center")]
    Center,
    #[serde(rename = "top-left")]
    TopLeft,
    #[serde(rename = "top")]
    Top,
    #[serde(rename = "top-right")]
    TopRight,
    #[serde(rename = "left")]
    Left,
    #[serde(rename = "right")]
    Right,
    #[serde(rename = "bottom-left")]
    BottomLeft,
    #[serde(rename = "bottom")]
    Bottom,
    #[serde(rename = "bottom-right")]
    BottomRight,
}

impl AsRef<str> for ImageGravity {
    fn as_ref(&self) -> &str {
        match self {
            ImageGravity::Center => "center",
            ImageGravity::TopLeft => "top-left",
            ImageGravity::Top => "top",
            ImageGravity::TopRight => "top-right",
            ImageGravity::Left => "left",
            ImageGravity::Right => "right",
            ImageGravity::BottomLeft => "bottom-left",
            ImageGravity::Bottom => "bottom",
            ImageGravity::BottomRight => "bottom-right",
        }
    }
}