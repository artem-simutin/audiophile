use std::time::Duration;

use rusty_ytdl::{Video, VideoError};

#[derive(Debug, Clone)]
pub struct Song {
    pub id: String,
    pub url: String,
    pub title: String,
    pub length: Duration,
    pub likes_count: i32,
    pub thumbnail_url: Option<String>,
    pub views: String,
    pub is_live: bool,
    pub author_name: Option<String>,
    pub upload_date: String,
}

impl Song {
    pub async fn new_by_yt_url(url: &String) -> Result<Self, VideoError> {
        let video = match Video::new(url) {
            Ok(v) => v,
            Err(why) => {
                return Err(why);
            }
        };
        let info = match video.get_info().await {
            Ok(i) => i,
            Err(why) => {
                return Err(why);
            }
        };
        let length_seconds: u64 = match info.video_details.length_seconds.parse() {
            Ok(sec) => sec,
            Err(_) => 0,
        };
        return Ok(Self {
            id: info.video_details.video_id,
            author_name: match info.video_details.author {
                Some(auth) => Some(auth.name),
                None => None,
            },
            title: info.video_details.title,
            likes_count: info.video_details.likes,
            url: info.video_details.video_url,
            views: info.video_details.view_count,
            is_live: info.video_details.is_live_content,
            upload_date: info.video_details.upload_date,
            length: Duration::from_secs(length_seconds),
            thumbnail_url: match info.video_details.thumbnails.get(0) {
                Some(t) => Some(t.url.clone()),
                None => None,
            },
        });
    }
}
