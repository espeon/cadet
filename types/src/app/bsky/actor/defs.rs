// @generated - This file is generated by esquema-codegen (forked from atrium-codegen). DO NOT EDIT.
//!Definitions for the `app.bsky.actor.defs` namespace.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct AdultContentPrefData {
    pub enabled: bool,
}
pub type AdultContentPref = atrium_api::types::Object<AdultContentPrefData>;
///If set, an active progress guide. Once completed, can be set to undefined. Should have unspecced fields tracking progress.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BskyAppProgressGuideData {
    pub guide: String,
}
pub type BskyAppProgressGuide = atrium_api::types::Object<BskyAppProgressGuideData>;
///A grab bag of state that's specific to the bsky.app program. Third-party apps shouldn't use this.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct BskyAppStatePrefData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub active_progress_guide: core::option::Option<BskyAppProgressGuide>,
    ///Storage for NUXs the user has encountered.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub nuxs: core::option::Option<Vec<crate::app::bsky::actor::defs::Nux>>,
    ///An array of tokens which identify nudges (modals, popups, tours, highlight dots) that should be shown to the user.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub queued_nudges: core::option::Option<Vec<String>>,
}
pub type BskyAppStatePref = atrium_api::types::Object<BskyAppStatePrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ContentLabelPrefData {
    pub label: String,
    ///Which labeler does this preference apply to? If undefined, applies globally.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub labeler_did: core::option::Option<atrium_api::types::string::Did>,
    pub visibility: String,
}
pub type ContentLabelPref = atrium_api::types::Object<ContentLabelPrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct FeedViewPrefData {
    ///The URI of the feed, or an identifier which describes the feed.
    pub feed: String,
    ///Hide quote posts in the feed.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hide_quote_posts: core::option::Option<bool>,
    ///Hide replies in the feed.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hide_replies: core::option::Option<bool>,
    ///Hide replies in the feed if they do not have this number of likes.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hide_replies_by_like_count: core::option::Option<i64>,
    ///Hide replies in the feed if they are not by followed users.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hide_replies_by_unfollowed: core::option::Option<bool>,
    ///Hide reposts in the feed.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub hide_reposts: core::option::Option<bool>,
}
pub type FeedViewPref = atrium_api::types::Object<FeedViewPrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct HiddenPostsPrefData {
    ///A list of URIs of posts the account owner has hidden.
    pub items: Vec<String>,
}
pub type HiddenPostsPref = atrium_api::types::Object<HiddenPostsPrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct InterestsPrefData {
    ///A list of tags which describe the account owner's interests gathered during onboarding.
    pub tags: Vec<String>,
}
pub type InterestsPref = atrium_api::types::Object<InterestsPrefData>;
///The subject's followers whom you also follow
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct KnownFollowersData {
    pub count: i64,
    pub followers: Vec<ProfileViewBasic>,
}
pub type KnownFollowers = atrium_api::types::Object<KnownFollowersData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelerPrefItemData {
    pub did: atrium_api::types::string::Did,
}
pub type LabelerPrefItem = atrium_api::types::Object<LabelerPrefItemData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct LabelersPrefData {
    pub labelers: Vec<LabelerPrefItem>,
}
pub type LabelersPref = atrium_api::types::Object<LabelersPrefData>;
///A word that the account owner has muted.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutedWordData {
    ///Groups of users to apply the muted word to. If undefined, applies to all users.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub actor_target: core::option::Option<String>,
    ///The date and time at which the muted word will expire and no longer be applied.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub expires_at: core::option::Option<atrium_api::types::string::Datetime>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub id: core::option::Option<String>,
    ///The intended targets of the muted word.
    pub targets: Vec<crate::app::bsky::actor::defs::MutedWordTarget>,
    ///The muted word itself.
    pub value: String,
}
pub type MutedWord = atrium_api::types::Object<MutedWordData>;
pub type MutedWordTarget = String;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct MutedWordsPrefData {
    ///A list of words the account owner has muted.
    pub items: Vec<crate::app::bsky::actor::defs::MutedWord>,
}
pub type MutedWordsPref = atrium_api::types::Object<MutedWordsPrefData>;
///A new user experiences (NUX) storage object
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct NuxData {
    pub completed: bool,
    ///Arbitrary data for the NUX. The structure is defined by the NUX itself. Limited to 300 characters.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub data: core::option::Option<String>,
    ///The date and time at which the NUX will expire and should be considered completed.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub expires_at: core::option::Option<atrium_api::types::string::Datetime>,
    pub id: String,
}
pub type Nux = atrium_api::types::Object<NuxData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PersonalDetailsPrefData {
    ///The birth date of account owner.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub birth_date: core::option::Option<atrium_api::types::string::Datetime>,
}
pub type PersonalDetailsPref = atrium_api::types::Object<PersonalDetailsPrefData>;
///Default post interaction settings for the account. These values should be applied as default values when creating new posts. These refs should mirror the threadgate and postgate records exactly.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct PostInteractionSettingsPrefData {
    ///Matches postgate record. List of rules defining who can embed this users posts. If value is an empty array or is undefined, no particular rules apply and anyone can embed.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub postgate_embedding_rules: core::option::Option<
        Vec<
            atrium_api::types::Union<
                PostInteractionSettingsPrefPostgateEmbeddingRulesItem,
            >,
        >,
    >,
    ///Matches threadgate record. List of rules defining who can reply to this users posts. If value is an empty array, no one can reply. If value is undefined, anyone can reply.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub threadgate_allow_rules: core::option::Option<
        Vec<
            atrium_api::types::Union<PostInteractionSettingsPrefThreadgateAllowRulesItem>,
        >,
    >,
}
pub type PostInteractionSettingsPref = atrium_api::types::Object<
    PostInteractionSettingsPrefData,
