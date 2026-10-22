#!/usr/bin/env bash
set -euo pipefail

VERSION="${1:-0.1.0}"
VERSION="${VERSION#v}" # strip leading 'v' if present
ARCH="amd64"
RPM_ARCH="x86_64"
DIST_DIR="$(pwd)/dist"
RELEASE_BIN="$(pwd)/target/release/madsquash"

echo "=== Building Madsquash v${VERSION} (${ARCH}) Packages ==="

if [ ! -f "${RELEASE_BIN}" ]; then
    echo "Compiling release binary..."
    cargo build --release
fi

rm -rf "${DIST_DIR}"
mkdir -p "${DIST_DIR}"

# -------------------------------------------------------------
# 1. Build .deb package
# -------------------------------------------------------------
echo "--> Creating .deb package..."
DEB_DIR="$(mktemp -d)/madsquash_${VERSION}_${ARCH}"
mkdir -p "${DEB_DIR}/DEBIAN"
mkdir -p "${DEB_DIR}/usr/bin"
mkdir -p "${DEB_DIR}/usr/share/applications"
mkdir -p "${DEB_DIR}/usr/share/icons/hicolor/scalable/apps"

install -m 755 "${RELEASE_BIN}" "${DEB_DIR}/usr/bin/madsquash"
install -m 644 assets/madsquash.desktop "${DEB_DIR}/usr/share/applications/madsquash.desktop"
install -m 644 assets/madsquash.svg "${DEB_DIR}/usr/share/icons/hicolor/scalable/apps/madsquash.svg"

cat <<EOF > "${DEB_DIR}/DEBIAN/control"
Package: madsquash
Version: ${VERSION}
Section: utils
Priority: optional
Architecture: ${ARCH}
Maintainer: Achmad Eko <achmad@madsquash.local>
Depends: libxcb1, libxkbcommon0, libxkbcommon-x11-0
Description: Modern pure-Rust AppImage manager and runner
 Madsquash is an ultra-fast, modern AppImage manager built in Rust,
 featuring instant SquashFS extraction, desktop menu integration,
 and clean isolated catalog management.
EOF

dpkg-deb --build --root-owner-group "${DEB_DIR}" "${DIST_DIR}/madsquash_${VERSION}_amd64.deb"
rm -rf "${DEB_DIR}"
echo "✓ Created: ${DIST_DIR}/madsquash_${VERSION}_amd64.deb"

# -------------------------------------------------------------
# 2. Build .rpm package
# -------------------------------------------------------------
echo "--> Creating .rpm package..."
if command -v alien >/dev/null 2>&1; then
    (
        cd "${DIST_DIR}"
        alien -r --nodeps "madsquash_${VERSION}_amd64.deb"
    )
    echo "✓ Created .rpm package in ${DIST_DIR}"
elif command -v rpmbuild >/dev/null 2>&1; then
    RPM_ROOT="$(mktemp -d)/rpmbuild"
    mkdir -p "${RPM_ROOT}"/{BUILD,RPMS,SOURCES,SPECS,SRPMS}
    mkdir -p "${RPM_ROOT}/BUILDROOT/madsquash-${VERSION}-1.${RPM_ARCH}"/usr/{bin,share/applications,share/icons/hicolor/scalable/apps}
    install -m 755 "${RELEASE_BIN}" "${RPM_ROOT}/BUILDROOT/madsquash-${VERSION}-1.${RPM_ARCH}/usr/bin/madsquash"
    install -m 644 assets/madsquash.desktop "${RPM_ROOT}/BUILDROOT/madsquash-${VERSION}-1.${RPM_ARCH}/usr/share/applications/"
    install -m 644 assets/madsquash.svg "${RPM_ROOT}/BUILDROOT/madsquash-${VERSION}-1.${RPM_ARCH}/usr/share/icons/hicolor/scalable/apps/"
    
    cat <<EOF > "${RPM_ROOT}/SPECS/madsquash.spec"
Name:           madsquash
Version:        ${VERSION}
Release:        1%{?dist}
Summary:        Modern pure-Rust AppImage manager and runner
License:        MIT OR Apache-2.0
BuildArch:      ${RPM_ARCH}

%description
Modern pure-Rust AppImage manager and runner.

%files
/usr/bin/madsquash
/usr/share/applications/madsquash.desktop
/usr/share/icons/hicolor/scalable/apps/madsquash.svg
EOF

    rpmbuild --define "_topdir ${RPM_ROOT}" -bb "${RPM_ROOT}/SPECS/madsquash.spec"
    cp "${RPM_ROOT}"/RPMS/${RPM_ARCH}/*.rpm "${DIST_DIR}/"
    rm -rf "${RPM_ROOT}"
    echo "✓ Created .rpm package in ${DIST_DIR}"
else
    echo "Notice: Neither alien nor rpmbuild found, skipping local .rpm generation (will run in GitHub Actions)."
fi

# -------------------------------------------------------------
# 3. Build .AppImage package
# -------------------------------------------------------------
echo "--> Creating .AppImage package..."
APPDIR="$(mktemp -d)/AppDir"
mkdir -p "${APPDIR}/usr/bin"
mkdir -p "${APPDIR}/usr/share/applications"
mkdir -p "${APPDIR}/usr/share/icons/hicolor/scalable/apps"

install -m 755 "${RELEASE_BIN}" "${APPDIR}/usr/bin/madsquash"
install -m 755 assets/AppRun "${APPDIR}/AppRun"
install -m 644 assets/madsquash.desktop "${APPDIR}/madsquash.desktop"
install -m 644 assets/madsquash.desktop "${APPDIR}/usr/share/applications/madsquash.desktop"
install -m 644 assets/madsquash.svg "${APPDIR}/madsquash.svg"
install -m 644 assets/madsquash.svg "${APPDIR}/.DirIcon"
install -m 644 assets/madsquash.svg "${APPDIR}/usr/share/icons/hicolor/scalable/apps/madsquash.svg"

# Use existing appimagetool or download if in CI
APPIMAGETOOL=""
if command -v appimagetool >/dev/null 2>&1; then
    APPIMAGETOOL="appimagetool"
elif [ -f "./appimagetool-x86_64.AppImage" ]; then
    APPIMAGETOOL="./appimagetool-x86_64.AppImage"
fi

if [ -n "${APPIMAGETOOL}" ]; then
    ARCH="x86_64" ${APPIMAGETOOL} --appimage-extract-and-run "${APPDIR}" "${DIST_DIR}/Madsquash-${VERSION}-x86_64.AppImage"
    echo "✓ Created: ${DIST_DIR}/Madsquash-${VERSION}-x86_64.AppImage"
else
    echo "Notice: appimagetool not found locally, will be downloaded and executed in GitHub Actions."
fi
rm -rf "${APPDIR}"

echo "=== Packaging Complete ==="
ls -lh "${DIST_DIR}" || true
