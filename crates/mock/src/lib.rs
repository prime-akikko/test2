//! Thin mock backend for Cloud/CI and local smoke without a live device.
//!
//! Simulates connect / disconnect only — no protocol I/O.

#![forbid(unsafe_code)]

use starter_core::{ConnectionStatus, ShellState};

/// In-memory mock session used by the starter shell.
#[derive(Debug, Default)]
pub struct MockSession {
    connected: bool,
}

impl MockSession {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn connect(&mut self, state: &mut ShellState) {
        state.status = ConnectionStatus::Connecting;
        state.status_message = "Verbinden (Mock)…".to_string();
        self.connected = true;
        state.status = ConnectionStatus::Connected;
        state.status_message = "Verbindung hergestellt (Mock).".to_string();
    }

    pub fn disconnect(&mut self, state: &mut ShellState) {
        self.connected = false;
        state.status = ConnectionStatus::Disconnected;
        state.status_message = "Verbindung getrennt.".to_string();
    }

    pub fn is_connected(&self) -> bool {
        self.connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn connect_disconnect_updates_state() {
        let mut session = MockSession::new();
        let mut state = ShellState::new(true);
        session.connect(&mut state);
        assert!(session.is_connected());
        assert_eq!(state.status, ConnectionStatus::Connected);
        session.disconnect(&mut state);
        assert!(!session.is_connected());
        assert_eq!(state.status, ConnectionStatus::Disconnected);
    }
}
