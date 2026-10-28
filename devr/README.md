# DEVR+

The editor for Rodet. How to use it: [Doc Book → The DEVR+ Editor](https://alzelemx.github.io/rodet/start/devr.html).

| Folder | What |
|---|---|
| `core/` | The backend: files, terminals, search, Silk, and the language server (`rodet-analyzer`) |
| `ui/` | The editor (TypeScript, Monaco, xterm.js) |
| `app/` | The desktop app (Tauri): one window with `ui/`, talking to `core/` |
| `server/` | `devr-server`: the same editor in a browser, over a WebSocket |
| `plugin-api/` | The plugin API for plugin authors (`devr.d.ts`) and example plugins |

## Work on it

Needs Node.js 22 and Rodet (or Rust). On Linux also the WebKitGTK libraries
(`libwebkit2gtk-4.1-dev`).

```sh
cd devr
npm install
npm run tauri dev        # the desktop app, reloading on changes
npm run tauri build      # installers in target/release/bundle/
```

Without a desktop:

```sh
npm run build
cargo run -p devr-server -- --ui ui/dist   # then open http://127.0.0.1:7878
```

Releases are built by `.github/workflows/devr.yml` with the Rodet
compiler itself, on every push that changes `devr/`.
