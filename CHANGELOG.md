# Changelog

## 0.1.6

- **Breaking: the bundle is now `dist/tooling.js`** (was `dist/modeling.js`).
  Update the `<script>` URL; it registers the same three elements.
- Web components: browser login reworked — authorization code + PKCE through
  the engine's own login, with no refresh tokens; `oidc-client-ts` is no
  longer bundled.
- Web components: the natural-key marker in the modeler uses the new
  `--sy-key` token (blue) in both themes.
- Portal binaries: unchanged (latest is v0.1.5).

## 0.1.1

- Portal: operator console can stop and start the engine from the Health
  panel — the daemon and console stay up while the engine is down, modules
  show as offline, Start relaunches the same version and combo. Restart now
  asks for confirmation. Portal binaries and the installer ship from this
  repo (`synthigy/tooling`); `synthigy version` reports the release tag.
- Web components: unchanged from 0.1.0.

## 0.1.0

- Initial release: `<synthigy-data-modeling>`, `<synthigy-data-console>`,
  `<synthigy-log-cockpit>` web components.
