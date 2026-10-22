use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::MadsquashApp;
use crate::model::{AppItem, format_bytes, mono, short_hash};

pub fn render_slot_icon() -> impl IntoElement {
    div()
        .size(px(56.))
        .rounded_2xl()
        .bg(rgb(0x0e1b27))
        .border_2()
        .border_dashed()
        .border_color(rgb(0x0fc5fc))
        .flex()
        .items_center()
        .justify_center()
        .child(
            div()
                .size(px(24.))
                .rounded_lg()
                .border_2()
                .border_color(rgb(0x0fc5fc))
                .bg(rgb(0x13384c)),
        )
}

pub fn render_detail_container(
    selected: Option<AppItem>,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .w(px(360.))
        .h_full()
        .mr_6()
        .mb_4()
        .bg(rgb(0x0e141d))
        .border_1()
        .border_color(rgb(0x1a2434))
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
    let app_id = app.id.clone();
    let app_id_folder = app.id.clone();

    let app_type_display = if app.app_type.starts_with("AppImage") {
        app.app_type.clone()
    } else {
        format!("AppImage {}", app.app_type)
    };

    let ver_label = if app.version != "—" && !app.version.starts_with('v') && !app.version.starts_with('V') {
        format!("v{}", app.version)
    } else {
        app.version.clone()
    };

    div()
        .flex()
        .flex_col()
        .gap_4()
        // Header
        .child(
            div()
                .flex()
                .items_start()
                .gap_3p5()
                .child(if let Some(ref icon_path) = app.icon_path {
                    div()
                        .size(px(58.))
                        .rounded_2xl()
                        .bg(rgb(0x141c28))
                        .border_1()
                        .border_color(rgb(0x202d40))
                        .p(px(4.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(gpui::img(icon_path.clone()).size(px(50.)))
                } else {
                    div()
                        .size(px(58.))
                        .rounded_2xl()
                        .bg(rgb(0x141c28))
                        .border_1()
                        .border_color(rgb(0x202d40))
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
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap(px(2.))
                        .child(
                            div()
                                .text_xl()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xf0f6fc))
                                .child(app.name.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x7e8fa6))
                                .child(app.comment.clone()),
                        )
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_2()
                                .mt_1()
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(rgb(0x0c251d))
                                        .border_1()
                                        .border_color(rgb(0x10b981))
                                        .flex()
                                        .items_center()
                                        .gap_1()
                                        .child(div().size(px(5.)).rounded_full().bg(rgb(0x10b981)))
                                        .child(
                                            div()
                                                .text_xs()
                                                .font_weight(FontWeight::SEMIBOLD)
                                                .text_color(rgb(0x10b981))
                                                .child("Terpasang"),
                                        ),
                                )
                                .child(
                                    div()
                                        .px_2()
                                        .py_0p5()
                                        .rounded_full()
                                        .bg(rgb(0x111e2c))
                                        .border_1()
                                        .border_color(rgb(0x1c374d))
                                        .text_xs()
                                        .font_weight(FontWeight::SEMIBOLD)
                                        .text_color(rgb(0x0fc5fc))
                                        .child(ver_label),
                                ),
                        ),
                ),
        )
        // Primary Action Button: "Jalankan Aplikasi"
        .child(
            div()
                .id("run-btn")
                .w_full()
                .py_2p5()
                .rounded_xl()
                .bg(rgb(0x0fc5fc))
                .text_color(rgb(0x06151c))
                .font_weight(FontWeight::BOLD)
                .text_sm()
                .flex()
                .items_center()
                .justify_center()
                .gap_2()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(0x38d3ff)))
                .active(|s| s.bg(rgb(0x08a6d8)))
                .child(
                    div()
                        .text_xs()
                        .child("▶"),
                )
                .child(div().child("Jalankan Aplikasi"))
                .on_click(cx.listener(move |this, _, _, cx| {
                    this.launch_app(&app_id, cx);
                })),
        )
        // Secondary Actions: "Buka Folder" & "Lepas"
        .child(
            div()
                .flex()
                .gap_2()
                .child(
                    div()
                        .id("open-folder-btn")
                        .flex_1()
                        .py_2()
                        .rounded_xl()
                        .border_1()
                        .border_color(rgb(0x1f2c3e))
                        .bg(rgb(0x111925))
                        .text_color(rgb(0xf0f6fc))
                        .font_weight(FontWeight::MEDIUM)
                        .text_xs()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap_1p5()
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(0x162334)).border_color(rgb(0x0fc5fc)))
                        .child(div().child("Buka Folder"))
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.open_app_folder(&app_id_folder, cx);
                        })),
                )
                .child(
                    div()
                        .id("remove-btn")
                        .flex_1()
                        .py_2()
                        .rounded_xl()
                        .border_1()
                        .border_color(rgb(0x1f2c3e))
                        .bg(rgb(0x111925))
                        .text_color(rgb(0xf0f6fc))
                        .font_weight(FontWeight::MEDIUM)
                        .text_xs()
                        .flex()
                        .items_center()
                        .justify_center()
                        .gap_1p5()
                        .cursor_pointer()
                        .hover(|s| s.bg(rgb(0x291417)).text_color(rgb(0xf87171)).border_color(rgb(0x7f1d1d)))
                        .child(div().child("Lepas"))
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.open_remove(cx);
                        })),
                ),
        )
        // Feature Chips
        .child(
            div()
                .flex()
                .flex_wrap()
                .gap_1p5()
                .child(render_chip("Di Menu OS"))
                .child(render_chip("Ikon Sistem"))
                .child(render_chip("Executable")),
        )
        // Card Group 1: Informasi Paket
        .child(
            div()
                .rounded_xl()
                .bg(rgb(0x111925))
                .border_1()
                .border_color(rgb(0x1a2636))
                .p_3()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x0fc5fc))
                        .pb_1()
                        .child("Informasi Paket"),
                )
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Versi", &app_clone.version))
                        .child(render_meta_col("Ukuran", &format_bytes(app_clone.size))),
                )
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Arsitektur", &app_clone.arch))
                        .child(render_meta_col("Format", &app_type_display)),
                )
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Dipasang", &app_clone.installed_at))
                        .child(render_meta_col("SHA256", &short_hash(&app_clone.sha256))),
                ),
        )
        // Card Group 2: Integrasi Sistem
        .child(
            div()
                .rounded_xl()
                .bg(rgb(0x111925))
                .border_1()
                .border_color(rgb(0x1a2636))
                .p_3()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .text_xs()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0x0fc5fc))
                        .pb_1()
                        .child("Integrasi Sistem"),
                )
                .when_some(app_clone.categories.as_ref(), |this, cat| {
                    this.child(render_meta_wide("Kategori", cat))
                })
                .when_some(app_clone.exec.as_ref(), |this, exec| {
                    this.child(render_meta_wide("Perintah", exec))
                })
                .child(render_meta_wide(
                    "Lokasi Binary",
                    &format!("~/.local/share/madsquash/app/{}/app.AppImage", app_clone.id),
                ))
                .child(render_meta_wide(
                    "Entri Menu OS",
                    &format!("~/.local/share/applications/madsquash-{}.desktop", app_clone.id),
                )),
        )
        // Footer Note
        .child(
            div()
                .px_1()
                .text_xs()
                .text_color(rgb(0x506079))
                .child("Melepas aplikasi hanya membersihkan slot dan entri launcher. Berkas unduhan asli Anda tetap utuh."),
        )
}

