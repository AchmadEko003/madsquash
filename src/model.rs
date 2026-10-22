use std::collections::hash_map::DefaultHasher;
use std::fs::{self, File};
use std::hash::{Hash, Hasher};
use std::io::{BufReader, Read};
use std::path::{Path, PathBuf};

use backhand::{FilesystemReader, InnerNode};

#[derive(Clone, Debug)]
pub struct AppItem {
    pub id: String,
    pub name: String,
    pub comment: String,
    pub version: String,
    pub size: u64,
    pub arch: String,
    pub app_type: String,
    pub sha256: String,
    pub installed_at: String,
    pub categories: Option<String>,
    pub exec: Option<String>,
    pub icon_path: Option<PathBuf>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum DialogKind {
    Install,
    Remove,
}

#[derive(Clone, Debug)]
pub struct PendingInstall {
    pub id: String,
    pub name: String,
    pub version: String,
    pub size: u64,
    pub file_name: String,
    pub comment: String,
    pub arch: String,
    pub app_type: String,
    pub exists: bool,
    pub source_path: Option<PathBuf>,
    pub icon_path: Option<PathBuf>,
    pub categories: Option<String>,
    pub exec: Option<String>,
}

pub fn madsquash_icon_dir() -> PathBuf {
    if let Ok(home) = std::env::var("HOME") {
        PathBuf::from(home).join(".local/share/madsquash/icons")
    } else {
        PathBuf::from("/tmp/madsquash/icons")
    }
}

pub fn known_comment(id: &str) -> &'static str {
    match id {
        "krita" => "Lukis digital",
        "inkscape" => "Gambar vektor",
        "obsidian" => "Catatan lokal",
        "joplin" => "Catatan tersinkron",
        "bitwarden" => "Brankas kata sandi",
        "logseq" => "Catatan garis besar",
        "kdenlive" => "Penyunting video",
        _ => "AppImage",
    }
}

pub fn slug(name: &str) -> String {
    let mut out = String::new();
    let mut prev_dash = false;
    for ch in name.chars() {
        if ch.is_ascii_alphanumeric() {
            out.push(ch.to_ascii_lowercase());
            prev_dash = false;
        } else if !prev_dash && !out.is_empty() {
            out.push('-');
            prev_dash = true;
        }
    }
    let trimmed = out.trim_matches('-').to_string();
    if trimmed.is_empty() {
        "app".to_string()
    } else {
        trimmed
    }
}

pub fn parse_appimage_name(file_name: &str) -> (String, String) {
    let base = if file_name.len() > 9 && file_name[file_name.len() - 9..].eq_ignore_ascii_case(".appimage") {
        &file_name[..file_name.len() - 9]
    } else {
        file_name
    };

    let raw_tokens: Vec<&str> = base
        .split(|c: char| c == '-' || c == '_' || c.is_whitespace())
        .filter(|s| !s.is_empty())
        .collect();

    let mut name_parts: Vec<String> = Vec::new();
    let mut version: Option<String> = None;
    let mut embedded_num_ver: Option<String> = None;

    for token in raw_tokens {
        let lower = token.to_ascii_lowercase();
        if matches!(
            lower.as_str(),
            "x86" | "64" | "x64" | "x86_64" | "aarch64" | "arm64" | "armhf" | "ia32" | "i686" | "amd64" | "linux" | "appimage"
        ) {
            continue;
        }

        let clean_ver = token.strip_prefix('v').or_else(|| token.strip_prefix('V')).unwrap_or(token);
        let starts_with_digit = clean_ver.chars().next().map(|c| c.is_ascii_digit()).unwrap_or(false);
        let has_dot = clean_ver.contains('.');
        let is_pure_digits = starts_with_digit && clean_ver.chars().all(|c| c.is_ascii_digit());
        let looks_like_version = starts_with_digit && (has_dot || is_pure_digits);

        if version.is_none() && looks_like_version {
            version = Some(clean_ver.to_string());
        } else if let Some(digit_pos) = token.find(|c: char| c.is_ascii_digit()) {
            if digit_pos > 0 && token[digit_pos..].chars().all(|c| c.is_ascii_digit()) {
                name_parts.push(token[..digit_pos].to_string());
                if embedded_num_ver.is_none() {
                    embedded_num_ver = Some(token[digit_pos..].to_string());
                }
            } else {
                name_parts.push(token.replace('.', " "));
            }
        } else {
            name_parts.push(token.replace('.', " "));
        }
    }

    if version.is_none() && embedded_num_ver.is_some() {
        version = embedded_num_ver;
    }

    let name = name_parts
        .iter()
        .flat_map(|s| s.split_whitespace())
        .map(|word| {
            let mut chars = word.chars();
            match chars.next() {
                Some(first) => first.to_uppercase().collect::<String>() + chars.as_str(),
                None => String::new(),
            }
        })
        .collect::<Vec<String>>()
        .join(" ");

    let name = if name.is_empty() {
        "AppImage".to_string()
    } else {
        name
    };

    (name, version.unwrap_or_else(|| "—".to_string()))
}

