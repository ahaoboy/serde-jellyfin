use crate::group_state_type::GroupStateType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct GroupInfoDto {
    #[serde(rename = "GroupId", skip_serializing_if = "Option:: is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "GroupName", skip_serializing_if = "Option:: is_none")]
    pub group_name: Option<String>,
    #[serde(rename = "State", skip_serializing_if = "Option:: is_none")]
    pub state: Option<GroupStateType>,
    #[serde(rename = "Participants", skip_serializing_if = "Option:: is_none")]
    pub participants: Option<Vec<String>>,
    #[serde(rename = "LastUpdatedAt", skip_serializing_if = "Option:: is_none")]
    pub last_updated_at: Option<String>,
}
