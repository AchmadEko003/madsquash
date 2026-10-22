use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::MadsquashApp;
use crate::model::{AppItem, format_bytes, mono, short_hash};

pub fn render_slot_icon() -> impl IntoElement {
    div()
        .size(px(48.))
        .rounded_xl()
        .border_2()
        .border_dashed()
        .border_color(rgb(0x0fc5fc))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .size(px(20.))
                .rounded_md()
                .border_2()
                .border_color(rgb(0x0fc5fc))
                .bg(rgb(0x102f3d)),
        )
}

pub fn render_detail_container(
    selected: Option<AppItem>,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .w(px(350.))
        .h_full()
        .mr_5()
        .mb_4()
        .bg(rgb(0x181714))
        .border_1()
        .border_color(rgb(0x3a342c))
        .rounded_2xl()
        .overflow_hidden()
        .child(
            div()
                .id("detail-scroll")
                .size_full()
                .p_5()
                .overflow_y_scroll()
                .child(match selected {
                    Some(app) => render_detail_pane(&app, cx).into_any_element(),
                    None => render_empty_detail_pane(cx).into_any_element(),
                }),
        )
}

pub fn render_detail_pane(app: &AppItem, cx: &mut Context<MadsquashApp>) -> impl IntoElement {
    let app_clone = app.clone();
    let app_name = app.name.clone();

    let app_type_display = if app.app_type.starts_with("AppImage") {
        app.app_type.clone()
    } else {
        format!("AppImage {}", app.app_type)
    };

    div()
        .flex()
        .flex_col()
        .gap_4()
        // Header
        .child(
            div()
                .flex()
                .items_end()
                .gap_3p5()
                .child(if let Some(ref icon_path) = app.icon_path {
                    div()
                        .size(px(56.))
                        .rounded_xl()
                        .bg(rgb(0x141210))
                        .p(px(4.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(gpui::img(icon_path.clone()).size(px(48.)))
                } else {
                    div()
                        .size(px(56.))
                        .rounded_xl()
                        .bg(rgb(0x141210))
                        .text_color(rgb(0x0fc5fc))
                        .font_weight(FontWeight::BOLD)
                        .text_xl()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(mono(&app.name))
                })
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .child(
                            div()
                                .text_2xl()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xf4efe8))
                                .child(app.name.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0xa3988c))
                                .child(app.comment.clone()),
                        ),
                ),
        )
        // Actions ("Jalankan" & "Lepas")
        .child(
            div()
                .flex()
                .gap_2()
                .child(
                    div()
                        .id("run-btn")
                        .px_4()
                        .py_2()
                        .rounded_lg()
                        .bg(rgb(0x0fc5fc))
                        .text_color(rgb(0x07151a))
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_sm()
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(0x3bd4fd)))
                        .active(|s| s.bg(rgb(0x09a5d4)))
                        .child("Jalankan")
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.show_toast(format!("Membuka {}", app_name), cx);
                        })),
                )
                .child(
                    div()
                        .id("remove-btn")
                        .px_4()
                        .py_2()
                        .rounded_lg()
                        .border_1()
                        .border_color(rgb(0x3a342c))
                        .text_color(rgb(0xf4efe8))
                        .font_weight(FontWeight::MEDIUM)
                        .text_sm()
                        .cursor_pointer()
                        .hover(|s| s.bg(rgba(0xffffff0f)))
                        .child("Lepas")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.open_remove(cx);
                        })),
                ),
        )
        // Chips
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_1p5()
                .child(render_chip("Di menu"))
                .child(render_chip("Ikon"))
                .child(render_chip("Dapat dijalankan")),
        )
        // Meta definition table
        .child(
            div()
                .flex()
                .flex_col()
                .border_t_1()
                .border_color(rgb(0x3a342c))
                .pt_2()
                // 2 columns row 1
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Versi", &app_clone.version))
                        .child(render_meta_col("Ukuran", &format_bytes(app_clone.size))),
                )
                // 2 columns row 2
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Arsitektur", &app_clone.arch))
                        .child(render_meta_col("Tipe", &app_type_display)),
                )
                // 2 columns row 3
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Dipasang", &app_clone.installed_at))
                        .child(render_meta_col("SHA256", &short_hash(&app_clone.sha256))),
                )
                .when_some(app_clone.categories.as_ref(), |this, cat| {
                    this.child(render_meta_wide("Kategori", cat))
                })
                .when_some(app_clone.exec.as_ref(), |this, exec| {
                    this.child(render_meta_wide("Perintah", exec))
                })
                // Wide row 1
                .child(render_meta_wide(
                    "Lokasi",
                    &format!("~/.local/share/madsquash/apps/{}/app.AppImage", app_clone.id),
                ))
                // Wide row 2
                .child(render_meta_wide(
                    "Entri menu",
                    &format!("~/.local/share/applications/madsquash-{}.desktop", app_clone.id),
                )),
        )
        // Footer Note
        .child(
            div()
                .text_xs()
                .text_color(rgb(0xa3988c))
                .child("Lepas hanya menghapus salinan di slot, entri menu, dan ikon. Berkas unduhan asli tidak disentuh."),
        )
}

pub fn render_empty_detail_pane(cx: &mut Context<MadsquashApp>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_start()
        .gap_3()
        .py_6()
        .child(render_slot_icon())
        .child(
            div()
                .text_xl()
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xf4efe8))
                .child("Slot masih kosong"),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(0xa3988c))
                .child("Jatuhkan berkas .AppImage ke jendela ini, atau pilih berkas untuk mendaftarkannya ke menu."),
        )
        .child(
            div()
                .id("empty-choose-btn")
                .mt_2()
                .px_4()
                .py_2()
                .rounded_lg()
                .bg(rgb(0x0fc5fc))
                .text_color(rgb(0x07151a))
                .font_weight(FontWeight::SEMIBOLD)
                .text_sm()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(0x3bd4fd)))
                .child("Pilih berkas")
                .on_click(cx.listener(|this, _, _, cx| {
                    this.open_install(cx);
                })),
        )
}

pub fn render_chip(text: &'static str) -> impl IntoElement {
    div()
        .px_2p5()
        .py_1()
        .rounded_full()
        .bg(rgb(0x10303e))
        .text_color(rgb(0x0fc5fc))
        .text_xs()
        .font_weight(FontWeight::MEDIUM)
        .child(text)
}

pub fn render_meta_col(label: &'static str, value: &str) -> impl IntoElement {
    div()
        .flex_1()
        .py_2()
        .border_b_1()
        .border_color(rgb(0x3a342c))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(0xa3988c))
                .child(label),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(0xf4efe8))
                .mt(px(2.))
                .child(value.to_string()),
        )
}

pub fn render_meta_wide(label: &'static str, value: &str) -> impl IntoElement {
    div()
        .w_full()
        .py_2()
        .border_b_1()
        .border_color(rgb(0x3a342c))
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(0xa3988c))
                .child(label),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(0xf4efe8))
                .mt(px(2.))
                .child(value.to_string()),
        )
}
