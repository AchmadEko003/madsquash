use gpui_kit::component::IconName;
use gpui_kit::component::input::{Input, InputState};
use gpui_kit::*;

use crate::MadsquashApp;

pub fn render_search_bar(
    search_input: &Entity<InputState>,
    count_text: String,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .w_full()
        .h(px(68.))
        .px_6()
        .py_3()
        .bg(rgb(0x13161c))
        .border_b_1()
        .border_color(rgb(0x202630))
        .flex()
        .items_center()
        .gap_4()
        // Search box (stretches across full available width)
        .child(
            div()
                .flex_1()
                .w_full()
                .min_w_0()
                .child(
                    Input::new(search_input)
                        .w_full()
                        .prefix(IconName::Search)
                        .cleanable(true)
                        .suffix(
                            div()
                                .text_xs()
                                .text_color(rgb(0x64748b))
                                .border_1()
                                .border_color(rgb(0x252e3c))
                                .rounded_sm()
                                .px_1p5()
                                .child("/"),
                        ),
                ),
        )
        // Count Pill
        .child(
            div()
                .px_3()
                .py_1p5()
                .rounded_full()
                .bg(rgb(0x182430))
                .border_1()
                .border_color(rgb(0x263a4d))
                .flex()
                .items_center()
                .gap_2()
                .child(
                    div()
                        .size(px(6.))
                        .rounded_full()
                        .bg(rgb(0x5bbfe6)),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x7bc7e8))
                        .child(count_text),
                ),
        )
        // "+ Pasang AppImage" Primary Button (Soft Pastel Sky)
        .child(
            div()
                .id("top-install-btn")
                .px_4()
                .py_2()
                .rounded_xl()
                .bg(rgb(0x5bbfe6))
                .text_color(rgb(0x0f202b))
                .font_weight(FontWeight::BOLD)
                .text_sm()
                .flex()
                .items_center()
                .gap_1p5()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(0x6ec8ed)))
                .active(|s| s.bg(rgb(0x4baecf)))
                .child(
                    div()
                        .text_base()
                        .line_height(px(16.))
                        .child("+"),
                )
                .child(div().child("Pasang"))
                .on_click(cx.listener(|this, _, _, cx| {
                    this.open_install(cx);
                })),
        )
}
