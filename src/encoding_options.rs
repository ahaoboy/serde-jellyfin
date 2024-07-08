#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EncodingOptions {
    #[serde(
        rename = "EncodingThreadCount",
        skip_serializing_if = "Option:: is_none"
    )]
    pub encoding_thread_count: Option<f32>,
    #[serde(
        rename = "TranscodingTempPath",
        skip_serializing_if = "Option:: is_none"
    )]
    pub transcoding_temp_path: Option<String>,
    #[serde(rename = "FallbackFontPath", skip_serializing_if = "Option:: is_none")]
    pub fallback_font_path: Option<String>,
    #[serde(
        rename = "EnableFallbackFont",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_fallback_font: Option<bool>,
    #[serde(rename = "DownMixAudioBoost", skip_serializing_if = "Option:: is_none")]
    pub down_mix_audio_boost: Option<f32>,
    #[serde(
        rename = "MaxMuxingQueueSize",
        skip_serializing_if = "Option:: is_none"
    )]
    pub max_muxing_queue_size: Option<f32>,
    #[serde(rename = "EnableThrottling", skip_serializing_if = "Option:: is_none")]
    pub enable_throttling: Option<bool>,
    #[serde(
        rename = "ThrottleDelaySeconds",
        skip_serializing_if = "Option:: is_none"
    )]
    pub throttle_delay_seconds: Option<f32>,
    #[serde(
        rename = "HardwareAccelerationType",
        skip_serializing_if = "Option:: is_none"
    )]
    pub hardware_acceleration_type: Option<String>,
    #[serde(rename = "EncoderAppPath", skip_serializing_if = "Option:: is_none")]
    pub encoder_app_path: Option<String>,
    #[serde(
        rename = "EncoderAppPathDisplay",
        skip_serializing_if = "Option:: is_none"
    )]
    pub encoder_app_path_display: Option<String>,
    #[serde(rename = "VaapiDevice", skip_serializing_if = "Option:: is_none")]
    pub vaapi_device: Option<String>,
    #[serde(rename = "EnableTonemapping", skip_serializing_if = "Option:: is_none")]
    pub enable_tonemapping: Option<bool>,
    #[serde(
        rename = "EnableVppTonemapping",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_vpp_tonemapping: Option<bool>,
    #[serde(
        rename = "TonemappingAlgorithm",
        skip_serializing_if = "Option:: is_none"
    )]
    pub tonemapping_algorithm: Option<String>,
    #[serde(rename = "TonemappingMode", skip_serializing_if = "Option:: is_none")]
    pub tonemapping_mode: Option<String>,
    #[serde(rename = "TonemappingRange", skip_serializing_if = "Option:: is_none")]
    pub tonemapping_range: Option<String>,
    #[serde(rename = "TonemappingDesat", skip_serializing_if = "Option:: is_none")]
    pub tonemapping_desat: Option<f32>,
    #[serde(rename = "TonemappingPeak", skip_serializing_if = "Option:: is_none")]
    pub tonemapping_peak: Option<f32>,
    #[serde(rename = "TonemappingParam", skip_serializing_if = "Option:: is_none")]
    pub tonemapping_param: Option<f32>,
    #[serde(
        rename = "VppTonemappingBrightness",
        skip_serializing_if = "Option:: is_none"
    )]
    pub vpp_tonemapping_brightness: Option<f32>,
    #[serde(
        rename = "VppTonemappingContrast",
        skip_serializing_if = "Option:: is_none"
    )]
    pub vpp_tonemapping_contrast: Option<f32>,
    #[serde(rename = "H264Crf", skip_serializing_if = "Option:: is_none")]
    pub h264_crf: Option<f32>,
    #[serde(rename = "H265Crf", skip_serializing_if = "Option:: is_none")]
    pub h265_crf: Option<f32>,
    #[serde(rename = "EncoderPreset", skip_serializing_if = "Option:: is_none")]
    pub encoder_preset: Option<String>,
    #[serde(
        rename = "DeinterlaceDoubleRate",
        skip_serializing_if = "Option:: is_none"
    )]
    pub deinterlace_double_rate: Option<bool>,
    #[serde(rename = "DeinterlaceMethod", skip_serializing_if = "Option:: is_none")]
    pub deinterlace_method: Option<String>,
    #[serde(
        rename = "EnableDecodingColorDepth10Hevc",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_decoding_color_depth10_hevc: Option<bool>,
    #[serde(
        rename = "EnableDecodingColorDepth10Vp9",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_decoding_color_depth10_vp9: Option<bool>,
    #[serde(
        rename = "EnableEnhancedNvdecDecoder",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_enhanced_nvdec_decoder: Option<bool>,
    #[serde(
        rename = "PreferSystemNativeHwDecoder",
        skip_serializing_if = "Option:: is_none"
    )]
    pub prefer_system_native_hw_decoder: Option<bool>,
    #[serde(
        rename = "EnableIntelLowPowerH264HwEncoder",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_intel_low_power_h264_hw_encoder: Option<bool>,
    #[serde(
        rename = "EnableIntelLowPowerHevcHwEncoder",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_intel_low_power_hevc_hw_encoder: Option<bool>,
    #[serde(
        rename = "EnableHardwareEncoding",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_hardware_encoding: Option<bool>,
    #[serde(rename = "AllowHevcEncoding", skip_serializing_if = "Option:: is_none")]
    pub allow_hevc_encoding: Option<bool>,
    #[serde(
        rename = "EnableSubtitleExtraction",
        skip_serializing_if = "Option:: is_none"
    )]
    pub enable_subtitle_extraction: Option<bool>,
    #[serde(
        rename = "HardwareDecodingCodecs",
        skip_serializing_if = "Option:: is_none"
    )]
    pub hardware_decoding_codecs: Option<Vec<String>>,
    #[serde(
        rename = "AllowOnDemandMetadataBasedKeyframeExtractionForExtensions",
        skip_serializing_if = "Option:: is_none"
    )]
    pub allow_on_demand_metadata_based_keyframe_extraction_for_extensions: Option<Vec<String>>,
}
