//! GPUI starter binary.
//!
//! Default Cloud/CI build: `mock` feature, no GPUI link.
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

    let mut state = ShellState::new(cfg!(feature = "mock"));
    println!(
        "Defaults: {}:{} unit {}",
        state.connection.host, state.connection.port, state.connection.unit_id
    );
    println!("Backend: {}", state.backend_label());
    println!("Status: {}", state.status.label_de());

    #[cfg(feature = "mock")]
    {
        let mut session = starter_mock::MockSession::new();
        session.connect(&mut state);
        println!("Nach Verbinden: {}", state.status.label_de());
        println!("{}", state.status_message);
        session.disconnect(&mut state);
        println!("Nach Trennen: {}", state.status.label_de());
    }

    #[cfg(not(feature = "mock"))]
    {
        println!("Mock-Feature aus — kein automatischer Connect.");
        println!("{}", strings::MSG_HEADLESS_HINT);
    }

    println!("Headless-Smoke bereit (GUI-Feature aus).");
}
