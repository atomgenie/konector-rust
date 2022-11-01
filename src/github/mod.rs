use futures::future;
use serde::Deserialize;

pub async fn fetch_users<'a>(
    usernames: Vec<String>,
) -> Result<Vec<GitHubResponse>, Box<dyn std::error::Error>> {
    let promises = usernames.iter().map(fetch_api);
    let data = future::try_join_all(promises).await?;
    let data: Vec<GitHubResponse> = data.into_iter().flatten().collect();

    Ok(data)
}

#[derive(Deserialize, Debug)]
pub struct GitHubResponse {
    pub key: String,
}

async fn fetch_api(username: &String) -> Result<Vec<GitHubResponse>, Box<dyn std::error::Error>> {
    let encoded_user = urlencoding::encode(username.as_str());
    let uri = format!(
        "https://api.github.com/users/{user}/keys",
        user = encoded_user
    );

    let req = reqwest::Client::new();
    let body = req
        .get(uri)
        .header("User-Agent", "Axios/1.1.3")
        .send()
        .await?
        .text()
        .await?;

    let json: Vec<GitHubResponse> = serde_json::from_str(body.as_str())?;

    Ok(json)
}
