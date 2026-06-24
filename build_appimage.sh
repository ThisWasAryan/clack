#!/bin/bash
set -e

OUT_DIR="$HOME/Projects/binaries"
APPDIR="Clack.AppDir"

mkdir -p "$OUT_DIR"
rm -rf "$APPDIR"
mkdir -p "$APPDIR/usr/bin"
mkdir -p "$APPDIR/usr/share/clack"

# Copy CLI binary and GUI
if [ -f "target/release/htype" ]; then
    cp target/release/htype "$APPDIR/usr/bin/clack"
else
    cp target/release/clack "$APPDIR/usr/bin/clack"
fi
chmod 755 "$APPDIR/usr/bin/clack"

cp -r gui "$APPDIR/usr/share/clack/"
python3 -m pip install pynput --target "$APPDIR/usr/share/clack"

# Bundle xdotool for AppImage
mkdir -p build_apt
pushd build_apt
apt-get download xdotool libxdo3
for pkg in *.deb; do
    dpkg-deb -x "$pkg" . || true
done
cp -a usr/bin/* ../$APPDIR/usr/bin/ || true
cp -a usr/lib/* ../$APPDIR/usr/lib/ || true
popd
rm -rf build_apt

find "$APPDIR/usr/share/clack" -type d -name "__pycache__" -exec rm -r {} + || true

# Entrypoint script
cat <<EOF > "$APPDIR/AppRun"
#!/bin/bash
HERE="\$(dirname "\$(readlink -f "\${0}")")"

# Start ydotoold using pkexec if not running
if ! pgrep ydotoold > /dev/null; then
    # We use pkexec because ydotoold requires root in AppImage without udev rules
    # Use --socket-perm 0666 to ensure all users can write to the /tmp/.ydotool_socket regardless of sudo/pkexec.
    pkexec ydotoold --socket-perm 0666 &
    sleep 0.5
fi

export PATH="\$HERE/usr/bin:\$PATH"
export PYTHONPATH="\$HERE/usr/share/clack:\$PYTHONPATH"
export LD_LIBRARY_PATH="\$HERE/usr/lib/x86_64-linux-gnu:\$HERE/usr/lib:\$LD_LIBRARY_PATH"
exec python3 -m gui "\$@"
EOF
chmod 755 "$APPDIR/AppRun"

# Desktop File
cat <<EOF > "$APPDIR/clack.desktop"
[Desktop Entry]
Name=Clack
Comment=Human Typing Simulator
Exec=AppRun
Icon=clack
Terminal=false
Type=Application
Categories=Utility;
EOF

# SVG Icon
cat <<EOF > "$APPDIR/clack.svg"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect width="64" height="64" rx="12" fill="#3B82F6"/>
  <text x="50%" y="50%" font-family="monospace" font-size="32" font-weight="bold" fill="white" text-anchor="middle" dominant-baseline="central">C_</text>
</svg>
EOF

# Download appimagetool if not exists
if [ ! -f "appimagetool-x86_64.AppImage" ] || [ ! -s "appimagetool-x86_64.AppImage" ]; then
    wget -O appimagetool-x86_64.AppImage "https://github.com/AppImage/AppImageKit/releases/download/continuous/appimagetool-x86_64.AppImage"
    chmod +x appimagetool-x86_64.AppImage
fi

# Build AppImage
./appimagetool-x86_64.AppImage --appimage-extract-and-run "$APPDIR" "${OUT_DIR}/clack-gui_v2.0.0_amd64.AppImage"

echo "Success! Package built at ${OUT_DIR}/clack-gui_v2.0.0_amd64.AppImage"
