use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::MadsquashApp;
use crate::component::detail::{render_meta_col, render_meta_wide};
use crate::model::{PendingInstall, format_bytes, mono};

pub fn render_install_dialog(
    pending: Option<PendingInstall>,
    drop_error: Option<String>,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    let has_pending = pending.is_some();
    let is_exists = pending.as_ref().map(|p| p.exists).unwrap_or(false);

    div()
        .id("install-dialog-overlay")
        .absolute()
        .inset_0()
        .bg(rgba(0x0e1116e6))
        .flex()
        .items_center()
        .justify_center()
        .on_click(cx.listener(|this, _, _, cx| {
            this.close_dialog(cx);
        }))
        .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
            this.handle_dropped_paths(paths, cx);
        }))
        .child(
            div()
                .id("install-dialog-content")
                .w(px(460.))
                .p_6()
                .rounded_2xl()
                .bg(rgb(0x171c24))
                .border_1()
                .border_color(rgb(0x222a36))
                .shadow_2xl()
                .flex()
                .flex_col()
                .gap_3()
                .on_click(|_, _, cx| cx.stop_propagation())
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .text_lg()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xe8edf4))
                                .child("Pasang AppImage"),
                        )
                        .child(
                            div()
                                .id("dialog-close-x")
                                .size(px(24.))
                                .rounded_md()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(rgb(0x64748b))
                                .cursor_pointer()
                                .hover(|s| s.text_color(rgb(0xe8edf4)))
                                .child("✕")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_dialog(cx);
                                })),
                        ),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x94a3b8))
                        .child("Berkas disalin ke direktori aplikasi Madsquash, lalu didaftarkan ke launcher OS."),
                )
                .child(match pending {
                    None => render_drop_zone(drop_error, cx).into_any_element(),
                    Some(ref p) => render_install_preview(p, cx).into_any_element(),
                })
                // Dialog buttons
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .gap_2()
                        .mt_3()
                        .child(
                            div()
                                .id("dialog-cancel-btn")
                                .px_4()
                                .py_2()
                                .rounded_xl()
                                .border_1()
                                .border_color(rgb(0x252e3c))
                                .bg(rgb(0x1b212b))
                                .text_color(rgb(0xe8edf4))
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0x222a36)))
                                .child("Batal")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_dialog(cx);
                                })),
                        )
                        .when(has_pending, |this| {
                            this.child(
                                div()
                                    .id("dialog-confirm-btn")
                                    .px_5()
                                    .py_2()
                                    .rounded_xl()
                                    .bg(rgb(0x5bbfe6))
                                    .text_color(rgb(0x0f202b))
                                    .font_weight(FontWeight::BOLD)
                                    .text_xs()
                                    .cursor_pointer()
                                    .hover(|s| s.bg(rgb(0x6ec8ed)))
                                    .child(if is_exists { "Ganti Versi Ini" } else { "Pasang ke Menu" })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        this.confirm_install(cx);
                                    })),
                            )
                        }),
                ),
        )
}

pub fn render_drop_zone(
    drop_error: Option<String>,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    let has_error = drop_error.is_some();
    let headline = drop_error.unwrap_or_else(|| "Jatuhkan berkas .AppImage di sini".to_string());

    div()
        .flex()
        .flex_col()
        .gap_2p5()
        .child(
            div()
                .id("install-dropzone")
                .h(px(140.))
                .rounded_2xl()
                .border_2()
                .when(!has_error, |this| {
                    this.border_dashed()
                        .border_color(rgb(0x252e3c))
                        .bg(rgb(0x13171e))
                })
                .when(has_error, |this| {
                    this.border_color(rgb(0xfca5a5)).bg(rgb(0x2d1b1f))
                })
                .cursor_pointer()
                .hover(|s| s.border_color(rgb(0x5bbfe6)).bg(rgb(0x172533)))
                .drag_over::<ExternalPaths>(|s, _, _, _| {
                    s.border_color(rgb(0x5bbfe6)).bg(rgb(0x172533))
                })
                .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
                    this.handle_dropped_paths(paths, cx);
                }))
                .flex()
                .flex_col()
                .items_center()
                .justify_center()
                .gap_1p5()
                .on_click(cx.listener(|this, _, _, cx| {
                    this.browse_file(cx);
                }))
                .child(
                    div()
                        .size(px(36.))
                        .rounded_xl()
                        .bg(rgb(0x1b242e))
                        .border_1()
                        .border_color(rgb(0x28394a))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(0x7bc7e8))
                        .text_lg()
                        .child("↓"),
                )
                .child(
                    div()
                        .text_sm()
                        .font_weight(FontWeight::BOLD)
                        .text_color(if has_error {
                            rgb(0xfca5a5)
                        } else {
                            rgb(0xe8edf4)
                        })
                        .child(headline),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x94a3b8))
                        .child("atau klik di sini untuk memilih berkas dari komputer"),
                ),
        )
}

