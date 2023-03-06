use lightningcss::rules::font_face::FontFaceProperty::Source;
use lightningcss::rules::font_face::FontFaceRule;
use lightningcss::rules::font_face::Source as SourceEnum;
use lightningcss::rules::CssRule::FontFace;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use rand::distributions::{Alphanumeric, DistString};
use std::collections::HashMap;
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

    #[arg(
        short = 'p',
        long = "font-url-prefix",
        value_name = "FONT_URL_PREFIX",
        default_value = ""
    )]
    pub font_url_prefix: String,
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

#[derive(Debug, PartialEq, Clone)]
pub struct Location {
    pub line: u32,
    pub column: u32,
}

#[derive(Debug, PartialEq, Clone)]
pub struct UrlData {
    pub url: String,
    pub location: Location,
}

pub struct UrlByLine {
    url_data: HashMap<u32, Vec<UrlData>>,
    length: usize,
}

impl UrlByLine {
    pub fn new(data: &Vec<UrlData>) -> UrlByLine {
        let mut map: HashMap<u32, Vec<UrlData>> = HashMap::new();
        let mut length = 0;
        for item in data {
            map.insert(item.location.line, Vec::new());
        }
        for item in data {
            length += 1;
            if let Some(items) = map.get_mut(&item.location.line) {
                items.push(item.clone());
            }
        }

        UrlByLine {
            url_data: map,
            length,
        }
    }

    pub fn len(&self) -> usize {
        self.length
    }

    pub fn is_empty(&self) -> bool {
        self.length == 0
    }

    pub fn at(&self, line: u32) -> Option<&Vec<UrlData>> {
        self.url_data.get(&line)
    }
}

#[must_use]
pub fn get_font_url(rule: &FontFaceRule) -> Option<UrlData> {
    for property in &rule.properties {
        if let Source(sources) = property {
            for source in sources {
                if let SourceEnum::Url(url_src) = source {
                    let loc = url_src.url.loc;
                    return Some(UrlData {
                        url: url_src.url.url.to_string(),
                        location: Location {
                            line: loc.line,
                            column: loc.column,
                        },
                    });
                }
            }
        }
    }
    None
}

pub fn get_url_data(css_str: &str) -> MyResult<Vec<UrlData>> {
    let mut font_urls: Vec<UrlData> = Vec::new();
    let stylesheet = StyleSheet::parse(css_str, ParserOptions::default()).unwrap();

    for rule in &stylesheet.rules.0 {
        if let FontFace(ff_rule) = rule {
            if let Some(url) = get_font_url(ff_rule) {
                font_urls.push(url);
            }
        }
    }
    Ok(font_urls)
}

#[must_use]
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

pub struct LinesWithEndings<'a> {
    input: &'a str,
}

impl<'a> LinesWithEndings<'a> {
    pub fn from(input: &'a str) -> LinesWithEndings<'a> {
        LinesWithEndings { input }
    }
}

impl<'a> Iterator for LinesWithEndings<'a> {
    type Item = &'a str;

    #[inline]
    fn next(&mut self) -> Option<&'a str> {
        if self.input.is_empty() {
            return None;
        }
        let split = self
            .input
            .find('\n')
            .map(|i| i + 1)
            .unwrap_or(self.input.len());
        let (line, rest) = self.input.split_at(split);
        self.input = rest;
        Some(line)
    }
}
