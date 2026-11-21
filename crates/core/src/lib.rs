//! Thin application-core stubs for the GPUI starter shell.
//!
//! Intentionally minimal: connection settings + status labels only.
//! No Modbus, catalog, or scheduler logic.

#![forbid(unsafe_code)]

/// Default connection placeholders (demo values for the shell).
pub mod defaults {
    pub const HOST: &str = "127.0.0.1";
    pub const PORT: u16 = 502;
    pub const UNIT_ID: u8 = 1;
}

/// Connection settings shown in the shell header panel.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ConnectionSettings {
    pub host: String,
    pub port: u16,
    pub unit_id: u8,
}

impl Default for ConnectionSettings {
    fn default() -> Self {
        Self {
            host: defaults::HOST.to_string(),
            port: defaults::PORT,
            unit_id: defaults::UNIT_ID,
        }
    }
}

/// High-level connection status for the status bar.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum ConnectionStatus {
    #[default]
    Disconnected,
    Connecting,
    Connected,
    Error,
}

impl ConnectionStatus {
    /// German status label.
    pub fn label_de(self) -> &'static str {
        match self {
            Self::Disconnected => "Getrennt",
            Self::Connecting => "Verbinden…",
            Self::Connected => "Verbunden",
            Self::Error => "Fehler",
        }
    }
}

/// Minimal shell state shared by headless smoke and the GPUI view.
#[derive(Debug, Clone)]
pub struct ShellState {
    pub connection: ConnectionSettings,
    pub status: ConnectionStatus,
    pub backend_mock: bool,
    pub status_message: String,
}

impl ShellState {
    pub fn new(backend_mock: bool) -> Self {
        Self {
            connection: ConnectionSettings::default(),
            status: ConnectionStatus::Disconnected,
            backend_mock,
            status_message: if backend_mock {
                "Cloud/Dev: Mock-Backend aktiv.".to_string()
            } else {
                "Bereit.".to_string()
            },
        }
    }

    pub fn backend_label(&self) -> &'static str {
        if self.backend_mock {
            "Mock-Backend"
        } else {
            "Live-Backend"
        }
    }

    pub fn is_connected(&self) -> bool {
        self.status == ConnectionStatus::Connected
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn defaults_match_placeholders() {
        let c = ConnectionSettings::default();
        assert_eq!(c.host, "127.0.0.1");
        assert_eq!(c.port, 502);
        assert_eq!(c.unit_id, 1);
    }

    #[test]
    fn german_status_labels() {
        assert_eq!(ConnectionStatus::Disconnected.label_de(), "Getrennt");
        assert_eq!(ConnectionStatus::Connected.label_de(), "Verbunden");
    }
}
