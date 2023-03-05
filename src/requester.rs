use reqwest;

#[derive(Debug)]
pub enum HttpError {
    NotFound,
    Unknown,
}

pub fn request(url: &str) -> Result<String, HttpError> {
    let response = reqwest::blocking::get(url);
    match response {
        Ok(mut res) => {
            if res.status().is_success() {
                let body = res.text().unwrap();
                Ok(body)
            } else if res.status().eq(&reqwest::StatusCode::NOT_FOUND) {
                Err(HttpError::NotFound)
            } else {
                Err(HttpError::Unknown)
            }
        }
        Err(_) => Err(HttpError::Unknown),
    }
}
