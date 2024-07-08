#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct StartupRemoteAccessDto {
    #[serde(rename = "EnableRemoteAccess")]
    pub enable_remote_access: bool,
    #[serde(rename = "EnableAutomaticPortMapping")]
    pub enable_automatic_port_mapping: bool,
}
