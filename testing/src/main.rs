use serde::{Deserialize, Serialize};
use starchart::backend::{Backend, CacheBackend};

#[derive(Debug, Clone, Serialize, Deserialize)]
struct Person {
    name: String,
    age: u8,
}

impl Person {
    const fn new(name: String, age: u8) -> Self {
        Self { name, age }
    }
}

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error + Send + Sync + 'static>> {
    let backend = CacheBackend::new();

    backend.init().await?;

    backend.ensure_table("people").await?;

    let person = Person::new("Ferris".to_string(), 19);

    backend.create("people", &person.name, &person).await?;

    dbg!(backend);

    Ok(())
}
