//! Orchestrator for the game loop.

/// Represents the game loop.
pub struct Game {}

impl Game {

    /// Starts the game loop.
    pub async fn start(&self) {

        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(500));

        loop {
            interval.tick().await;
            println!("Ticking!");
        }
    }
}