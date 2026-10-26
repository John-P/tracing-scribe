#!/usr/bin/env bash
# Regenerates the README screenshots in .images/ by running each example and
# rendering its output with charmbracelet/freeze
# (https://github.com/charmbracelet/freeze).
#
# Usage: scripts/screenshots.sh
#
# Uses `freeze` from PATH if present, otherwise runs it through Nix.
set -euo pipefail

cd "$(dirname "$0")/.."

if command -v freeze >/dev/null; then
    freeze=(freeze)
else
    freeze=(nix run nixpkgs#charm-freeze --)
fi

# example name -> image name
shots=(
    "readme_success:success"
    "readme_error_handling:error_handling"
    "readme_extra_info:extra_info"
    "custom_colors:custom_colours"
)

cargo build --quiet --examples

for shot in "${shots[@]}"; do
    example="${shot%%:*}"
    image=".images/${shot##*:}.svg"
    # ConsoleLayer writes to stderr; drop stdout so only the tree is captured.
    cargo run --quiet --example "$example" 2>&1 >/dev/null |
        "${freeze[@]}" --language ansi --window --background '#0d1117' \
            --border.radius 8 --shadow.blur 10 --margin 20 --padding 20,40 \
            --output "$image"
done
