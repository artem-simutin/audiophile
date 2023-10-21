use serde_aux::prelude::*;
use std::{collections::HashMap, fmt::Display};

use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoListResponsePageInfo {
    total_results: i32,
    results_per_page: i32,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoListResponse {
    pub kind: String,
    pub etag: String,
    pub next_page_token: Option<String>,
    pub prev_page_token: Option<String>,
    pub page_info: VideoListResponsePageInfo,
    pub items: Vec<VideoResourse>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoResourse {
    pub kind: String,
    pub etag: String,
    pub id: String,
    pub snippet: Option<VideoResourseSnippet>,
    pub content_details: Option<VideoResourseContentDetails>,
    pub status: Option<VideoResourseStatus>,
    pub statistics: Option<VideoResourseStatistics>,
    // TODO: implement remaining fields
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoResourseSnippet {
    pub published_at: DateTime<Utc>,
    pub channel_id: String,
    pub title: String,
    pub description: String,
    pub thumbnails: HashMap<VideoResourseThumbnailType, VideoResourseThumbnail>,
    pub channel_title: String,
    pub tags: Vec<String>,
    pub category_id: String,
    pub live_broadcast_content: VideoResourseLiveBroadcaseContent,
    pub default_language: Option<String>,
    // TODO: Implement localized parsing
    // pub localized:
    pub default_audio_language: Option<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoResourseContentDetails {
    pub duration: String,
    pub dimensions: String,
    pub definition: VideoResourseDifinition,
    pub caption: String,
    pub licensed_content: bool,
    pub region_restriction: VideoResourseRestriction,
    pub content_rating: VideoResourseContentRating,
    pub projection: VideoResourseProjection,
    pub has_custom_thumbnail: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoResourseStatistics {
    #[serde(deserialize_with = "deserialize_number_from_string")]
    view_count: u64,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    like_count: u64,
    dislike_count: Option<u64>,
    #[serde(deserialize_with = "deserialize_number_from_string")]
    comment_count: u64,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoResourseStatus {
    upload_status: VideoResourseUploadStatus,
    failure_status: Option<VideoResourseFailureReason>,
    rejection_reason: Option<VideoResourseRejectionReason>,
    privacy_status: VideoResoursePrivacyStatus,
    publish_at: Option<DateTime<Utc>>,
    license: VideoResourseLicense,
    embeddable: bool,
    public_stats_viewable: bool,
    made_for_kids: bool,
    #[serde(default)]
    self_declared_made_for_kids: bool,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum VideoResourseLicense {
    CreativeCommon,
    Youtube,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoResourseUploadStatus {
    Deleted,
    Failed,
    Processed,
    Rejected,
    Uploaded,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum VideoResourseRejectionReason {
    Claim,
    Copyright,
    Duplicate,
    Inappropriate,
    Legal,
    Length,
    TermsOfUse,
    Trademark,
    UploaderAccountClosed,
    UploaderAccountSuspended,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoResoursePrivacyStatus {
    Private,
    Public,
    Unlisted,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub enum VideoResourseFailureReason {
    Codec,
    Conversion,
    EmptyFile,
    InvalidFile,
    TooSmall,
    UploadAborted,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoResourseProjection {
    #[serde(rename = "360")]
    View360,
    Rectangular,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct VideoResourseContentRating {
    pub acb_rating: String,
    pub agcom_rating: String,
    pub anatel_rating: String,
    pub bbfc_rating: String,
    pub bfvc_rating: String,
    pub bmukk_rating: String,
    pub catv_rating: String,
    pub catvfr_rating: String,
    pub cbfc_rating: String,
    pub ccc_rating: String,
    pub cce_rating: String,
    pub chfilm_rating: String,
    pub chvrs_rating: String,
    pub cicf_rating: String,
    pub cna_rating: String,
    pub cnc_rating: String,
    pub csa_rating: String,
    pub cscf_rating: String,
    pub czfilm_rating: String,
    pub djctq_rating: String,
    pub djctq_rating_reasons: Option<String>,
    pub ecbmct_rating: String,
    pub eefilm_rating: String,
    pub egfilm_rating: String,
    pub eirin_rating: String,
    pub fcbm_rating: String,
    pub fco_rating: String,
    pub fpb_rating: String,
    pub fpb_rating_reasons: Vec<String>,
    pub fsk_rating: String,
    pub grfilm_rating: String,
    pub icaa_rating: String,
    pub ifco_rating: String,
    pub ilfilm_rating: String,
    pub incaa_rating: String,
    pub kfcb_rating: String,
    pub kijkwijzer_rating: String,
    pub kmrb_rating: String,
    pub lsf_rating: String,
    pub mccaa_rating: String,
    pub mccyp_rating: String,
    pub mcst_rating: String,
    pub mda_rating: String,
    pub medietilsynet_rating: String,
    pub meku_rating: String,
    pub mibac_rating: String,
    pub moc_rating: String,
    pub moctw_rating: String,
    pub mpaa_rating: String,
    pub mpaat_rating: String,
    pub mtrcb_rating: String,
    pub nbc_rating: String,
    pub nfrc_rating: String,
    pub nfvcb_rating: String,
    pub nkclv_rating: String,
    pub oflc_rating: String,
    pub pefilm_rating: String,
    pub resorteviolencia_rating: String,
    pub rtc_rating: String,
    pub rte_rating: String,
    pub russia_rating: String,
    pub skfilm_rating: String,
    pub smais_rating: String,
    pub smsa_rating: String,
    pub tvpg_rating: String,
    pub yt_rating: String,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoResourseRestriction {
    pub allowed: Vec<String>,
    pub blocked: Vec<String>,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoResourseDifinition {
    HD,
    SD,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(rename_all = "lowercase")]
pub enum VideoResourseLiveBroadcaseContent {
    Live,
    None,
    Upcoming,
}

impl Display for VideoResourseLiveBroadcaseContent {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().to_lowercase())
    }
}

#[derive(Debug, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(rename_all = "lowercase")]
pub enum VideoResourseThumbnailType {
    Default,
    Medium,
    High,
    Standard,
    MaxRes,
}

impl Display for VideoResourseThumbnailType {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.to_string().to_lowercase())
    }
}

#[derive(Serialize, Deserialize, Debug)]
pub struct VideoResourseThumbnail {
    pub url: String,
    pub width: i32,
    pub height: i32,
}
