use serde::{Deserialize, Serialize};
use starchart::backend::{Backend, CacheBackend};

#[derive(Debug, Default, Clone, Serialize, Deserialize)]
struct Settings {
    id: String,
    name: String,
}

impl Settings {
    fn new(id: String) -> Self {
        Self {
            id,
            ..Self::default()
        }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let backend = CacheBackend::new();

    backend.init().await?;

    backend.ensure_table("guilds").await?;

    backend
        .create("guilds", "hey", &Settings::new("hey".to_string()))
        .await?;

    dbg!(backend);

    Ok(())
}