pub fn compute_file_hash(path: Option<&Path>, fallback_seed: &str) -> String {
    let mut buffer = vec![0u8; 64 * 1024];
    let bytes_read = path
        .and_then(|p| File::open(p).ok())
        .and_then(|mut f| f.read(&mut buffer).ok())
        .unwrap_or(0);

    let mut hex = String::with_capacity(64);
    for pass in 0u64..4 {
        let mut hasher = DefaultHasher::new();
        pass.hash(&mut hasher);
        fallback_seed.hash(&mut hasher);
        if bytes_read > 0 {
            buffer[..bytes_read].hash(&mut hasher);
        }
        let val = hasher.finish();
        hex.push_str(&format!("{:016x}", val));
    }
    hex
}

pub fn find_squashfs_offset(path: &Path) -> Option<u64> {
    let mut file = File::open(path).ok()?;
    let mut buf = vec![0u8; 4 * 1024 * 1024]; // search up to 4MB
    let n = file.read(&mut buf).ok()?;
    for (i, w) in buf[..n].windows(32).enumerate() {
        if &w[0..4] == b"hsqs" || &w[0..4] == b"sqsh" {
            let major = u16::from_le_bytes([w[28], w[29]]);
            let minor = u16::from_le_bytes([w[30], w[31]]);
            if major == 4 && minor == 0 {
                return Some(i as u64);
            }
        }
    }
    None
}

#[derive(Default, Debug)]
pub struct AppImageDetails {
    pub name: String,
    pub version: String,
    pub comment: String,
    pub arch: String,
    pub app_type: String,
    pub categories: Option<String>,
    pub exec: Option<String>,
    pub icon_path: Option<PathBuf>,
}

