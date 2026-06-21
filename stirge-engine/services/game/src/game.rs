//! Orchestrator for the game loop.

// Note that Stirge Hunters gameplay is balanced around this tick rate.
// Modify at your own risk.
const TICK_DURATION_IN_MS: u64 = 500;

/// Represents the game loop.
pub struct Game {}

impl Game {

    /// Starts the game loop.
    pub async fn start(&self) {

        let mut interval = tokio::time::interval(tokio::time::Duration::from_millis(TICK_DURATION_IN_MS));

        loop {
            interval.tick().await;
            println!("Ticking!");
        }
    }
}