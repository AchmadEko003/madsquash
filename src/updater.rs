use std::path::PathBuf;
use std::process::Command;
use serde::Deserialize;

pub const REPO_OWNER_NAME: &str = "AchmadEko003/madsquash";
pub const CURRENT_VERSION: &str = env!("CARGO_PKG_VERSION");

#[derive(Clone, Debug, Deserialize)]
pub struct GitHubAsset {
    pub name: String,
    pub browser_download_url: String,
    pub size: u64,
}

#[derive(Clone, Debug, Deserialize)]
pub struct GitHubReleaseResponse {
    pub tag_name: String,
    pub html_url: String,
    pub name: Option<String>,
    pub body: Option<String>,
    pub published_at: Option<String>,
    #[serde(default)]
    pub assets: Vec<GitHubAsset>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct ReleaseInfo {
    pub version: String,
    pub tag_name: String,
    pub html_url: String,
    pub title: String,
    pub body: String,
    pub published_at: String,
    pub appimage_download_url: Option<String>,
    pub appimage_size: Option<u64>,
}

/// Membandingkan semver sederhana (major.minor.patch)
pub fn is_newer_version(current: &str, latest: &str) -> bool {
    let clean_current = current.trim().trim_start_matches('v');
    let clean_latest = latest.trim().trim_start_matches('v');

    let parse_nums = |s: &str| -> Vec<u64> {
        s.split('.')
            .map(|part| part.chars().take_while(|c| c.is_ascii_digit()).collect::<String>())
            .filter_map(|p| p.parse::<u64>().ok())
            .collect()
    };

    let curr_parts = parse_nums(clean_current);
    let latest_parts = parse_nums(clean_latest);

    for (c, l) in curr_parts.iter().zip(latest_parts.iter()) {
        if l > c {
            return true;
        } else if l < c {
            return false;
        }
    }

    latest_parts.len() > curr_parts.len()
}

/// Mengambil rilis terbaru dari GitHub API
pub fn fetch_latest_release(repo: &str) -> Result<Option<ReleaseInfo>, String> {
    let url = format!("https://api.github.com/repos/{}/releases/latest", repo);

    let output = Command::new("curl")
        .arg("-s")
        .arg("-L")
        .arg("--max-time")
        .arg("10")
        .arg("-H")
        .arg("Accept: application/vnd.github.v3+json")
        .arg("-H")
        .arg("User-Agent: madsquash-updater")
        .arg(&url)
        .output()
        .map_err(|e| format!("Gagal menjalankan curl: {}", e))?;

    if !output.status.success() {
        return Err("Gagal terhubung ke GitHub".to_string());
    }

    let stdout_str = String::from_utf8_lossy(&output.stdout);

    // Cek jika rilis tidak ditemukan / 404
    if stdout_str.contains("\"message\": \"Not Found\"")
        || stdout_str.contains("\"message\":\"Not Found\"")
    {
        return Ok(None);
    }

    let parsed: GitHubReleaseResponse = serde_json::from_str(&stdout_str)
        .map_err(|e| format!("Format respon GitHub tidak valid: {}", e))?;

    let latest_tag = parsed.tag_name.clone();
    let version = latest_tag.trim_start_matches('v').to_string();

    // Cari asset AppImage jika ada
    let appimage_asset = parsed.assets.iter().find(|a| {
        let n = a.name.to_lowercase();
        n.ends_with(".appimage") && (n.contains("x86_64") || n.contains("amd64"))
    });

    let appimage_download_url = appimage_asset.map(|a| a.browser_download_url.clone());
    let appimage_size = appimage_asset.map(|a| a.size);

    Ok(Some(ReleaseInfo {
        version,
        tag_name: latest_tag,
        html_url: parsed.html_url,
        title: parsed.name.unwrap_or_else(|| parsed.tag_name.clone()),
        body: parsed.body.unwrap_or_default(),
        published_at: parsed.published_at.unwrap_or_default(),
        appimage_download_url,
        appimage_size,
    }))
}

/// Mendapatkan path file AppImage yang sedang berjalan saat ini jika ada
pub fn current_appimage_path() -> Option<PathBuf> {
    if let Ok(appimage) = std::env::var("APPIMAGE") {
        let path = PathBuf::from(appimage);
        let name = path
            .file_name()
            .and_then(|n| n.to_str())
            .unwrap_or("")
            .to_lowercase();
        if name.contains("madsquash") {
            return Some(path);
        }
    }
    None
}

/// Download berkas AppImage baru dan timpa berkas saat ini
pub fn download_and_replace_appimage(
    download_url: &str,
    target_path: &PathBuf,
) -> Result<(), String> {
    let tmp_file = target_path.with_extension("download_tmp");

    // Download menggunakan curl
    let status = Command::new("curl")
        .arg("-s")
        .arg("-L")
        .arg("--max-time")
        .arg("300")
        .arg("-o")
        .arg(&tmp_file)
        .arg(download_url)
        .status()
        .map_err(|e| format!("Gagal mengunduh: {}", e))?;

    if !status.success() {
        let _ = std::fs::remove_file(&tmp_file);
        return Err("Download gagal atau terputus".to_string());
    }

    // Set izin eksekusi chmod +x
    #[cfg(unix)]
    {
        use std::os::unix::fs::PermissionsExt;
        if let Ok(metadata) = std::fs::metadata(&tmp_file) {
            let mut perms = metadata.permissions();
            perms.set_mode(0o755);
            let _ = std::fs::set_permissions(&tmp_file, perms);
        }
    }

    // Timpa file target lama
    std::fs::rename(&tmp_file, target_path)
        .map_err(|e| format!("Gagal menimpa berkas AppImage: {}", e))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_newer_version() {
        assert!(is_newer_version("0.1.0", "v0.1.1"));
        assert!(is_newer_version("0.1.0", "0.2.0"));
        assert!(is_newer_version("0.1.0", "1.0.0"));
        assert!(!is_newer_version("0.1.0", "0.1.0"));
        assert!(!is_newer_version("0.2.0", "0.1.9"));
        assert!(!is_newer_version("1.0.0", "0.9.9"));
    }
}
