use clap::Parser;
use reqwest::Error;
use std::process::Command;
use std::{env, fs, path};

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    /// The advent of code day you want to solve
    #[clap(short, long, default_value = "1")]
    day: i32,
}

#[tokio::main]
async fn main() {
    let aoc_session_id = env::var("AOC_SESSION_ID")
        .expect("You need to have the AOC_SESSION_ID as an environment variable");

    let main_rs_content = include_str!("./template_main");

    let args = Cli::parse();

    create_cargo_project(&args.day);

    let res = get_input(&aoc_session_id, &args.day).await;

    if res.is_err() {
        panic!("Unable to get input from Advent of Code");
    }

    write_input_to_file(&args.day, &res.ok().unwrap());

    update_main_rs(&args.day,&main_rs_content);
}

fn create_cargo_project(day: &i32) {
    Command::new("sh")
        .arg("-c")
        .arg(format!("cargo init q{day}", day = day))
        .output()
        .expect("error executing cargo init");
}

async fn get_input(aoc_session_id: &str, day: &i32) -> Result<String, Error> {
    let request_url = format!(
        "{request_url}{day}/input",
        request_url = "https://adventofcode.com/2022/day/",
        day = day
    );

    let client = reqwest::Client::new();

    let session_cookie = format!("session={aoc_session_id}", aoc_session_id = aoc_session_id);

    let response = client
        .get(&request_url)
        .header(reqwest::header::CONTENT_TYPE, "application/json")
        .header(reqwest::header::COOKIE, session_cookie)
        .send()
        .await?;

    let res: String = response.text().await?.to_string();

    return Ok(res);
}

fn write_input_to_file(day: &i32, input: &str) {
    let path = format!("./q{day}/input.txt", day = day);
    let exists = path::Path::new(&path).exists();
    if exists {
        fs::remove_file(&path).expect("Unable to write file");
    }
    fs::write(&path, input).expect("Unable to write file");
}

fn update_main_rs(day: &i32, content:&str) {
    let main_rs = format!("./q{day}/src/main.rs", day = day);
    let exists = path::Path::new(&main_rs).exists();
    if exists {
        fs::remove_file(&main_rs).expect("Unable to write file");
    }
    fs::write(&main_rs, content).expect("Unable to write file");
}
