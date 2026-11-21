//! Root window: header | nav panel | main content | status bar.
//!
//! Pure GPUI shell — German labels, panel look inspired by test1.
//! No protocol, device, or network UI.

use gpui::{
    div, prelude::*, px, rgb, size, App, Application, Bounds, ClickEvent, Context, FontWeight,
    InteractiveElement, IntoElement, ParentElement, Rgba, SharedString, Styled, TitlebarOptions,
    Window, WindowBounds, WindowOptions,
};

use crate::strings;
use starter_core::ShellState;

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
}

struct RootView {
    state: ShellState,
}

impl RootView {
    fn new(window: &mut Window, _cx: &mut Context<Self>) -> Self {
        window.set_window_title(strings::WINDOW_TITLE);
        let mut state = ShellState::new();
        #[cfg(feature = "mock")]
        {
            starter_mock::apply_demo_status(&mut state);
        }
        Self { state }
    }

    fn on_select_nav(&mut self, item: String, cx: &mut Context<Self>) {
        self.state.select_nav(item);
        cx.notify();
    }

    fn nav_button(&self, item: String, selected: bool, cx: &mut Context<Self>) -> impl IntoElement {
        let label = item.clone();
        div()
            .id(SharedString::from(format!("nav-{item}")))
            .px_3()
            .py_2()
            .rounded_md()
            .bg(if selected {
                theme::accent()
            } else {
                theme::panel_alt()
            })
            .text_sm()
            .font_weight(if selected {
                FontWeight::SEMIBOLD
            } else {
                FontWeight::NORMAL
            })
            .cursor_pointer()
            .hover(|s| s.bg(theme::accent_dim()))
            .on_click(cx.listener(move |this, _: &ClickEvent, _window, cx| {
                this.on_select_nav(item.clone(), cx);
            }))
            .child(label)
    }
}

impl Render for RootView {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let selected = self.state.selected_nav.clone();
        let message = self.state.status_message.clone();
        let nav_items = self.state.nav_items.clone();

        div()
            .id("root")
            .size_full()
            .flex()
            .flex_col()
            .bg(theme::bg())
            .text_color(theme::text())
            .font_family("Segoe UI")
            // Header: logo top-left, then title + subheading
            .child(
                div()
                    .flex()
                    .flex_row()
                    .items_center()
                    .gap_3()
                    .px_4()
                    .py_3()
                    .bg(theme::panel())
                    .border_b_1()
                    .border_color(theme::border())
                    .child(div().id("header-logo").text_2xl().child(strings::APP_LOGO))
                    .child(
                        div()
                            .flex()
                            .flex_col()
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
                    ),
            )
            // Body: nav | main
            .child(
                div()
                    .flex_1()
                    .flex()
                    .flex_row()
                    .min_h(px(0.))
                    // Side nav panel (layout inspired by test1 groups column)
                    .child(
                        div()
                            .w(px(200.))
                            .flex()
                            .flex_col()
                            .gap_2()
                            .px_3()
                            .py_3()
                            .bg(theme::panel_alt())
                            .border_r_1()
                            .border_color(theme::border())
                            .child(
                                div()
                                    .text_xs()
                                    .text_color(theme::muted())
                                    .child(strings::NAV_SECTION),
                            )
                            .children(nav_items.into_iter().map(|item| {
                                let is_selected = item == selected;
                                self.nav_button(item, is_selected, cx)
                            })),
                    )
                    // Main content
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
                                    .child(format!("{} — {selected}", strings::MAIN_SECTION)),
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
                    .child(format!("{}: {message}", strings::STATUS_LABEL)),
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
