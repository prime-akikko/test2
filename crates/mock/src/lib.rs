//! Thin demo helpers for Cloud/CI headless smoke.
//!
//! No network or protocol — only sample UI copy for the starter shell.

#![forbid(unsafe_code)]

use starter_core::ShellState;

/// Apply a short demo status line (headless / CI smoke).
pub fn apply_demo_status(state: &mut ShellState) {
    state.status_message = "Demo-Modus (kein Live-Gerät, keine Protokollschicht).".to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn demo_status_is_german() {
        let mut state = ShellState::new();
        apply_demo_status(&mut state);
        assert!(state.status_message.contains("Demo-Modus"));
    }
}
