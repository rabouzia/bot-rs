use anyhow::{Context, Result, bail};
#[derive(Debug, Default)]
pub struct Tiktok {
    video: bool,
    img: bool,
    url: String,
    id: String,
}

impl Tiktok {
    pub fn new(mut self, context: &'static str, _url: String, _id: String) -> Tiktok {
        match context {
            "video" => self.video = true,
            "img" => self.img = true,
            _ => {}
        }
        self.url = _url;
        self.id = _id;
        return self;
    }
}
// https://d.rapidcdn.app/v2?token=eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJ1cmwiOiJodHRwczovL3Y0NS1ldS50aWt0b2tjZG4uY29tLzVlYmU1ZDliYzg1NDc0MTE5YTNkZWVhMWI5MGE1YzNjLzY5MGU1YjJjL3ZpZGVvL3Rvcy9ubzFhL3Rvcy1ubzFhLXZlLTAwNjhjMDAxLW5vL280UFNEWXRIUXZiV2U0ZkFNQTZnR1FOZWpsendNTEFpSThMUlBJLz9hPTExODAmYnRpPWJHUnVaSHh2TVhJeGNtNTNabTFjWUY5ZWJXRnphSEZtT2clM0QlM0QmY2g9MCZjcj0wJmRyPTAmZXI9MCZjZD0wJTdDMCU3QzAlN0MwJmN2PTEmYnI9NTUyJmJ0PTI3NiZjcz0wJmRzPTYmZnQ9WHNGYjhxNGZtYmRQRDEyWndxV3Mzd1UxeFRReGFlRn5PNSZtaW1lX3R5cGU9dmlkZW9fbXA0JnFzPTAmcmM9TkRVNE5UeHBPRGM1TlRnNE9HVTdhRUJwTXpaeWRXNDVjbWM2TnpNemJ6Y3pOVUJmWW1Fd01TNDJYelF4TkdBd00ySXRZU051WTJjMU1tUnpiVE5oTFMxa01URnpjdyUzRCUzRCZ2dnBsPTEmbD0yMDI1MTEwNzA0NDc0NUVFOTU1NkQ2NTM5MDQ1OEZFMkFCJmJ0YWc9ZTAwMDhkMDAwIiwiZmlsZW5hbWUiOiJzbmFwdGlrXzc1Njk1NjMxNTM2OTMwMTk0MTVfdjIubXA0IiwiaGVhZGVycyI6eyJ1c2VyLWFnZW50IjoiVGVsZWdyYW1Cb3QgKGxpa2UgVHdpdHRlckJvdCkifSwiaWF0IjoxNzYyNDYyMDY1fQ.Z_-ye0Xr7tnw3Di2wQL7QKshK7sx8FTGBd2VO-Q2rbE&dl=1

// https://vm.tiktok.com/ZNdwh5QQP/

// https://tikcdn.io/ssstik/ZNdwh5QQP

pub fn tiktok_url_parser(handle: String) -> Result<String> {
    let extract: Vec<&str> = handle.splitn(6, "/").collect();
    if extract.len() <= 1 {
        bail!("profile_pic_url_hd not found in JSON.");
    }
    match extract[2] {
        "tiktok.com" | "vm.tiktok.com" | "vt.tiktok.com" => {
            ();
        }
        _ => {}
    }
    let mut mid = extract[5];
    if mid.contains("?") {
        let last: Vec<&str>;
        last = mid.split("?").collect();
        println!("new URL: {}", last[0]);
        mid = last[0];
    }

    return Ok(mid.to_string());
}

pub async fn tiktok(handle: String) -> Result<Vec<Tiktok>> {
    let vt: Vec<Tiktok> = Vec::new();
    let last: String = tiktok_url_parser(handle)?;

    let scrapper_key = env::var("TK_SCRAPPER").context("TK_SCRAPPER not found in .env")?;


    let _body = reqwest::get(&format!("{}{}", scrapper_key, last))
        .await
        .context("REASON")?
        .text()
        .await
        .context("REASON")?;

    // let  mut easy = Easy::new();
    Ok(vt)
}
