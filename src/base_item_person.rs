use crate::base_item_person_image_blur_hashes::BaseItemPersonImageBlurHashes;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseItemPerson {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "Role", skip_serializing_if = "Option:: is_none")]
    pub role: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "PrimaryImageTag", skip_serializing_if = "Option:: is_none")]
    pub primary_image_tag: Option<String>,
    #[serde(rename = "ImageBlurHashes", skip_serializing_if = "Option:: is_none")]
    pub image_blur_hashes: Option<BaseItemPersonImageBlurHashes>,
}