pub fn inspect_appimage_metadata(path: &Path, file_name: &str) -> AppImageDetails {
    let mut details = AppImageDetails::default();

    // 1. Basic fallback values from filename
    let (parsed_name, parsed_version) = parse_appimage_name(file_name);
    let id = slug(&parsed_name);
    details.name = parsed_name;
    details.version = parsed_version;
    details.comment = known_comment(&id).to_string();
    details.arch = "x86_64".to_string();
    details.app_type = "AppImage 2".to_string();

    // 2. Read ELF Header (magic + arch)
    if let Ok(mut f) = File::open(path) {
        let mut header = [0u8; 64];
        if f.read_exact(&mut header).is_ok() {
            if &header[8..11] == b"AI\x01" {
                details.app_type = "AppImage 1".to_string();
            } else if &header[8..11] == b"AI\x02" {
                details.app_type = "AppImage 2".to_string();
            }

            let e_machine = u16::from_le_bytes([header[18], header[19]]);
            details.arch = match e_machine {
                62 => "x86_64".to_string(),
                183 => "aarch64".to_string(),
                3 => "i686".to_string(),
                40 => "armhf".to_string(),
                _ => "x86_64".to_string(),
            };
        }
    }

    // 3. Inspect SquashFS for .desktop file and icons
    if let Some(offset) = find_squashfs_offset(path) {
        if let Ok(file) = File::open(path) {
            let reader = BufReader::new(file);
            if let Ok(fs) = FilesystemReader::from_reader_with_offset(reader, offset) {
                let mut desktop_icon_name: Option<String> = None;

                // Scan files for .desktop file
                for entry in fs.files() {
                    let p = entry.fullpath.to_string_lossy();
                    if p.ends_with(".desktop") {
                        if let InnerNode::File(ref f_node) = entry.inner {
                            if let Ok(mut f_reader) = fs.file(f_node).reader_checked() {
                                let mut content = String::new();
                                if f_reader.read_to_string(&mut content).is_ok() {
                                    for line in content.lines() {
                                        let line = line.trim();
                                        if let Some(val) = line.strip_prefix("Name=") {
                                            if !val.trim().is_empty() {
                                                details.name = val.trim().to_string();
                                            }
                                        } else if let Some(val) = line.strip_prefix("Comment=") {
                                            if !val.trim().is_empty() {
                                                details.comment = val.trim().to_string();
                                            }
                                        } else if let Some(val) = line.strip_prefix("GenericName=") {
                                            if details.comment == "AppImage" && !val.trim().is_empty() {
                                                details.comment = val.trim().to_string();
                                            }
                                        } else if let Some(val) = line.strip_prefix("X-AppImage-Version=") {
                                            if !val.trim().is_empty() {
                                                details.version = val.trim().to_string();
                                            }
                                        } else if let Some(val) = line.strip_prefix("X-Version=") {
                                            if !val.trim().is_empty() {
                                                details.version = val.trim().to_string();
                                            }
                                        } else if let Some(val) = line.strip_prefix("Version=") {
                                            let v = val.trim();
                                            // Desktop Entry Spec versions are typically 1.0, 1.1, 1.2, 1.3, 1.4, 1.5
                                            if !v.is_empty() && !matches!(v, "1.0" | "1.1" | "1.2" | "1.3" | "1.4" | "1.5") {
                                                details.version = v.to_string();
                                            }
                                        } else if let Some(val) = line.strip_prefix("Categories=") {
                                            details.categories = Some(val.trim().to_string());
                                        } else if let Some(val) = line.strip_prefix("Exec=") {
                                            details.exec = Some(val.trim().to_string());
                                        } else if let Some(val) = line.strip_prefix("Icon=") {
                                            desktop_icon_name = Some(val.trim().to_string());
                                        }
                                    }
                                    break;
                                }
                            }
                        }
                    }
                }

                // Scan files for matching icon or .DirIcon / root icon
                let target_slug = slug(&details.name);
                let icon_dir = madsquash_icon_dir();
                let _ = fs::create_dir_all(&icon_dir);

                let mut best_icon: Option<(String, Vec<u8>)> = None;

                for entry in fs.files() {
                    let p = entry.fullpath.to_string_lossy();
                    let is_dir_icon = p == "/.DirIcon" || p.ends_with("/.DirIcon");
                    let is_matching_icon = desktop_icon_name
                        .as_deref()
                        .map(|name| p.contains(&format!("/{}.", name)) || p.contains(&format!("/{}", name)))
                        .unwrap_or(false);
                    let is_root_image = (p.starts_with('/') && !p[1..].contains('/'))
                        && (p.ends_with(".png") || p.ends_with(".svg"));

                    if is_dir_icon || is_matching_icon || is_root_image {
                        if let InnerNode::File(ref f_node) = entry.inner {
                            if let Ok(mut f_reader) = fs.file(f_node).reader_checked() {
                                let mut icon_bytes = Vec::new();
                                if f_reader.read_to_end(&mut icon_bytes).is_ok() && !icon_bytes.is_empty() {
                                    let ext = if p.ends_with(".svg") { "svg" } else { "png" };
                                    best_icon = Some((ext.to_string(), icon_bytes));
                                    if is_dir_icon || is_matching_icon {
                                        break;
                                    }
                                }
                            }
                        }
                    }
                }

                if let Some((ext, bytes)) = best_icon {
                    let target_path = icon_dir.join(format!("{}.{}", target_slug, ext));
                    if fs::write(&target_path, bytes).is_ok() {
                        details.icon_path = Some(target_path);
                    }
                }
            }
        }
    }

    // 4. Fallback: if version is still "—", check if app name ends with a version / release number (e.g. "Navicat Premium Lite 18")
    if details.version == "—" {
        if let Some(last_token) = details.name.split_whitespace().last() {
            let clean = last_token.strip_prefix('v').or_else(|| last_token.strip_prefix('V')).unwrap_or(last_token);
            if clean.chars().all(|c| c.is_ascii_digit() || c == '.') && clean.chars().any(|c| c.is_ascii_digit()) {
                details.version = clean.to_string();
            }
        }
    }

    details
}

