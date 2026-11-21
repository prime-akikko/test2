//! Thin application-core stubs for the GPUI starter shell.
//!
//! Pure UI shell state: navigation selection + status message.
//! No protocol, device, or network concepts.

#![forbid(unsafe_code)]

/// Default German navigation labels for the side panel.
pub fn default_nav_items() -> Vec<String> {
    ["Übersicht", "Einstellungen", "Hilfe"]
        .into_iter()
        .map(str::to_string)
        .collect()
}

/// Minimal shell state shared by headless smoke and the GPUI view.
#[derive(Debug, Clone)]
pub struct ShellState {
    pub nav_items: Vec<String>,
    pub selected_nav: String,
    pub status_message: String,
}

impl ShellState {
    pub fn new() -> Self {
        let nav_items = default_nav_items();
        let selected_nav = nav_items
            .first()
            .cloned()
            .unwrap_or_else(|| "Übersicht".to_string());
        Self {
            nav_items,
            selected_nav,
            status_message: "Bereit.".to_string(),
        }
    }

    pub fn select_nav(&mut self, item: String) {
        if self.nav_items.iter().any(|n| n == &item) {
            self.selected_nav = item;
            self.status_message = format!("Auswahl: {}", self.selected_nav);
        }
    }
}

impl Default for ShellState {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn default_nav_has_german_labels() {
        let state = ShellState::new();
        assert!(state.nav_items.contains(&"Übersicht".to_string()));
        assert_eq!(state.selected_nav, "Übersicht");
    }

    #[test]
    fn select_nav_updates_status() {
        let mut state = ShellState::new();
        state.select_nav("Hilfe".to_string());
        assert_eq!(state.selected_nav, "Hilfe");
        assert!(state.status_message.contains("Hilfe"));
    }
}
