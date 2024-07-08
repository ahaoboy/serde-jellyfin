use crate::general_command_type::GeneralCommandType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GeneralCommand {
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<GeneralCommandType>,
    #[serde(rename = "ControllingUserId", skip_serializing_if = "Option:: is_none")]
    pub controlling_user_id: Option<String>,
    #[serde(rename = "Arguments", skip_serializing_if = "Option:: is_none")]
    pub arguments: Option<std::collections::HashMap<String, String>>,
}
