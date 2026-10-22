use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::MadsquashApp;
use crate::model::{AppItem, format_bytes, mono};

pub fn render_app_card(
    app: AppItem,
    is_selected: bool,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement + use<> {
    let app_id = app.id.clone();
    let monogram = mono(&app.name);

    div()
        .id(format!("card-{}", app.id))
        .w(px(200.))
        .h(px(144.))
        .p_3p5()
        .rounded_xl()
        .border_1()
        .border_color(if is_selected {
            rgb(0x0fc5fc)
        } else {
            rgb(0x3a342c)
        })
        .bg(if is_selected {
            rgb(0x0e2532)
        } else {
            rgb(0x221f1b)
        })
        .hover(|s| {
            if !is_selected {
                s.border_color(rgb(0x5c5146))
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
        .child(if let Some(ref icon_path) = app.icon_path {
            div()
                .size(px(36.))
                .rounded_lg()
                .bg(rgb(0x141210))
                .p(px(2.))
                .flex()
                .items_center()
                .justify_center()
                .child(gpui::img(icon_path.clone()).size(px(32.)))
        } else {
            div()
                .size(px(36.))
                .rounded_lg()
                .bg(rgb(0x141210))
                .text_color(rgb(0x0fc5fc))
                .font_weight(FontWeight::BOLD)
                .text_sm()
                .flex()
                .items_center()
                .justify_center()
                .child(monogram)
        })
        .child(
            div()
                .flex()
                .flex_col()
                .gap(px(2.))
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0xf4efe8))
                        .child(app.name),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0xa3988c))
                        .child(format!("{} · {}", app.version, format_bytes(app.size))),
                ),
        )
}

pub fn render_list_pane(
    visible: Vec<AppItem>,
    selected_id: Option<&str>,
    query_is_empty: bool,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .flex_1()
        .h_full()
        .flex()
        .flex_col()
        .px_5()
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
                        .gap_3()
                        .pb_4()
                        .when(visible.is_empty(), |this| {
                            this.child(
                                div()
                                    .text_sm()
                                    .text_color(rgb(0xa3988c))
                                    .py_8()
                                    .child(if query_is_empty {
                                        "Belum ada AppImage di slot."
                                    } else {
                                        "Tidak ada yang cocok."
                                    }),
                            )
                        })
                        .children(visible.into_iter().map(|app| {
                            let is_selected = selected_id == Some(app.id.as_str());
                            render_app_card(app, is_selected, cx)
                        })),
                ),
        )
        // Reset button
        .child(
            div().pt_2().child(
                div()
                    .id("reset-btn")
                    .cursor_pointer()
                    .text_xs()
                    .text_color(rgb(0xa3988c))
                    .hover(|s| s.text_color(rgb(0xf4efe8)))
                    .child("Kembalikan contoh")
                    .on_click(cx.listener(|this, _, window, cx| {
                        this.reset_seed(window, cx);
                    })),
            ),
        )
}
