# Devlog

Each entry is one `## ` heading: `## <date> · <title>`, or `## <date> · <tags> · <title>`
when the work shipped in a release (`v0.1.4`, several tags separated by spaces, or `next`).
Everything under the heading is plain markdown. Newest first.

## 2026-11-08 · v0.1.5 · Check for updates

A "Check for Updates" item in the mdk menu and the command palette. mdk asks GitHub for the latest release only when you click it, then says you are up to date or offers the download.

mdk still compiles in no HTTP client. The one request goes through the system `curl`, which ships with macOS, Windows 10 and later, and most Linux. Apart from this check, mdk makes no network requests.

The site also got its own [releases](releases.html) and devlog pages, written as markdown in the repo.

## 2026-11-07 · v0.1.4 · This website, and a better sidebar

The landing page went live on GitHub Pages, with screenshots taken from the app on personal projects only.

The sidebar got a pinned header and footer, resizable project and folder lists, and a right-click menu for rescan and remove. Release builds moved to macOS 15 after GitHub retired its macOS 14 machines, which had quietly broken the macOS CI job.

## 2026-10-28 · v0.1.3 · First run on a Windows laptop

Two problems showed up straight away: a console window opened beside the app (and closing it quit mdk), and the app had no icon anywhere. Both are fixed: release builds use the Windows GUI subsystem, and the icon is embedded in `mdk.exe` as the resource GPUI loads.

Windows still warns that the installer is unsigned. Only a code-signing certificate fixes that.

## 2026-10-18 · v0.1.1 v0.1.2 · Fixes from daily use

A week of running mdk on real boards gave three fixes: a resizable sidebar and card sheet, subtasks that wrap, and no stray text selection after a click inside the sheet. A second launch on Windows now reaches the first window instead of opening another.

## 2026-10-14 · v0.1.0 · App icon and the first release

The kanban mark in lime on an ink tile. Pushing a version tag now builds the macOS, Windows and Linux files and publishes them as a GitHub release.

## 2026-10-12 · A first look, then a redesign

The first screenshots of the real app showed four bugs that no headless test could see, all fixed the same day. Then the whole app moved to the ink and lime design from Figma.

Measured against the Electron app on an M1:

| | mdk | Electron |
| --- | --- | --- |
| Download | 11 MB | 196 MB |
| On disk | 28 MB | 603 MB |
| Memory at idle | 67 MB | 250 MB |

## 2026-10-11 · Day one: from spec to a working board

Eighteen commits, from four test spikes (file watching, Vim keys, drag and drop, the preview) to a full app with the board, the card editor, Vim mode, the preview, filters and the command palette.

The card format is tested against 46 cards run through the parser of the Electron app, so both apps write the same bytes.

## 2026-09-10 · Before mdk: MD Kanban on Electron

The first version, built with Vue and Electron. It proved the idea, but the download was 196 MB and it took 603 MB on disk. mdk rebuilds it as a native Rust app that reads the same files.
