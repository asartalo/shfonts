use lightningcss::rules::font_face::FontFaceProperty::Source;
use lightningcss::rules::font_face::FontFaceRule;
use lightningcss::rules::font_face::Source as SourceEnum;
use rand::distributions::{Alphanumeric, DistString};
use std::error::Error;
use std::path::PathBuf;
use url::Url;

pub type MyResult<T> = Result<T, Box<dyn Error>>;

use clap::Parser;

#[derive(Parser)]
#[command(
    author = "Wayne Duran <asartalo@gmail.com>",
    version = "0.1.0",
    about = "Download fonts for self-hosting",
    long_about = None
)]
pub struct Cli {
    pub css_path: String,

    #[arg(short = 'd', long = "dir", value_name = "DIRECTORY")]
    pub dir: Option<PathBuf>,
}

pub fn get_base_url(full_url: &Url) -> MyResult<Url> {
    let mut url = full_url.clone();
    if let Ok(mut path) = url.path_segments_mut() {
        path.clear();
    }

    url.set_fragment(None);
    url.set_query(None);

    Ok(url)
}

pub fn get_font_url(rule: &FontFaceRule) -> Option<String> {
    for property in &rule.properties {
        if let Source(sources) = property {
            for source in sources {
                if let SourceEnum::Url(url_src) = source {
                    return Some(url_src.url.url.to_string());
                }
            }
        }
    }
    None
}

pub fn get_file_name(url: &Url) -> String {
    if let Some(mut segments) = url.path_segments() {
        if let Some(last) = segments.next_back() {
            if !last.is_empty() {
                return last.to_string();
            }
        }
    }
    Alphanumeric.sample_string(&mut rand::thread_rng(), 10)
}
