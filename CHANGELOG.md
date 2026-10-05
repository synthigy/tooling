# Changelog

## 0.1.11

- Portal: an engine killed by the operating system — the kernel's
  out-of-memory killer, a JVM crash — is started again by the portal. Before,
  the portal took the kill for a deliberate stop and exited with it, leaving
  the instance down. An engine that fails while it boots still stays down,
  with the reason shown in the portal.
- Portal: its own log lines carry a UTC timestamp and say how the engine
  exited; a kill by the out-of-memory killer names itself.
- Portal: `synthigy service install` takes over an instance already started
  with `synthigy up` (one engine restart) instead of leaving the service
  inactive, and the unit leaves an out-of-memory kill to the portal
  (`OOMPolicy=continue`).
- Portal: `synthigy down` stops the portal first, which stops its engine.
- Portal: the Engine log shows times in your browser's timezone instead of the
  server's, which on a cloud host is UTC.
- Portal: stopping the engine after a restart no longer reports a failed start.
- Web components: unchanged since 0.1.9.

## 0.1.10

- Portal: an app jar — your code with the engine embedded — runs under
  `synthigy up` like a release. Point `SYNTHIGY_JAR` at it and list your
  namespaces and modules in `.env` (`SYNTHIGY_REQUIRE`, `SYNTHIGY_MODULES`);
  the console then starts, stops and restarts them and creates the first
  superuser. Needs engine `0.2.13-SNAPSHOT` or newer.
- Portal: the Engine panel shows the version the running engine reports, and
  lists your app's modules plus any module that failed to start.
- Web components: unchanged since 0.1.9.

## 0.1.9

- Portal: `synthigy schema check` tells you whether your local `xsql/schema.json`
  still matches the deployed model, and every command warns on stderr when it
  doesn't — run `synthigy schema pull` to refresh it.
- Portal: the setup page links the console at the instance's
  `SYNTHIGY_IAM_ROOT_URL` instead of always `localhost:<port>`.
- Web components: rebuilt against engine v0.2.10.

## 0.1.8

- Portal: an instance upgraded from an older `.env` no longer tries to start
  its old pinned engine. The one-time `.env` upgrade now also resets a
  `SYNTHIGY_VERSION` pin to `latest` — a pin beside the old keys names v0.2.6
  or older, which cannot run the `sqlite` / `postgres` / `postgres-clickhouse`
  bundles ("release v0.2.6 has no combo postgres"). An instance already
  upgraded by 0.1.7 gets an error naming the fix: set
  `SYNTHIGY_VERSION=latest` in `.env`, or run `synthigy up --version latest`.
- Web components: unchanged since 0.1.7.

## 0.1.7

- Portal (`synthigy` CLI + daemon) binaries for this release — upgrade before
  running engine v0.2.8: it runs the new bundles (`sqlite`, `postgres`,
  `postgres-clickhouse`) and upgrades an older instance's `.env` in place on
  the next `synthigy up`.
- Portal console: the Observability panel is now **Dataline** (audit history,
  logs and traffic), and turns red when the engine reports it degraded.
- Web components: the data console reworked — editor, help, execution,
  schema-aware variables.
- Web components: shared design tokens, notifications and sign-in across the
  modeler, data console and log cockpit.

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
