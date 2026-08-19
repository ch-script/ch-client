PACKAGE="$1"
PACKAGES_FILE="/etc/nixos/packages.nix"

if [ -z "$PACKAGE" ]; then
    echo "[ch] Error: Please provide a package name to remove."
    exit 1
fi

echo "[ch] Removing $PACKAGE"

if ! grep -q "^\s*$PACKAGE\s*$" "$PACKAGES_FILE"; then
    echo "[ch] Package '$PACKAGE' is not declared in $PACKAGES_FILE. So there's really nothing to do."
    exit 0
fi

echo "[ch] Creating backup of $PACKAGES_FILE"
ch --backup pkgs "$PACKAGES_FILE"

sudo sed -i "/^\s*$PACKAGE\s*$/d" "$PACKAGES_FILE"

echo "[ch] Rebuilding the system"
sudo nixos-rebuild switch