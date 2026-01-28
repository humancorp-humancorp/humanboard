//! Settings modal component.

use crate::actions::{ModalFocusNext, ModalFocusPrev, OpenSettings};
use crate::app::{Humanboard, SettingsTab};
use crate::constants::{MODAL_HEIGHT_MD, MODAL_WIDTH_LG};
use crate::focus::FocusContext;
use crate::settings::Settings;
use gpui::prelude::FluentBuilder;
use gpui::*;
use gpui_component::{h_flex, v_flex, ActiveTheme as _, Icon, IconName};

use super::modal_base::{
    modal_intercept_backdrop_clicks_stateful, render_modal_backdrop, render_section_header,
    render_setting_row, FontDropdownOpen, ThemeDropdownOpen,
};
use super::settings_dropdowns::{
    render_font_dropdown, render_font_dropdown_menu, render_theme_dropdown,
    render_theme_dropdown_menu,
};

/// Render a settings sidebar tab button
fn render_settings_tab_button(
    id: impl Into<ElementId>,
    tab: SettingsTab,
    active_tab: SettingsTab,
    icon: IconName,
    label: &str,
    fg: Hsla,
    muted_fg: Hsla,
    list_active: Hsla,
    list_hover: Hsla,
    cx: &mut Context<Humanboard>,
) -> Stateful<Div> {
    let is_active = active_tab == tab;
    let text_color = if is_active { fg } else { muted_fg };
    
    div()
        .id(id)
        .w_full()
        .px_2()
        .py_1p5()
        .rounded(px(4.0))
        .cursor(CursorStyle::PointingHand)
        .when(is_active, |d| d.bg(list_active))
        .when(!is_active, |d| d.hover(|s| s.bg(list_hover)))
        .on_mouse_down(
            MouseButton::Left,
            cx.listener(move |this, _, _, cx| {
                this.set_settings_tab(tab, cx);
            }),
        )
        .child(
            h_flex()
                .gap_2()
                .items_center()
                .child(
                    Icon::new(icon)
                        .size(px(14.0))
                        .text_color(text_color),
                )
                .child(
                    div()
                        .text_sm()
                        .text_color(text_color)
                        .child(label.to_string()),
                ),
        )
}

/// Render the settings modal
pub fn render_settings_modal(
    current_theme: &str,
    current_font: &str,
    _theme_index: usize,
    _theme_scroll: &ScrollHandle,
    active_tab: SettingsTab,
    modal_focus: &FocusHandle,
    opacity: f32,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    let themes = Settings::available_themes(cx);
    let fonts = Settings::available_fonts();
    let current_theme_display = current_theme.to_string();
    let current_font_display = current_font.to_string();

    let bg = cx.theme().background;
    let border = cx.theme().border;
    let fg = cx.theme().foreground;
    let muted_fg = cx.theme().muted_foreground;
    let title_bar = cx.theme().title_bar;
    let list_active = cx.theme().list_active;
    let list_hover = cx.theme().list_hover;
    let input_bg = cx.theme().secondary;

    // Build sidebar first (doesn't capture cx in closures)
    let sidebar = render_settings_sidebar(
        active_tab, title_bar, border, fg, muted_fg, list_active, list_hover, cx,
    );
    
    // Create action handlers before building content
    let on_toggle_settings = cx.listener(|this, _: &OpenSettings, window, cx| {
        this.toggle_settings(window, cx);
    });
    let on_focus_next = cx.listener(|this, _: &ModalFocusNext, window, cx| {
        this.modal_focus_next(window, cx);
    });
    let on_focus_prev = cx.listener(|this, _: &ModalFocusPrev, window, cx| {
        this.modal_focus_prev(window, cx);
    });
    
    // Build content after creating handlers
    let content = render_settings_content(
        active_tab,
        &current_theme_display,
        &current_font_display,
        &themes,
        &fonts,
        bg,
        border,
        fg,
        muted_fg,
        input_bg,
        list_active,
        list_hover,
        cx,
    );
    
    // Build modal with backdrop click handlers
    let modal_with_handlers = modal_intercept_backdrop_clicks_stateful(
        h_flex()
            .id("settings-modal")
            .track_focus(modal_focus)
            .key_context(FocusContext::KEY_MODAL)
            .w(px(MODAL_WIDTH_LG))
            .h(px(MODAL_HEIGHT_MD))
            .bg(bg.opacity(opacity))
            .border_1()
            .border_color(border.opacity(opacity))
            .rounded(px(10.0))
            .overflow_hidden()
            .shadow_lg()
            .on_scroll_wheel(|_, _, _| {})
            .on_action(on_toggle_settings)
            .on_action(on_focus_next)
            .on_action(on_focus_prev)
            .child(sidebar)
            .child(content),
        cx,
        // Modal mouse down: reset flag
        |this, _, _, _| {
            this.settings.backdrop_clicked = false;
        },
        // Modal mouse up: reset flag
        |this, _, _, _| {
            this.settings.backdrop_clicked = false;
        },
    );
    
    render_modal_backdrop(
        "settings-backdrop",
        opacity,
        cx,
        // Backdrop mouse down: set flag
        |this, _, _, cx| {
            this.settings.backdrop_clicked = true;
            cx.notify();
        },
        // Backdrop mouse up: close if flag is set
        |this, _, window, cx| {
            if this.settings.backdrop_clicked {
                this.settings.show = false;
                this.system.focus.force_canvas_focus(window);
            }
            this.settings.backdrop_clicked = false;
            cx.notify();
        },
        modal_with_handlers,
    )
}

