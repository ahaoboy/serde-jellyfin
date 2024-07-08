#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ConfigImageTypes {
    #[serde(rename = "BackdropSizes", skip_serializing_if = "Option:: is_none")]
    pub backdrop_sizes: Option<Vec<String>>,
    #[serde(rename = "BaseUrl", skip_serializing_if = "Option:: is_none")]
    pub base_url: Option<String>,
    #[serde(rename = "LogoSizes", skip_serializing_if = "Option:: is_none")]
    pub logo_sizes: Option<Vec<String>>,
    #[serde(rename = "PosterSizes", skip_serializing_if = "Option:: is_none")]
    pub poster_sizes: Option<Vec<String>>,
    #[serde(rename = "ProfileSizes", skip_serializing_if = "Option:: is_none")]
    pub profile_sizes: Option<Vec<String>>,
    #[serde(rename = "SecureBaseUrl", skip_serializing_if = "Option:: is_none")]
    pub secure_base_url: Option<String>,
    #[serde(rename = "StillSizes", skip_serializing_if = "Option:: is_none")]
    pub still_sizes: Option<Vec<String>>,
}
