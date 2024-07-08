use crate::http_header_info::HttpHeaderInfo;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct DeviceIdentification {
    #[serde(rename = "FriendlyName", skip_serializing_if = "Option:: is_none")]
    pub friendly_name: Option<String>,
    #[serde(rename = "ModelNumber", skip_serializing_if = "Option:: is_none")]
    pub model_number: Option<String>,
    #[serde(rename = "SerialNumber", skip_serializing_if = "Option:: is_none")]
    pub serial_number: Option<String>,
    #[serde(rename = "ModelName", skip_serializing_if = "Option:: is_none")]
    pub model_name: Option<String>,
    #[serde(rename = "ModelDescription", skip_serializing_if = "Option:: is_none")]
    pub model_description: Option<String>,
    #[serde(rename = "ModelUrl", skip_serializing_if = "Option:: is_none")]
    pub model_url: Option<String>,
    #[serde(rename = "Manufacturer", skip_serializing_if = "Option:: is_none")]
    pub manufacturer: Option<String>,
    #[serde(rename = "ManufacturerUrl", skip_serializing_if = "Option:: is_none")]
    pub manufacturer_url: Option<String>,
    #[serde(rename = "Headers", skip_serializing_if = "Option:: is_none")]
    pub headers: Option<Vec<HttpHeaderInfo>>,
}
