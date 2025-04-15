// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `fm.teal.alpha.feed.defs` namespace.
//!This lexicon is in a not officially released state. It is subject to change. | Misc. items related to feeds.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PlayViewData {
    ///Array of Musicbrainz artist IDs
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub artist_mb_ids: core::option::Option<Vec<String>>,
    ///Array of artist names in order of original appearance.
    pub artist_names: Vec<String>,
    ///The length of the track in seconds
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub duration: core::option::Option<i64>,
    ///The ISRC code associated with the recording
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub isrc: core::option::Option<String>,
    ///The base domain of the music service. e.g. music.apple.com, tidal.com, spotify.com. Defaults to 'local' if not provided.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub music_service_base_domain: core::option::Option<String>,
    ///The URL associated with this track
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub origin_url: core::option::Option<String>,
    ///The unix timestamp of when the track was played
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub played_time: core::option::Option<atrium_api::types::string::Datetime>,
    ///The Musicbrainz recording ID of the track
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub recording_mb_id: core::option::Option<String>,
    ///The Musicbrainz release ID
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub release_mb_id: core::option::Option<String>,
    ///The name of the release/album
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub release_name: core::option::Option<String>,
    ///A user-agent style string specifying the user agent. e.g. tealtracker/0.0.1b (Linux; Android 13; SM-A715F). Defaults to 'manual/unknown' if not provided.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub submission_client_agent: core::option::Option<String>,
    ///The Musicbrainz ID of the track
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub track_mb_id: core::option::Option<String>,
    ///The name of the track
    pub track_name: String,
}
pub type PlayView = atrium_api::types::Object<PlayViewData>;
