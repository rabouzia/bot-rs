// pub XPost {
// 	videos_url: Vec<String>,
// 	photos_url: Vec<String>,
// 	text: Option<String>,
// 	id: String,
// }

use rustc_serialize::json::Json;

#[derive(Debug, Default)]
pub struct X {
    pub video: bool,
    pub img: bool,
    pub text: bool,
    pub id: String,
    pub url: String,
}

impl X {
    pub fn new(mut self, context: &'static str, _url: String, _id: String) -> X {
        match context {
            "video" => self.video = true,
            "img" => self.img = true,
            "text" => self.text = true,
            _ => {}
        }
        self.url = _url;
        self.id = _id;
        return self;
    }
}

pub fn x_url_parser(handle: String) -> Result<String, bool> {
    let extract: Vec<&str> = handle.splitn(6, "/").collect();
    if extract.len() <= 1 {
        return Err(false);
    }
    // println!("Twitter URL: {}", extract[5]);
    // println!("Twitter last: {}", extract[2]);
    match extract[2] {
        "x.com" | "twitter.com" => {
            ();
        }
        _ => {
            return Err(false);
        }
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

// fn get_id() -> String
// {

// }

// async fn twitter(handle: String, mut vec: Vec<String>) -> X
pub async fn twitter(handle: String) -> Result<Vec<X>, bool> {
    let mut vx: Vec<X> = Vec::new();
    let last: String = x_url_parser(handle)?;
    // let  mut easy = Easy::new();
    let scrapper_key = env::var("X_SCRAPPER").expect("X_SCRAPPER not found in .env");
    let body = reqwest::get(&format!("{}{}", scrapper_key, last))
        .await
        .expect("REASON")
        .text()
        .await
        .expect("REASON");
    // println!("Body: {body:?}");
    let json = Json::from_str(&body).expect("REASON");
    // println!("{}",json);
    let media = json["data"]["media"].as_array().expect("REASON");
    for item in media {
        let media_type = item["type"].as_string().unwrap_or("unknown");
        let caca = item["url"].as_string().unwrap_or_default();

        // Extract the file name from the URL (without extension)
        let jsom = caca
            .rsplit('/')
            .next()
            .and_then(|filename| filename.split_once('.').map(|(name_no_ext, _)| name_no_ext))
            .unwrap_or("unknown")
            .to_string();

        // Print debugging info
        println!("media type: {}", media_type);
        println!("new URL: {}", jsom);

        match media_type {
            "photo" => {
                let url = item["url"].as_string().unwrap_or_default();
                println!("Image URL: {}", url);
                let ix = X::default().new("img", url.to_string(), jsom.clone());
                vx.push(ix);
            }

            "video" => {
                let video_url = item["videoUrl"].as_string().unwrap_or_default();
                println!("Video URL: {}", video_url);
                let ok =X::default().new("video", video_url.to_string(), jsom.clone());
                vx.push(ok);
            }

            _ => {
                println!("Unknown media type: {}", media_type);
            }

        }
    }
    return Ok(vx);
}

/*
    - if empty add text as quote

*/
