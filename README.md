# @synthigy/tooling

Synthigy's browser tooling, desktop app and portal binaries.

## Install

**Portal CLI:**

```sh
curl -fsSL https://raw.githubusercontent.com/synthigy/tooling/main/install.sh | sh
```

```powershell
irm https://raw.githubusercontent.com/synthigy/tooling/main/install.ps1 | iex
```

**Web components**, straight from a CDN:

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@synthigy/tooling@0.2.0/dist/tooling.css">
<script src="https://cdn.jsdelivr.net/npm/@synthigy/tooling@0.2.0/dist/tooling.js"></script>

<synthigy-tools endpoint="https://api.example.com" open></synthigy-tools>
```

or from npm:

```sh
npm install @synthigy/tooling
```

## What's here

- `dist/tooling.js` registers `<synthigy-tools>`: the modeler, the data console
  and the log viewer as one element (Model · Data · Logs).
- `dist/tooling.css` is optional: it loads the Inter and JetBrains Mono fonts
  from Google Fonts. Without it the elements use the system fonts and make no
  request to a third party. Every color token carries a fallback, so they
  render correctly either way.
- GitHub Releases carry `synthigy-portal-{os}-{arch}` static binaries
  (`install.sh`/`install.ps1` above fetch these automatically) and the
  desktop app installers.
- `desktop/` is the desktop app's source as released: the compiled tools and
  the Tauri host. GitHub Actions builds the installers from it on every tag.

## Web components

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@synthigy/tooling@0.2.0/dist/tooling.css">
<script src="https://cdn.jsdelivr.net/npm/@synthigy/tooling@0.2.0/dist/tooling.js"></script>

<synthigy-tools endpoint="https://api.example.com" open></synthigy-tools>
```

The same files are served straight from this repository's tags:
`https://cdn.jsdelivr.net/gh/synthigy/tooling@v0.2.0/dist/tooling.js`.

Attributes:

- `open`: shows the tools; the element clears it itself when the user closes them.
- `endpoint`: the engine's base URL. Defaults to the page's origin.
- `tool`: `model`, `data` or `logs`, the tool to open on. Without it the
  element opens on the last tool used.

Properties: `xsql` (an array of `.xsql` file texts to open in Data), `onClose`.

Method: `showTool("model" | "data" | "logs")` switches an open element, or sets
the tool the next open starts on. Event: `close`.

### Signing in

The element signs in against `endpoint` itself as the Synthigy Tools app
(authorization code + PKCE, OpenID Connect). The host does three things:

1. Serve `dist/callback.html` from this package at
   `https://<your origin>/synthigy-callback.html`. It must be a real static
   file, outside any SPA fallback, without a `Cross-Origin-Opener-Policy`
   header and without injected scripts. It has no script itself.
2. Add that exact URL to the Redirect URIs of the Synthigy Tools app
   (console → Apps). This also lets the page call the engine across origins (CORS).
3. If the page sends a `Cross-Origin-Opener-Policy`, make it
   `same-origin-allow-popups`. With a `Content-Security-Policy`, allow the
   engine in `connect-src` and `frame-src`.

How it behaves:

- **Sign In** opens a small window on the engine. When the engine already has a
  session, the window closes without asking for a password. The page itself
  never navigates.
- Tokens stay in memory. The engine renews them in a hidden frame, and after a
  reload the session comes back the same way.
- When renewal fails, the tools stay open with their drafts and undo:
  - While the token is still valid, a "Session expires soon" notice offers Continue.
  - Once it has expired, the Sign In screen covers the tools, and Continue
    picks up where the user left off.
- **Sign Out** ends the engine session in the background. If the engine does not
  confirm it, a notice says so and offers Try again. Signing out needs engine
  0.4.0 or newer.
- When the page and the engine are on different sites, Safari, Firefox, every
  iOS browser and private windows keep the engine's cookie out of the hidden
  frame. There, a reload costs one click on Sign In and an expired token one
  click on Continue, with no password while the engine session lives. To
  avoid it, put the engine on your site: `app.example.com` and
  `engine.example.com` are the same site and renew silently in every browser.

## Portal

The installers above put a `synthigy` binary on your PATH (`~/.synthigy/bin`,
override with `SYNTHIGY_INSTALL_DIR`). Pin a version:

```sh
curl -fsSL https://raw.githubusercontent.com/synthigy/tooling/main/install.sh | sh -s -- v0.2.0
```

Binaries and `sha256sums-portal.txt` are attached to each release here.

## Desktop app

The same tools as a native app for macOS, Windows and Linux, with an MCP
server an agent can drive the modeler through (`synthigy-desktop --mcp`).
Each release carries the installers: `.dmg` (macOS, Intel and Apple Silicon),
`.msi` and `-setup.exe` (Windows), `.deb`, `.rpm` and `.AppImage` (Linux),
with `sha256sums-desktop.txt`.

The installers are not signed yet. On macOS, open the app the first time with
right-click → Open; on Windows, choose "More info" → "Run anyway" in
SmartScreen.
