mod component;
mod model;
mod updater;

use std::path::PathBuf;
use std::sync::Arc;
use std::time::Duration;

use gpui_kit::component::input::{InputEvent, InputState};
use gpui_kit::component::theme::{Theme, ThemeMode};
use gpui_kit::prelude::FluentBuilder;
use gpui_kit::*;

use crate::component::{
    card::render_list_pane,
    detail::render_detail_container,
    dialog::{render_install_dialog, render_remove_dialog, render_update_dialog},
    rail::render_rail,
    search::render_search_bar,
    toast::render_toast,
};
use crate::model::{
    AppItem, DialogKind, PendingInstall, app_installed_binary_path, app_storage_dir,
    compute_file_hash, inspect_appimage_metadata, install_app_files, launch_app,
    load_installed_apps, register_madsquash_desktop_entry, save_installed_apps, slug,
    uninstall_app_files,
};
use crate::updater::ReleaseInfo;

pub struct MadsquashApp {
    apps: Vec<AppItem>,
    selected_id: Option<String>,
    query: String,
    search_input: Entity<InputState>,
    dialog: Option<DialogKind>,
    pending: Option<PendingInstall>,
    drop_error: Option<String>,
    toast: Option<String>,
    _toast_task: Option<Task<()>>,
    pub update_info: Option<ReleaseInfo>,
    pub is_checking_update: bool,
    pub is_updating_appimage: bool,
    pub update_status: Option<String>,
    _update_task: Option<Task<()>>,
}

impl MadsquashApp {
    pub fn new(
        search_input: Entity<InputState>,
        cli_file: Option<PathBuf>,
        cx: &mut Context<Self>,
    ) -> Self {
        let apps = load_installed_apps();
        let selected_id = apps.first().map(|a| a.id.clone());

        cx.subscribe(
            &search_input,
            |this: &mut Self, input_state, event: &InputEvent, cx| {
                if matches!(event, InputEvent::Change) {
                    this.query = input_state.read(cx).text().to_string();
                    cx.notify();
                }
            },
        )
        .detach();

        let mut app = Self {
            apps,
            selected_id,
            query: String::new(),
            search_input,
            dialog: None,
            pending: None,
            drop_error: None,
            toast: None,
            _toast_task: None,
            update_info: None,
            is_checking_update: false,
            is_updating_appimage: false,
            update_status: None,
            _update_task: None,
        };

        if let Some(path) = cli_file {
            let canon = path.canonicalize().unwrap_or_else(|_| path.clone());
            let matching_id = app
                .apps
                .iter()
                .find(|a| {
                    let bin = app_installed_binary_path(&a.id);
                    bin == path || bin.canonicalize().map(|b| b == canon).unwrap_or(false)
                })
                .map(|a| a.id.clone());

            if let Some(id) = matching_id {
                app.selected_id = Some(id.clone());
                app.launch_app(&id, cx);
            } else {
                app.take_file_path(path, cx);
            }
        }

        app.check_for_updates(true, cx);

        app
    }

    pub fn visible_apps(&self) -> Vec<&AppItem> {
        let q = self.query.trim().to_lowercase();
        if q.is_empty() {
            return self.apps.iter().collect();
        }
        self.apps
            .iter()
            .filter(|app| {
                app.name.to_lowercase().contains(&q)
                    || app.comment.to_lowercase().contains(&q)
                    || app.version.to_lowercase().contains(&q)
            })
            .collect()
    }

    pub fn selected_app(&self) -> Option<AppItem> {
        let id = self.selected_id.as_ref()?;
        self.apps.iter().find(|a| &a.id == id).cloned()
    }

    pub fn select_app(&mut self, id: String, cx: &mut Context<Self>) {
        self.selected_id = Some(id);
        cx.notify();
    }

    pub fn show_toast(&mut self, text: impl Into<String>, cx: &mut Context<Self>) {
        self.toast = Some(text.into());
        cx.notify();

        self._toast_task = Some(cx.spawn(async move |this, cx| {
            cx.background_executor()
                .timer(Duration::from_millis(2500))
                .await;
            this.update(cx, |this, cx| {
                this.toast = None;
                cx.notify();
            })
            .ok();
        }));
    }

    pub fn open_install(&mut self, cx: &mut Context<Self>) {
        self.dialog = Some(DialogKind::Install);
        self.pending = None;
        self.drop_error = None;
        cx.notify();
    }

    pub fn open_remove(&mut self, cx: &mut Context<Self>) {
        if self.selected_app().is_some() {
            self.dialog = Some(DialogKind::Remove);
            cx.notify();
        }
    }

