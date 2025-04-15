// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `tools.ozone.moderation.queryStatuses` namespace.
pub const NSID: &str = "tools.ozone.moderation.queryStatuses";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ParametersData {
    ///Get subjects in unresolved appealed status
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub appealed: core::option::Option<bool>,
    ///If specified, subjects belonging to the given collections will be returned. When subjectType is set to 'account', this will be ignored.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub collections: core::option::Option<Vec<atrium_api::types::string::Nsid>>,
    ///Search subjects by keyword from comments
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub comment: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub exclude_tags: core::option::Option<Vec<String>>,
    ///Search subjects where the associated record/account was deleted after a given timestamp
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hosting_deleted_after: core::option::Option<atrium_api::types::string::Datetime>,
    ///Search subjects where the associated record/account was deleted before a given timestamp
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hosting_deleted_before: core::option::Option<
        atrium_api::types::string::Datetime,
    >,
    ///Search subjects by the status of the associated record/account
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hosting_statuses: core::option::Option<Vec<String>>,
    ///Search subjects where the associated record/account was updated after a given timestamp
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hosting_updated_after: core::option::Option<atrium_api::types::string::Datetime>,
    ///Search subjects where the associated record/account was updated before a given timestamp
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hosting_updated_before: core::option::Option<
        atrium_api::types::string::Datetime,
    >,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub ignore_subjects: core::option::Option<Vec<String>>,
    ///All subjects, or subjects from given 'collections' param, belonging to the account specified in the 'subject' param will be returned.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub include_all_user_records: core::option::Option<bool>,
    ///By default, we don't include muted subjects in the results. Set this to true to include them.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub include_muted: core::option::Option<bool>,
    ///Get all subject statuses that were reviewed by a specific moderator
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub last_reviewed_by: core::option::Option<atrium_api::types::string::Did>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub limit: core::option::Option<atrium_api::types::LimitedNonZeroU8<100u8>>,
    ///If specified, only subjects that belong to an account that has at least this many suspensions will be returned.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub min_account_suspend_count: core::option::Option<i64>,
    ///If specified, only subjects that have priority score value above the given value will be returned.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub min_priority_score: core::option::Option<atrium_api::types::LimitedU8<100u8>>,
    ///If specified, only subjects that belong to an account that has at least this many reported records will be returned.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub min_reported_records_count: core::option::Option<i64>,
    ///If specified, only subjects that belong to an account that has at least this many taken down records will be returned.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub min_takendown_records_count: core::option::Option<i64>,
    ///When set to true, only muted subjects and reporters will be returned.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub only_muted: core::option::Option<bool>,
    ///Number of queues being used by moderators. Subjects will be split among all queues.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub queue_count: core::option::Option<i64>,
    ///Index of the queue to fetch subjects from. Works only when queueCount value is specified.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub queue_index: core::option::Option<i64>,
    ///A seeder to shuffle/balance the queue items.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub queue_seed: core::option::Option<String>,
    ///Search subjects reported after a given timestamp
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub reported_after: core::option::Option<atrium_api::types::string::Datetime>,
    ///Search subjects reported before a given timestamp
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub reported_before: core::option::Option<atrium_api::types::string::Datetime>,
    ///Specify when fetching subjects in a certain state
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub review_state: core::option::Option<String>,
    ///Search subjects reviewed after a given timestamp
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub reviewed_after: core::option::Option<atrium_api::types::string::Datetime>,
    ///Search subjects reviewed before a given timestamp
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub reviewed_before: core::option::Option<atrium_api::types::string::Datetime>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub sort_direction: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub sort_field: core::option::Option<String>,
    ///The subject to get the status for.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub subject: core::option::Option<String>,
    ///If specified, subjects of the given type (account or record) will be returned. When this is set to 'account' the 'collections' parameter will be ignored. When includeAllUserRecords or subject is set, this will be ignored.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub subject_type: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub tags: core::option::Option<Vec<String>>,
    ///Get subjects that were taken down
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub takendown: core::option::Option<bool>,
}
pub type Parameters = atrium_api::types::Object<ParametersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub cursor: core::option::Option<String>,
    pub subject_statuses: Vec<crate::tools::ozone::moderation::defs::SubjectStatusView>,
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
