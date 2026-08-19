CONFIG_FILE="/etc/nixos/configuration.nix"
PACKAGES_FILE="/etc/nixos/packages.nix"

echo "[ch] Setting up declarative environment in $CONFIG_FILE"

if grep -q "\./packages.nix" "$CONFIG_FILE"; then
    echo "[ch] ./packages.nix is already in your imports. All good!!"
else
    echo "[ch] Creating backup of $CONFIG_FILE"
    ch --backup pkgs "$CONFIG_FILE"
    sudo sed -i '/imports =/,/]/ s/^\(\s*\)]/\1  .\/packages.nix\n\1]/' "$CONFIG_FILE"
    echo "[ch] ./packages.nix injected successfully."
fi

if [ ! -f "$PACKAGES_FILE" ]; then
    sudo tee "$PACKAGES_FILE" > /dev/null <<EOF
{ pkgs, ... }:

{
  environment.systemPackages = with pkgs; [
    # ch_packages_start
    # ch_packages_end
  ];
}
EOF
    echo "[ch] $PACKAGES_FILE created successfully."
fi