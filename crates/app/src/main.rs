//! GPUI starter binary.
//!
//! Default Cloud/CI build: no GPUI link (headless smoke).
//! Windows GUI: `cargo run -p starter_app --features gui`.

#![forbid(unsafe_code)]

fn main() {
    #[cfg(feature = "gui")]
    {
        starter_app::ui::run_gui();
    }

    #[cfg(not(feature = "gui"))]
    {
        run_headless_smoke();
    }
}

/// Headless smoke path for Cloud/CI without native GUI libs.
#[cfg(not(feature = "gui"))]
fn run_headless_smoke() {
    use starter_app::strings;
    use starter_core::ShellState;

    println!("{}", strings::WINDOW_TITLE);
    println!("{}", strings::APP_SUBHEADING);

    let mut state = ShellState::new();
    println!("Navigation: {}", state.nav_items.join(", "));
    println!("Auswahl: {}", state.selected_nav);

    #[cfg(feature = "mock")]
    {
        starter_mock::apply_demo_status(&mut state);
        println!("{}", state.status_message);
        state.select_nav("Einstellungen".to_string());
        println!(
            "Nach Auswahl: {} — {}",
            state.selected_nav, state.status_message
        );
    }

    #[cfg(not(feature = "mock"))]
    {
        println!("{}", strings::MSG_HEADLESS_HINT);
    }

    println!("Headless-Smoke bereit (GUI-Feature aus).");
}
