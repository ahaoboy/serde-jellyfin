use crate::base_item::BaseItem;
use crate::base_item_dto::BaseItemDto;
use crate::client_capabilities::ClientCapabilities;
use crate::general_command_type::GeneralCommandType;
use crate::player_state_info::PlayerStateInfo;
use crate::queue_item::QueueItem;
use crate::session_user_info::SessionUserInfo;
use crate::transcoding_info::TranscodingInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct SessionInfo {
    #[serde(rename = "PlayState", skip_serializing_if = "Option:: is_none")]
    pub play_state: Option<PlayerStateInfo>,
    #[serde(rename = "AdditionalUsers", skip_serializing_if = "Option:: is_none")]
    pub additional_users: Option<Vec<SessionUserInfo>>,
    #[serde(rename = "Capabilities", skip_serializing_if = "Option:: is_none")]
    pub capabilities: Option<ClientCapabilities>,
    #[serde(rename = "RemoteEndPoint", skip_serializing_if = "Option:: is_none")]
    pub remote_end_point: Option<String>,
    #[serde(
        rename = "PlayableMediaTypes",
        skip_serializing_if = "Option:: is_none"
    )]
    pub playable_media_types: Option<Vec<String>>,
    #[serde(rename = "Id", skip_serializing_if = "Option:: is_none")]
    pub id: Option<String>,
    #[serde(rename = "UserId", skip_serializing_if = "Option:: is_none")]
    pub user_id: Option<String>,
    #[serde(rename = "UserName", skip_serializing_if = "Option:: is_none")]
    pub user_name: Option<String>,
    #[serde(rename = "Client", skip_serializing_if = "Option:: is_none")]
    pub client: Option<String>,
    #[serde(rename = "LastActivityDate", skip_serializing_if = "Option:: is_none")]
    pub last_activity_date: Option<String>,
    #[serde(
        rename = "LastPlaybackCheckIn",
        skip_serializing_if = "Option:: is_none"
    )]
    pub last_playback_check_in: Option<String>,
    #[serde(rename = "DeviceName", skip_serializing_if = "Option:: is_none")]
    pub device_name: Option<String>,
    #[serde(rename = "DeviceType", skip_serializing_if = "Option:: is_none")]
    pub device_type: Option<String>,
    #[serde(rename = "NowPlayingItem", skip_serializing_if = "Option:: is_none")]
    pub now_playing_item: Option<BaseItemDto>,
    #[serde(
        rename = "FullNowPlayingItem",
        skip_serializing_if = "Option:: is_none"
    )]
    pub full_now_playing_item: Option<BaseItem>,
    #[serde(rename = "NowViewingItem", skip_serializing_if = "Option:: is_none")]
    pub now_viewing_item: Option<BaseItemDto>,
    #[serde(rename = "DeviceId", skip_serializing_if = "Option:: is_none")]
    pub device_id: Option<String>,
    #[serde(
        rename = "ApplicationVersion",
        skip_serializing_if = "Option:: is_none"
    )]
    pub application_version: Option<String>,
    #[serde(rename = "TranscodingInfo", skip_serializing_if = "Option:: is_none")]
    pub transcoding_info: Option<TranscodingInfo>,
    #[serde(rename = "IsActive", skip_serializing_if = "Option:: is_none")]
    pub is_active: Option<bool>,
    #[serde(
        rename = "SupportsMediaControl",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_media_control: Option<bool>,
    #[serde(
        rename = "SupportsRemoteControl",
        skip_serializing_if = "Option:: is_none"
    )]
    pub supports_remote_control: Option<bool>,
    #[serde(rename = "NowPlayingQueue", skip_serializing_if = "Option:: is_none")]
    pub now_playing_queue: Option<Vec<QueueItem>>,
    #[serde(
        rename = "NowPlayingQueueFullItems",
        skip_serializing_if = "Option:: is_none"
    )]
    pub now_playing_queue_full_items: Option<Vec<BaseItemDto>>,
    #[serde(
        rename = "HasCustomDeviceName",
        skip_serializing_if = "Option:: is_none"
    )]
    pub has_custom_device_name: Option<bool>,
    #[serde(rename = "PlaylistItemId", skip_serializing_if = "Option:: is_none")]
    pub playlist_item_id: Option<String>,
    #[serde(rename = "ServerId", skip_serializing_if = "Option:: is_none")]
    pub server_id: Option<String>,
    #[serde(
        rename = "UserPrimaryImageTag",
        skip_serializing_if = "Option:: is_none"
    )]
    pub user_primary_image_tag: Option<String>,
    #[serde(rename = "SupportedCommands", skip_serializing_if = "Option:: is_none")]
    pub supported_commands: Option<Vec<GeneralCommandType>>,
}
