use serde::{Deserialize, Serialize};
use starchart::backend::{Backend, JsonBackend};

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
    let json_backend = JsonBackend::new("../target/db")?;

    json_backend.init().await?;

    json_backend.ensure_table("guilds").await?;

    json_backend
        .create("guilds", "hey", &Settings::new("hey".to_string()))
        .await?;

    Ok(())
}
