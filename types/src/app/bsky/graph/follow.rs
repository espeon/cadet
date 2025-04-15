// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `app.bsky.graph.follow` namespace.
use atrium_api::types::TryFromUnknown;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct RecordData {
    pub created_at: atrium_api::types::string::Datetime,
    pub subject: atrium_api::types::string::Did,
}
pub type Record = atrium_api::types::Object<RecordData>;
impl From<atrium_api::types::Unknown> for RecordData {
    fn from(value: atrium_api::types::Unknown) -> Self {
        Self::try_from_unknown(value).unwrap()
    }
}
