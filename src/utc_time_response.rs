#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct UtcTimeResponse {
    #[serde(
        rename = "RequestReceptionTime",
        skip_serializing_if = "Option:: is_none"
    )]
    pub request_reception_time: Option<String>,
    #[serde(
        rename = "ResponseTransmissionTime",
        skip_serializing_if = "Option:: is_none"
    )]
    pub response_transmission_time: Option<String>,
}
