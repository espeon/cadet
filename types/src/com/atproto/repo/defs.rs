// @generated - This file is generated by atrium-codegen. DO NOT EDIT.
//!Definitions for the `com.atproto.repo.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct CommitMetaData {
    pub cid: crate::types::string::Cid,
    pub rev: crate::types::string::Tid,
}
pub type CommitMeta = crate::types::Object<CommitMetaData>;
