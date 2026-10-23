use gpui_kit::*;

use crate::MadsquashApp;
use crate::updater::CURRENT_VERSION;

pub fn render_brand_mark() -> impl IntoElement {
    div()
        .size(px(36.))
        .rounded_xl()
        .bg(rgb(0x182735))
        .border_2()
        .border_color(rgb(0x5bbfe6))
        .p(px(6.))
        .flex()
        .flex_col()
        .justify_between()
        .items_center()
        .child(div().w(px(20.)).h(px(2.5)).bg(rgb(0x5bbfe6)).rounded_full())
        .child(div().w(px(14.)).h(px(2.5)).bg(rgb(0x5bbfe6)).rounded_full())
        .child(div().w(px(8.)).h(px(2.5)).bg(rgb(0x5bbfe6)).rounded_full())
}

pub fn render_rail(
    has_update: bool,
    update_tag: Option<&str>,
    is_checking: bool,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    let (dot_color, label_text, is_clickable_update) = if has_update {
        let tag = update_tag.unwrap_or("Baru");
        (rgb(0x5bbfe6), format!("⬆ {}", tag), true)
    } else if is_checking {
        (rgb(0xfbbf24), "Cek...".to_string(), false)
    } else {
        (rgb(0x6ee7b7), format!("v{}", CURRENT_VERSION), false)
    };

    div()
        .w(px(68.))
        .h_full()
        .bg(rgb(0x101217))
        .flex()
        .flex_col()
        .items_center()
        .justify_between()
        .py_5()
        .border_r_1()
        .border_color(rgb(0x202630))
        // Top section: Logo brand mark
        .child(
            div()
                .flex()
                .flex_col()
                .items_center()
                .gap_3()
                .child(render_brand_mark()),
        )
        // Bottom section: Version / Update button
        .child(
            div()
                .id("rail-update-badge")
                .flex()
                .flex_col()
                .items_center()
                .gap_1()
                .p_1()
                .rounded_lg()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(0x171c24)))
                .on_click(cx.listener(move |this, _, _, cx| {
                    if is_clickable_update {
                        this.open_update_dialog(cx);
                    } else if !this.is_checking_update {
                        this.check_for_updates(false, cx);
                    }
                }))
                .child(
                    div()
                        .size(px(8.))
                        .rounded_full()
                        .bg(dot_color)
                        .shadow_sm(),
                )
                .child(
                    div()
                        .text_xs()
                        .font_weight(if has_update { FontWeight::BOLD } else { FontWeight::MEDIUM })
                        .text_color(if has_update { rgb(0x5bbfe6) } else { rgb(0x64748b) })
                        .child(label_text),
                ),
        )
}
