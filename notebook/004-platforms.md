# `5 targets` | Windows, Linux And macOS Builds

> 📅 **2026-10-22** · 🟢 Done · 🏷️ Tooling

Every push to `main` builds Rodet for five platforms, tests it, and
publishes one release with all of them.

| Platform | Package |
|---|---|
| Windows x86_64 | `rodet-x86_64-pc-windows-msvc.zip` |
| Linux x86_64 (glibc 2.39+) | `rodet-x86_64-unknown-linux-gnu.tar.xz` |
| Linux ARM64 (glibc 2.39+) | `rodet-aarch64-unknown-linux-gnu.tar.xz` |
| macOS Apple Silicon | `rodet-aarch64-apple-darwin.tar.xz` |
| macOS Intel | `rodet-x86_64-apple-darwin.tar.xz` |

### Before

```text
Build the compiler yourself on each computer (hours, 30 GB).
```

### After

```text
Push to main → about 1 hour later a new release:
v1.101.0-dev-rodet.<number>
```

Editing only documentation (`*.md`, `docbook/`, `notebook/`) does not start a
build.

<details><summary>Where</summary>

- `.github/workflows/build-rodet.yml`: builds and tests
- `.github/workflows/publish-release.yml`: publishes the release; can also be
  run by hand with the id of an earlier build to publish it again

</details>