pub fn render_install_preview(
    pending: &PendingInstall,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .id("install-preview-zone")
        .flex()
        .flex_col()
        .gap_2p5()
        .mt_1()
        .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
            this.handle_dropped_paths(paths, cx);
        }))
        .child(
            div()
                .flex()
                .items_center()
                .justify_between()
                .gap_2()
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x94a3b8))
                        .child(pending.file_name.clone()),
                )
                .child(
                    div()
                        .id("change-file-btn")
                        .cursor_pointer()
                        .text_xs()
                        .font_weight(FontWeight::SEMIBOLD)
                        .text_color(rgb(0x7bc7e8))
                        .hover(|s| s.text_color(rgb(0x5bbfe6)))
                        .child("Ganti berkas")
                        .on_click(cx.listener(|this, _, _, cx| {
                            this.browse_file(cx);
                        })),
                ),
        )
        // Extracted app card
        .child(
            div()
                .rounded_xl()
                .bg(rgb(0x1b212b))
                .border_1()
                .border_color(rgb(0x252e3c))
                .p_3()
                .flex()
                .items_center()
                .gap_3()
                .child(if let Some(ref icon_path) = pending.icon_path {
                    div()
                        .size(px(48.))
                        .rounded_xl()
                        .bg(rgb(0x171c24))
                        .border_1()
                        .border_color(rgb(0x5bbfe6))
                        .p(px(3.))
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(gpui::img(icon_path.clone()).size(px(42.)))
                } else {
                    div()
                        .size(px(48.))
                        .rounded_xl()
                        .bg(rgb(0x171c24))
                        .border_1()
                        .border_color(rgb(0x5bbfe6))
                        .text_color(rgb(0x7bc7e8))
                        .font_weight(FontWeight::BOLD)
                        .text_lg()
                        .flex()
                        .items_center()
                        .justify_center()
                        .child(mono(&pending.name))
                })
                .child(
                    div()
                        .flex_1()
                        .flex()
                        .flex_col()
                        .gap(px(1.))
                        .child(
                            div()
                                .text_base()
                                .font_weight(FontWeight::BOLD)
                                .text_color(rgb(0xe8edf4))
                                .child(pending.name.clone()),
                        )
                        .child(
                            div()
                                .text_xs()
                                .text_color(rgb(0x94a3b8))
                                .child(if !pending.comment.is_empty() {
                                    pending.comment.clone()
                                } else {
                                    "Paket Aplikasi AppImage".to_string()
                                }),
                        ),
                ),
        )
        // Metadata Table
        .child(
            div()
                .rounded_xl()
                .bg(rgb(0x1b212b))
                .border_1()
                .border_color(rgb(0x252e3c))
                .p_3()
                .flex()
                .flex_col()
                .gap_1()
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Versi Terdeteksi", &pending.version))
                        .child(render_meta_col("Ukuran", &format_bytes(pending.size))),
                )
                .child(
                    div()
                        .flex()
                        .child(render_meta_col("Arsitektur", &pending.arch))
                        .child(render_meta_col("Format", &pending.app_type)),
                )
                .when_some(pending.categories.as_ref(), |this, cat| {
                    this.child(render_meta_wide("Kategori", cat))
                })
                .child(render_meta_wide(
                    "Tujuan Pemasangan",
                    &format!("~/.local/share/madsquash/app/{}/", pending.id),
                )),
        )
        .when(pending.exists, |this| {
            this.child(
                div()
                    .p_3()
                    .rounded_xl()
                    .bg(rgb(0x1b2a38))
                    .border_1()
                    .border_color(rgb(0x5bbfe6))
                    .text_xs()
                    .text_color(rgb(0xe8edf4))
                    .child(format!(
                        "ℹ {} sudah terpasang. Memasang kembali akan menimpa binary yang ada ke versi ini.",
                        pending.name
                    )),
            )
        })
}

