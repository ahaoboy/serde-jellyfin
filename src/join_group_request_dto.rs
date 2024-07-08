#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct JoinGroupRequestDto {
    #[serde(rename = "GroupId", skip_serializing_if = "Option:: is_none")]
    pub group_id: Option<String>,
}
