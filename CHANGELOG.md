# Changelog

## 0.3.0

Needs engine 0.4.2 or newer, as 0.2.2 did.

**Synthigy Studio updates itself.**

- **How:** it checks at launch, every six hours (time asleep counts), and when you come back to
  the window after an hour away. It downloads a new release in the background. Restart to update then appears on the engine list, and as an Update button in the
  tools header.
- **When it won't restart:** Restart to update waits while the canvas has unsaved changes, a deploy
  or save runs, or an agent call runs.
- **Minimum version:** a release can set one. Below it, Studio shows Update required until it
  has updated. That screen installs even with unsaved canvas changes; they come back after the
  restart.
- **Signing:** updates are signed, and Studio installs only a release signed for its own
  version.
- **Linux:** the AppImage updates itself. A `.deb` or `.rpm` install says when a release is out,
  and Download opens its page; install it by hand. A minimum version never forces it.
- **This release** installs by hand, once.

**Menus, links and one window.**

- **Menus:** Studio has a native menu: About and its version, Check for Updates…, Open Project…
  (⌘O), New Project…, the tools, Keyboard Shortcuts and the release notes.
- **Links:** `synthigy://open?path=<folder>` opens a project and `synthigy://engine?url=<endpoint>`
  an engine; `&tool=data` picks the tool. An engine you haven't added is only filled into the add
  form.
- **One window:** starting Studio again brings the open window forward instead of a second app,
  and a folder given on the command line opens there.

**Engines too old for Studio.** Studio checks an engine's version before opening it. An engine
older than 0.4.2 shows "needs engine 0.4.2+" in the engine list. Opening it shows what to
upgrade, instead of tools that fail. Agents get the same answer, and `engines` refuses to switch
to it.

**Local projects run side by side.**

- **Portal port:** a project's portal takes any free port each time it starts
  (`SYNTHIGY_PORTAL_PORT=0`); `synthigy portal` and Studio find it. `synthigy env init` writes it
  for a new project; an existing project gets it on its next `synthigy up` when its `.env` names no
  portal port. `~/.synthigy` and servers keep 7888.
- **Engine port:** `synthigy up` refuses an engine port anything else listens on (another engine,
  an ssh tunnel), and suggests a free one. Before, an engine could share `localhost:<port>` with a
  tunnel on the other loopback address, and Studio could end up on the wrong engine.
- **Use port N:** when a stopped project's engine port is taken, Studio offers to move the project to
  a free port in one click instead of Start.
