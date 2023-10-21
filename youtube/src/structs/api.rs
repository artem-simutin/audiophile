use std::collections::HashMap;

use url::Url;

use crate::{prelude::AudiophileError, utils::regex};

use super::video::YoutubeVideoBuilder;

static YOUTUBE_API_URL: &str = "https://youtube.googleapis.com/youtube/v3";
static YOUTUBE_DEFAULT_API_KEY: &str = "AIzaSyBrtWA4cBD4NvADC8O5-_amrgqPt5eEnNA";
static YOUTUBE_ID_REGEX_PATTERN: &str = r#"([^\"&?/ ]{11})"#;

#[derive(Clone)]
pub struct Authentication {
    pub token: String,
}

impl Default for Authentication {
    fn default() -> Self {
        Self {
            token: YOUTUBE_DEFAULT_API_KEY.into(),
        }
    }
}

#[derive(Clone)]
pub struct YoutubeAPI {
    pub base_url: &'static str,
    pub authentication: Authentication,
}

impl YoutubeAPI {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn video(self: &Self, id_or_url: &str) -> Result<YoutubeVideoBuilder, AudiophileError> {
        let id = {
            let is_plain_id = regex::fullmatch(id_or_url, YOUTUBE_ID_REGEX_PATTERN)?;

            if is_plain_id {
                String::from(id_or_url)
            } else {
                match Url::parse(id_or_url) {
                    Ok(url) => {
                        let params = url
                            .query_pairs()
                            .into_owned()
                            .collect::<HashMap<String, String>>();
                        let video_id = params.get("v").ok_or(AudiophileError {
                            location: "youtube api video url parse",
                            message: "The url does not have a valid youtube video id",
                            cause: None,
                        })?;
                        video_id.clone()
                    }
                    Err(why) => {
                        return Err(AudiophileError {
                            location: "youtube api video url parser",
                            message: "Cannot parse the url",
                            cause: Some(Box::new(why)),
                        })
                    }
                }
            }
        };
        Ok(YoutubeVideoBuilder::new(self.clone(), id.as_str()))
    }
}

impl Default for YoutubeAPI {
    fn default() -> Self {
        Self {
            base_url: YOUTUBE_API_URL,
            authentication: Authentication::default(),
        }
    }
}
