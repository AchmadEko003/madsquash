use std::collections::hash_map::DefaultHasher;
use std::fs::File;
use std::hash::{Hash, Hasher};
use std::io::Read;
use std::path::{Path, PathBuf};

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
    pub exists: bool,
    pub source_path: Option<PathBuf>,
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
    // Strip .AppImage extension (case-insensitive)
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

    for token in raw_tokens {
        let lower = token.to_ascii_lowercase();
        if matches!(
            lower.as_str(),
            "x86" | "64" | "x86_64" | "aarch64" | "arm64" | "i686" | "amd64" | "linux"
        ) {
            continue;
        }

        let clean_ver = token.strip_prefix('v').or_else(|| token.strip_prefix('V')).unwrap_or(token);
        let looks_like_version = clean_ver
            .chars()
            .next()
            .map(|c| c.is_ascii_digit())
            .unwrap_or(false)
            && clean_ver.contains('.');

        if version.is_none() && looks_like_version {
            version = Some(clean_ver.to_string());
        } else {
            name_parts.push(token.replace('.', " "));
        }
    }

    let mut name = name_parts.join(" ").trim().to_string();
    if name.is_empty() {
        name = "AppImage".to_string();
    } else if let Some(first) = name.chars().next() {
        if first.is_lowercase() {
            let mut chars = name.chars();
            if let Some(f) = chars.next() {
                name = f.to_uppercase().collect::<String>() + chars.as_str();
            }
        }
    }

    (name, version.unwrap_or_else(|| "—".to_string()))
}

pub fn compute_file_hash(path: Option<&Path>, fallback_seed: &str) -> String {
    let mut buffer = vec![0u8; 64 * 1024];
    let bytes_read = path
        .and_then(|p| File::open(p).ok())
        .and_then(|mut f| f.read(&mut buffer).ok())
        .unwrap_or(0);

    // Produce a 64-character hex digest from 4 seeded passes
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

pub fn seed_apps() -> Vec<AppItem> {
    vec![
        AppItem {
            id: "krita".into(),
            name: "Krita".into(),
            comment: "Lukis digital".into(),
            version: "5.2.9".into(),
            size: 287309824,
            arch: "x86_64".into(),
            app_type: "2".into(),
            sha256: "a91c4e2b7d08f1c64e0ab9d3c7e15f8a22c1d0e9b4f6a8c3d5e7f9012345678a".into(),
            installed_at: "12 Sep 2026".into(),
        },
        AppItem {
            id: "inkscape".into(),
            name: "Inkscape".into(),
            comment: "Gambar vektor".into(),
            version: "1.4.2".into(),
            size: 123731968,
            arch: "x86_64".into(),
            app_type: "2".into(),
            sha256: "18bb90c2e4d71a55f0c8e6b3a9d24c7710ef55aa63b8c1d2904e7f6a1234abcd".into(),
            installed_at: "2 Agu 2026".into(),
        },
        AppItem {
            id: "obsidian".into(),
            name: "Obsidian".into(),
            comment: "Catatan lokal".into(),
            version: "1.8.10".into(),
            size: 96468992,
            arch: "x86_64".into(),
            app_type: "2".into(),
            sha256: "77e0c1aa94b238d5f6e1098c4b2a77d1e5c3089ab6410f2d3c8e7a6b5d4c3b2a".into(),
            installed_at: "28 Sep 2026".into(),
        },
        AppItem {
            id: "joplin".into(),
            name: "Joplin".into(),
            comment: "Catatan tersinkron".into(),
            version: "3.3.12".into(),
            size: 102760448,
            arch: "x86_64".into(),
            app_type: "2".into(),
            sha256: "c0ffee12ab3481907766554433221100abcdef99887766554433221100ffeed".into(),
            installed_at: "19 Jul 2026".into(),
        },
        AppItem {
            id: "bitwarden".into(),
            name: "Bitwarden".into(),
            comment: "Brankas kata sandi".into(),
            version: "2026.2.0".into(),
            size: 109051904,
            arch: "x86_64".into(),
            app_type: "2".into(),
            sha256: "51aa09e8b7c6d5e4f3a2b1c0d9e8f7a6b5c4d3e2f1a0b9c8d7e6f5a4b3c2d1e0".into(),
            installed_at: "3 Jun 2026".into(),
        },
        AppItem {
            id: "logseq".into(),
            name: "Logseq".into(),
            comment: "Catatan garis besar".into(),
            version: "0.10.12".into(),
            size: 126877696,
            arch: "x86_64".into(),
            app_type: "2".into(),
            sha256: "09f8e7d6c5b4a39281706f5e4d3c2b1a098877665544332211abcdef00112233".into(),
            installed_at: "21 Mei 2026".into(),
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