>;
pub type Preferences = Vec<atrium_api::types::Union<PreferencesItem>>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileAssociatedData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub chat: core::option::Option<ProfileAssociatedChat>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub feedgens: core::option::Option<i64>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub labeler: core::option::Option<bool>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub lists: core::option::Option<i64>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub starter_packs: core::option::Option<i64>,
}
pub type ProfileAssociated = atrium_api::types::Object<ProfileAssociatedData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileAssociatedChatData {
    pub allow_incoming: String,
}
pub type ProfileAssociatedChat = atrium_api::types::Object<ProfileAssociatedChatData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileViewData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub associated: core::option::Option<ProfileAssociated>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub avatar: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub created_at: core::option::Option<atrium_api::types::string::Datetime>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub description: core::option::Option<String>,
    pub did: atrium_api::types::string::Did,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub display_name: core::option::Option<String>,
    pub handle: atrium_api::types::string::Handle,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub indexed_at: core::option::Option<atrium_api::types::string::Datetime>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub labels: core::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub viewer: core::option::Option<ViewerState>,
}
pub type ProfileView = atrium_api::types::Object<ProfileViewData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileViewBasicData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub associated: core::option::Option<ProfileAssociated>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub avatar: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub created_at: core::option::Option<atrium_api::types::string::Datetime>,
    pub did: atrium_api::types::string::Did,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub display_name: core::option::Option<String>,
    pub handle: atrium_api::types::string::Handle,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub labels: core::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub viewer: core::option::Option<ViewerState>,
}
pub type ProfileViewBasic = atrium_api::types::Object<ProfileViewBasicData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ProfileViewDetailedData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub associated: core::option::Option<ProfileAssociated>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub avatar: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub banner: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub created_at: core::option::Option<atrium_api::types::string::Datetime>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub description: core::option::Option<String>,
    pub did: atrium_api::types::string::Did,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub display_name: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub followers_count: core::option::Option<i64>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub follows_count: core::option::Option<i64>,
    pub handle: atrium_api::types::string::Handle,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub indexed_at: core::option::Option<atrium_api::types::string::Datetime>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub joined_via_starter_pack: core::option::Option<
        crate::app::bsky::graph::defs::StarterPackViewBasic,
    >,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub labels: core::option::Option<Vec<crate::com::atproto::label::defs::Label>>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub pinned_post: core::option::Option<crate::com::atproto::repo::strong_ref::Main>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub posts_count: core::option::Option<i64>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub viewer: core::option::Option<ViewerState>,
}
pub type ProfileViewDetailed = atrium_api::types::Object<ProfileViewDetailedData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SavedFeedData {
    pub id: String,
    pub pinned: bool,
    pub r#type: String,
    pub value: String,
}
pub type SavedFeed = atrium_api::types::Object<SavedFeedData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SavedFeedsPrefData {
    pub pinned: Vec<String>,
    pub saved: Vec<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub timeline_index: core::option::Option<i64>,
}
pub type SavedFeedsPref = atrium_api::types::Object<SavedFeedsPrefData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct SavedFeedsPrefV2Data {
    pub items: Vec<crate::app::bsky::actor::defs::SavedFeed>,
}
pub type SavedFeedsPrefV2 = atrium_api::types::Object<SavedFeedsPrefV2Data>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ThreadViewPrefData {
    ///Show followed users at the top of all replies.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub prioritize_followed_users: core::option::Option<bool>,
    ///Sorting mode for threads.
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub sort: core::option::Option<String>,
}
pub type ThreadViewPref = atrium_api::types::Object<ThreadViewPrefData>;
///Metadata about the requesting account's relationship with the subject account. Only has meaningful content for authed requests.
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(rename_all = "camelCase")]
pub struct ViewerStateData {
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub blocked_by: core::option::Option<bool>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub blocking: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub blocking_by_list: core::option::Option<
        crate::app::bsky::graph::defs::ListViewBasic,
    >,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub followed_by: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub following: core::option::Option<String>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub known_followers: core::option::Option<KnownFollowers>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub muted: core::option::Option<bool>,
    #[serde(skip_serializing_if = "core::option::Option::is_none")]
    pub muted_by_list: core::option::Option<
        crate::app::bsky::graph::defs::ListViewBasic,
    >,
}
pub type ViewerState = atrium_api::types::Object<ViewerStateData>;
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum PostInteractionSettingsPrefPostgateEmbeddingRulesItem {
    #[serde(rename = "app.bsky.feed.postgate#disableRule")]
    AppBskyFeedPostgateDisableRule(Box<crate::app::bsky::feed::postgate::DisableRule>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum PostInteractionSettingsPrefThreadgateAllowRulesItem {
    #[serde(rename = "app.bsky.feed.threadgate#mentionRule")]
    AppBskyFeedThreadgateMentionRule(
        Box<crate::app::bsky::feed::threadgate::MentionRule>,
    ),
    #[serde(rename = "app.bsky.feed.threadgate#followerRule")]
    AppBskyFeedThreadgateFollowerRule(
        Box<crate::app::bsky::feed::threadgate::FollowerRule>,
    ),
    #[serde(rename = "app.bsky.feed.threadgate#followingRule")]
    AppBskyFeedThreadgateFollowingRule(
        Box<crate::app::bsky::feed::threadgate::FollowingRule>,
    ),
    #[serde(rename = "app.bsky.feed.threadgate#listRule")]
    AppBskyFeedThreadgateListRule(Box<crate::app::bsky::feed::threadgate::ListRule>),
}
#[derive(serde::Serialize, serde::Deserialize, Debug, Clone, PartialEq, Eq)]
#[serde(tag = "$type")]
pub enum PreferencesItem {
    #[serde(rename = "app.bsky.actor.defs#adultContentPref")]
    AdultContentPref(Box<AdultContentPref>),
    #[serde(rename = "app.bsky.actor.defs#contentLabelPref")]
    ContentLabelPref(Box<ContentLabelPref>),
    #[serde(rename = "app.bsky.actor.defs#savedFeedsPref")]
    SavedFeedsPref(Box<SavedFeedsPref>),
    #[serde(rename = "app.bsky.actor.defs#savedFeedsPrefV2")]
    SavedFeedsPrefV2(Box<SavedFeedsPrefV2>),
    #[serde(rename = "app.bsky.actor.defs#personalDetailsPref")]
    PersonalDetailsPref(Box<PersonalDetailsPref>),
    #[serde(rename = "app.bsky.actor.defs#feedViewPref")]
    FeedViewPref(Box<FeedViewPref>),
    #[serde(rename = "app.bsky.actor.defs#threadViewPref")]
    ThreadViewPref(Box<ThreadViewPref>),
    #[serde(rename = "app.bsky.actor.defs#interestsPref")]
    InterestsPref(Box<InterestsPref>),
    #[serde(rename = "app.bsky.actor.defs#mutedWordsPref")]
    MutedWordsPref(Box<MutedWordsPref>),
    #[serde(rename = "app.bsky.actor.defs#hiddenPostsPref")]
    HiddenPostsPref(Box<HiddenPostsPref>),
    #[serde(rename = "app.bsky.actor.defs#bskyAppStatePref")]
    BskyAppStatePref(Box<BskyAppStatePref>),
    #[serde(rename = "app.bsky.actor.defs#labelersPref")]
    LabelersPref(Box<LabelersPref>),
    #[serde(rename = "app.bsky.actor.defs#postInteractionSettingsPref")]
    PostInteractionSettingsPref(Box<PostInteractionSettingsPref>),
}
