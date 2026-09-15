# @synthigy/tooling

Synthigy's browser tooling and portal binaries.

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
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@synthigy/tooling@0.1.1/dist/tooling.css">
<script src="https://cdn.jsdelivr.net/npm/@synthigy/tooling@0.1.1/dist/modeling.js"></script>

<synthigy-data-modeling endpoint="https://api.example.com" open></synthigy-data-modeling>
```

or from npm:

```sh
npm install @synthigy/tooling
```

## What's here

- `dist/modeling.js` registers `<synthigy-data-modeling>`, `<synthigy-data-console>`
  and `<synthigy-log-cockpit>`.
- `dist/tooling.css` is optional — the Inter font the components style
  themselves with. Every color token carries a fallback, so the elements
  render correctly without it.
- GitHub Releases carry `synthigy-portal-{os}-{arch}` static binaries
  (`install.sh`/`install.ps1` above fetch these automatically).

## Web components

```html
<link rel="stylesheet" href="https://cdn.jsdelivr.net/npm/@synthigy/tooling@0.1.1/dist/tooling.css">
<script src="https://cdn.jsdelivr.net/npm/@synthigy/tooling@0.1.1/dist/modeling.js"></script>

<synthigy-data-modeling endpoint="https://api.example.com" open></synthigy-data-modeling>
```

The same files are served straight from this repository's tags:
`https://cdn.jsdelivr.net/gh/synthigy/tooling@v0.1.1/dist/modeling.js`.

Attributes: `endpoint`, `open`. Properties: `tokenResolver`, `onClose`
(`<synthigy-data-console>` also takes `entities`, `schema`). Events: `close`,
`modeling-message`. Without a `tokenResolver` the element performs its own
OIDC login against `endpoint`.

## Portal

The installers above put a `synthigy` binary on your PATH (`~/.synthigy/bin`,
override with `SYNTHIGY_INSTALL_DIR`). Pin a version:

```sh
curl -fsSL https://raw.githubusercontent.com/synthigy/tooling/main/install.sh | sh -s -- v0.1.1
```

Binaries and `sha256sums-portal.txt` are attached to each release here.