pub fn render_remove_dialog(
    app_name: String,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    div()
        .id("remove-dialog-overlay")
        .absolute()
        .inset_0()
        .bg(rgba(0x0e1116e6))
        .flex()
        .items_center()
        .justify_center()
        .on_click(cx.listener(|this, _, _, cx| {
            this.close_dialog(cx);
        }))
        .child(
            div()
                .id("remove-dialog-content")
                .w(px(420.))
                .p_6()
                .rounded_2xl()
                .bg(rgb(0x171c24))
                .border_1()
                .border_color(rgb(0x222a36))
                .shadow_2xl()
                .flex()
                .flex_col()
                .gap_3()
                .on_click(|_, _, cx| cx.stop_propagation())
                .child(
                    div()
                        .size(px(40.))
                        .rounded_xl()
                        .bg(rgb(0x2d1b1f))
                        .border_1()
                        .border_color(rgb(0x4a262d))
                        .flex()
                        .items_center()
                        .justify_center()
                        .text_color(rgb(0xfca5a5))
                        .text_lg()
                        .child("⚠"),
                )
                .child(
                    div()
                        .text_lg()
                        .font_weight(FontWeight::BOLD)
                        .text_color(rgb(0xe8edf4))
                        .child(format!("Lepas {}?", app_name)),
                )
                .child(
                    div()
                        .text_xs()
                        .text_color(rgb(0x94a3b8))
                        .child("Entri launcher OS, ikon, dan berkas salinan AppImage di Madsquash akan dihapus dari sistem."),
                )
                .child(
                    div()
                        .flex()
                        .justify_end()
                        .gap_2()
                        .mt_4()
                        .child(
                            div()
                                .id("remove-cancel-btn")
                                .px_4()
                                .py_2()
                                .rounded_xl()
                                .border_1()
                                .border_color(rgb(0x252e3c))
                                .bg(rgb(0x1b212b))
                                .text_color(rgb(0xe8edf4))
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0x222a36)))
                                .child("Batal")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.close_dialog(cx);
                                })),
                        )
                        .child(
                            div()
                                .id("confirm-remove-btn")
                                .px_5()
                                .py_2()
                                .rounded_xl()
                                .bg(rgb(0xe05656))
                                .text_color(rgb(0xffffff))
                                .font_weight(FontWeight::BOLD)
                                .text_xs()
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0xc94343)))
                                .child("Lepas Aplikasi")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.confirm_remove(cx);
                                })),
                        ),
                ),
        )
}