pub fn seed_apps() -> Vec<AppItem> {
    vec![
        AppItem {
            id: "krita".into(),
            name: "Krita".into(),
            comment: "Lukis digital".into(),
            version: "5.2.9".into(),
            size: 287309824,
            arch: "x86_64".into(),
            app_type: "AppImage 2".into(),
            sha256: "a91c4e2b7d08f1c64e0ab9d3c7e15f8a22c1d0e9b4f6a8c3d5e7f9012345678a".into(),
            installed_at: "12 Sep 2026".into(),
            categories: Some("Graphics;2DGraphics;RasterGraphics;".into()),
            exec: Some("krita %F".into()),
            icon_path: None,
        },
        AppItem {
            id: "inkscape".into(),
            name: "Inkscape".into(),
            comment: "Gambar vektor".into(),
            version: "1.4.2".into(),
            size: 123731968,
            arch: "x86_64".into(),
            app_type: "AppImage 2".into(),
            sha256: "18bb90c2e4d71a55f0c8e6b3a9d24c7710ef55aa63b8c1d2904e7f6a1234abcd".into(),
            installed_at: "2 Agu 2026".into(),
            categories: Some("Graphics;VectorGraphics;".into()),
            exec: Some("inkscape %F".into()),
            icon_path: None,
        },
        AppItem {
            id: "obsidian".into(),
            name: "Obsidian".into(),
            comment: "Catatan lokal".into(),
            version: "1.8.10".into(),
            size: 96468992,
            arch: "x86_64".into(),
            app_type: "AppImage 2".into(),
            sha256: "77e0c1aa94b238d5f6e1098c4b2a77d1e5c3089ab6410f2d3c8e7a6b5d4c3b2a".into(),
            installed_at: "28 Sep 2026".into(),
            categories: Some("Office;Notes;".into()),
            exec: Some("obsidian %u".into()),
            icon_path: None,
        },
        AppItem {
            id: "joplin".into(),
            name: "Joplin".into(),
            comment: "Catatan tersinkron".into(),
            version: "3.3.12".into(),
            size: 102760448,
            arch: "x86_64".into(),
            app_type: "AppImage 2".into(),
            sha256: "c0ffee12ab3481907766554433221100abcdef99887766554433221100ffeed".into(),
            installed_at: "19 Jul 2026".into(),
            categories: Some("Office;Notes;".into()),
            exec: Some("joplin %u".into()),
            icon_path: None,
        },
        AppItem {
            id: "bitwarden".into(),
            name: "Bitwarden".into(),
            comment: "Brankas kata sandi".into(),
            version: "2026.2.0".into(),
            size: 109051904,
            arch: "x86_64".into(),
            app_type: "AppImage 2".into(),
            sha256: "51aa09e8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d7e6f5a4b3c2d1e0".into(),
            installed_at: "3 Jun 2026".into(),
            categories: Some("Utility;Security;".into()),
            exec: Some("bitwarden".into()),
            icon_path: None,
        },
        AppItem {
            id: "logseq".into(),
            name: "Logseq".into(),
            comment: "Catatan garis besar".into(),
            version: "0.10.12".into(),
            size: 126877696,
            arch: "x86_64".into(),
            app_type: "AppImage 2".into(),
            sha256: "09f8e7d6c5b4a39281706f5e4d3c2b1a098877665544332211abcdef00112233".into(),
            installed_at: "21 Mei 2026".into(),
            categories: Some("Office;Notes;".into()),
            exec: Some("logseq".into()),
            icon_path: None,
        },
    ]
}

