use clap::Parser;
use lightningcss::rules::CssRule::FontFace;
use lightningcss::stylesheet::{ParserOptions, StyleSheet};
use shfonts::{Cli, MyResult};
use std::env;
use std::fs;
use std::io::Write;
use std::path::PathBuf;
use url::Url;

fn get_output_dir(cli: &Cli) -> MyResult<PathBuf> {
    match &cli.dir {
        Some(dir) => Ok(dir.clone()),
        // TODO: Instead of panicking, how to return proper result
        None => Ok(env::current_dir().unwrap()),
    }
}

fn run() -> MyResult<()> {
    let cli = Cli::parse();
    let path = cli.css_path.clone();
    let output_dir = get_output_dir(&cli)?;

    let response = minreq::get(&path).send()?;
    let stylesheet = StyleSheet::parse(response.as_str()?, ParserOptions::default()).unwrap();

    let mut font_urls: Vec<String> = Vec::new();
    for rule in &stylesheet.rules.0 {
        if let FontFace(ff_rule) = rule {
            if let Some(url) = shfonts::get_font_url(ff_rule) {
                font_urls.push(url)
            }
        }
    }

    let css_url = Url::parse(&path)?;
    let base = shfonts::get_base_url(&css_url).unwrap();
    for font_url in &font_urls {
        let full_url = if font_url.starts_with("http://") || font_url.starts_with("https://") {
            Url::parse(font_url)?
        } else if font_url.starts_with("/") && !font_url.starts_with("//") {
            let stripped = match font_url.strip_prefix("/") {
                Some(str) => str,
                None => font_url,
            };
            Url::parse(&(base.as_str().to_owned() + stripped))?
        } else {
            let base_url = Url::parse(&path)?;
            base_url.join(&font_url)?;
            base_url
        };
        let response = minreq::get(full_url.to_string()).send()?;
        let file_name = shfonts::get_file_name(&full_url);

        let file_path = output_dir.join(file_name);
        let mut file = fs::OpenOptions::new()
            .create(true)
            .write(true)
            .open(file_path)
            .unwrap();
        file.write_all(response.as_bytes())?;
    }
    Ok(())
}

fn main() {
    if let Err(e) = run() {
        eprintln!("{}", e);
        std::process::exit(1);
    }
}
