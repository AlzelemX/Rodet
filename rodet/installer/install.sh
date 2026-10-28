#!/bin/sh
# rodet_init.sh: the Rodet installer for Linux and macOS.
#
#   curl -fsSL https://github.com/AlzelemX/rodet/releases/latest/download/rodet_init.sh | sh
#
# Installs the Rodet toolchain (rodet, rodetc, rodetdoc, standard library) into
# ~/.rodet and adds ~/.rodet/bin to the PATH in your shell startup files, so
# `rodet` and `rodetc` work in a new terminal. There is no toolchain
# switching: Rodet is the only toolchain it knows about.
#
# Options (when running the file directly: sh rodet_init.sh ...):
#   --uninstall          remove Rodet and its PATH lines
#   --from-file <tar.xz> install from a local package instead of downloading
#
# Environment: RODET_HOME changes the install folder (default ~/.rodet).

set -eu

REPO="AlzelemX/rodet"
INSTALL_DIR="${RODET_HOME:-$HOME/.rodet}"
MARKER="# added by Rodet installer"

say() { printf '  %s\n' "$*"; }
step() { printf '  > %s...\n' "$*"; }
die() { printf '\nerror: %s\n' "$*" >&2; exit 1; }

detect_target() {
    os=$(uname -s)
    arch=$(uname -m)
    case "$arch" in
        x86_64 | amd64) arch=x86_64 ;;
        aarch64 | arm64) arch=aarch64 ;;
        *) die "unsupported CPU: $arch" ;;
    esac
    case "$os" in
        Linux) echo "$arch-unknown-linux-gnu" ;;
        Darwin)
            # A shell running under Rosetta reports x86_64 on Apple Silicon.
            if [ "$arch" = x86_64 ] && [ "$(sysctl -n hw.optional.arm64 2>/dev/null || echo 0)" = 1 ]; then
                arch=aarch64
            fi
            echo "$arch-apple-darwin"
            ;;
        *) die "unsupported OS: $os (on Windows use rodet_init.exe)" ;;
    esac
}

# Shell startup files that should put Rodet on the PATH.
profile_files() {
    echo "$HOME/.profile"
    [ -f "$HOME/.bashrc" ] || [ "$(uname -s)" = Linux ] && echo "$HOME/.bashrc"
    [ -f "$HOME/.bash_profile" ] && echo "$HOME/.bash_profile"
    # zsh is the default shell on macOS.
    [ -f "$HOME/.zshrc" ] || [ "$(uname -s)" = Darwin ] && echo "$HOME/.zshrc"
    return 0
}

remove_path_lines() {
    for f in $(profile_files); do
        [ -f "$f" ] || continue
        if grep -qF "$MARKER" "$f"; then
            { grep -vF "$MARKER" "$f" || true; } > "$f.rodet-tmp"
            mv "$f.rodet-tmp" "$f"
        fi
    done
}

add_path_lines() {
    remove_path_lines
    line="export PATH=\"$INSTALL_DIR/bin:\$PATH\" $MARKER"
    for f in $(profile_files); do
        printf '\n%s\n' "$line" >> "$f"
        say "    $f"
    done
}

download() {
    url="$1"
    out="$2"
    if command -v curl > /dev/null 2>&1; then
        curl -fL --retry 3 --progress-bar -o "$out" "$url"
    elif command -v wget > /dev/null 2>&1; then
        wget -q --show-progress -O "$out" "$url"
    else
        die "need curl or wget to download Rodet"
    fi
}

install() {
    from_file="$1"
    target=$(detect_target)
    say "Rodet installer"
    say "================"
    say ""
    say "platform: $target"
    say "install folder: $INSTALL_DIR"
    say ""

    tmp=$(mktemp -d)
    trap 'rm -rf "$tmp"' EXIT

    if [ -n "$from_file" ]; then
        package="$from_file"
    else
        package="$tmp/rodet.tar.xz"
        url="https://github.com/$REPO/releases/latest/download/rodet-$target.tar.xz"
        step "Downloading $url"
        download "$url" "$package"
    fi

    step "Unpacking"
    tar -xJf "$package" -C "$tmp" ||
        die "could not unpack (on Linux, install xz: sudo apt install xz-utils)"
    [ -x "$tmp/rodet/bin/rodetc" ] || die "$package does not look like a Rodet package"
    rm -rf "$INSTALL_DIR"
    mkdir -p "$(dirname "$INSTALL_DIR")"
    mv "$tmp/rodet" "$INSTALL_DIR"

    step "Checking the installation"
    "$INSTALL_DIR/bin/rodetc" --version > "$tmp/version" 2>&1 || {
        cat "$tmp/version" >&2
        die "rodetc does not run on this system (on Linux this usually means glibc is too old)"
    }
    say "    $(cat "$tmp/version")"
    say "    $("$INSTALL_DIR/bin/rodet" --version)"

    step "Adding $INSTALL_DIR/bin to your PATH in"
    add_path_lines

    # Rodet uses the system C compiler/linker to produce executables.
    if ! command -v cc > /dev/null 2>&1; then
        say ""
        say "NOTE: no C compiler/linker (cc) was found. Rodet needs one to build programs:"
        case "$target" in
            *apple-darwin) say "    xcode-select --install" ;;
            *) say "    sudo apt install build-essential   (Debian/Ubuntu)"
               say "    sudo dnf install gcc                (Fedora)" ;;
        esac
    fi

    say ""
    say "Rodet is installed. Open a NEW terminal (or run: export PATH=\"$INSTALL_DIR/bin:\$PATH\")"
    say "and try:"
    say "    rodet --version"
    say "    rodet new hello && cd hello && rodet run"
}

uninstall() {
    step "Removing Rodet PATH lines"
    remove_path_lines
    step "Deleting $INSTALL_DIR"
    rm -rf "$INSTALL_DIR"
    say "Rodet was removed. Open a new terminal for the PATH change to apply."
}

main() {
    from_file=""
    while [ $# -gt 0 ]; do
        case "$1" in
            --uninstall) uninstall; return ;;
            --from-file) [ $# -ge 2 ] || die "--from-file needs a path"; from_file="$2"; shift ;;
            -y | --yes) ;;
            -h | --help) sed -n '2,16p' "$0"; return ;;
            *) die "unknown argument: $1" ;;
        esac
        shift
    done
    install "$from_file"
}

main "$@"
