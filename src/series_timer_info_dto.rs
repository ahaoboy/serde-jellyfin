use crate::day_of_week::DayOfWeek;
use crate::day_pattern::DayPattern;
use crate::KeepUntil;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SeriesTimerInfoDto {
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "Type", skip_serializing_if = "Option:: is_none")]
    pub r#type: Option<String>,
    #[serde(rename = "ServerId", skip_serializing_if = "Option:: is_none")]
    pub server_id: Option<String>,
    #[serde(rename = "ExternalId", skip_serializing_if = "Option:: is_none")]
    pub external_id: Option<String>,
    #[serde(rename = "ChannelId", skip_serializing_if = "Option:: is_none")]
    pub channel_id: Option<String>,
    #[serde(rename = "ExternalChannelId", skip_serializing_if = "Option:: is_none")]
    pub external_channel_id: Option<String>,
    #[serde(rename = "ChannelName", skip_serializing_if = "Option:: is_none")]
    pub channel_name: Option<String>,
    #[serde(
        rename = "ChannelPrimaryImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub channel_primary_image_tag: Option<String>,
    #[serde(rename = "ProgramId", skip_serializing_if = "Option:: is_none")]
    pub program_id: Option<String>,
    #[serde(rename = "ExternalProgramId", skip_serializing_if = "Option:: is_none")]
    pub external_program_id: Option<String>,
    #[serde(rename = "Name", skip_serializing_if = "Option:: is_none")]
    pub name: Option<String>,
    #[serde(rename = "Overview", skip_serializing_if = "Option:: is_none")]
    pub overview: Option<String>,
    #[serde(rename = "StartDate", skip_serializing_if = "Option:: is_none")]
    pub start_date: Option<String>,
    #[serde(rename = "EndDate", skip_serializing_if = "Option:: is_none")]
    pub end_date: Option<String>,
    #[serde(rename = "ServiceName", skip_serializing_if = "Option:: is_none")]
    pub service_name: Option<String>,
    #[serde(rename = "Priority", skip_serializing_if = "Option:: is_none")]
    pub priority: Option<f32>,
    #[serde(rename = "PrePaddingSeconds", skip_serializing_if = "Option:: is_none")]
    pub pre_padding_seconds: Option<f32>,
    #[serde(
        rename = "PostPaddingSeconds",
        skip_serializing_if = "Option:: is_none"
    )]
    pub post_padding_seconds: Option<f32>,
    #[serde(
        rename = "IsPrePaddingRequired",
        skip_serializing_if = "Option:: is_none"
    )]
    pub is_pre_padding_required: Option<bool>,
    #[serde(
        rename = "ParentBackdropItemId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_backdrop_item_id: Option<String>,
    #[serde(
        rename = "ParentBackdropImageTags",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_backdrop_image_tags: Option<Vec<String>>,
    #[serde(
        rename = "IsPostPaddingRequired",
        skip_serializing_if = "Option:: is_none"
    )]
    pub is_post_padding_required: Option<bool>,
    #[serde(rename = "KeepUntil", skip_serializing_if = "Option:: is_none")]
    pub keep_until: Option<KeepUntil>,
    #[serde(rename = "RecordAnyTime", skip_serializing_if = "Option:: is_none")]
    pub record_any_time: Option<bool>,
    #[serde(
        rename = "SkipEpisodesInLibrary",
        skip_serializing_if = "Option:: is_none"
    )]
    pub skip_episodes_in_library: Option<bool>,
    #[serde(rename = "RecordAnyChannel", skip_serializing_if = "Option:: is_none")]
    pub record_any_channel: Option<bool>,
    #[serde(rename = "KeepUpTo", skip_serializing_if = "Option:: is_none")]
    pub keep_up_to: Option<f32>,
    #[serde(rename = "RecordNewOnly", skip_serializing_if = "Option:: is_none")]
    pub record_new_only: Option<bool>,
    #[serde(rename = "Days", skip_serializing_if = "Option:: is_none")]
    pub days: Option<Vec<DayOfWeek>>,
    #[serde(rename = "DayPattern", skip_serializing_if = "Option:: is_none")]
    pub day_pattern: Option<DayPattern>,
    #[serde(rename = "ImageTags", skip_serializing_if = "Option:: is_none")]
    pub image_tags: Option<std::collections::HashMap<String, String>>,
    #[serde(rename = "ParentThumbItemId", skip_serializing_if = "Option:: is_none")]
    pub parent_thumb_item_id: Option<String>,
    #[serde(
        rename = "ParentThumbImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_thumb_image_tag: Option<String>,
    #[serde(
        rename = "ParentPrimaryImageItemId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_primary_image_item_id: Option<String>,
    #[serde(
        rename = "ParentPrimaryImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub parent_primary_image_tag: Option<String>,
}
