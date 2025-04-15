// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `tools.ozone.communication.updateTemplate` namespace.
pub const NSID: &str = "tools.ozone.communication.updateTemplate";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    ///Content of the template, markdown supported, can contain variable placeholders.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub content_markdown: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub disabled: core::option::Option<bool>,
    ///ID of the template to be updated.
    pub id: String,
    ///Message language.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub lang: core::option::Option<atrium_api::types::string::Language>,
    ///Name of the template.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub name: core::option::Option<String>,
    ///Subject of the message, used in emails.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub subject: core::option::Option<String>,
    ///DID of the user who is updating the template.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub updated_by: core::option::Option<atrium_api::types::string::Did>,
}
pub type Input = atrium_api::types::Object<InputData>;
pub type Output = crate::tools::ozone::communication::defs::TemplateView;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    DuplicateTemplateName(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::DuplicateTemplateName(msg) => {
                write!(_f, "DuplicateTemplateName")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
