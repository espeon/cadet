// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `app.bsky.feed.getQuotes` namespace.
pub const NSID: &str = "app.bsky.feed.getQuotes";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    ///If supplied, filters to quotes of specific version (by CID) of the post record.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cid: core::option::Option<atrium_api::types::string::Cid>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub limit: core::option::Option<atrium_api::types::LimitedNonZeroU8<100u8>>,
    ///Reference (AT-URI) of post record
    pub uri: String,
}
pub type Parameters = atrium_api::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cid: core::option::Option<atrium_api::types::string::Cid>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    pub posts: Vec<crate::app::bsky::feed::defs::PostView>,
    pub uri: String,
}
pub type Output = atrium_api::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
