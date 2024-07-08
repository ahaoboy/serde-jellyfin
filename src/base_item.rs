use crate::media_url::MediaUrl;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseItem {
    #[serde(rename = "Size", skip_serializing_if = "Option:: is_none")]
    pub size: Option<f32>,
    #[serde(rename = "Container", skip_serializing_if = "Option:: is_none")]
    pub container: Option<String>,
    #[serde(rename = "IsHD", skip_serializing_if = "Option:: is_none")]
    pub is_hd: Option<bool>,
    #[serde(rename = "IsShortcut", skip_serializing_if = "Option:: is_none")]
    pub is_shortcut: Option<bool>,
    #[serde(rename = "ShortcutPath", skip_serializing_if = "Option:: is_none")]
    pub shortcut_path: Option<String>,
    #[serde(rename = "Width", skip_serializing_if = "Option:: is_none")]
    pub width: Option<f32>,
    #[serde(rename = "Height", skip_serializing_if = "Option:: is_none")]
    pub height: Option<f32>,
    #[serde(rename = "ExtraIds", skip_serializing_if = "Option:: is_none")]
    pub extra_ids: Option<Vec<String>>,
    #[serde(rename = "DateLastSaved", skip_serializing_if = "Option:: is_none")]
    pub date_last_saved: Option<String>,
    #[serde(rename = "RemoteTrailers", skip_serializing_if = "Option:: is_none")]
    pub remote_trailers: Option<Vec<MediaUrl>>,
    #[serde(
        rename = "SupportsExternalTransfer",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_external_transfer: Option<bool>,
}
