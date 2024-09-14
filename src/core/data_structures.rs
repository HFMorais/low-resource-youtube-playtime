use std::fmt;

#[derive(Clone, Debug)]
pub struct Channel {
    pub id: i64,
    pub name: String,
    pub channel_id: Option<String>,
    pub url: String,
}

impl fmt::Display for Channel {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.name)
    }
}

#[derive(Debug)]
pub struct VideoEntry {
    pub id: i64,
    pub video_id: String,
    pub title: String,
    pub channel_id: i64,
    pub added_date: i64,
    pub seen: bool,
}