pub fn render_empty_detail_pane(cx: &mut Context<MadsquashApp>) -> impl IntoElement {
    div()
        .flex()
        .flex_col()
        .items_center()
        .text_center()
        .gap_3()
        .py_10()
        .child(render_slot_icon())
        .child(
            div()
                .text_lg()
                .font_weight(FontWeight::BOLD)
                .text_color(rgb(0xf0f6fc))
                .child("Slot Masih Kosong"),
        )
        .child(
            div()
                .text_xs()
                .text_color(rgb(0x7e8fa6))
                .max_w(px(260.))
                .child("Jatuhkan berkas .AppImage langsung ke jendela ini, atau klik tombol di bawah untuk memasang."),
        )
        .child(
            div()
                .id("empty-choose-btn")
                .mt_2()
                .px_5()
                .py_2()
                .rounded_xl()
                .bg(rgb(0x0fc5fc))
                .text_color(rgb(0x06151c))
                .font_weight(FontWeight::BOLD)
                .text_xs()
                .cursor_pointer()
                .hover(|s| s.bg(rgb(0x38d3ff)))
                .child("+ Pilih Berkas AppImage")
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
        .bg(rgb(0x0e1b27))
        .border_1()
        .border_color(rgb(0x163447))
        .text_color(rgb(0x0fc5fc))
        .text_xs()
        .font_weight(FontWeight::SEMIBOLD)
        .child(text)
}

pub fn render_meta_col(label: &'static str, value: &str) -> impl IntoElement {
    div()
        .flex_1()
        .py_1p5()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(0x7e8fa6))
                .child(label),
        )
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0xf0f6fc))
                .mt(px(1.))
                .child(value.to_string()),
        )
}

pub fn render_meta_wide(label: &'static str, value: &str) -> impl IntoElement {
    div()
        .w_full()
        .py_1p5()
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::MEDIUM)
                .text_color(rgb(0x7e8fa6))
                .child(label),
        )
        .child(
            div()
                .text_xs()
                .font_weight(FontWeight::SEMIBOLD)
                .text_color(rgb(0xf0f6fc))
                .mt(px(1.))
                .child(value.to_string()),
        )
}
