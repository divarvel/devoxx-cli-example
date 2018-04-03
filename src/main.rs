extern crate reqwest;

use reqwest::Error;

fn get_text() -> Result<String, Error> {
    let text = reqwest::get("http://cfp.devoxx.fr/api/conferences/DevoxxFR2018/speakers/76606cd0-6261-44b2-ad0e-3518a0e66995")?.text()?;
    Ok(text)
}

fn main() {
    let body = get_text().unwrap();
    println!("body = {:?}", body);
}
