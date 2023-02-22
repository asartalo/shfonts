use lightningcss::rules::font_face::{FontFaceRule, Source};
use lightningcss::rules::CssRule::FontFace;
use lightningcss::stylesheet::{ParserOptions, PrinterOptions, StyleSheet};
use std::error::Error;
use std::path::PathBuf;

use clap::Parser;

type MyResult<T> = Result<T, Box<dyn Error>>;

#[derive(Parser)]
#[command(
    author = "Wayne Duran <asartalo@gmail.com>",
    version = "0.1.0",
    about = "Download fonts for self-hosting",
    long_about = None
)]
struct Cli {
    css_path: String,

    #[arg(short = 'd', long = "dir", value_name = "DIRECTORY")]
    dir: Option<PathBuf>,
}

fn get_font_url(rule: &FontFaceRule) -> Option<String> {
    for property in rule.properties {
        match property {
            Source::Url(url_src) => return Some(url_src.url.url.to_string()),
            _ => {
                // do nothing
            }
        }
    }
    None
}

fn run() -> MyResult<()> {
    let cli = Cli::parse();
    let path = cli.css_path;

    let response = minreq::get(path).send()?;
    let stylesheet = StyleSheet::parse(response.as_str()?, ParserOptions::default()).unwrap();

    let mut font_urls: Vec<String> = Vec::new();
    for rule in &stylesheet.rules.0 {
        match rule {
            FontFace(ff_rule) => {
                println!("Font Face: {:?}", ff_rule.properties);
                match get_font_url(ff_rule) {
                    None => {
                        // do nothing
                    }
                    Some(url) => font_urls.push(url),
                }
            }
            _ => {
                // Do nothing.
            }
        }
    }
    let res = stylesheet.to_css(PrinterOptions::default()).unwrap();
    println!("{}", res.code);
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
