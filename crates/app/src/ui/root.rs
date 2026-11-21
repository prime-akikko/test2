//! Root window: header | connection panel | main content | status bar.
//!
//! Slim skeleton modeled on the test1 shell layout — no Modbus/catalog UI.

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, ClickEvent, Context, FontWeight,
    InteractiveElement, IntoElement, ParentElement, Rgba, SharedString, Styled, TitlebarOptions,
    Window, WindowBounds, WindowOptions,
};

use crate::strings;
use starter_core::ShellState;

#[cfg(feature = "mock")]
use starter_mock::MockSession;

mod theme {
    use super::{rgb, Rgba};

    pub fn bg() -> Rgba {
        rgb(0x1a_2332)
    }
    pub fn panel() -> Rgba {
        rgb(0x24_3044)
    }
    pub fn panel_alt() -> Rgba {
        rgb(0x2c_3b52)
    }
    pub fn border() -> Rgba {
        rgb(0x3d_4f66)
    }
    pub fn text() -> Rgba {
        rgb(0xe8_eef6)
    }
    pub fn muted() -> Rgba {
        rgb(0x9a_abbf)
    }
    pub fn accent() -> Rgba {
        rgb(0x3d_9b8f)
    }
    pub fn accent_dim() -> Rgba {
        rgb(0x2f_7a71)
    }
    pub fn danger() -> Rgba {
        rgb(0xc4_5c5c)
    }
    pub fn field_bg() -> Rgba {
        rgb(0x15_202e)
    }
}

struct RootView {
    state: ShellState,
    #[cfg(feature = "mock")]
    session: MockSession,
}

impl RootView {
    fn new(window: &mut Window, _cx: &mut Context<Self>) -> Self {
        window.set_window_title(strings::WINDOW_TITLE);
        Self {
            state: ShellState::new(cfg!(feature = "mock")),
            #[cfg(feature = "mock")]
            session: MockSession::new(),
        }
    }

    fn on_connect(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        #[cfg(feature = "mock")]
        {
            self.session.connect(&mut self.state);
        }
        #[cfg(not(feature = "mock"))]
        {
            self.state.status_message =
                "Kein Mock — Live-Backend ist in diesem Starter nicht verdrahtet.".to_string();
        }
        cx.notify();
    }

    fn on_disconnect(&mut self, _: &ClickEvent, _window: &mut Window, cx: &mut Context<Self>) {
        #[cfg(feature = "mock")]
        {
            self.session.disconnect(&mut self.state);
        }
        #[cfg(not(feature = "mock"))]
        {
            self.state.status = starter_core::ConnectionStatus::Disconnected;
            self.state.status_message = strings::MSG_DISCONNECT_OK.to_string();
        }
        cx.notify();
    }

    fn labeled_value(&self, label: &'static str, value: String) -> impl IntoElement {
        div()
            .flex()
            .flex_col()
            .gap_1()
            .min_w(px(120.))
            .child(div().text_xs().text_color(theme::muted()).child(label))
            .child(
                div()
                    .px_3()
                    .py_2()
                    .rounded_md()
                    .bg(theme::field_bg())
                    .border_1()
                    .border_color(theme::border())
                    .text_sm()
                    .child(value),
            )
    }