pub fn render_update_dialog(
    update: Option<crate::updater::ReleaseInfo>,
    is_updating: bool,
    status_msg: Option<String>,
    cx: &mut Context<MadsquashApp>,
) -> impl IntoElement {
    let has_appimage_url = update.as_ref().and_then(|u| u.appimage_download_url.as_ref()).is_some();
    let is_running_appimage = crate::updater::current_appimage_path().is_some();
    let can_auto_update = is_running_appimage && has_appimage_url;

    let target_version = update.as_ref().map(|u| u.tag_name.clone()).unwrap_or_else(|| "Terbaru".into());
    let release_notes = update
        .as_ref()
        .map(|u| {
            if u.body.trim().is_empty() {
                "Pembaruan performa dan perbaikan bug sistem.".to_string()
            } else {
                u.body.clone()
            }
        })
        .unwrap_or_else(|| "Memeriksa rilis...".into());

    div()
        .id("update-dialog-overlay")
        .absolute()
        .inset_0()
        .bg(rgba(0x0e1116e6))
        .flex()
        .items_center()
        .justify_center()
        .on_click(cx.listener(|this, _, _, cx| {
            if !this.is_updating_appimage {
                this.close_dialog(cx);
            }
        }))
        .child(
            div()
                .id("update-dialog-content")
                .w(px(480.))
                .p_6()
                .rounded_2xl()
                .bg(rgb(0x171c24))
                .border_1()
                .border_color(rgb(0x222a36))
                .shadow_2xl()
                .flex()
                .flex_col()
                .gap_4()
                .on_click(|_, _, cx| cx.stop_propagation())
                // Header
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .child(
                            div()
                                .flex()
                                .items_center()
                                .gap_3()
                                .child(
                                    div()
                                        .size(px(32.))
                                        .rounded_lg()
                                        .bg(rgb(0x182735))
                                        .border_1()
                                        .border_color(rgb(0x5bbfe6))
                                        .flex()
                                        .items_center()
                                        .justify_center()
                                        .text_color(rgb(0x5bbfe6))
                                        .font_weight(FontWeight::BOLD)
                                        .text_sm()
                                        .child("⬆"),
                                )
                                .child(
                                    div()
                                        .text_lg()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0xe8edf4))
                                        .child("Pembaruan Sistem Madsquash"),
                                ),
                        )
                        .child(
                            div()
                                .id("update-close-x")
                                .size(px(24.))
                                .rounded_md()
                                .flex()
                                .items_center()
                                .justify_center()
                                .text_color(rgb(0x64748b))
                                .cursor_pointer()
                                .hover(|s| s.text_color(rgb(0xe8edf4)))
                                .child("✕")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    if !this.is_updating_appimage {
                                        this.close_dialog(cx);
                                    }
                                })),
                        ),
                )
                // Version Banner
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_between()
                        .px_4()
                        .py_3()
                        .rounded_xl()
                        .bg(rgb(0x13171e))
                        .border_1()
                        .border_color(rgb(0x1f2733))
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x64748b))
                                        .child("Versi Sekarang"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x94a3b8))
                                        .child(format!("v{}", crate::updater::CURRENT_VERSION)),
                                ),
                        )
                        .child(
                            div()
                                .text_base()
                                .text_color(rgb(0x5bbfe6))
                                .child("➜"),
                        )
                        .child(
                            div()
                                .flex()
                                .flex_col()
                                .items_end()
                                .child(
                                    div()
                                        .text_xs()
                                        .text_color(rgb(0x5bbfe6))
                                        .child("Versi Baru Tersedia"),
                                )
                                .child(
                                    div()
                                        .text_sm()
                                        .font_weight(FontWeight::BOLD)
                                        .text_color(rgb(0x6ee7b7))
                                        .child(target_version),
                                ),
                        ),
                )
                // Release notes box
                .child(
                    div()
                        .flex()
                        .flex_col()
                        .gap_1()
                        .child(
                            div()
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .text_color(rgb(0x94a3b8))
                                .child("Catatan Rilis:"),
                        )
                        .child(
                            div()
                                .id("release-notes-box")
                                .max_h(px(120.))
                                .overflow_y_scroll()
                                .p_3()
                                .rounded_lg()
                                .bg(rgb(0x101318))
                                .border_1()
                                .border_color(rgb(0x1e242e))
                                .text_xs()
                                .text_color(rgb(0xc5d1de))
                                .child(release_notes),
                        ),
                )
                // Status message (if updating)
                .when_some(status_msg, |this, msg| {
                    this.child(
                        div()
                            .px_3()
                            .py_2()
                            .rounded_lg()
                            .bg(rgb(0x1c2b38))
                            .border_1()
                            .border_color(rgb(0x274b6b))
                            .text_xs()
                            .text_color(rgb(0x5bbfe6))
                            .child(msg),
                    )
                })
                // Buttons footer
                .child(
                    div()
                        .flex()
                        .items_center()
                        .justify_end()
                        .gap_2()
                        .mt_2()
                        .child(
                            div()
                                .id("update-cancel-btn")
                                .px_4()
                                .py_2()
                                .rounded_xl()
                                .border_1()
                                .border_color(rgb(0x252e3c))
                                .bg(rgb(0x1b212b))
                                .text_color(rgb(0xe8edf4))
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0x222a36)))
                                .child("Tutup")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    if !this.is_updating_appimage {
                                        this.close_dialog(cx);
                                    }
                                })),
                        )
                        .child(
                            div()
                                .id("open-github-release-btn")
                                .px_4()
                                .py_2()
                                .rounded_xl()
                                .border_1()
                                .border_color(rgb(0x354050))
                                .bg(rgb(0x1f2733))
                                .text_color(rgb(0xe8edf4))
                                .text_xs()
                                .font_weight(FontWeight::MEDIUM)
                                .cursor_pointer()
                                .hover(|s| s.bg(rgb(0x293444)))
                                .child("Buka di GitHub")
                                .on_click(cx.listener(|this, _, _, cx| {
                                    this.open_release_web(cx);
                                })),
                        )
                        .when(can_auto_update, |this| {
                            this.child(
                                div()
                                    .id("auto-update-appimage-btn")
                                    .px_4()
                                    .py_2()
                                    .rounded_xl()
                                    .bg(if is_updating { rgb(0x2d5f78) } else { rgb(0x5bbfe6) })
                                    .text_color(rgb(0x0c141d))
                                    .text_xs()
                                    .font_weight(FontWeight::BOLD)
                                    .cursor_pointer()
                                    .hover(|s| {
                                        if is_updating {
                                            s
                                        } else {
                                            s.bg(rgb(0x78cef0))
                                        }
                                    })
                                    .child(if is_updating {
                                        "Mengunduh..."
                                    } else {
                                        "Update Otomatis"
                                    })
                                    .on_click(cx.listener(|this, _, _, cx| {
                                        if !this.is_updating_appimage {
                                            this.apply_appimage_update(cx);
                                        }
                                    })),
                            )
                        }),
                ),
        )
}
