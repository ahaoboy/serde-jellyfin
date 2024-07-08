pub mod access_schedule;
pub mod activity_log_entry;
pub mod activity_log_entry_query_result;
pub mod add_virtual_folder_dto;
pub mod admin_notification_dto;
pub mod album_info;
pub mod album_info_remote_search_query;
pub mod all_theme_media_result;
pub mod architecture;
pub mod artist_info;
pub mod artist_info_remote_search_query;
pub mod authenticate_user_by_name;
pub mod authentication_info;
pub mod authentication_info_query_result;
pub mod authentication_result;
pub mod base_item;
pub mod base_item_dto;
pub mod base_item_dto_image_blur_hashes;
pub mod base_item_dto_query_result;
pub mod base_item_kind;
pub mod base_item_person;
pub mod base_item_person_image_blur_hashes;
pub mod book_info;
pub mod book_info_remote_search_query;
pub mod box_set_info;
pub mod box_set_info_remote_search_query;
pub mod branding_options;
pub mod buffer_request_dto;
pub mod channel_features;
pub mod channel_item_sort_field;
pub mod channel_mapping_options_dto;
pub mod channel_media_content_type;
pub mod channel_media_type;
pub mod channel_type;
pub mod chapter_info;
pub mod client_capabilities;
pub mod client_capabilities_dto;
pub mod client_log_document_response_dto;
pub mod codec_profile;
pub mod codec_type;
pub mod collection_creation_result;
pub mod collection_type_options;
pub mod config_image_types;
pub mod configuration_page_info;
pub mod container_profile;
pub mod control_response;
pub mod country_info;
pub mod create_playlist_dto;
pub mod create_user_by_name;
pub mod culture_dto;
pub mod day_of_week;
pub mod day_pattern;
pub mod default_directory_browser_info_dto;
pub mod device_identification;
pub mod device_info;
pub mod device_info_query_result;
pub mod device_options;
pub mod device_options_dto;
pub mod device_profile;
pub mod device_profile_info;
pub mod device_profile_type;
pub mod direct_play_profile;
pub mod display_preferences_dto;
pub mod dlna_options;
pub mod dlna_profile_type;
pub mod dynamic_day_of_week;
pub mod embedded_subtitle_options;
pub mod encoding_context;
pub mod encoding_options;
pub mod end_point_info;
pub mod external_id_info;
pub mod external_id_media_type;
pub mod external_url;
pub mod ffmpeg_location;
pub mod file_system_entry_info;
pub mod file_system_entry_type;
pub mod font_file;
pub mod forgot_password_action;
pub mod forgot_password_dto;
pub mod forgot_password_pin_dto;
pub mod forgot_password_result;
pub mod general_command;
pub mod general_command_type;
pub mod get_programs_dto;
pub mod group_info_dto;
pub mod group_queue_mode;
pub mod group_repeat_mode;
pub mod group_shuffle_mode;
pub mod group_state_type;
pub mod group_update_type;
pub mod guide_info;
pub mod hardware_encoding_type;
pub mod header_match_type;
pub mod http_header_info;
pub mod ignore_wait_request_dto;
pub mod image_by_name_info;
pub mod image_format;
pub mod image_info;
pub mod image_option;
pub mod image_orientation;
pub mod image_provider_info;
pub mod image_saving_convention;
pub mod image_type;
pub mod installation_info;
pub mod iplugin;
pub mod iso_type;
pub mod item_counts;
pub mod item_fields;
pub mod item_filter;
pub mod join_group_request_dto;
pub mod keep_until;
pub mod library_option_info_dto;
pub mod library_options;
pub mod library_options_result_dto;
pub mod library_type_options_dto;
pub mod library_update_info;
pub mod listings_provider_info;
pub mod live_stream_response;
pub mod live_tv_info;
pub mod live_tv_options;
pub mod live_tv_service_info;
pub mod live_tv_service_status;
pub mod localization_option;
pub mod location_type;
pub mod log_file;
pub mod log_level;
pub mod media_attachment;
pub mod media_encoder_path_dto;
pub mod media_path_dto;
pub mod media_path_info;
pub mod media_protocol;
pub mod media_source_info;
pub mod media_source_type;
pub mod media_stream;
pub mod media_stream_type;
pub mod media_update_info_dto;
pub mod media_update_info_path_dto;
pub mod media_url;
pub mod message_command;
pub mod metadata_configuration;
pub mod metadata_editor_info;
pub mod metadata_field;
pub mod metadata_options;
pub mod metadata_refresh_mode;
pub mod move_playlist_item_request_dto;
pub mod movie_info;
pub mod movie_info_remote_search_query;
pub mod music_video_info;
pub mod music_video_info_remote_search_query;
pub mod name_guid_pair;
pub mod name_id_pair;
pub mod name_value_pair;
pub mod network_configuration;
pub mod new_group_request_dto;
pub mod next_item_request_dto;
pub mod notification_dto;
pub mod notification_level;
pub mod notification_option;
pub mod notification_options;
pub mod notification_result_dto;
pub mod notification_type_info;
pub mod notifications_summary_dto;
pub mod object_group_update;
pub mod open_live_stream_dto;
pub mod package_info;
pub mod parental_rating;
pub mod path_substitution;
pub mod person_lookup_info;
pub mod person_lookup_info_remote_search_query;
pub mod pin_redeem_result;
pub mod ping_request_dto;
pub mod play_access;
pub mod play_command;
pub mod play_method;
pub mod play_request;
pub mod play_request_dto;
pub mod playback_error_code;
pub mod playback_info_dto;
pub mod playback_info_response;
pub mod playback_progress_info;
pub mod playback_start_info;
pub mod playback_stop_info;
pub mod player_state_info;
pub mod playlist_creation_result;
pub mod playstate_command;
pub mod playstate_request;
pub mod plugin_info;
pub mod plugin_status;
pub mod previous_item_request_dto;
pub mod problem_details;
pub mod profile_condition;
pub mod profile_condition_type;
pub mod profile_condition_value;
pub mod program_audio;
pub mod public_system_info;
pub mod query_filters;
pub mod query_filters_legacy;
pub mod queue_item;
pub mod queue_request_dto;
pub mod quick_connect_dto;
pub mod quick_connect_result;
pub mod rating_type;
pub mod ready_request_dto;
pub mod recommendation_dto;
pub mod recommendation_type;
pub mod recording_status;
pub mod remote_image_info;
pub mod remote_image_result;
pub mod remote_search_result;
pub mod remote_subtitle_info;
pub mod remove_from_playlist_request_dto;
pub mod repeat_mode;
pub mod repository_info;
pub mod response_profile;
pub mod scroll_direction;
pub mod search_hint;
pub mod search_hint_result;
pub mod seek_request_dto;
pub mod send_command;
pub mod send_command_type;
pub mod send_to_user_type;
pub mod series_info;
pub mod series_info_remote_search_query;
pub mod series_status;
pub mod series_timer_info_dto;
pub mod series_timer_info_dto_query_result;
pub mod server_configuration;
pub mod server_discovery_info;
pub mod session_info;
pub mod session_message_type;
pub mod session_user_info;
pub mod set_channel_mapping_dto;
pub mod set_playlist_item_request_dto;
pub mod set_repeat_mode_request_dto;
pub mod set_shuffle_mode_request_dto;
pub mod song_info;
pub mod sort_order;
pub mod special_view_option_dto;
pub mod startup_configuration_dto;
pub mod startup_remote_access_dto;
pub mod startup_user_dto;
pub mod subtitle_delivery_method;
pub mod subtitle_options;
pub mod subtitle_playback_mode;
pub mod subtitle_profile;
pub mod sync_play_user_access_type;
pub mod system_info;
pub mod task_completion_status;
pub mod task_info;
pub mod task_result;
pub mod task_state;
pub mod task_trigger_info;
pub mod theme_media_result;
pub mod timer_event_info;
pub mod timer_info_dto;
pub mod timer_info_dto_query_result;
pub mod trailer_info;
pub mod trailer_info_remote_search_query;
pub mod transcode_reason;
pub mod transcode_seek_info;
pub mod transcoding_info;
pub mod transcoding_profile;
pub mod transport_stream_timestamp;
pub mod tuner_channel_mapping;
pub mod tuner_host_info;
pub mod type_options;
pub mod unrated_item;
pub mod update_library_options_dto;
pub mod update_media_path_request_dto;
pub mod update_user_easy_password;
pub mod update_user_password;
pub mod upload_subtitle_dto;
pub mod user_configuration;
pub mod user_dto;
pub mod user_item_data_dto;
pub mod user_policy;
pub mod utc_time_response;
pub mod validate_path_dto;
pub mod version_info;
pub mod video3_dformat;
pub mod video_type;
pub mod virtual_folder_info;
pub mod wake_on_lan_info;
pub mod xbmc_metadata_options;
pub mod xml_attribute;

