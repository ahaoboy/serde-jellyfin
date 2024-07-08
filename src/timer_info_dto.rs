use crate::base_item_dto::BaseItemDto;
use crate::KeepUntil;
use crate::RecordingStatus;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct TimerInfoDto {
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
    #[serde(rename = "Status", skip_serializing_if = "Option:: is_none")]
    pub status: Option<RecordingStatus>,
    #[serde(rename = "SeriesTimerId", skip_serializing_if = "Option:: is_none")]
    pub series_timer_id: Option<String>,
    #[serde(
        rename = "ExternalSeriesTimerId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub external_series_timer_id: Option<String>,
    #[serde(rename = "RunTimeTicks", skip_serializing_if = "Option:: is_none")]
    pub run_time_ticks: Option<f32>,
    #[serde(rename = "ProgramInfo", skip_serializing_if = "Option:: is_none")]
    pub program_info: Option<BaseItemDto>,
}