    pub fn close_dialog(&mut self, cx: &mut Context<Self>) {
        if let Some(ref pending) = self.pending {
            if let Some(ref icon) = pending.icon_path {
                let is_used = self.apps.iter().any(|a| a.icon_path.as_ref() == Some(icon));
                if !is_used && icon.exists() {
                    let _ = std::fs::remove_file(icon);
                }
            }
        }
        self.dialog = None;
        self.pending = None;
        self.drop_error = None;
        self.update_status = None;
        cx.notify();
    }

    pub fn open_update_dialog(&mut self, cx: &mut Context<Self>) {
        self.dialog = Some(DialogKind::Update);
        cx.notify();
    }

    pub fn open_release_web(&mut self, cx: &mut Context<Self>) {
        let url = self
            .update_info
            .as_ref()
            .map(|u| u.html_url.clone())
            .unwrap_or_else(|| {
                format!("https://github.com/{}/releases", crate::updater::REPO_OWNER_NAME)
            });

        let _ = std::process::Command::new("xdg-open").arg(&url).spawn();
        self.show_toast("Membuka rilis di peramban...", cx);
    }

    pub fn check_for_updates(&mut self, silent: bool, cx: &mut Context<Self>) {
        if self.is_checking_update {
            return;
        }

        self.is_checking_update = true;
        if !silent {
            self.show_toast("Memeriksa pembaruan sistem...", cx);
        }
        cx.notify();

        self._update_task = Some(cx.spawn(async move |this, cx| {
            let res = cx
                .background_executor()
                .spawn(async move {
                    crate::updater::fetch_latest_release(crate::updater::REPO_OWNER_NAME)
                })
                .await;

            this.update(cx, |this, cx| {
                this.is_checking_update = false;
                match res {
                    Ok(Some(release)) => {
                        if crate::updater::is_newer_version(
                            crate::updater::CURRENT_VERSION,
                            &release.tag_name,
                        ) {
                            let tag = release.tag_name.clone();
                            this.update_info = Some(release);
                            this.show_toast(format!("Pembaruan {} tersedia!", tag), cx);
                        } else {
                            this.update_info = None;
                            if !silent {
                                this.show_toast(
                                    format!("Madsquash v{} sudah versi terbaru", crate::updater::CURRENT_VERSION),
                                    cx,
                                );
                            }
                        }
                    }
                    Ok(None) => {
                        this.update_info = None;
                        if !silent {
                            this.show_toast(
                                format!("Madsquash v{} sudah versi terbaru", crate::updater::CURRENT_VERSION),
                                cx,
                            );
                        }
                    }
                    Err(e) => {
                        if !silent {
                            this.show_toast(format!("Gagal memeriksa update: {}", e), cx);
                        }
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    pub fn apply_appimage_update(&mut self, cx: &mut Context<Self>) {
        let Some(ref update) = self.update_info else {
            return;
        };
        let Some(ref download_url) = update.appimage_download_url else {
            self.show_toast("Paket .AppImage tidak ditemukan pada rilis ini", cx);
            return;
        };
        let Some(target_path) = crate::updater::current_appimage_path() else {
            self.show_toast("Aplikasi tidak dijalankan sebagai AppImage", cx);
            return;
        };

        let url = download_url.clone();
        self.is_updating_appimage = true;
        self.update_status = Some("Mengunduh berkas AppImage baru...".to_string());
        cx.notify();

        self._update_task = Some(cx.spawn(async move |this, cx| {
            let res = cx
                .background_executor()
                .spawn(async move {
                    crate::updater::download_and_replace_appimage(&url, &target_path)
                })
                .await;

            this.update(cx, |this, cx| {
                this.is_updating_appimage = false;
                match res {
                    Ok(()) => {
                        this.update_status = Some("Pembaruan berhasil! Silakan restart aplikasi.".into());
                        this.show_toast("Madsquash berhasil diperbarui. Silakan buka ulang aplikasi.", cx);
                    }
                    Err(e) => {
                        this.update_status = Some(format!("Gagal update: {}", e));
                        this.show_toast(format!("Gagal update: {}", e), cx);
                    }
                }
                cx.notify();
            })
            .ok();
        }));
    }

    pub fn handle_dropped_paths(&mut self, paths: &ExternalPaths, cx: &mut Context<Self>) {
        if let Some(path) = paths.paths().first() {
            self.take_file_path(path.clone(), cx);
        }
    }

    pub fn browse_file(&mut self, cx: &mut Context<Self>) {
        let rx = cx.prompt_for_paths(PathPromptOptions {
            files: true,
            directories: false,
            multiple: false,
            prompt: Some("Pilih berkas .AppImage".into()),
        });

        cx.spawn(async move |this, cx| {
            if let Ok(Ok(Some(paths))) = rx.await
                && let Some(path) = paths.into_iter().next()
            {
                this.update(cx, |this, cx| {
                    this.take_file_path(path, cx);
                })
                .ok();
            }
        })
        .detach();
    }

    pub fn take_file_path(&mut self, path: PathBuf, cx: &mut Context<Self>) {
        let file_name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_string();

        let is_appimage =
            file_name.len() > 9 && file_name[file_name.len() - 9..].eq_ignore_ascii_case(".appimage");

        self.dialog = Some(DialogKind::Install);

        if !is_appimage {
            self.pending = None;
            self.drop_error = Some("Berkas ini bukan .AppImage.".to_string());
            cx.notify();
            return;
        }

        let size = std::fs::metadata(&path).map(|m| m.len()).unwrap_or(0);
        let details = inspect_appimage_metadata(&path, &file_name);
        let id = slug(&details.name);
        let exists = self.apps.iter().any(|a| a.id == id);

        self.drop_error = None;
        self.pending = Some(PendingInstall {
            comment: details.comment,
            id,
            name: details.name,
            version: details.version,
            size,
            file_name,
            arch: details.arch,
            app_type: details.app_type,
            exists,
            source_path: Some(path),
            icon_path: details.icon_path,
            categories: details.categories,
            exec: details.exec,
        });
        cx.notify();
    }

    pub fn confirm_install(&mut self, cx: &mut Context<Self>) {
        let Some(pending) = self.pending.clone() else {
            return;
        };

        match install_app_files(&pending) {
            Ok(_) => {
                let seed_str = format!("{}|{}|{}", pending.name, pending.size, pending.file_name);
                let sha256 = compute_file_hash(pending.source_path.as_deref(), &seed_str);

                let new_app = AppItem {
                    id: pending.id.clone(),
                    name: pending.name.clone(),
                    comment: pending.comment,
                    version: pending.version,
                    size: pending.size,
                    arch: pending.arch,
                    app_type: pending.app_type,
                    sha256,
                    installed_at: "22 Okt 2026".into(),
                    categories: pending.categories,
                    exec: pending.exec,
                    icon_path: pending.icon_path,
                };

                self.apps.retain(|a| a.id != pending.id);
                self.apps.insert(0, new_app);
                self.selected_id = Some(pending.id);
                self.dialog = None;
                self.pending = None;
                self.drop_error = None;
                save_installed_apps(&self.apps);

                self.show_toast(format!("{} dipasang ke menu", pending.name), cx);
            }
            Err(e) => {
                self.drop_error = Some(e);
                cx.notify();
            }
        }
    }

    pub fn confirm_remove(&mut self, cx: &mut Context<Self>) {
        let Some(selected) = self.selected_app() else {
            return;
        };
        let name = selected.name;
        let id = selected.id;

        let _ = uninstall_app_files(&id);
        self.apps.retain(|a| a.id != id);
        self.selected_id = self.apps.first().map(|a| a.id.clone());
        self.dialog = None;
        save_installed_apps(&self.apps);
        self.show_toast(format!("{} dilepas", name), cx);
    }

    pub fn launch_app(&mut self, id: &str, cx: &mut Context<Self>) {
        let app_name = self
            .apps
            .iter()
            .find(|a| a.id == id)
            .map(|a| a.name.clone())
            .unwrap_or_else(|| "Aplikasi".to_string());

        match launch_app(id) {
            Ok(()) => {
                self.show_toast(format!("Membuka {}", app_name), cx);
            }
            Err(e) => {
                self.show_toast(e, cx);
            }
        }
    }

    pub fn open_app_folder(&mut self, id: &str, cx: &mut Context<Self>) {
        let folder = app_storage_dir(id);
        if folder.exists() {
            let _ = std::process::Command::new("xdg-open")
                .arg(&folder)
                .spawn();
            self.show_toast("Membuka folder aplikasi...", cx);
        } else {
            self.show_toast("Folder belum ada di disk (pasang berkas terlebih dahulu)", cx);
        }
    }
}

impl Render for MadsquashApp {
    fn render(&mut self, _window: &mut Window, cx: &mut Context<Self>) -> impl IntoElement {
        let visible = self.visible_apps();
        let total = self.apps.len();
        let query_is_empty = self.query.trim().is_empty();
        let count_text = if query_is_empty {
            if total > 0 {
                format!("{} terpasang", total)
            } else {
                "Kosong".to_string()
            }
        } else {
            format!("{} dari {}", visible.len(), total)
        };

        let selected = self.selected_app();
        let selected_id = self.selected_id.clone();
        let search_input = self.search_input.clone();
        let dialog = self.dialog.clone();
        let pending = self.pending.clone();
        let drop_error = self.drop_error.clone();
        let toast = self.toast.clone();

        let has_update = self.update_info.is_some();
        let update_tag = self.update_info.as_ref().map(|u| u.tag_name.clone());
        let is_checking = self.is_checking_update;
        let update_info = self.update_info.clone();
        let is_updating_appimage = self.is_updating_appimage;
        let update_status = self.update_status.clone();

        div()
            .id("madsquash-root")
            .relative()
            .size_full()
            .bg(rgb(0x13161c))
            .text_color(rgb(0xe8edf4))
            .flex()
            .flex_row()
            .drag_over::<ExternalPaths>(|s, _, _, _| {
                s.border_2().border_dashed().border_color(rgb(0x5bbfe6)).bg(rgb(0x162533))
            })
            .on_drop(cx.listener(|this, paths: &ExternalPaths, _window, cx| {
                this.handle_dropped_paths(paths, cx);
            }))
            // Left Sidebar Rail
            .child(render_rail(
                has_update,
                update_tag.as_deref(),
                is_checking,
                cx,
            ))
            // Main Stage
            .child(
                div()
                    .flex_1()
                    .h_full()
                    .flex()
                    .flex_col()
                    .overflow_hidden()
                    // Top Bar (Search + Counter + Install)
                    .child(render_search_bar(&search_input, count_text, cx))
                    // Workspace (Cards Grid + Detail Pane)
                    .child(
                        div()
                            .flex_1()
                            .flex()
                            .flex_row()
                            .overflow_hidden()
                            .child(render_list_pane(
                                &visible,
                                selected_id.as_deref(),
                                query_is_empty,
                                cx,
                            ))
                            .child(render_detail_container(selected.clone(), cx)),
                    ),
            )
            // Modal Dialog (when open)
            .when_some(dialog, |this, dlg| match dlg {
                DialogKind::Install => {
                    this.child(render_install_dialog(pending, drop_error, cx))
                }
                DialogKind::Remove => {
                    let app_name = selected.as_ref().map(|a| a.name.clone()).unwrap_or_default();
                    this.child(render_remove_dialog(app_name, cx))
                }
                DialogKind::Update => {
                    this.child(render_update_dialog(update_info, is_updating_appimage, update_status, cx))
                }
            })
            // Toast Notification Banner (when active)
            .when_some(toast, |this, toast_msg| this.child(render_toast(toast_msg)))
    }
}

fn parse_cli_appimage_arg() -> Option<PathBuf> {
    for arg in std::env::args().skip(1) {
        if arg.starts_with('-') {
            continue;
        }
        let clean = arg.trim_start_matches("file://");
        let path = PathBuf::from(clean);
        if path.exists() && path.is_file() {
            let name = path.file_name().and_then(|n| n.to_str()).unwrap_or("");
            if name.len() > 9 && name[name.len() - 9..].eq_ignore_ascii_case(".appimage") {
                return Some(path);
            }
        }
    }
    None
}

const MADSQUASH_PNG_256: &[u8] = include_bytes!("../assets/icons/madsquash_256x256.png");

fn load_window_icon() -> Option<Arc<image::RgbaImage>> {
    image::load_from_memory(MADSQUASH_PNG_256)
        .ok()
        .map(|img| Arc::new(img.into_rgba8()))
}

fn main() {
    register_madsquash_desktop_entry();
    let cli_file = parse_cli_appimage_arg();

    gpui_kit::application()
        .with_assets(gpui_kit::assets::Assets)
        .run(move |cx| {
            gpui_kit::init(cx);
            cx.set_app_identity("madsquash", "Madsquash");
            Theme::change(ThemeMode::Dark, None, cx);
            Theme::update(cx, |theme| {
                theme.background = rgb(0x12110f).into();
                theme.foreground = rgb(0xf4efe8).into();
                theme.muted = rgb(0x221f1b).into();
                theme.muted_foreground = rgb(0xa3988c).into();
                theme.border = rgb(0x3a342c).into();
                theme.input = rgb(0x3a342c).into();
                theme.primary = rgb(0x5bbfe6).into();
                theme.primary_foreground = rgb(0x0c141d).into();
                theme.accent = rgb(0x5bbfe6).into();
                theme.accent_foreground = rgb(0x0c141d).into();
                theme.ring = rgb(0x5bbfe6).into();
            });

            let window_bounds = WindowBounds::Windowed(Bounds::centered(
                None,
                size(px(1060.0), px(700.0)),
                cx,
            ));

            let window_icon = load_window_icon();
            let window_options = WindowOptions {
                window_bounds: Some(window_bounds),
                titlebar: Some(TitlebarOptions {
                    title: Some("Madsquash — AppImage".into()),
                    ..Default::default()
                }),
                app_id: Some("madsquash".into()),
                icon: window_icon,
                ..Default::default()
            };

            let cli_file_for_win = cli_file.clone();
            gpui_kit::open_window(window_options, cx, move |window, cx| {
                let search_input =
                    cx.new(|cx| InputState::new(window, cx).placeholder("Cari slot"));
                cx.new(|cx| MadsquashApp::new(search_input, cli_file_for_win.clone(), cx))
            })
            .expect("failed to open window");
        });
}
