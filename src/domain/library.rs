use serde_derive::Deserialize;
use serde_derive::Serialize;
use std::fs;

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Library {
    #[serde(rename = "Audiobooks")]
    pub audiobooks: Vec<Audiobook>,
    #[serde(skip_serializing, skip_deserializing)]
    pub file: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Audiobook {
    #[serde(rename = "Id")]
    pub id: i64,
    #[serde(rename = "Source")]
    pub source: Source,
    #[serde(rename = "Artist")]
    pub artist: String,
    #[serde(rename = "Album")]
    pub album: String,
    #[serde(rename = "Title")]
    pub title: Option<String>,
    #[serde(rename = "Genre")]
    pub genre: Option<String>,
    #[serde(rename = "Duration")]
    pub duration: String,
    #[serde(rename = "HasPicture")]
    pub has_picture: bool,
    #[serde(rename = "State")]
    pub state: String,
    #[serde(rename = "Rating")]
    pub rating: Option<Rating>,
    #[serde(rename = "AlbumArtist")]
    pub album_artist: Option<String>,
    #[serde(rename = "Comment")]
    pub comment: Option<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Source {
    #[serde(rename = "SingleFile")]
    pub single_file: Option<String>,
    #[serde(rename = "MultiFile")]
    #[serde(default)]
    pub multi_file: Vec<String>,
}

#[derive(Default, Debug, Clone, PartialEq, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Rating {
    #[serde(rename = "Rating")]
    pub rating: i64,
}

impl Rating {
    pub fn as_symbol(self: &Rating) -> &str {
        return match self.rating {
            0 => " ",
            1 => "1",
            2 => "2",
            3 => "3",
            4 => "4",
            5 => "5",
            _ => "☠",
        };
    }
}

pub fn parse_library_json(filename: String) -> Result<Library, String> {
    let raw = fs::read_to_string(filename.clone()).map_err(|e| e.to_string())?;
    let mut library: Library = serde_json::from_str(&raw).map_err(|e| e.to_string())?;
    library.file = Some(filename);
    Ok(library)
}

pub fn create_dummy() -> Library {
    let book1 = Audiobook {
        id: 1,
        source: Source {
            single_file: None,
            multi_file: vec![],
        },
        artist: "Artisticus".to_string(),
        album: "Albumicus".to_string(),
        title: Some("Titellicus".to_string()),
        genre: Some("post progressive poem".to_string()),
        duration: "01:00:00".to_string(),
        has_picture: false,
        state: "Finished".to_string(),
        rating: Some(Rating { rating: 2 }),
        album_artist: None,
        comment: None,
    };

    let book2 = Audiobook {
        id: 2,
        source: Source {
            single_file: None,
            multi_file: vec![],
        },
        artist: "Artisticus".to_string(),
        album: "Albumicus 2".to_string(),
        title: Some("Titellicus 2".to_string()),
        genre: Some("progressive punk poem".to_string()),
        duration: "01:00:00".to_string(),
        has_picture: false,
        state: "Finished".to_string(),
        rating: Some(Rating { rating: 4 }),
        album_artist: None,
        comment: None,
    };

    Library {
        audiobooks: vec![book1, book2],
        file: None,
    }
}