- **Open project:** Studio refused a project that had an `xsql/schema.json` ("isn't in a Synthigy
  project"), because the CLI misreported where it found the project. Fixed in both.
- **Project list:** a project shows running only when its own engine answers. When another
  project's engine holds its port, the row says so and offers no Start.
- **Errors** in the project picker show under the card without moving it, and a failed start
  names its cause (for example a port already in use) instead of "did not become ready".

**Corrections to the 0.2.2 notes.** A user, group or role reference also filters by `xid`, not only
by `display_name` or `name`. The window keeps running behind other windows on macOS 14 and newer.

## 0.2.2

Needs engine 0.4.2 or newer: the modeler writes stamp attributes (model format
3), Data counts with operations, and Files reads its list through `/data`. The
modeler refuses to deploy to an older engine ("This engine still runs audit
fields"). Projects need the synthigy CLI 0.2.2 (`synthigy upgrade`).

**Breaking for agents.** `model_edit` has `set_history` instead of `set_audit`,
`get_entity` reports `history` instead of `audit`, and `remove_attribute` takes
the attribute out of the version. After updating, reconnect the MCP server
(`/mcp` in Claude Code): a running `--mcp` process keeps the old tool list. The
README now registers the server as `synthigy-studio`; an existing `synthigy`
registration keeps working.

**Projects** in Synthigy Studio. The start page has three tabs: Projects,
Engines and Agents.

- Open project… takes a folder with a `.synthigy/` (or a folder below one).
  Studio asks the synthigy CLI for the project's engine, adds it when it is
  missing, signs in and opens it, remote engines included.
- New project… runs `synthigy env init` in a folder, starts the engine, opens
  its setup page, and then the project.
- A recent project whose local engine is stopped gets Start. The project button
  in the header shows the folder and the engine, and offers Stop for an engine
  started with `synthigy up`.
- The project's `.xsql` files are the Data workspaces: typing saves the file,
  changes on disk reload, and a clash with unsaved edits asks which to keep. New
  creates a file; scratch workspaces stay, with Save to project.
- Every model save also writes `<dataset>.dataset.json` into the project. A
  model file whose version the engine lacks is offered with Load as draft.
- The Agents tab shows how to register the MCP server in Claude Code, Claude
  Desktop, Codex, ChatGPT, Gemini CLI and Cursor.
- A strip says when the synthigy CLI is missing, too old or broken, with the
  fix (Install, Upgrade or Reinstall).

**Agents** work in the agent's own project. Every MCP call carries the folder
the agent was started in, and Studio opens that folder's project and engine
first; it refuses while there are unsaved changes. `project_open` opens a
folder explicitly, and `get_app_state` names the project. In a project,
`data_open` writes the `.xsql` file. After a deploy, the agent's next result
says to run `synthigy schema pull` and regenerate. On macOS the window keeps
running behind other windows, so agent calls no longer stall.

**Stamps replace audit fields** (Model):

- A timestamp or user attribute can be stamped On create or On modify. "Add
  created/modified stamps" adds Created On, Created By, Modified On and Modified
  By in one step.
- Older models open with their audit fields as these four attributes, on the
  same columns. The entity's Audit tab is now History.
- Attribute rows show marks for stamp, classification and access. The settings
  button opens Attribute settings with Stamp, Classification and Access.
- Removing an attribute takes it out of the version. A removed deployed
  attribute can be restored, with its column and data. Recalling the last
  deployed version that holds an attribute drops its column and data.

**Counts and aggregates are operations** (Data), one line each:
`votes: count(ratings)`, `leads: count(actors.role = "lead")`,
`rating: avg(ratings.value)`, with completion, lint and help; `order by` takes
the alias. A `_count` / `_agg` block is now an error that names the
replacement, and the context panel's Aggregates toggles are gone. A user, group
or role reference filters and sorts only by its label: `display_name`, or
`name` for a group or role.

**Files:**

- Big folders load 200 files at a time with infinite scroll. Name, Size and
  Modified sort, and the filter searches the engine when not every file is
  loaded.
- A folder opens with one round of requests, and live updates reload only what
  changed.
- One file can be moved from its drawer. Copy link works for private files too,
  and says whether the link opens without signing in.
- The list shows the uploader's display name.

**Logs:** the Files lens has its own view, with time, level, action, subject,
folder, user and message, pickers for each, and drill-down.

What changed for users:

- With no dataset open, the canvas offers Open dataset and New dataset, and the
  open dataset can be closed from the switcher (×).
- A relation to a clone is a full relation, with a tab for each end.
- An error inside a tool shows "Something went wrong" with Copy report, and the
  tool recovers. Nothing is sent.
- Agent notices have their own look.
- `like` / `ilike` work on enum fields.

Fixes:

- + with no dataset open broke the interface.
- After Open dataset, ↓, Enter, keys stopped working until a click.
- A deploy made while the engine restarted stayed unseen until a reload; the
  modeler now refetches when the window gets focus.
- Dragging attributes to reorder them did nothing in Studio.
- Notifications could cover the action bar in Model.
- After a reload the modeler could reopen a stale version, and a save then
  failed with `VERSION_NAME_TAKEN`; clicking New version twice did the same.
- Right after a reload, the first Data, Logs and Files calls could fail with
  "Not connected".
- Entities an agent added in one batch landed in a staircase.

## 0.2.1

Needs engine 0.4.1 or newer: Synthigy Studio signs in with the engine's
Synthigy Studio client, which discovery now names `studio_client_id`.

**Breaking.** The desktop app is now Synthigy Studio: `Synthigy Studio.app`,
binary `synthigy-studio` (was `Synthigy.app`, `synthigy-desktop`), app id
`com.synthigy.studio`. Re-add the MCP server with the new path (see the
README). Saved engines and sign-ins do not carry over: add your engines again
and sign in once.

**License.** The web components (`tooling.js`) are now under the Business
Source License 1.1 with the engine's Additional Use Grant (`dist/LICENSE`).
0.2.0 and earlier stay MIT.

**Files**, a tool between Data and Logs (⌘3 / ⌥3), for engines
running the files module:

- A folder tree, with files in no folder (attached to records) pinned under it.
- Upload by button or drop, with progress; a name that is taken asks to
  replace (a new version) or keep both. Download, rename, move, delete,
  several files at once.
- Versions with download and restore, and a preview for images, PDFs and text.
- Folder settings: Public, Cache, and who may read, write and delete, with
  users and groups picked from a list.
- Go to path (⌘P), and live updates while someone else changes files.

Synthigy Studio's MCP server now lets an agent work in the window you are
looking at, as the account you signed in with:

- Model: read and edit the model, list datasets, open a version, start a
  dataset, and save drafts or a new version. An agent never deploys, recalls
  or deletes, never saves over a deployed version or a draft it didn't
  create, and refuses to load anything over unsaved changes on your canvas.
- Data: write, check and run XSQL, show it to you in Data, read your
  workspaces. How agent writes are handled is set per engine with the bot
  button in the header: ask every time, ask once per session, allow, or off.
- Files: search files and folders by path, and show one to you in Files.
- Engines: list the saved engines and switch to one that is already signed
  in. Signing in and out stays with you.
- XSQL help is built in: the server hands agents a short XSQL reference when
  they connect, lint errors suggest the fix, and an `xsql` prompt walks from a
  task to a checked operation file in `xsql/` (`/mcp__synthigy__xsql` in Claude
  Code).
- `project_guide` explains how a project fits together (the `synthigy` CLI for
  the engine, schema snapshot and code generation, Synthigy Studio for the
  model and data), and a `setup_project` prompt has the agent write an
  `AGENTS.md` for the project.

The bot button glows green while an agent is working, and every agent load,
save and switch shows a notification. On desktop, your name in the header
signs that engine out and returns to the engine list.

What changed for users:

- Model: both switchers show a fixed-height list with a New dataset section,
  rename and delete the open dataset, and mark the live version. A new
  dataset gets its name and first version (0.1.0) in a dialog; names that
  differ only by case, spaces or punctuation are refused. Drafts can be
  deleted; deleting the live version is a Recall, which rolls the engine back
  to the previous deployed version. Deployed versions can't be renamed.
- Notifications sit at the bottom left, clear of the drawers.
- Button labels can no longer be selected as text.

Fixes:

- A new dataset could not be saved or deployed when the dataset open before it
  had a deployed version of the same name, and a save could then have written
  over that other dataset's version.
- A new dataset could take the name of the dataset that was open.
- Desktop: an expired session is refreshed instead of failing, and popups no
  longer fall back to a serif font.

## 0.2.0

Portal binaries, web components and the desktop app ship together. **Needs
engine 0.4.0 or newer:** on 0.3.1 Sign Out fails and the session stays
signed in.

**Breaking.** The three elements are now one: `<synthigy-tools>`, with Model,
Data and Logs as tools inside it. `<synthigy-data-modeling>`,
`<synthigy-data-console>` and `<synthigy-log-cockpit>` are no longer
registered; a page that still uses them gets an unknown element.

Migrating:

- Replace the tag with `<synthigy-tools>`. `endpoint`, `open`, `onClose`
  and the `close` event work as before; `tokenResolver` does not (see
  Signing in).
- To open on the console or the log viewer, set `tool="data"` or
  `tool="logs"`, or call `el.showTool("data")`. The old way of writing
  `synthigy-modeling-console-open` / `synthigy-modeling-cockpit-open` to
  `sessionStorage` before opening the modeler does nothing now.
- `xsql` moves from the modeler to `<synthigy-tools>`.
- The `entities` and `schema` properties of the old console are gone; Data
  reads the deployed model itself.

New: the desktop app. The same tools as a native app for macOS, Windows and
Linux, built from `desktop/` by GitHub Actions; the installers are attached to
this release, unsigned for now (see the README). Its MCP server
(`synthigy-desktop --mcp`) lets an agent such as Claude Code edit the model,
write, check and run XSQL (writes wait for your OK in the window), and read the
logs.

Signing in (**breaking**):

- One way for every web host. The element signs in through a popup and
  renews through a hidden frame, both answered by the static
  `dist/callback.html`. Serve it at `/synthigy-callback.html` on your origin
  and register that URL on the Synthigy Tools app. See Signing in in the README.
- The full-page redirect login is gone, and so are
  `window.SYNTHIGY_REDIRECT_URI` and `window.SYNTHIGY_POST_LOGOUT_REDIRECT_URI`.
  Loading `tooling.js` on the redirect page no longer does anything special.
- `tokenResolver` is honored only inside the Synthigy desktop app. Web hosts that
  set it get one console warning and the built-in sign-in.
- A failed renewal no longer signs the user out. The tools stay mounted, with a
  "Session expires soon" notice and then a Continue screen.
- Sign Out ends the engine session in the background and needs no logout
  redirect. It needs engine 0.4.0 or newer. When the engine does not confirm
  it, a notice says so and offers Try again.
- Signing out, or signing in as someone else, clears what the previous user saw:
  Data results, Logs rows and the Model's undo history. A reload alone no longer
  clears undo.

Fonts:

- The element no longer adds a Google Fonts link to the host page. Include
  `tooling.css` for Inter and JetBrains Mono; without it the system fonts are
  used.

What changed for users:

- One header for all three tools: switch with the tabs or ⌥1 / ⌥2 / ⌥3, theme
  with ⌘B, `?` lists the shortcuts of the tool you are in. One status line at
  the bottom shows counts, state and key hints.
- Model: dataset and version live in the header (O opens a dataset, V the
  versions). Import, Export, Save (⌘S) and Deploy are header commands.
  Entity, relation and deploy drawers dock as an inspector on the right. Bare
  letters replace the old ⌘ shortcuts: E add entity, I import, D deploy,
  A dataset access. ⌘D and ⌘H are no longer taken from the browser.
- Data: each operation in the buffer has its own run marker in the gutter and
  shows its result inline; ⌘↵ runs the operation under the cursor. "Run all"
  is gone. Parameters: an optional `?x?` can't have a default, and a
  string default must be double-quoted; both are flagged as you type.
- Logs: one filter bar (active filters as chips, pickers, search). Search
  waits until you stop typing; a request id searches all time. Clicking a cell
  adds it as a filter, clicking it again removes it. The timeline shows
  errors on top of each bar, and Live follows the newest rows. IP addresses
  show in the Dataset and Auth lenses.

Portal:

- **License: Business Source License 1.1** (was MIT), the same terms as the
  engine. See LICENSE.
- `POSTGRES_JIT` is known to `doctor` and editable in Effective configuration
  on PostgreSQL installs (engine 0.4.0 turns JIT off by default).
- Updated to the current Tyrell components: tags, selects and modals render
  the same, without the old workarounds.

## 0.1.16

Web components only; the portal binaries stay at 0.1.15. Made for engine 0.3.1:
on engine 0.3.0 the modeler shows "Loading system datasets — INTERNAL_ERROR"
when it opens, because of an engine bug that 0.3.1 fixes.

- A host `tokenResolver` is now called before every request, and it may return
  a Promise. Before, the components called it once and kept that token, so a
  host whose tokens expire (one-hour access tokens, for example) started failing
  mid-session. A resolver that returns a fixed token behaves as before.
- Modeler on Safari and other WebKit browsers: attribute type icons stay inside
  their entity boxes. They used to pile up in the top-left corner of the canvas.

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