    fn action_button(
        &self,
        id: &'static str,
        label: &'static str,
        enabled: bool,
        cx: &mut Context<Self>,
        on_click: impl Fn(&mut Self, &ClickEvent, &mut Window, &mut Context<Self>) + 'static,
    ) -> impl IntoElement {
        let bg = if enabled {
            theme::accent()
        } else {
            theme::accent_dim()
        };
        div()
            .id(id)
            .px_3()
            .py_2()
            .rounded_md()
            .bg(bg)
            .text_sm()
            .font_weight(FontWeight::MEDIUM)
            .cursor_pointer()
            .opacity(if enabled { 1.0 } else { 0.5 })
            .when(enabled, |d| d.on_click(cx.listener(on_click)))
            .child(label)
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let connected = self.state.is_connected();
        let status_label = self.state.status.label_de();
        let message = self.state.status_message.clone();
        let host = self.state.connection.host.clone();
        let port = self.state.connection.port.to_string();
        let unit = self.state.connection.unit_id.to_string();
        let backend = self.state.backend_label();

        div()
            .id("root")
            .size_full()
            .flex()
            .flex_col()
            .bg(theme::bg())
            .text_color(theme::text())
            .font_family("Segoe UI")
            // Header
            .child(
                div()
                    .flex()
                    .flex_col()
                    .px_4()
                    .py_3()
                    .bg(theme::panel())
                    .border_b_1()
                    .border_color(theme::border())
                    .child(
                        div()
                            .text_xl()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(strings::APP_HEADING),
                    )
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme::muted())
                            .child(strings::APP_SUBHEADING),
                    ),
            )
            // Connection-style panel
            .child(
                div()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .px_4()
                    .py_3()
                    .bg(theme::panel_alt())
                    .border_b_1()
                    .border_color(theme::border())
                    .child(
                        div()
                            .text_xs()
                            .text_color(theme::muted())
                            .child(strings::CONNECTION_SECTION),
                    )
                    .child(
                        div()
                            .flex()
                            .flex_row()
                            .items_end()
                            .gap_3()
                            .child(self.labeled_value(strings::LABEL_HOST, host))
                            .child(self.labeled_value(strings::LABEL_PORT, port))
                            .child(self.labeled_value(strings::LABEL_UNIT, unit))
                            .child(self.action_button(
                                "btn-connect",
                                strings::BTN_CONNECT,
                                !connected,
                                cx,
                                Self::on_connect,
                            ))
                            .child(self.action_button(
                                "btn-disconnect",
                                strings::BTN_DISCONNECT,
                                connected,
                                cx,
                                Self::on_disconnect,
                            )),
                    ),
            )
            // Main content area
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_col()
                    .gap_2()
                    .px_4()
                    .py_4()
                    .child(
                        div()
                            .text_sm()
                            .font_weight(FontWeight::SEMIBOLD)
                            .child(strings::MAIN_SECTION),
                    )
                    .child(
                        div()
                            .flex_1()
                            .rounded_md()
                            .border_1()
                            .border_color(theme::border())
                            .bg(theme::panel())
                            .p_4()
                            .text_sm()
                            .text_color(theme::muted())
                            .child(strings::MAIN_PLACEHOLDER),
                    ),
            )
            // Status bar
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_4()
                    .px_4()
                    .py_2()
                    .border_t_1()
                    .border_color(theme::border())
                    .bg(theme::panel())
                    .text_xs()
                    .text_color(theme::muted())
                    .child(format!("{}: {status_label}", strings::STATUS_CONNECTION))
                    .child(backend)
                    .child(
                        div()
                            .flex_1()
                            .text_color(if connected {
                                theme::accent()
                            } else {
                                theme::danger()
                            })
                            .child(message),
                    ),
            )
    }
}

/// Launch the GPUI application (blocks).
pub fn run_gui() {
    Application::new().run(|cx: &mut App| {
        let bounds = Bounds::centered(None, size(px(1100.), px(720.)), cx);
        cx.open_window(
            WindowOptions {
                window_bounds: Some(WindowBounds::Windowed(bounds)),
                titlebar: Some(TitlebarOptions {
                    title: Some(SharedString::from(strings::WINDOW_TITLE)),
                    ..Default::default()
                }),
                window_min_size: Some(size(px(800.), px(520.))),
                app_id: Some("gpui-starter".into()),
                ..Default::default()
            },
            |window, cx| cx.new(|cx| RootView::new(window, cx)),
        )
        .expect("Fenster konnte nicht geöffnet werden");
        cx.activate(true);
    });
}
