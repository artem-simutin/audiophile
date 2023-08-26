use crate::song::Song;

pub struct ServerContext {
    pub songs: Vec<Song>,
}

impl ServerContext {
    pub fn new() -> Self {
        Self { songs: vec![] }
    }

    pub fn add_song(self: &mut Self, song: Song) {
        self.songs.push(song)
    }
}
