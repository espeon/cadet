// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `app.bsky.unspecced.searchActorsSkeleton` namespace.
pub const NSID: &str = "app.bsky.unspecced.searchActorsSkeleton";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    ///Optional pagination mechanism; may not necessarily allow scrolling through entire result set.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub limit: core::option::Option<atrium_api::types::LimitedNonZeroU8<100u8>>,
    ///Search query string; syntax, phrase, boolean, and faceting is unspecified, but Lucene query syntax is recommended. For typeahead search, only simple term match is supported, not full syntax.
    pub q: String,
    ///If true, acts as fast/simple 'typeahead' query.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub typeahead: core::option::Option<bool>,
    ///DID of the account making the request (not included for public/unauthenticated queries). Used to boost followed accounts in ranking.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub viewer: core::option::Option<atrium_api::types::string::Did>,
}
pub type Parameters = atrium_api::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub actors: Vec<crate::app::bsky::unspecced::defs::SkeletonSearchActor>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    ///Count of search hits. Optional, may be rounded/truncated, and may not be possible to paginate through all hits.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hits_total: core::option::Option<i64>,
}
pub type Output = atrium_api::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    BadQueryString(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::BadQueryString(msg) => {
                write!(_f, "BadQueryString")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
