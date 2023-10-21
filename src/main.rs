static VIDEO_ID: &str = "https://www.youtube.com/watch?v=H2PYtvIYDHE";

#[tokio::main]
async fn main() {
    let yt = youtube::YoutubeAPI::new();
    let video = yt.video(VIDEO_ID).unwrap().build().fetch().await.unwrap();
    dbg!(video.items);
    // let video = video.items.get(0).unwrap();
    // println!("Result: {:?}", video.snippet.as_ref().unwrap().title);
}
