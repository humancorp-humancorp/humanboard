//! Shared modal utilities - keyboard badges, setting rows, section headers, dropdown markers, backdrop helpers.

use crate::app::Humanboard;
use crate::constants::MODAL_BACKDROP_OPACITY;
use gpui::*;
use gpui_component::{h_flex, v_flex, ActiveTheme as _};

// ============================================================================
// Backdrop Click-to-Close Pattern
// ============================================================================

/// Renders a modal backdrop with click-to-close behavior using state-tracking.
///
/// This pattern uses a boolean flag to distinguish between clicks on the backdrop
/// versus clicks on the modal content. The flag is set on mouse down and checked
/// on mouse up - only if the mouse went down on the backdrop (not the modal) does
/// the close action trigger.
///
/// # Arguments
/// * `id` - Element ID for the backdrop
/// * `opacity` - Modal animation opacity (0.0 to 1.0)
/// * `cx` - GPUI context
/// * `on_backdrop_mouse_down` - Listener for backdrop mouse down (should set flag)
/// * `on_backdrop_mouse_up` - Listener for backdrop mouse up (should check flag and close)
/// * `child` - The modal content element
pub fn render_modal_backdrop(
    id: impl Into<ElementId>,
    opacity: f32,
    cx: &mut Context<Humanboard>,
    on_backdrop_mouse_down: impl Fn(&mut Humanboard, &MouseDownEvent, &mut Window, &mut Context<Humanboard>) + 'static,
    on_backdrop_mouse_up: impl Fn(&mut Humanboard, &MouseUpEvent, &mut Window, &mut Context<Humanboard>) + 'static,
    child: impl IntoElement,
) -> impl IntoElement {
    deferred(
        div()
            .id(id)
            .absolute()
            .top_0()
            .left_0()
            .size_full()
            .bg(hsla(0.0, 0.0, 0.0, MODAL_BACKDROP_OPACITY * opacity))
            .flex()
            .items_center()
            .justify_center()
            .on_mouse_down(MouseButton::Left, cx.listener(on_backdrop_mouse_down))
            .on_mouse_up(MouseButton::Left, cx.listener(on_backdrop_mouse_up))
            .on_scroll_wheel(cx.listener(|_, _, _, _| {}))
            .child(child),
    )
    .with_priority(1500)
}

/// Adds mouse event handlers to prevent backdrop close when clicking on modal content.
///
/// This overload works with `Stateful<Div>` (returned by `.track_focus()`).
///
/// # Arguments
/// * `div` - The Stateful div element to add handlers to
/// * `cx` - GPUI context
/// * `on_mouse_down` - Listener for mouse down on modal content
/// * `on_mouse_up` - Listener for mouse up on modal content
pub fn modal_intercept_backdrop_clicks_stateful(
    div: Stateful<Div>,
    cx: &mut Context<Humanboard>,
    on_mouse_down: impl Fn(&mut Humanboard, &MouseDownEvent, &mut Window, &mut Context<Humanboard>) + 'static,
    on_mouse_up: impl Fn(&mut Humanboard, &MouseUpEvent, &mut Window, &mut Context<Humanboard>) + 'static,
) -> Stateful<Div> {
    div.on_mouse_down(MouseButton::Left, cx.listener(on_mouse_down))
        .on_mouse_up(MouseButton::Left, cx.listener(on_mouse_up))
}

// ============================================================================
// Legacy Helpers (for backward compatibility)
// ============================================================================

/// Render a keyboard key badge
pub fn render_kbd(key: &str, cx: &Context<Humanboard>) -> Div {
    let muted = cx.theme().muted;
    let border = cx.theme().border;
    let muted_fg = cx.theme().muted_foreground;

    div()
        .px(px(8.0))
        .py(px(4.0))
        .bg(muted)
        .border_1()
        .border_color(border)
        .rounded(px(6.0))
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .text_color(muted_fg)
        .child(key.to_string())
}

/// Render a shortcut row with key and description
pub fn render_shortcut_row(key: &str, description: &str, cx: &Context<Humanboard>) -> Div {
    let fg = cx.theme().foreground;

    h_flex()
        .h(px(28.0))
        .items_center()
        .justify_between()
        .child(
            div()
                .text_sm()
                .text_color(fg)
                .child(description.to_string()),
        )
        .child(render_kbd(key, cx))
}

/// Render a section of shortcuts with title
pub fn render_shortcut_section(
    title: &str,
    shortcuts: Vec<(&str, &str)>,
    cx: &Context<Humanboard>,
) -> Div {
    let muted_fg = cx.theme().muted_foreground;

    let mut section = v_flex().gap_1().child(
        div()
            .text_xs()
            .font_weight(FontWeight::BOLD)
            .text_color(muted_fg)
            .mb_1()
            .child(title.to_string().to_uppercase()),
    );

    for (key, desc) in shortcuts {
        section = section.child(render_shortcut_row(key, desc, cx));
    }

    section
}

/// Render a setting row with title, description, and control on the right
pub fn render_setting_row(
    title: &str,
    description: &str,
    control: impl IntoElement,
    cx: &Context<Humanboard>,
) -> Div {
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;

    h_flex()
        .w_full()
        .py_3()
        .items_center()
        .justify_between()
        .gap_4()
        .child(
            v_flex()
                .flex_1()
                .min_w_0()
                .gap(px(2.0))
                .child(div().text_sm().text_color(fg).child(title.to_string()))
                .child(
                    div()
                        .text_xs()
                        .text_color(muted_fg)
                        .child(description.to_string()),
                ),
        )
        .child(div().flex_shrink_0().child(control))
}

/// Render a section header
pub fn render_section_header(title: &str, cx: &Context<Humanboard>) -> Div {
    let muted_fg = cx.theme().muted_foreground;
    let border = cx.theme().border;

    div()
        .w_full()
        .pb_2()
        .mb_2()
        .border_b_1()
        .border_color(border)
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(muted_fg)
                .child(title.to_string().to_uppercase()),
        )
}

/// Global marker for which settings dropdown is open
#[derive(Clone, PartialEq)]
pub enum SettingsDropdown {
    Theme,
    Font,
}

impl gpui::Global for SettingsDropdown {}

/// Marker for theme dropdown being open
#[derive(Clone)]
pub struct ThemeDropdownOpen;

impl gpui::Global for ThemeDropdownOpen {}

/// Marker for font dropdown being open
#[derive(Clone)]
pub struct FontDropdownOpen;

impl gpui::Global for FontDropdownOpen {}
