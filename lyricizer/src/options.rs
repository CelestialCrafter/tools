use std::{
    env,
    fs,
    path::{Path, PathBuf},
    sync::LazyLock,
};

use serde::Deserialize;

#[derive(Deserialize)]
pub struct Options {
    #[serde(default = "default_address")]
    pub address: String,
    #[serde(default = "default_lrclib_instance")]
    pub lrclib_instance: String,
    #[serde(default = "default_request_delay")]
    pub request_delay: f32,
    pub music_dir: PathBuf
}

fn default_address() -> String {
    "localhost:6600".to_string() 
}

fn default_lrclib_instance() -> String {
    "https://lrclib.net".to_string() 
}

fn default_request_delay() -> f32 {
    1.0
}

pub static OPTIONS: LazyLock<Options> = LazyLock::new(|| {
    let path =
        Path::new(&env::var("HOME").expect("$HOME should be set")).join(".config/lyricizer.toml");

    let opts_str =
        fs::read_to_string(path).expect("should be able to read $HOME/.config/lyricizer.toml");
    toml::from_str(&opts_str).expect("should be able to parse options")
});
