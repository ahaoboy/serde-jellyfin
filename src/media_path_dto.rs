use crate::media_path_info::MediaPathInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaPathDto {
    #[serde(rename = "Name")]
    pub name: String,
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(rename = "PathInfo", skip_serializing_if = "Option:: is_none")]
    pub path_info: Option<MediaPathInfo>,
}
