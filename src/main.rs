extern crate reqwest;
extern crate serde;
extern crate serde_json;

#[macro_use]
extern crate serde_derive;

use std::error::Error;

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

fn get_speaker(uuid: &str) -> Result<Speaker, Box<Error>> {
    let url = format!(
        "http://cfp.devoxx.fr/api/conferences/DevoxxFR2018/speakers/{}",
        uuid
    );
    let text = reqwest::get(&url)?.text()?;
    let s: Speaker = serde_json::from_str(&text)?;
    Ok(s)
}

fn main() {
    let s = get_speaker("76606cd0-6261-44b2-ad0e-3518a0e66995").unwrap();
    println!("{:?}", s);
}
