//! Small shared utilities.

/// Heuristic token estimate (~4 chars/token).
///
/// v0.1 uses a heuristic so the build stays light and offline-deterministic.
/// It is intentionally the single choke point for tokenization: swap in
/// `tiktoken-rs` here later without touching any caller. For L1 regression
/// metrics, *consistency* matters more than absolute accuracy.
pub fn estimate_tokens(text: &str) -> u32 {
    let chars = text.chars().count();
    ((chars as f32) / 4.0).ceil() as u32
}
