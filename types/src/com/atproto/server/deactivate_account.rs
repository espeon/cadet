// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.server.deactivateAccount` namespace.
pub const NSID: &str = "com.atproto.server.deactivateAccount";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    ///A recommendation to server as to how long they should hold onto the deactivated account before deleting.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub delete_after: core::option::Option<crate::types::string::Datetime>,
}
pub type Input = crate::types::Object<InputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        Ok(())
    }
}
