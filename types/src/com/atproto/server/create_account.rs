// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `com.atproto.server.createAccount` namespace.
pub const NSID: &str = "com.atproto.server.createAccount";
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InputData {
    ///Pre-existing atproto DID, being imported to a new account.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub did: core::option::Option<atrium_api::types::string::Did>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub email: core::option::Option<String>,
    ///Requested handle for the account.
    pub handle: atrium_api::types::string::Handle,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub invite_code: core::option::Option<String>,
    ///Initial account password. May need to meet instance-specific password strength requirements.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub password: core::option::Option<String>,
    ///A signed DID PLC operation to be submitted as part of importing an existing account to this instance. NOTE: this optional field may be updated when full account migration is implemented.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub plc_op: core::option::Option<atrium_api::types::Unknown>,
    ///DID PLC rotation key (aka, recovery key) to be included in PLC creation operation.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub recovery_key: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub verification_code: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub verification_phone: core::option::Option<String>,
}
pub type Input = atrium_api::types::Object<InputData>;
///Account login session returned on successful account creation.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct OutputData {
    pub access_jwt: String,
    ///The DID of the new account.
    pub did: atrium_api::types::string::Did,
    ///Complete DID document.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub did_doc: core::option::Option<atrium_api::types::Unknown>,
    pub handle: atrium_api::types::string::Handle,
    pub refresh_jwt: String,
}
pub type Output = atrium_api::types::Object<OutputData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "error", content = "message")]
pub enum Error {
    InvalidHandle(Option<String>),
    InvalidPassword(Option<String>),
    InvalidInviteCode(Option<String>),
    HandleNotAvailable(Option<String>),
    UnsupportedDomain(Option<String>),
    UnresolvableDid(Option<String>),
    IncompatibleDidDoc(Option<String>),
}
impl std::fmt::Display for Error {
    fn fmt(&self, _f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Error::InvalidHandle(msg) => {
                write!(_f, "InvalidHandle")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::InvalidPassword(msg) => {
                write!(_f, "InvalidPassword")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::InvalidInviteCode(msg) => {
                write!(_f, "InvalidInviteCode")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::HandleNotAvailable(msg) => {
                write!(_f, "HandleNotAvailable")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::UnsupportedDomain(msg) => {
                write!(_f, "UnsupportedDomain")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::UnresolvableDid(msg) => {
                write!(_f, "UnresolvableDid")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
            Error::IncompatibleDidDoc(msg) => {
                write!(_f, "IncompatibleDidDoc")?;
                if let Some(msg) = msg {
                    write!(_f, ": {msg}")?;
                }
            }
        }
        Ok(())
    }
}
