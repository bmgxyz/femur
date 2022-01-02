use serde::{Deserialize, Serialize};
use std::{fmt::Display, str::FromStr};

pub fn validate_username(_username: &str) -> bool {
    // TODO: implement
    unimplemented!();
}

#[derive(Clone, Deserialize, Serialize)]
pub struct Avatar {
    original: String,
    // TODO: implement other sizes?
}

#[derive(Clone, Deserialize, Serialize)]
pub enum MediaType {
    Unspecified = 0,
    Text = 1,
    Movie = 2,
    TVShow = 3,
    Music = 4,
    Speech = 5,
    Game = 6,
}

// TODO: write custom implementations that set Option<_> correctly?
#[derive(Clone, Deserialize, Serialize)]
pub struct UserData {
    avatar: Option<Avatar>,
    name: Option<String>,
    status: Option<String>,
    emoji: Option<char>, // TODO: this may not be right
    media: Option<String>,
    media_type: Option<MediaType>,
}

impl UserData {
    pub fn update_status_fields(&self, update: UserData) -> UserData {
        let mut new_status = self.clone();
        if update.emoji.is_some() {
            new_status.emoji = update.emoji;
        }
        if update.media.is_some() {
            new_status.media = update.media;
        }
        if update.media_type.is_some() {
            new_status.media_type = update.media_type;
        }
        if update.name.is_some() {
            new_status.name = update.name;
        }
        if update.status.is_some() {
            new_status.status = update.status;
        }
        new_status
    }
}

impl FromStr for UserData {
    type Err = serde_json::Error;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let user_data: Self = serde_json::from_str(s)?;
        Ok(user_data)
    }
}

impl Display for UserData {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match serde_json::to_string(self) {
                Ok(user_data) => user_data,
                Err(e) => e.to_string(),
            }
        )
    }
}
