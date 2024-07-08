use crate::name_value_pair::NameValuePair;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ListingsProviderInfo {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "Username", skip_serializing_if = "Option:: is_none")]
    pub username: Option<String>,
    #[serde(rename = "Password", skip_serializing_if = "Option:: is_none")]
    pub password: Option<String>,
    #[serde(rename = "ListingsId", skip_serializing_if = "Option:: is_none")]
    pub listings_id: Option<String>,
    #[serde(rename = "ZipCode", skip_serializing_if = "Option:: is_none")]
    pub zip_code: Option<String>,
    #[serde(rename = "Country", skip_serializing_if = "Option:: is_none")]
    pub country: Option<String>,
    #[serde(rename = "Path", skip_serializing_if = "Option:: is_none")]
    pub path: Option<String>,
    #[serde(rename = "EnabledTuners", skip_serializing_if = "Option:: is_none")]
    pub enabled_tuners: Option<Vec<String>>,
    #[serde(rename = "EnableAllTuners", skip_serializing_if = "Option:: is_none")]
    pub enable_all_tuners: Option<bool>,
    #[serde(rename = "NewsCategories", skip_serializing_if = "Option:: is_none")]
    pub news_categories: Option<Vec<String>>,
    #[serde(rename = "SportsCategories", skip_serializing_if = "Option:: is_none")]
    pub sports_categories: Option<Vec<String>>,
    #[serde(rename = "KidsCategories", skip_serializing_if = "Option:: is_none")]
    pub kids_categories: Option<Vec<String>>,
    #[serde(rename = "MovieCategories", skip_serializing_if = "Option:: is_none")]
    pub movie_categories: Option<Vec<String>>,
    #[serde(rename = "ChannelMappings", skip_serializing_if = "Option:: is_none")]
    pub channel_mappings: Option<Vec<NameValuePair>>,
    #[serde(rename = "MoviePrefix", skip_serializing_if = "Option:: is_none")]
    pub movie_prefix: Option<String>,
    #[serde(rename = "PreferredLanguage", skip_serializing_if = "Option:: is_none")]
    pub preferred_language: Option<String>,
    #[serde(rename = "UserAgent", skip_serializing_if = "Option:: is_none")]
    pub user_agent: Option<String>,
}
