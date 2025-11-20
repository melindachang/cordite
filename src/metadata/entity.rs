use std::path::PathBuf;

use time::{Date, Duration};

#[derive(Debug)]
#[allow(unused)]
pub struct Artist {
    id: String,
    musicbrainz_id: Option<String>,
    spotify_id: Option<String>,
    discogs_id: Option<String>,

    name: String,
    name_sort: String,
    kind: Option<ArtistKind>,
    country: Option<String>,
}

#[derive(Debug)]
pub enum ArtistKind {
    Person,
    Group,
    Character,
    Choir,
    Orchestra,
    Other,
    Unknown,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Release {
    id: String,
    musicbrainz_id: Option<String>,
    spotify_id: Option<String>,
    discogs_id: Option<String>,

    title: String,
    title_sort: String,
    year_released: Date,
    kind: ReleaseKind,
    language: Option<String>,

    path: PathBuf,
    date_added: Date,
    last_modified: Date,
}

#[derive(Debug)]
pub enum ReleaseKind {
    Album,
    Single,
    Compilation,
}

#[derive(Debug)]
#[allow(unused)]
pub struct Track {
    id: String,
    musicbrainz_id: Option<String>,
    spotify_id: Option<String>,
    discogs_id: Option<String>,

    title: String,
    title_sort: String,
    duration: Duration,
    track_number: Option<u32>,
    disc_number: Option<u32>,
    language: Option<String>,

    release_id: Option<String>,

    path: PathBuf,
    date_added: Date,
    last_modified: Date,
}

#[derive(Debug)]
#[allow(unused)]
pub struct TrackArtist {
    track_id: String,
    artist_id: String,
}
