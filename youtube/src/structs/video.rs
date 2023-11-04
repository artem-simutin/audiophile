use crate::{interfaces::video::VideoListResponse, prelude::AudiophileError};

use super::api::YoutubeAPI;

#[derive(Clone)]
pub enum YoutubeVideoPart {
    ContentDetails,
    // FileDetails,
    Id,
    // LiveStreamingDetails,
    // Localizations,
    // Player,
    // ProcessingDetails,
    // RecordingDetails,
    Snippet,
    Statistics,
    Status,
    // Suggestions,
    // TopicDetails,
}

impl From<&YoutubeVideoPart> for String {
    fn from(value: &YoutubeVideoPart) -> Self {
        match value {
            YoutubeVideoPart::ContentDetails => String::from("contentDetails"),
            // YoutubeVideoPart::FileDetails => String::from("fileDetails"),
            YoutubeVideoPart::Id => String::from("id"),
            // YoutubeVideoPart::LiveStreamingDetails => String::from("liveStreamingDetails"),
            // YoutubeVideoPart::Localizations => String::from("localizations"),
            // YoutubeVideoPart::Player => String::from("player"),
            // YoutubeVideoPart::ProcessingDetails => String::from("processingDetails"),
            // YoutubeVideoPart::RecordingDetails => String::from("recordingDetails"),
            YoutubeVideoPart::Snippet => String::from("snippet"),
            YoutubeVideoPart::Statistics => String::from("statistics"),
            YoutubeVideoPart::Status => String::from("status"),
            // YoutubeVideoPart::Suggestions => String::from("suggestions"),
            // YoutubeVideoPart::TopicDetails => String::from("topicDetails"),
        }
    }
}

#[derive(Clone)]
pub enum YoutubeVideoChart {
    MostPopular,
}

#[derive(Clone)]
pub enum YoutubeMyRating {
    // Like,
    // Dislike,
    None,
}

#[derive(Clone)]
pub struct YoutubeVideoBuilder {
    api: YoutubeAPI,
    parts: Vec<YoutubeVideoPart>,
    // chart: YoutubeVideoChart,
    id: String,
    // my_rating: YoutubeMyRating,
}

#[derive(Clone)]
pub struct YoutubeVideo {
    api: YoutubeAPI,
    parts: Vec<YoutubeVideoPart>,
    // chart: YoutubeVideoChart,
    id: String,
    // my_rating: YoutubeMyRating,
}

impl YoutubeVideo {
    pub async fn fetch(self: &Self) -> Result<VideoListResponse, AudiophileError> {
        let parts = self
            .parts
            .iter()
            .map(|i| String::from(i))
            .collect::<Vec<String>>()
            .join(",");

        let url = format!(
            "{}/videos?part={}&id={}&key={}",
            self.api.base_url, parts, self.id, self.api.authentication.token
        );

        let req_client = reqwest::Client::new();
        let response = req_client
            .get(url)
            .send()
            .await
            .map_err(|e| AudiophileError {
                location: "youtube video fetch",
                message: "Something went wrong getting the video",
                // cause: Some(Box::new(e)),
            })?;
        // Test
        let json = response
            .json::<VideoListResponse>()
            .await
            .map_err(|e| AudiophileError {
                location: "youtube video fetch",
                message: "Cannot parse json from the response",
                // cause: Some(Box::new(e)),
            })?;
        Ok(json)
    }
}

impl YoutubeVideoBuilder {
    pub fn new(api: YoutubeAPI, id: &str) -> Self {
        Self {
            api,
            parts: vec![
                YoutubeVideoPart::ContentDetails,
                YoutubeVideoPart::Snippet,
                YoutubeVideoPart::Statistics,
                YoutubeVideoPart::Status,
                YoutubeVideoPart::Id,
            ],
            // chart: YoutubeVideoChart::MostPopular,
            id: String::from(id),
            // my_rating: YoutubeMyRating::None,
        }
    }

    pub fn build(self: &Self) -> YoutubeVideo {
        YoutubeVideo {
            api: self.api.clone(),
            parts: self.parts.clone(),
            // chart: self.chart.clone(),
            id: self.id.clone(),
            // my_rating: self.my_rating.clone(),
        }
    }
}
