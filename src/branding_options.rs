#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct BrandingOptions {
    #[serde(rename = "LoginDisclaimer", skip_serializing_if = "Option:: is_none")]
    pub login_disclaimer: Option<String>,
    #[serde(rename = "CustomCss", skip_serializing_if = "Option:: is_none")]
    pub custom_css: Option<String>,
    #[serde(
        rename = "SplashscreenEnabled",
        skip_serializing_if = "Option:: is_none"
    )]
    pub splashscreen_enabled: Option<bool>,
}
