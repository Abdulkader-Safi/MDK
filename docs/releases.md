# Releases

Each release is one `## ` heading: `## <tag> · <date> · <title>`. Everything under the
heading, down to the next `## `, is that release's notes in plain markdown. Write as much
as you like. The release workflow copies the section for a tag into its GitHub release.

## v0.1.6 · 2026-11-08 · Your card files stay as you wrote them

- A comment after a value, like `status: in-progress # waiting`, no longer resets the card to Backlog when it's saved.
- A `labels` list written over several lines is read properly. It used to be saved back as `labels: []`.
- Your own frontmatter keys survive a save even with a space, a dot or an accent in the name, such as `story points: 3`.
- A `# ` line inside a code block is no longer taken as the card's title, so renaming the card can't overwrite your code.
- Pressing Escape after picking up a card with Space puts the whole column back exactly as it was.
- A move that fails halfway, for example next to a card with broken frontmatter, now undoes what it already wrote.
- On Windows, an image link in a card can't reach files outside the project folder, or a network share.
- A settings file mdk can't read is set aside and kept, instead of being overwritten.

## v0.1.5 · 2026-11-08 · Check for updates

- "Check for Updates..." in the mdk menu, and "Check for updates" in the command palette. mdk asks GitHub for the latest release, then tells you you're on the latest version or offers the download.
- This is the only time mdk goes online, and only when you click it. mdk still has no account, no telemetry and no automatic checks.

## v0.1.4 · 2026-11-07 · A sidebar that stays in order

- The header and the settings stay put while projects and work folders scroll on their own, so the footer sits at the bottom even with one project.
- Drag the line between projects and work folders to resize them. Click a heading to fold that section.
- Right-click any project to rescan it or remove it from the list. A project whose folder went missing offers "Locate folder" instead.

## v0.1.3 · 2026-10-28 · Windows fixes from the first real run

- No console window opens next to mdk any more. Closing that console used to quit the app.
- The app icon now shows on the window, the taskbar, the desktop shortcut and in Explorer.

Windows still shows "Windows protected your PC" on first launch, because the installer is not code-signed. Click "More info", then "Run anyway".

## v0.1.2 · 2026-10-18 · One mdk at a time on Windows

Opening mdk a second time brings the running window to the front instead of starting a second copy.

## v0.1.1 · 2026-10-18 · Resizable panels

- Drag the edge of the sidebar (180 to 420 px) or the card sheet to resize it. The widths are remembered.
- Subtasks wrap, and inline code in them renders as code.
- A click in the card sheet no longer starts a text selection that follows the pointer.

## v0.1.0 · 2026-10-14 · First release

The first public build of mdk, for macOS (Apple silicon), Windows and Linux, with the lime and ink app icon.
