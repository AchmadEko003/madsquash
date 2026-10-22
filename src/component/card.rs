use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::MadsquashApp;
use crate::model::{AppItem, format_bytes, mono};

pub fn render_app_card(
    app: &AppItem,
    is_selected: bool,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement + use<> {
    let app_id = app.id.clone();
    let app_name = app.name.clone();
    let monogram = mono(&app.name);

    let ver_label = if app.version != "—" && !app.version.starts_with('v') && !app.version.starts_with('V') {
        format!("v{}", app.version)
    } else {
        app.version.clone()
    };

    div()
        .id(format!("card-{}", app.id))
        .w(px(216.))
        .h(px(150.))
        .p_4()
        .rounded_2xl()
        .border_1()
        .border_color(if is_selected {
            rgb(0x5bbfe6)
        } else {
            rgb(0x222a36)
        })
        .bg(if is_selected {
            rgb(0x162533)
        } else {
            rgb(0x171c24)
        })
        .hover(|s| {
            if !is_selected {
                s.bg(rgb(0x1c222c)).border_color(rgb(0x2e3a4b))
            } else {
                s
            }
        })
        .cursor_pointer()
        .flex()
        .flex_col()
        .justify_between()
        .on_click(cx.listener(move |this, _, _, cx| {
            this.select_app(app_id.clone(), cx);
        }))
        // Top row: Icon on left, version pill on right
        .child(
            div()
                .flex()
                .items_start()
                .justify_between()
                .child(if let Some(ref icon_path) = app.icon_path {
                    div()
                        .size(px(44.))
                        .rounded_xl()
                        .bg(rgb(0x1b222c))
                        .border_1()
                        .border_color(if is_selected {
                            rgb(0x5bbfe6)
                        } else {
                            rgb(0x283240)
                        })
                        .p(px(3.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(gpui::img(icon_path.clone()).size(px(36.)))
                } else {
                    div()
                        .size(px(44.))
                        .rounded_xl()
                        .bg(rgb(0x1b222c))
                        .border_1()
                        .border_color(if is_selected {
                            rgb(0x5bbfe6)
                        } else {
                            rgb(0x283240)
                        })
                        .text_color(rgb(0x7bc7e8))
                        .font_weight(FontWeight::BOLD)
                        .text_sm()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(monogram)
                })
                .child(
                    div()
                        .px_2()
                        .py_0p5()
                        .rounded_full()
                        .bg(if is_selected {
                            rgb(0x1d3345)
                        } else {
                            rgb(0x1c2430)
                        })
                        .border_1()
                        .border_color(if is_selected {
                            rgb(0x5bbfe6)
                        } else {
                            rgb(0x2a384a)
                        })
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(if is_selected {
                            rgb(0x7bc7e8)
                        } else {
                            rgb(0x94a3b8)
                        })
                        .child(ver_label),
                ),
        )
        // Bottom: App name and size/arch info
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xe8edf4))
                        .child(app_name),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x8a99ad))
                        .child(format!("{} · {}", format_bytes(app.size), app.arch)),
                ),
        )
}

pub fn render_list_pane(
    visible: &[&AppItem],
    selected_id: Option<&str>,
    query_is_empty: bool,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
        .px_6()
        .pt_4()
        .pb_4()
        .overflow_hidden()
        // Scrollable Cards
        .child(
            div()
                .id("cards-scroll-container")
                .flex_1()
                .overflow_y_scroll()
                .child(
                    div()
                        .flex()
                        .flex_wrap()
                        .gap_3p5()
                        .pb_4()
                        .when(visible.is_empty(), |this| {
                            this.child(
                                div()
                                    .w_full()
                                    .py_12()
                                    .flex()
                                    .flex_col()
                                    .items_center()
                                    .justify_center()
                                    .gap_3()
                                    .child(
                                        div()
                                            .size(px(48.))
                                            .rounded_2xl()
                                            .border_2()
                                            .border_dashed()
                                            .border_color(rgb(0x252e3c))
                                            .flex()
                                            .items_center()
                                            .justify_center()
                                            .text_color(rgb(0x64748b))
                                            .child("✦"),
                                    )
                                    .child(
                                        div()
                                            .text_sm()
                                            .font_weight(FontWeight::MEDIUM)
                                            .text_color(rgb(0x94a3b8))
                                            .child(if query_is_empty {
                                                "Belum ada AppImage terpasang di slot."
                                            } else {
                                                "Tidak ada aplikasi yang cocok dengan pencarian."
                                            }),
                                    ),
                            )
                        })
                        .children(visible.iter().map(|app| {
                            let is_selected = selected_id == Some(app.id.as_str());
                            render_app_card(app, is_selected, cx)
                        })),
                ),
        )
}
