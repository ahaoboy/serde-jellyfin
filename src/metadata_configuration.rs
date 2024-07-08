#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct MetadataConfiguration {
    #[serde(
        rename = "UseFileCreationTimeForDateAdded",
        skip_serializing_if = "Option:: is_none"
    )]
    pub use_file_creation_time_for_date_added: Option<bool>,
}
