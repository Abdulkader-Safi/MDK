# Releases

Each release is one `## ` heading: `## <tag> · <date> · <title>`. Everything under the
heading, down to the next `## `, is that release's notes in plain markdown. Write as much
as you like. The release workflow copies the section for a tag into its GitHub release.

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