pub type Architecture = String;
pub type BaseItemKind = String;
pub type ChannelItemSortField = String;
pub type ChannelMediaContentType = String;
pub type ChannelMediaType = String;
pub type ChannelType = String;
pub type CodecType = String;
pub type CollectionTypeOptions = String;
pub type DayOfWeek = String;
pub type DayPattern = String;
pub type DeviceProfileType = String;
pub type DlnaProfileType = String;
pub type DynamicDayOfWeek = String;
pub type EmbeddedSubtitleOptions = String;
pub type EncodingContext = String;
pub type ExternalIdMediaType = String;
pub type FFmpegLocation = String;
pub type FileSystemEntryType = String;
pub type ForgotPasswordAction = String;
pub type GeneralCommandType = String;
pub type GroupQueueMode = String;
pub type GroupRepeatMode = String;
pub type GroupShuffleMode = String;
pub type GroupStateType = String;
pub type GroupUpdateType = String;
pub type HardwareEncodingType = String;
pub type HeaderMatchType = String;
pub type ImageFormat = String;
pub type ImageOrientation = String;
pub type ImageSavingConvention = String;
pub type ImageType = String;
pub type IsoType = String;
pub type ItemFields = String;
pub type ItemFilter = String;
pub type KeepUntil = String;
pub type LiveTvServiceStatus = String;
pub type LocationType = String;
pub type LogLevel = String;
pub type MediaProtocol = String;
pub type MediaSourceType = String;
pub type MediaStreamType = String;
pub type MetadataField = String;
pub type MetadataRefreshMode = String;
pub type NotificationLevel = String;
pub type PlayAccess = String;
pub type PlayCommand = String;
pub type PlayMethod = String;
pub type PlaybackErrorCode = String;
pub type PlaystateCommand = String;
pub type PluginStatus = String;
pub type ProfileConditionType = String;
pub type ProfileConditionValue = String;
pub type ProgramAudio = String;
pub type RatingType = String;
pub type RecommendationType = String;
pub type RecordingStatus = String;
pub type RepeatMode = String;
pub type ScrollDirection = String;
pub type SendCommandType = String;
pub type SendToUserType = String;
pub type SeriesStatus = String;
pub type SessionMessageType = String;
pub type SortOrder = String;
pub type SubtitleDeliveryMethod = String;
pub type SubtitlePlaybackMode = String;
pub type SyncPlayUserAccessType = String;
pub type TaskCompletionStatus = String;
pub type TaskState = String;
pub type TranscodeReason = String;
pub type TranscodeSeekInfo = String;
pub type TransportStreamTimestamp = String;
pub type UnratedItem = String;
pub type Video3DFormat = String;
pub type VideoType = String;
