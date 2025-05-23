// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `app.bsky.richtext.facet` namespace.
///Annotation of a sub-string within rich text.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MainData {
    pub features: Vec<atrium_api::types::Union<MainFeaturesItem>>,
    pub index: ByteSlice,
}
pub type Main = atrium_api::types::Object<MainData>;
///Specifies the sub-string range a facet feature applies to. Start index is inclusive, end index is exclusive. Indices are zero-indexed, counting bytes of the UTF-8 encoded text. NOTE: some languages, like Javascript, use UTF-16 or Unicode codepoints for string slice indexing; in these languages, convert to byte arrays before working with facets.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ByteSliceData {
    pub byte_end: usize,
    pub byte_start: usize,
}
pub type ByteSlice = atrium_api::types::Object<ByteSliceData>;
///Facet feature for a URL. The text URL may have been simplified or truncated, but the facet reference should be a complete URL.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LinkData {
    pub uri: String,
}
pub type Link = atrium_api::types::Object<LinkData>;
///Facet feature for mention of another account. The text is usually a handle, including a '@' prefix, but the facet reference is a DID.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MentionData {
    pub did: atrium_api::types::string::Did,
}
pub type Mention = atrium_api::types::Object<MentionData>;
///Facet feature for a hashtag. The text usually includes a '#' prefix, but the facet reference should not (except in the case of 'double hash tags').
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct TagData {
    pub tag: String,
}
pub type Tag = atrium_api::types::Object<TagData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum MainFeaturesItem {
    #[serde(rename = "app.bsky.richtext.facet#mention")]
    Mention(Box<Mention>),
    #[serde(rename = "app.bsky.richtext.facet#link")]
    Link(Box<Link>),
    #[serde(rename = "app.bsky.richtext.facet#tag")]
    Tag(Box<Tag>),
}
