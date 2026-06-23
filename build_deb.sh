#!/bin/bash
set -e

VERSION="2.0.0"
ARCH="amd64"
PKG_NAME="clack"
BUILD_DIR="build_deb"
OUT_DIR="$HOME/Projects/binaries"

# Create output directory
mkdir -p "$OUT_DIR"

# Clean old build
rm -rf "$BUILD_DIR"
mkdir -p "$BUILD_DIR/DEBIAN"
mkdir -p "$BUILD_DIR/usr/bin"
mkdir -p "$BUILD_DIR/usr/share/clack"
mkdir -p "$BUILD_DIR/usr/share/applications"
mkdir -p "$BUILD_DIR/usr/share/icons/hicolor/scalable/apps"

# 1. Control File
cat <<EOF > "$BUILD_DIR/DEBIAN/control"
Package: $PKG_NAME
Version: $VERSION
Architecture: $ARCH
Maintainer: ThisWasAryan
Depends: xdotool, ydotool, ydotoold, python3-pynput, python3-gi, python3-gi-cairo, gir1.2-gtk-4.0
Description: Behaviorally realistic human typing simulator
 Clack simulates human typing by injecting keystrokes securely via /dev/uinput.
 It includes a CLI simulation engine and a GTK4 graphical interface.
EOF

# 2. Postinst Script
cat <<EOF > "$BUILD_DIR/DEBIAN/postinst"
#!/bin/bash
set -e
echo "Configuring Clack udev rules for ydotool..."
echo 'KERNEL=="uinput", GROUP="input", MODE="0660", OPTIONS+="static_node=uinput"' > /etc/udev/rules.d/99-uinput.rules
udevadm control --reload-rules
udevadm trigger

# Try to enable ydotoold service if it exists
if systemctl list-unit-files | grep -q ydotoold; then
    systemctl enable --now ydotoold || true
fi

if [ -n "\$SUDO_USER" ]; then
    usermod -aG input "\$SUDO_USER"
    echo "Added \$SUDO_USER to the 'input' group. You may need to log out and back in."
fi

exit 0
EOF
chmod 755 "$BUILD_DIR/DEBIAN/postinst"

# 3. Binaries and Python files
# The binary name might be htype or clack depending on cargo configuration
if [ -f "target/release/htype" ]; then
    cp target/release/htype "$BUILD_DIR/usr/bin/clack"
else
    cp target/release/clack "$BUILD_DIR/usr/bin/clack"
fi
chmod 755 "$BUILD_DIR/usr/bin/clack"

cp -r gui "$BUILD_DIR/usr/share/clack/"
# Clean up any pycache before packaging
find "$BUILD_DIR/usr/share/clack/gui" -type d -name "__pycache__" -exec rm -r {} + || true

# Entrypoint script
cat <<EOF > "$BUILD_DIR/usr/bin/clack-gui"
#!/bin/bash
# Start ydotoold in background if not running
if ! pgrep ydotoold > /dev/null; then
    ydotoold &
    sleep 0.5
fi
export PYTHONPATH="/usr/share/clack:\$PYTHONPATH"
exec python3 -m gui "\$@"
EOF
chmod 755 "$BUILD_DIR/usr/bin/clack-gui"

# 4. Desktop Entry
cat <<EOF > "$BUILD_DIR/usr/share/applications/clack.desktop"
[Desktop Entry]
Name=Clack
Comment=Human Typing Simulator
Exec=clack-gui
Icon=clack
Terminal=false
Type=Application
Categories=Utility;
EOF

# 5. Simple SVG Icon
cat <<EOF > "$BUILD_DIR/usr/share/icons/hicolor/scalable/apps/clack.svg"
<svg xmlns="http://www.w3.org/2000/svg" viewBox="0 0 64 64">
  <rect width="64" height="64" rx="12" fill="#3B82F6"/>
  <text x="50%" y="50%" font-family="monospace" font-size="32" font-weight="bold" fill="white" text-anchor="middle" dominant-baseline="central">C_</text>
</svg>
EOF

# Build package
dpkg-deb --build "$BUILD_DIR" "${OUT_DIR}/${PKG_NAME}_v2_${ARCH}.deb"

echo "Success! Package built at ${OUT_DIR}/${PKG_NAME}_v2_${ARCH}.deb"
