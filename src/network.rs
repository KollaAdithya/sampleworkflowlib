#[derive(Debug)]
pub enum MyError {
    Reqwest(reqwest::Error),
    Other(String),
}


use reqwest;

pub async fn make_internet_call(url: &str) -> Result<String, MyError> {
    let response = reqwest::get(url).await.map_err(MyError::Reqwest)?;

    if response.status().is_success() {
        let body = response.text().await.map_err(MyError::Reqwest)?;
        Ok(body)
    } else {
        eprintln!("Request failed with status: {}", response.status());
        Err(MyError::Other(format!("Failed to fetch data: {}", response.status())))
    }
}
