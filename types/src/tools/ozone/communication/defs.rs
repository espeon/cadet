// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `tools.ozone.communication.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TemplateViewData {
    ///Subject of the message, used in emails.
    pub content_markdown: String,
    pub created_at: atrium_api::types::string::Datetime,
    pub disabled: bool,
    pub id: String,
    ///Message language.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub lang: core::option::Option<atrium_api::types::string::Language>,
    ///DID of the user who last updated the template.
    pub last_updated_by: atrium_api::types::string::Did,
    ///Name of the template.
    pub name: String,
    ///Content of the template, can contain markdown and variable placeholders.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub subject: core::option::Option<String>,
    pub updated_at: atrium_api::types::string::Datetime,
}
pub type TemplateView = atrium_api::types::Object<TemplateViewData>;
