use youtube::{prelude::AudiophileError, url::Url};

#[derive(Debug, Clone)]
pub struct Song {
    pub url: youtube::url::Url,
    pub title: String,
    pub thumbnail_url: String,
}

impl TryFrom<&youtube::interfaces::video::VideoResourse> for Song {
    type Error = AudiophileError;

    fn try_from(value: &youtube::interfaces::video::VideoResourse) -> Result<Self, Self::Error> {
        let url = format!("https://youtube.com/watch?v={}", value.id);
        let url = match Url::parse(&url) {
            Ok(url) => url,
            Err(_why) => {
                return Err(AudiophileError {
                    location: "Song::from<VideoResourse>",
                    message: "Cannot parse YouTube URL while constructing the song",
                    // cause: Some(Box::new(why)),
                });
            }
        };
        let song_snippet = match &value.snippet {
            Some(s) => s,
            None => {
                return Err(AudiophileError {
                    location: "Song::from<VideoResourse>",
                    message: "There is not requred information in the video resourse to construct Song object",
                    // cause: None 
                });
            }
        };
        let thumbnail = song_snippet
            .thumbnails
            .get(&youtube::interfaces::video::VideoResourseThumbnailType::Default);
        let thumbnail_url = match thumbnail {
            Some(t) => t.url.clone(),
            None => "default thumbnail".to_string(),
        };
        Ok(Self {
            url,
            title: song_snippet.title.clone(),
            thumbnail_url,
        })
    }
}
