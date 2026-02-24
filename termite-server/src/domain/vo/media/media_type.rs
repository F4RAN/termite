use serde::{Deserialize, Serialize};

#[derive(Clone, Debug, PartialEq, Eq, Serialize, Deserialize)]
#[serde(rename_all = "lowercase")]
pub enum MediaType {
    Image,
    Video,
    Audio,
    Document,
}

impl MediaType {
    /// Media type: image, video, audio, or document (case-insensitive).
    pub fn new(value: String) -> Result<Self, String> {
        match value.trim().to_lowercase().as_str() {
            "image" => Ok(MediaType::Image),
            "video" => Ok(MediaType::Video),
            "audio" => Ok(MediaType::Audio),
            "document" => Ok(MediaType::Document),
            _ => Err("Media type must be image, video, audio, or document".into()),
        }
    }

    pub fn as_db_str(&self) -> &'static str {
        match self {
            MediaType::Image => "image",
            MediaType::Video => "video",
            MediaType::Audio => "audio",
            MediaType::Document => "document",
        }
    }
}
