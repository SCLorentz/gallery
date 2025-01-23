#!/bin/bash

APP_NAME="Gallery"
ICON_NAME="Gallery.icns" # TODO: add icon
BINARY_NAME="gallery"

# Criar estrutura do .app
mkdir -p "$APP_NAME.app/Contents/MacOS"
mkdir -p "$APP_NAME.app/Contents/Resources"

# Copiar binário e ícone
cp "target/release/$BINARY_NAME" "$APP_NAME.app/Contents/MacOS/"
cp "$ICON_NAME" "$APP_NAME.app/Contents/Resources/"

# Criar Info.plist
cat << EOF > "$APP_NAME.app/Contents/Info.plist"
<?xml version="1.0" encoding="UTF-8"?>
<!DOCTYPE plist PUBLIC "-//Apple//DTD PLIST 1.0//EN" "http://www.apple.com/DTDs/PropertyList-1.0.dtd">
<plist version="1.0">
<dict>
    <key>CFBundleDisplayName</key>
    <string>$APP_NAME</string>
    <key>CFBundleExecutable</key>
    <string>$BINARY_NAME</string>
    <key>CFBundleIdentifier</key>
    <string>com.example.$APP_NAME</string>
    <key>CFBundleVersion</key>
    <string>1.0</string>
    <key>CFBundlePackageType</key>
    <string>APPL</string>
    <key>CFBundleIconFile</key>
    <string>${ICON_NAME%.icns}</string>
</dict>
</plist>
EOF