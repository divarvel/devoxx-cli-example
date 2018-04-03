extern crate reqwest;

use reqwest::Error;

fn get_text(uuid: &str) -> Result<String, Error> {
    let url = format!(
        "http://cfp.devoxx.fr/api/conferences/DevoxxFR2018/speakers/{}",
        uuid
    );
    let text = reqwest::get(&url)?.text()?;
    Ok(text)
}

fn main() {
    let body = get_text("76606cd0-6261-44b2-ad0e-3518a0e66995").unwrap();
    println!("body = {:?}", body);
}
