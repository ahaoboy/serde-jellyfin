use crate::access_schedule::AccessSchedule;
use crate::SyncPlayUserAccessType;
use crate::UnratedItem;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UserPolicy {
    #[serde(rename = "IsAdministrator", skip_serializing_if = "Option:: is_none")]
    pub is_administrator: Option<bool>,
    #[serde(rename = "IsHidden", skip_serializing_if = "Option:: is_none")]
    pub is_hidden: Option<bool>,
    #[serde(rename = "IsDisabled", skip_serializing_if = "Option:: is_none")]
    pub is_disabled: Option<bool>,
    #[serde(rename = "MaxParentalRating", skip_serializing_if = "Option:: is_none")]
    pub max_parental_rating: Option<f32>,
    #[serde(rename = "BlockedTags", skip_serializing_if = "Option:: is_none")]
    pub blocked_tags: Option<Vec<String>>,
    #[serde(
        rename = "EnableUserPreferenceAccess",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_user_preference_access: Option<bool>,
    #[serde(rename = "AccessSchedules", skip_serializing_if = "Option:: is_none")]
    pub access_schedules: Option<Vec<AccessSchedule>>,
    #[serde(rename = "BlockUnratedItems", skip_serializing_if = "Option:: is_none")]
    pub block_unrated_items: Option<Vec<UnratedItem>>,
    #[serde(
        rename = "EnableRemoteControlOfOtherUsers",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_remote_control_of_other_users: Option<bool>,
    #[serde(
        rename = "EnableSharedDeviceControl",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_shared_device_control: Option<bool>,
    #[serde(
        rename = "EnableRemoteAccess",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_remote_access: Option<bool>,
    #[serde(
        rename = "EnableLiveTvManagement",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_live_tv_management: Option<bool>,
    #[serde(
        rename = "EnableLiveTvAccess",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_live_tv_access: Option<bool>,
    #[serde(
        rename = "EnableMediaPlayback",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_media_playback: Option<bool>,
    #[serde(
        rename = "EnableAudioPlaybackTranscoding",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_audio_playback_transcoding: Option<bool>,
    #[serde(
        rename = "EnableVideoPlaybackTranscoding",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_video_playback_transcoding: Option<bool>,
    #[serde(
        rename = "EnablePlaybackRemuxing",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_playback_remuxing: Option<bool>,
    #[serde(
        rename = "ForceRemoteSourceTranscoding",
        skip_serializing_if = "Option:: is_none"
    )]
    pub force_remote_source_transcoding: Option<bool>,
    #[serde(
        rename = "EnableContentDeletion",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_content_deletion: Option<bool>,
    #[serde(
        rename = "EnableContentDeletionFromFolders",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_content_deletion_from_folders: Option<Vec<String>>,
    #[serde(
        rename = "EnableContentDownloading",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_content_downloading: Option<bool>,
    #[serde(
        rename = "EnableSyncTranscoding",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_sync_transcoding: Option<bool>,
    #[serde(
        rename = "EnableMediaConversion",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_media_conversion: Option<bool>,
    #[serde(rename = "EnabledDevices", skip_serializing_if = "Option:: is_none")]
    pub enabled_devices: Option<Vec<String>>,
    #[serde(rename = "EnableAllDevices", skip_serializing_if = "Option:: is_none")]
    pub enable_all_devices: Option<bool>,
    #[serde(rename = "EnabledChannels", skip_serializing_if = "Option:: is_none")]
    pub enabled_channels: Option<Vec<String>>,
    #[serde(rename = "EnableAllChannels", skip_serializing_if = "Option:: is_none")]
    pub enable_all_channels: Option<bool>,
    #[serde(rename = "EnabledFolders", skip_serializing_if = "Option:: is_none")]
    pub enabled_folders: Option<Vec<String>>,
    #[serde(rename = "EnableAllFolders", skip_serializing_if = "Option:: is_none")]
    pub enable_all_folders: Option<bool>,
    #[serde(
        rename = "InvalidLoginAttemptCount",
        skip_serializing_if = "Option:: is_none"
    )]
    pub invalid_login_attempt_count: Option<f32>,
    #[serde(
        rename = "LoginAttemptsBeforeLockout",
        skip_serializing_if = "Option:: is_none"
    )]
    pub login_attempts_before_lockout: Option<f32>,
    #[serde(rename = "MaxActiveSessions", skip_serializing_if = "Option:: is_none")]
    pub max_active_sessions: Option<f32>,
    #[serde(
        rename = "EnablePublicSharing",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_public_sharing: Option<bool>,
    #[serde(
        rename = "BlockedMediaFolders",
        skip_serializing_if = "Option:: is_none"
    )]
    pub blocked_media_folders: Option<Vec<String>>,
    #[serde(rename = "BlockedChannels", skip_serializing_if = "Option:: is_none")]
    pub blocked_channels: Option<Vec<String>>,
    #[serde(
        rename = "RemoteClientBitrateLimit",
        skip_serializing_if = "Option:: is_none"
    )]
    pub remote_client_bitrate_limit: Option<f32>,
    #[serde(
        rename = "AuthenticationProviderId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub authentication_provider_id: Option<String>,
    #[serde(
        rename = "PasswordResetProviderId",
        skip_serializing_if = "Option:: is_none"
    )]
    pub password_reset_provider_id: Option<String>,
    #[serde(rename = "SyncPlayAccess", skip_serializing_if = "Option:: is_none")]
    pub sync_play_access: Option<SyncPlayUserAccessType>,
}
