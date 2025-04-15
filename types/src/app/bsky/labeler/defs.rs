// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `app.bsky.labeler.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelerPoliciesData {
    ///Label values created by this labeler and scoped exclusively to it. Labels defined here will override global label definitions for this labeler.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub label_value_definitions: core::option::Option<
        Vec<crate::com::atproto::label::defs::LabelValueDefinition>,
    >,
    ///The label values which this labeler publishes. May include global or custom labels.
    pub label_values: Vec<crate::com::atproto::label::defs::LabelValue>,
}
pub type LabelerPolicies = atrium_api::types::Object<LabelerPoliciesData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelerViewData {
    pub cid: atrium_api::types::string::Cid,
    pub creator: crate::app::bsky::actor::defs::ProfileView,
    pub indexed_at: atrium_api::types::string::Datetime,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub labels: core::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub like_count: core::option::Option<usize>,
    pub uri: String,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub viewer: core::option::Option<LabelerViewerState>,
}
pub type LabelerView = atrium_api::types::Object<LabelerViewData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelerViewDetailedData {
    pub cid: atrium_api::types::string::Cid,
    pub creator: crate::app::bsky::actor::defs::ProfileView,
    pub indexed_at: atrium_api::types::string::Datetime,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub labels: core::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub like_count: core::option::Option<usize>,
    pub policies: crate::app::bsky::labeler::defs::LabelerPolicies,
    pub uri: String,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub viewer: core::option::Option<LabelerViewerState>,
}
pub type LabelerViewDetailed = atrium_api::types::Object<LabelerViewDetailedData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelerViewerStateData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub like: core::option::Option<String>,
}
pub type LabelerViewerState = atrium_api::types::Object<LabelerViewerStateData>;
