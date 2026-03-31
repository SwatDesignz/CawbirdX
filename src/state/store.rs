use anyhow::{Context, Result};
use heed::{Env, EnvOpenOptions, Database};
use serde::{Deserialize, Serialize};
use std::path::Path;
use std::sync::Arc;

/// Cached tweet data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CachedTweet {
    pub id: String,
    pub data: String, // JSON serialized tweet
    pub cached_at: i64, // Unix timestamp
}

/// Local cache store using LMDB (heed)
pub struct CacheStore {
    env: Arc<Env>,
    tweets_db: Database<heed::types::Str, heed::types::Str>,
}

impl CacheStore {
    /// Open or create cache database
    pub fn open(path: &Path) -> Result<Self> {
        let env = unsafe {
            EnvOpenOptions::new()
                .map_size(10 * 1024 * 1024) // 10MB
                .max_dbs(10)
                .open(path)
                .context("Failed to open cache database")?
        };

        let mut wtxn = env.write_txn().context("Failed to begin write transaction")?;
        let tweets_db = env
            .create_database(&mut wtxn, Some("tweets"))
            .context("Failed to create tweets database")?;
        wtxn.commit().context("Failed to commit transaction")?;

        Ok(Self {
            env: Arc::new(env),
            tweets_db,
        })
    }

    /// Cache a tweet
    pub fn put_tweet(&self, tweet_id: &str, data: &str) -> Result<()> {
        let mut wtxn = self
            .env
            .write_txn()
            .context("Failed to begin write transaction")?;

        let cached = CachedTweet {
            id: tweet_id.to_string(),
            data: data.to_string(),
            cached_at: chrono::Local::now().timestamp(),
        };

        let json = serde_json::to_string(&cached).context("Failed to serialize cached tweet")?;
        self.tweets_db
            .put(&mut wtxn, tweet_id, &json)
            .context("Failed to put tweet in cache")?;

        wtxn.commit().context("Failed to commit transaction")?;
        Ok(())
    }

    /// Get cached tweet
    pub fn get_tweet(&self, tweet_id: &str) -> Result<Option<String>> {
        let rtxn = self
            .env
            .read_txn()
            .context("Failed to begin read transaction")?;

        match self.tweets_db.get(&rtxn, tweet_id) {
            Ok(Some(json)) => {
                let cached: CachedTweet =
                    serde_json::from_str(&json).context("Failed to deserialize cached tweet")?;

                // Check if cache is stale (1 hour)
                let now = chrono::Local::now().timestamp();
                if now - cached.cached_at > 3600 {
                    Ok(None)
                } else {
                    Ok(Some(cached.data))
                }
            }
            Ok(None) => Ok(None),
            Err(e) => Err(e).context("Failed to get tweet from cache"),
        }
    }

    /// Clear all cached tweets
    pub fn clear_tweets(&self) -> Result<()> {
        let mut wtxn = self
            .env
            .write_txn()
            .context("Failed to begin write transaction")?;

        self.tweets_db
            .clear(&mut wtxn)
            .context("Failed to clear tweets cache")?;

        wtxn.commit().context("Failed to commit transaction")?;
        Ok(())
    }
}
