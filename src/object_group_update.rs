use crate::group_update_type::GroupUpdateType;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ObjectGroupUpdate {
    #[serde(rename = "GroupId", skip_serializing_if = "Option:: is_none")]
    pub group_id: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<GroupUpdateType>,
    //  not support any type
    //  pub data: Option<undefined>,
}
