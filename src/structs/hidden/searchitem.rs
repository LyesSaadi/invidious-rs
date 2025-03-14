use crate::structs::hidden::{AuthorThumbnail, VideoThumbnail};
use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchItemTransition {
    pub r#type: String,
    pub title: Option<String>,
    #[serde(rename = "videoId")]
    pub video_id: Option<String>,
    pub author: String,
    #[serde(rename = "authorId")]
    pub author_id: String,
    #[serde(rename = "authorUrl")]
    pub author_url: String,
    #[serde(rename = "videoThumbnails")]
    pub video_thumbnails: Option<Vec<VideoThumbnail>>,
    pub description: Option<String>,
    #[serde(rename = "descriptionHtml")]
    pub description_html: Option<String>,
    #[serde(rename = "viewCount")]
    pub view_count: Option<u64>,
    pub published: Option<u64>,
    #[serde(rename = "publishedText")]
    pub published_text: Option<String>,
    #[serde(rename = "lengthSeconds")]
    pub length_seconds: Option<u64>,
    #[serde(rename = "liveNow")]
    pub live_now: Option<bool>,
    pub paid: Option<bool>,
    pub premium: Option<bool>,

    #[serde(rename = "playlistId")]
    pub playlist_id: Option<String>,
    #[serde(rename = "playlistThumbnail")]
    pub playlist_thumbnail: Option<String>,
    #[serde(rename = "videoCount")]
    pub video_count: Option<u32>,
    pub videos: Option<Vec<SearchPlaylistVideo>>,
    #[serde(rename = "authorVerified")]
    pub verified: Option<bool>,
    #[serde(rename = "authorThumbnails")]
    pub author_thumbnails: Option<Vec<AuthorThumbnail>>,
    #[serde(rename = "subCount")]
    pub sub_count: Option<u32>,
}

impl SearchItemTransition {
    pub fn proccess(self) -> SearchItem {
        match self.r#type.as_str() {
            "video" => SearchItem::Video {
                title: self.title.unwrap_or(String::new()),
                id: self.video_id.unwrap_or(String::new()),
                length: self.length_seconds.unwrap_or(0),
                thumbnails: self.video_thumbnails.unwrap_or(Vec::new()),
                description: self.description.unwrap_or(String::new()),
                description_html: self.description_html.unwrap_or(String::new()),
                views: self.view_count.unwrap_or(0),
                published: self.published.unwrap_or(0),
                published_text: self.published_text.unwrap_or(String::new()),
                live: self.live_now.unwrap_or(false),
                paid: self.paid.unwrap_or(false),
                premium: self.premium.unwrap_or(false),
                author: self.author,
                author_id: self.author_id,
                author_url: self.author_url,
            },
            "playlist" => SearchItem::Playlist {
                title: self.title.unwrap_or(String::new()),
                id: self.playlist_id.unwrap_or(String::new()),
                thumbnail: self.playlist_thumbnail.unwrap(),
                author: self.author,
                author_id: self.author_id,
                author_url: self.author_url,
                author_verified: self.verified.unwrap(),
                video_count: self.video_count.unwrap_or(0),
                videos: self.videos.unwrap_or(Vec::new()),
            },
            "channel" => SearchItem::Channel {
                name: self.author,
                id: self.author_id,
                url: self.author_url,
                verified: self.verified.unwrap(),
                thumbnails: self.author_thumbnails.unwrap(),
                subscribers: self.sub_count.unwrap(),
                video_count: self.video_count.unwrap(),
                description: self.description.unwrap(),
                description_html: self.description_html.unwrap(),
            },
            _ => SearchItem::Unknown(self),
        }
    }
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub enum SearchItem {
    Video {
        title: String,
        id: String,
        author: String,
        author_id: String,
        author_url: String,
        length: u64,
        thumbnails: Vec<VideoThumbnail>,
        description: String,
        description_html: String,
        views: u64,
        published: u64,
        published_text: String,
        live: bool,
        paid: bool,
        premium: bool,
    },

    Playlist {
        title: String,
        id: String,
        author: String,
        author_id: String,
        author_url: String,
        author_verified: bool,
        video_count: u32,
        videos: Vec<SearchPlaylistVideo>,
        thumbnail: String,
    },

    Channel {
        name: String,
        id: String,
        url: String,
        verified: bool,
        thumbnails: Vec<AuthorThumbnail>,
        subscribers: u32,
        video_count: u32,
        description: String,
        description_html: String,
    },

    Unknown(SearchItemTransition),
}

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct SearchPlaylistVideo {
    pub title: String,
    #[serde(rename(serialize = "videoId", deserialize = "videoId"))]
    pub id: String,
    #[serde(rename(serialize = "lengthSeconds", deserialize = "lengthSeconds"))]
    pub length: u32,
    #[serde(rename(serialize = "videoThumbnails", deserialize = "videoThumbnails"))]
    pub thumbnails: Vec<VideoThumbnail>,
}
