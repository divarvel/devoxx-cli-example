extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;
#[macro_use]
extern crate structopt;

use std::process::Command;
use std::error::Error;
use structopt::StructOpt;

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Link {
    href: String,
    rel: String,
    title: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Talk {
    id: String,
    title: String,
    talk_type: String,
    track: String,
    links: Vec<Link>,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Speaker {
    uuid: String,
    bio_as_html: String,
    accepted_talks: Vec<Talk>,
}

#[derive(StructOpt, Debug)]
struct Options {
    #[structopt(short = "v", long = "verbose")]
    verbose: bool,
    #[structopt(name = "SPEAKER ID")]
    speaker_id: String,
}

fn get_speaker(uuid: &str, verbose: &bool) -> Result<Speaker, Box<Error>> {
    let url = format!(
        "http://cfp.devoxx.fr/api/conferences/DevoxxFR2018/speakers/{}",
        uuid
    );
    let text = reqwest::get(&url)?.text()?;
    if *verbose {
        println!("GET {}", &url);
    }
    let s: Speaker = serde_json::from_str(&text)?;
    Ok(s)
}

fn say_thing(sentence: &str) {
    let _result = Command::new("say").arg(sentence).status();
    ()
}

fn main() {
    let command = Options::from_args();
    // "76606cd0-6261-44b2-ad0e-3518a0e66995"
    let s = get_speaker(&command.speaker_id, &command.verbose).unwrap();
    for t in s.accepted_talks {
        println!("Talk: {}", &t.title);
        if command.verbose {
            say_thing(&t.title);
        }
    }
}
