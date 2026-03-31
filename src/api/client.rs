use anyhow::{Context, Result};
use reqwest::header::{HeaderMap, HeaderValue};
use serde::Deserialize;
use std::sync::Arc;

/// RapidAPI client for Twitter241 API
#[derive(Clone)]
pub struct TwitterClient {
    client: reqwest::Client,
    api_key: Arc<String>,
    api_host: Arc<String>,
}

impl TwitterClient {
    /// Create a new Twitter client with RapidAPI credentials
    pub fn new(api_key: String, api_host: String) -> Self {
        let mut headers = HeaderMap::new();
        headers.insert("x-rapidapi-key", HeaderValue::from_str(&api_key).unwrap());
        headers.insert("x-rapidapi-host", HeaderValue::from_str(&api_host).unwrap());
        headers.insert("x-rapidapi-user-agent", HeaderValue::from_static("CawbirdX/0.1"));

        let client = reqwest::Client::builder()
            .default_headers(headers)
            .build()
            .expect("Failed to create HTTP client");

        Self {
            client,
            api_key: Arc::new(api_key),
            api_host: Arc::new(api_host),
        }
    }

    /// Get home timeline
    pub async fn get_timeline(&self, count: Option<u32>) -> Result<Vec<Tweet>> {
        let mut params: Vec<(String, String)> = vec![];
        if let Some(c) = count {
            params.push(("count".to_string(), c.to_string()));
        }

        let response = self
            .client
            .get(&format!("https://{}/timeline", self.api_host))
            .query(&params)
            .send()
            .await
            .context("Failed to fetch timeline")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, text);
        }

        let tweets: Vec<Tweet> = response.json().await.context("Failed to parse timeline")?;
        Ok(tweets)
    }

    /// Get tweet by ID
    pub async fn get_tweet(&self, id: &str) -> Result<Tweet> {
        let response = self
            .client
            .get(&format!("https://{}/tweet", self.api_host))
            .query(&[("id", id)])
            .send()
            .await
            .context("Failed to fetch tweet")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, text);
        }

        let tweet: Tweet = response.json().await.context("Failed to parse tweet")?;
        Ok(tweet)
    }

    /// Post a new tweet
    pub async fn post_tweet(&self, text: &str) -> Result<Tweet> {
        let response = self
            .client
            .post(&format!("https://{}/tweet", self.api_host))
            .json(&serde_json::json!({ "text": text }))
            .send()
            .await
            .context("Failed to post tweet")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, text);
        }

        let tweet: Tweet = response.json().await.context("Failed to parse response")?;
        Ok(tweet)
    }

    /// Search tweets
    pub async fn search(&self, query: &str, count: Option<u32>) -> Result<Vec<Tweet>> {
        let mut params: Vec<(String, String)> = vec![("q".to_string(), query.to_string())];
        if let Some(c) = count {
            params.push(("count".to_string(), c.to_string()));
        }

        let response = self
            .client
            .get(&format!("{}/search", self.api_host))
            .query(&params)
            .send()
            .await
            .context("Failed to search tweets")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, text);
        }

        let tweets: Vec<Tweet> = response.json().await.context("Failed to parse search results")?;
        Ok(tweets)
    }

    /// Get user profile
    pub async fn get_user(&self, username: &str) -> Result<User> {
        let response = self
            .client
            .get(&format!("{}/user", self.api_host))
            .query(&[("username", username)])
            .send()
            .await
            .context("Failed to fetch user")?;

        if !response.status().is_success() {
            let status = response.status();
            let text = response.text().await.unwrap_or_default();
            anyhow::bail!("API error {}: {}", status, text);
        }

        let user: User = response.json().await.context("Failed to parse user")?;
        Ok(user)
    }
}

/// Tweet data model
#[derive(Debug, Clone, Deserialize)]
pub struct Tweet {
    pub id: String,
    pub text: String,
    pub author: User,
    pub created_at: String,
    pub like_count: u64,
    pub retweet_count: u64,
    pub reply_count: u64,
    pub media: Vec<Media>,
}

/// User data model
#[derive(Debug, Clone, Deserialize)]
pub struct User {
    pub id: String,
    pub username: String,
    pub name: String,
    pub profile_image_url: Option<String>,
    pub verified: bool,
    pub followers_count: u64,
    pub following_count: u64,
}

/// Media attachment
#[derive(Debug, Clone, Deserialize)]
pub struct Media {
    pub media_url: String,
    pub media_type: String,
    pub width: Option<u32>,
    pub height: Option<u32>,
}