pub fn mono(name: &str) -> String {
    let parts: Vec<&str> = name.split_whitespace().collect();
    if parts.len() >= 2 {
        let first = parts[0].chars().next().unwrap_or('A');
        let second = parts[1].chars().next().unwrap_or('P');
        format!("{}{}", first, second).to_uppercase()
    } else {
        name.chars().take(2).collect::<String>().to_uppercase()
    }
}

pub fn format_bytes(bytes: u64) -> String {
    const UNITS: [&str; 4] = ["B", "KB", "MB", "GB"];
    let mut val = bytes as f64;
    let mut i = 0;
    while val >= 1024.0 && i < UNITS.len() - 1 {
        val /= 1024.0;
        i += 1;
    }
    if i == 0 || val >= 10.0 {
        format!("{:.0} {}", val, UNITS[i])
    } else {
        format!("{:.1} {}", val, UNITS[i])
    }
}

pub fn short_hash(hash: &str) -> String {
    if hash.len() > 18 {
        format!("{}…{}", &hash[..12], &hash[hash.len() - 6..])
    } else {
        hash.to_string()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_appimage_name_variants() {
        let (name, ver) = parse_appimage_name("navicat18-premium-lite-id-x86_64.AppImage");
        assert_eq!(name, "Navicat Premium Lite Id");
        assert_eq!(ver, "18");

        let (name, ver) = parse_appimage_name("Grok_Bot_0.75.0.AppImage");
        assert_eq!(name, "Grok Bot");
        assert_eq!(ver, "0.75.0");

        let (name, ver) = parse_appimage_name("Kdenlive-25.04.0-x86_64.AppImage");
        assert_eq!(name, "Kdenlive");
        assert_eq!(ver, "25.04.0");

        let (name, ver) = parse_appimage_name("krita-5.2.9-x86_64.AppImage");
        assert_eq!(name, "Krita");
        assert_eq!(ver, "5.2.9");

        let (name, ver) = parse_appimage_name("cursor-0.42.3-x86_64.AppImage");
        assert_eq!(name, "Cursor");
        assert_eq!(ver, "0.42.3");

        let (name, ver) = parse_appimage_name("Postman-v10.15.0-linux-x64.AppImage");
        assert_eq!(name, "Postman");
        assert_eq!(ver, "10.15.0");

        let (name, ver) = parse_appimage_name("balenaEtcher-1.18.11-ia32.AppImage");
        assert_eq!(name, "BalenaEtcher");
        assert_eq!(ver, "1.18.11");
    }

    #[test]
    fn test_inspect_navicat_if_present() {
        let path = PathBuf::from("/home/madko/Downloads/navicat18-premium-lite-id-x86_64.AppImage");
        if !path.exists() {
            return;
        }

        let details = inspect_appimage_metadata(&path, "navicat18-premium-lite-id-x86_64.AppImage");
        assert_eq!(details.name, "Navicat Premium Lite 18");
        assert_eq!(details.version, "18");
        assert_eq!(details.arch, "x86_64");
        assert_eq!(details.app_type, "AppImage 2");
        assert!(details.icon_path.is_some());
        if let Some(ref icon) = details.icon_path {
            assert!(icon.exists());
        }
    }
}
