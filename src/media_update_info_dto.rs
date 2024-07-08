use crate::media_update_info_path_dto::MediaUpdateInfoPathDto;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MediaUpdateInfoDto {
    #[serde(rename = "Updates", skip_serializing_if = "Option:: is_none")]
    pub updates: Option<Vec<MediaUpdateInfoPathDto>>,
}
