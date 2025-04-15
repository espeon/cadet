// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `app.bsky.video.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct JobStatusData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub blob: core::option::Option<atrium_api::types::BlobRef>,
    pub did: atrium_api::types::string::Did,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub error: core::option::Option<String>,
    pub job_id: String,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub message: core::option::Option<String>,
    ///Progress within the current processing state.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub progress: core::option::Option<atrium_api::types::LimitedU8<100u8>>,
    ///The state of the video processing job. All values not listed as a known value indicate that the job is in process.
    pub state: String,
}
pub type JobStatus = atrium_api::types::Object<JobStatusData>;
