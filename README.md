# Madsquash

<p align="center">
  <img src="assets/madsquash.svg" alt="Madsquash Logo" width="96" height="96" />
</p>

<p align="center">
  <strong>Pengelola Berkas AppImage Modern, Cepat, dan Ringan Berbasis Rust murni untuk Linux.</strong>
</p>

<p align="center">
  <a href="https://github.com/AchmadEko003/madsquash/releases/latest">
    <img src="https://img.shields.io/github/v/release/AchmadEko003/madsquash?color=5bbfe6&label=release" alt="Latest Release" />
  </a>
  <img src="https://img.shields.io/badge/rust-2024%20edition-orange.svg" alt="Rust 2024 Edition" />
  <img src="https://img.shields.io/badge/gui-GPUI%20Kit-7bc7e8.svg" alt="GPUI Kit" />
  <img src="https://img.shields.io/badge/platform-Linux-blue.svg" alt="Platform Linux" />
  <a href="LICENSE">
    <img src="https://img.shields.io/badge/license-MIT-green.svg" alt="License" />
  </a>
</p>

---

## Tentang Madsquash

**Madsquash** adalah aplikasi pengelola AppImage modern yang dibangun sepenuhnya menggunakan bahasa pemrograman **Rust** dan framework UI GPU-accelerated (**GPUI**). Madsquash dirancang untuk memberikan pengalaman pengguna terbaik dalam mengelola, mengekstrak, mengintegrasikan, dan menjalankan berkas `.AppImage` di desktop Linux tanpa ketergantungan runtime yang berat.

---

## Unduh & Instalasi

Kunjungi halaman [**GitHub Releases**](https://github.com/AchmadEko003/madsquash/releases/latest) untuk mengunduh paket siap pakai sesuai distro Anda:

### 1. Ubuntu / Debian / Linux Mint / Pop!_OS (`.deb`)
```bash
# Unduh berkas .deb terbaru dari Releases, lalu jalankan:
sudo dpkg -i madsquash_*_amd64.deb
# Jika ada dependensi yang kurang:
sudo apt-get install -f
```

### 2. Fedora / RHEL / openSUSE (`.rpm`)
```bash
sudo rpm -i madsquash-*.x86_64.rpm
# atau menggunakan dnf:
sudo dnf install ./madsquash-*.x86_64.rpm
```

### 3. Arch Linux / Manjaro / EndeavourOS (`.pkg.tar.zst`)
```bash
sudo pacman -U madsquash-*-x86_64.pkg.tar.zst
```

### 4. Universal Portabel (`.AppImage`)
```bash
chmod +x Madsquash-*-x86_64.AppImage
./Madsquash-*-x86_64.AppImage
```

---

## Membangun dari Sumber (Build from Source)

### Kebutuhan Sistem (Prerequisites)
Pastikan Anda telah memasang Rust toolchain dan pustaka pengembangan sistem:

**Ubuntu / Debian:**
```bash
sudo apt update
sudo apt install -y build-essential libxcb1-dev libxkbcommon-dev libxkbcommon-x11-dev libxcb-xkb-dev
```

**Fedora:**
```bash
sudo dnf install libxcb-devel libxkbcommon-devel libxkbcommon-x11-devel
```

**Arch Linux:**
```bash
sudo pacman -S --needed base-devel libxcb libxkbcommon libxkbcommon-x11
```

### Kompilasi & Jalankan
```bash
# Clone repositori
git clone https://github.com/AchmadEko003/madsquash.git
cd madsquash

# Jalankan mode pengembangan
cargo run

# Atau build binary release teroptimasi
cargo build --release
```
Binary release yang dihasilkan berada di `target/release/madsquash` dengan ukuran ramping (~21 MB).

---

## Membuat Paket Rilis (.deb, .rpm, .pkg.tar.zst, .AppImage)

Madsquash menyertakan skrip otomatisasi packaging lokal:

```bash
# Pastikan sudah build release
cargo build --release

# Jalankan skrip pembuatan paket (argumen adalah versi)
./scripts/build-packages.sh 0.1.0
```
Seluruh berkas installer akan tersedia di folder `dist/`:
- `dist/madsquash_0.1.0_amd64.deb`
- `dist/madsquash-0.1.0-1.x86_64.rpm`
- `dist/madsquash-0.1.0-1-x86_64.pkg.tar.zst`
- `dist/Madsquash-0.1.0-x86_64.AppImage`

---

## Struktur Direktori Data Madsquash

Ketika Anda memasang berkas AppImage menggunakan Madsquash, file akan ditata rapi pada direktori standar XDG Linux:

```
~/.local/share/
├── applications/
│   ├── madsquash.desktop               # Launcher Madsquash
│   └── madsquash-<id>.desktop          # Shortcut menu untuk aplikasi yang dipasang
└── madsquash/
    ├── apps.json                       # Katalog aplikasi terpasang
    ├── app/
    │   └── <id>/
    │       └── app.AppImage            # Berkas binary AppImage
    └── icon/
        └── <id>.<png/svg>              # Ikon aplikasi hasil ekstraksi SquashFS
```

---

## ✦ Lisensi

Proyek ini dilisensikan di bawah [**MIT License**](LICENSE).
