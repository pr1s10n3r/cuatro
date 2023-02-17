use clap::Parser;
use log::{error, info};
use regex::Regex;

struct Post {
    tim: u64,
    ext: Option<String>,
}

#[derive(Parser, Debug)]
#[command(author, version, long_about = None)]
struct CommandArgs {
    thread_url: String,
    #[arg(short, long)]
    verbose: bool,
    #[arg(short, long, default_value = "None")]
    exclude: String,
    #[arg(short, long, default_value = "1")]
    threads: u8,
    #[arg(short, long, default_value = ".")]
    output: String,
}

#[tokio::main]
async fn main() {
    let args = CommandArgs::parse();

    let mut log_builder = env_logger::Builder::from_default_env();

    if args.verbose {
        log_builder.filter_level(log::LevelFilter::Debug);
    } else {
        log_builder.filter_level(log::LevelFilter::Info);
    }

    log_builder.init();

    let re = Regex::new(r"https://boards\.4chan\.org/(?P<board>\S+)/thread/(?P<id>\d+)").unwrap();

    let caps = re.captures(&args.thread_url).unwrap_or_else(|| {
        error!("Invalid 4chan thread URL. Format: https://boards.4chan.org/<BOARD>/thread/<ID>");
        std::process::exit(1);
    });

    let board = &caps["board"];
    let thread_id = &caps["id"];

    info!("board: {}", board);
    info!("id: {}", thread_id);

    let threads = args.threads;
    println!("threads: {threads}");

    let mut excludes: Vec<&str> = Vec::new();
    if args.exclude != "None" {
        excludes = args.exclude.trim().split(",").collect();
    }

    for e in excludes {
        info!("excluding {e}");
    }
}

fn get_thread_posts(board: &str, id: u64) -> Result<Vec<Post>, dyn std::error::Error> {
    return Ok(vec![]);
}