fn render_settings_sidebar(
    active_tab: SettingsTab,
    title_bar: Hsla,
    border: Hsla,
    fg: Hsla,
    muted_fg: Hsla,
    list_active: Hsla,
    list_hover: Hsla,
    cx: &mut Context<Humanboard>,
) -> Div {
    v_flex()
        .w(px(180.0))
        .h_full()
        .bg(title_bar)
        .border_r_1()
        .border_color(border)
        .rounded_l(px(10.0))
        .p_2()
        .gap_1()
        // Appearance tab
        .child(render_settings_tab_button(
            "tab-appearance",
            SettingsTab::Appearance,
            active_tab,
            IconName::Palette,
            "Appearance",
            fg,
            muted_fg,
            list_active,
            list_hover,
            cx,
        ))
        // Integrations tab
        .child(render_settings_tab_button(
            "tab-integrations",
            SettingsTab::Integrations,
            active_tab,
            IconName::Settings,
            "Integrations",
            fg,
            muted_fg,
            list_active,
            list_hover,
            cx,
        ))
}

#[allow(clippy::too_many_arguments)]
fn render_settings_content(
    active_tab: SettingsTab,
    current_theme: &str,
    current_font: &str,
    themes: &[String],
    fonts: &[&str],
    bg: Hsla,
    border: Hsla,
    fg: Hsla,
    muted_fg: Hsla,
    input_bg: Hsla,
    list_active: Hsla,
    list_hover: Hsla,
    cx: &mut Context<Humanboard>,
) -> impl IntoElement {
    let current_theme_clone = current_theme.to_string();
    let current_font_clone = current_font.to_string();
    let themes_clone = themes.to_vec();
    let fonts_clone: Vec<String> = fonts.iter().map(|s| s.to_string()).collect();

    v_flex()
        .id("settings-content")
        .flex_1()
        .h_full()
        .overflow_hidden()
        .px_6()
        .py_6()
        // Content - Appearance tab
        .when(active_tab == SettingsTab::Appearance, |d| {
            d.child(
                v_flex()
                    .gap_4()
                    .child(render_section_header("Theme", cx))
                    .child(render_setting_row(
                        "Theme",
                        "Choose a color theme for the interface",
                        render_theme_dropdown(
                            &current_theme_clone, fg, muted_fg, input_bg, border, cx,
                        ),
                        cx,
                    ))
                    .child(render_section_header("Font", cx))
                    .child(render_setting_row(
                        "Font Family",
                        "Choose a font for the interface",
                        render_font_dropdown(
                            &current_font_clone, fg, muted_fg, input_bg, border, cx,
                        ),
                        cx,
                    )),
            )
        })
        // Content - Integrations tab
        .when(active_tab == SettingsTab::Integrations, |d| {
            d.child(
                v_flex().gap_4().child(
                    div()
                        .py_8()
                        .text_color(muted_fg)
                        .text_sm()
                        .child("No integrations available yet."),
                ),
            )
        })
        // Theme dropdown menu
        .when(cx.try_global::<ThemeDropdownOpen>().is_some(), |d| {
            d.child(render_theme_dropdown_menu(
                &themes_clone,
                &current_theme_clone,
                bg,
                border,
                fg,
                muted_fg,
                list_active,
                list_hover,
                cx,
            ))
        })
        // Font dropdown menu
        .when(cx.try_global::<FontDropdownOpen>().is_some(), |d| {
            d.child(render_font_dropdown_menu(
                &fonts_clone,
                &current_font_clone,
                bg,
                border,
                fg,
                muted_fg,
                list_active,
                list_hover,
                cx,
            ))
        })
}
