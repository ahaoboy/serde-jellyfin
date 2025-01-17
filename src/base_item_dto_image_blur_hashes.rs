#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BaseItemDtoImageBlurHashes {
    #[serde(rename = "Primary", skip_serializing_if = "Option:: is_none")]
    pub primary: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Art", skip_serializing_if = "Option:: is_none")]
    pub art: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Backdrop", skip_serializing_if = "Option:: is_none")]
    pub backdrop: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Banner", skip_serializing_if = "Option:: is_none")]
    pub banner: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Logo", skip_serializing_if = "Option:: is_none")]
    pub logo: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Thumb", skip_serializing_if = "Option:: is_none")]
    pub thumb: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Disc", skip_serializing_if = "Option:: is_none")]
    pub disc: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Box", skip_serializing_if = "Option:: is_none")]
    pub r#box: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Screenshot", skip_serializing_if = "Option:: is_none")]
    pub screenshot: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Menu", skip_serializing_if = "Option:: is_none")]
    pub menu: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Chapter", skip_serializing_if = "Option:: is_none")]
    pub chapter: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "BoxRear", skip_serializing_if = "Option:: is_none")]
    pub box_rear: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "Profile", skip_serializing_if = "Option:: is_none")]
    pub profile: Option<std::collections::HashMap<String, String>>,
}
