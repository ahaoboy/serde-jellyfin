#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct NewGroupRequestDto {
    #[serde(rename = "GroupName", skip_serializing_if = "Option:: is_none")]
    pub group_name: Option<String>,
}
