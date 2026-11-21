#!/bin/bash
set -euo pipefail
ROOT="$(cd "$(dirname "${BASH_SOURCE[0]}")/.." && pwd)"
PACKAGE="$ROOT/apps/macos"
OUTPUT="${1:-$ROOT/dist/MacPlay.app}"
BUILD_CONFIG=release
command -v swift >/dev/null || { echo "Swift toolchain is required" >&2; exit 1; }
command -v lipo >/dev/null || { echo "lipo is required (run this script on macOS)" >&2; exit 1; }
swift build --package-path "$PACKAGE" --configuration "$BUILD_CONFIG" --arch arm64
ARM_BIN="$(swift build --package-path "$PACKAGE" --configuration "$BUILD_CONFIG" --arch arm64 --show-bin-path)/MacPlay"
swift build --package-path "$PACKAGE" --configuration "$BUILD_CONFIG" --arch x86_64
X86_BIN="$(swift build --package-path "$PACKAGE" --configuration "$BUILD_CONFIG" --arch x86_64 --show-bin-path)/MacPlay"
test -x "$ARM_BIN" || { echo "arm64 executable not found: $ARM_BIN" >&2; exit 1; }
test -x "$X86_BIN" || { echo "x86_64 executable not found: $X86_BIN" >&2; exit 1; }
CONTENTS="$OUTPUT/Contents"
mkdir -p "$CONTENTS/MacOS"
lipo -create "$ARM_BIN" "$X86_BIN" -output "$CONTENTS/MacOS/MacPlay"
cat > "$CONTENTS/Info.plist" <<'PLIST'
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleExecutable</key><string>MacPlay</string>
    <key>CFBundleIdentifier</key><string>dev.macplay.app</string>
    <key>CFBundleName</key><string>MacPlay</string>
    <key>CFBundleDisplayName</key><string>MacPlay</string>
    <key>CFBundlePackageType</key><string>APPL</string>
    <key>CFBundleShortVersionString</key><string>0.1.0</string>
    <key>CFBundleVersion</key><string>1</string>
    <key>LSMinimumSystemVersion</key><string>14.0</string>
    <key>NSHighResolutionCapable</key><true/>
</dict>
</plist>
PLIST
plutil -lint "$CONTENTS/Info.plist"
echo "Created universal app bundle: $OUTPUT"
echo "Not signed or notarized. Do not distribute until Developer ID signing and notarization are configured."
