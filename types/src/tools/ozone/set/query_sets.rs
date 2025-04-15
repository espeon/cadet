// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `tools.ozone.set.querySets` namespace.
pub const NSID: &str = "tools.ozone.set.querySets";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub limit: core::option::Option<atrium_api::types::LimitedNonZeroU8<100u8>>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub name_prefix: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub sort_by: core::option::Option<String>,
    ///Defaults to ascending order of name field.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub sort_direction: core::option::Option<String>,
}
pub type Parameters = atrium_api::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    pub sets: Vec<crate::tools::ozone::set::defs::SetView>,
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
