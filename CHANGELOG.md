# Changelog

## 0.1.15

Portal binaries and web components ship together again. Made for engine 0.3.0,
whose single system dataset, Synthigy, the modeler now knows about.

- Portal: `synthigy portal` opens the portal; `synthigy console` still works
  as an alias, and `synthigy status` labels the row `portal`.
- Portal: with `SYNTHIGY_IAM_ROOT_URL` set, every page shows the instance under
  the logo (a link to it) and in the browser tab title, so a portal reached
  through an SSH tunnel — always `localhost:7888` — still says which server it
  is.
- Portal: after it restarts a crashed engine (an out-of-memory kill, for one),
  the portal shows when it happened, why, and how many times since the portal
  started. A failed restart names the original cause too.
- Portal: `synthigy whoami` on a service profile signs in as the client and
  prints the client id, granted scope and token lifetime. Engine 0.3.0's
  UserInfo refuses service tokens, which never carry the `openid` scope.
- Portal: `SYNTHIGY_SERVER_TRUST_PROXY` is a text field in Effective
  configuration, not a switch. It takes the IPs or CIDRs of your proxies,
  comma separated (`127.0.0.1/32,::1/128` for a proxy on the same host). Engine
  0.3.0 no longer accepts `true`.
- Portal: `synthigy iam add-client` requires `--id`, because the server does
  not generate one.
- Portal: the upgrade progress names the new Synthigy model step.
- Modeler: the system datasets, Synthigy and OAuth, carry a "system" badge.
  They are read only unless you are a superuser, because they change only
  through engine upgrades, and only a superuser can delete them or their
  versions. A superuser's deploy that changes system entities shows a warning,
  and Deploy turns into "Deploy anyway".
- Modeler, deploy drawer: the engine checks every change before you deploy.
  Entity name conflicts, duplicate field names, changes to system entities,
  broken RLS guards and taken version names show as banners that block Deploy.
  The button says "Checking…" while it waits.
- Modeler: a deployed version is read only throughout. The entity and relation
  drawers dim their editing controls, the entity drawer hides its add and
  delete buttons, the deploy drawer its revert buttons, and undo no longer
  records changes to a deployed version.
- Modeler: "New version x.y.z" next to every read-only note saves the model as
  the next patch version in one click; the deployed version is left as it is.
  In Versions, Save becomes New when the name belongs to a deployed version.
- Modeler: while a deploy, recall or destroy runs on the engine, the deploy
  drawer says which and by whom, and Deploy and the delete buttons are disabled
  until it finishes.
- Modeler: deleting the version you have open loads the last deployed version,
  or the newest draft, instead of leaving an empty canvas.
- Modeler, entity drawer, RLS: the path picker has a "Match all (AND)" switch.
  With it off, each selected path becomes its own guard row (OR). New rows take
  their Read/Write/Delete grants in the picker. Paths that already have a row
  are greyed out, and paths are lettered.
- Modeler, entity drawer, RLS: a warning appears when no guard row grants Read
  (nobody can read the rows), Write or Delete. "+ Add Guard Row" moved above
  the rows.
- Removed the `login-hint` attribute added in 0.1.14. Engine 0.3.0 can name a
  demo account on its own login page instead (engine console → System → Custom
  pages), so the components carry no demo text.
- Modeler notifications stay bottom-right while a drawer is open, drawn over
  the drawer (they used to jump to its left).

## 0.1.14

Web components only; the portal binaries stay at 0.1.12.

- Modeler, entity drawer: Tab and Enter add the attribute and keep the cursor
  in the new-attribute row, so attributes can be typed one after another.
- Modeler, entity drawer: the inactive-attributes toggle shows its state —
  "Inactive attributes present" or "Showing inactive attributes", with a
  count — and is hidden when there are none.
- Modeler, entity drawer: attribute types are listed most used first; `transit`
  is no longer offered for new attributes (existing ones still show it).
- Modeler, data console, log cockpit: opening a tool shows "connecting to
  <host> …" instead of a blank screen, and "can't reach <host>" with Retry when
  the engine is down, instead of a Sign In that leads to a refused page.
- New `login-hint` attribute on all three elements: shows a demo login under
  Sign In.

## 0.1.13

Web components only; the portal binaries stay at 0.1.12.

- Modeler, deploy drawer: a change that only deletes entities or relations can
  be deployed; before, the drawer said the model matched the deployed version.
- Modeler, deploy drawer: Suppress works on an added entity or relation, and
  reverting an added UNIQUE attribute keeps the deployed unique key.
- Modeler, deploy drawer: entities shared with other models no longer show
  false Audit, RBAC or History warnings; settings this model doesn't set are
  shown as left to the other models.
- Modeler, deploy drawer: reactivated attributes, renames and soft-deletes
  each show once with their detail. RLS guard rows can be reverted.
- Modeler, entity drawer: an always-present row for a new attribute, and a
  toggle to show or hide inactive attributes (read-only).
- Modeler: pulling an entity restores its position from this dataset's last
  deploy; a relation can't be pulled before its entity.
- Modeler: messages appear bottom-right.
- Log cockpit: level filter on the Dataset and Auth lenses.
- Data console and log cockpit: sign-out button.
- Smaller dialog buttons.
- Smaller bundle: about 140 KB less (cljs.pprint dropped, logo SVGs optimized).

## 0.1.12

- Portal: SQLite files have one fixed place. `synthigy.db`, `audit.db` and
  `logs.db` live in `<home>/db/`; `SQLITE_PATH`, `SQLITE_AUDIT_PATH` and
  `SQLITE_LOGS_PATH` are gone. The setup page and the Database and Dataline
  panels show the paths instead of asking for them, and `up` no longer writes
  them into `.env`.
- Portal: `synthigy status` shows what the running engine runs — the release
  version and bundle, or the local jar.
- Web components: unchanged since 0.1.9.

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
