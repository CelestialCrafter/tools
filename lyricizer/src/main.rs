mod options;

use std::{fmt, fs, path::Path, time::Duration};

use eyre::{Result, eyre};
use log::{info, warn, LevelFilter};
use mpd_client::{
    Client,
    commands::{Find, TagTypes},
    filter::{Filter, Operator},
    responses::Song,
    tag::Tag,
};
use options::OPTIONS;
use serde::Deserialize;
use tokio::{net::TcpStream, time::sleep};
use url::form_urlencoded;

#[derive(Deserialize)]
#[serde(rename_all = "camelCase")]
struct LrclibLyrics {
    plain_lyrics: Option<String>,
    synced_lyrics: Option<String>,
}

enum Lyrics {
    Synced(String),
    Plain(String),
}

impl Into<Option<Lyrics>> for LrclibLyrics {
    fn into(self) -> Option<Lyrics> {
        if let Some(synced) = self.synced_lyrics {
            return Some(Lyrics::Synced(synced));
        }

        if let Some(plain) = self.plain_lyrics {
            return Some(Lyrics::Plain(plain));
        }

        None
    }
}

#[derive(Clone, Copy)]
struct SimpleSong<'a> {
    path: &'a Path,
    artist: &'a str,
    album: &'a str,
    title: &'a str,
}

impl fmt::Display for SimpleSong<'_> {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{} - {}", self.artist, self.title)
    }
}

fn simplify<'a>(song: &'a Song) -> Option<SimpleSong<'a>> {
    Some(SimpleSong {
        path: song.file_path(),
        artist: song.artists().get(0)?,
        album: song.album()?,
        title: song.title()?,
    })
}

async fn fetch_lyrics<'a>(song: SimpleSong<'a>) -> Result<LrclibLyrics> {
    let params = form_urlencoded::Serializer::new(String::new())
        .append_pair("artist_name", song.artist)
        .append_pair("album_name", song.album)
        .append_pair("track_name", song.title)
        .finish();

    let response = reqwest::get(format!(
        "{}/api/get?{}",
        OPTIONS.lrclib_instance.clone(),
        params
    ))
    .await
    .map_err(|err| eyre!("could not fetch lyrics: {err}"))?;

    let code = response.status();
    if !code.is_success() {
        return Err(eyre!("unsuccessful response code: {code}"));
    }

    response
        .json()
        .await
        .map_err(|err| eyre!("could not decode lrclib response: {err}"))
}

fn format_lyrics<'a>(song: SimpleSong<'a>, lyrics: Lyrics) -> String {
    let body: Vec<_> = match lyrics {
        Lyrics::Synced(synced) => synced.lines().map(|l| l.to_string()).collect(),
        Lyrics::Plain(plain) => plain
            .lines()
            .map(|line| "[00:00.00] ".to_string() + line)
            .collect(),
    };

    let mut output = format!(
        "[ti:{}]\n[ar:{}]\n[al:{}]\n{}",
        song.title,
        song.artist,
        song.album,
        body.join("\n")
    );

    if !output.ends_with('\n') {
        output += "\n";
    }

    output
}

#[tokio::main]
async fn main() -> Result<()> {
    pretty_env_logger::formatted_builder().filter_level(LevelFilter::Info).init();

    let stream = TcpStream::connect(OPTIONS.address.clone()).await?;
    let (mpd, _) = Client::connect(stream).await?;

    let artist_filter = Filter::new(Tag::Artist, Operator::NotEqual, "");
    let album_filter = Filter::new(Tag::Album, Operator::NotEqual, "");
    let title_filter = Filter::new(Tag::Title, Operator::NotEqual, "");

    let (_, _, songs) = mpd
        .command_list((
            TagTypes::disable_all(),
            TagTypes::enable(&[Tag::Title, Tag::Album, Tag::Artist]),
            Find::new(artist_filter.and(album_filter).and(title_filter)),
        ))
        .await?;

    for song in songs.iter().filter_map(|song| {
        let s = simplify(song);
        if let None = s {
            info!("skipping song due to incomplete metadata");
        }
        s
    }) {
        let path = OPTIONS.music_dir.join(song.path).with_extension("lrc");
        match fs::exists(&path) {
            Ok(exists) if exists => {
                info!("lyrics for \"{song}\" already exist");
                continue;
            }
            Err(err) => {
                warn!("could not check if lyrics for \"{song}\" exist: {err}");
                continue;
            }
            _ => (),
        }

        sleep(Duration::from_secs_f32(OPTIONS.request_delay)).await;
        info!("downloading lyrics for \"{song}\"");

        let lrclib_lyrics = fetch_lyrics(song).await.unwrap_or_else(|err| {
            warn!("{err}, writing empty lyrics");
            LrclibLyrics {
                plain_lyrics: None,
                synced_lyrics: Some("".to_string()),
            }
        });

        let lyrics: Option<Lyrics> = lrclib_lyrics.into();
        let lyrics = lyrics.unwrap_or_else(|| {
            info!("no lyrics found, writing empty lyrics");
            Lyrics::Synced("".to_string())
        });

        if let Err(err) = fs::write(path, format_lyrics(song, lyrics)) {
            warn!("{err}");
        }
    }

    Ok(())
}
