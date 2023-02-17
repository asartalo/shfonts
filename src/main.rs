use clap::Parser;

#[derive(Parser)]
#[command(author = "Wayne Duran <asartalo@gmail.com>", version = "0.1.0", about = "Download fonts for self-hosting", long_about = None)]
struct Cli {
    css_path: String,
}

fn main() {
    let cli = Cli::parse();
    let path = cli.css_path;
    println!("{}", path);
    // let matches = Command::new("shfonts")
    //     .version("0.1.0")
    //     .author("Wayne Duran <asartalo@gmail.com>")
    //     .about("Download fonts for self-hosting")
    //     .arg(
    //         Arg::new("cssPath")
    //             .value_name("CSS_PATH")
    //             .help("Path or URI to css file to source fonts")
    //             .required(true)
    //             .num_args(1)
    //             .action(ArgAction::Set)
    //     )
}
