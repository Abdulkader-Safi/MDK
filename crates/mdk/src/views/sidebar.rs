//! The left sidebar: the brand, projects, work folders, and the refresh toggles.

use super::{Colors, label, toggle};
use crate::app::{Mdk, SplitProjects};
use crate::board::ProjectStatus;
use crate::theme::{icon, status_color};
use gpui_kit::component::{ActiveTheme, Icon};
use gpui_kit::prelude::*;
use gpui_kit::*;
use mdk_format::Status;

/// One selectable row. The selected row has the accent edge.
fn row(id: impl Into<ElementId>, selected: bool, c: Colors) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap_2()
        .w_full()
        .px_3()
        .py_2()
        .border_l_2()
        .cursor_pointer()
        .map(|d| match selected {
            true => d
                .bg(c.secondary)
                .border_color(c.primary)
                .font_weight(FontWeight::MEDIUM),
            false => d
                .border_color(c.primary.opacity(0.))
                .text_color(c.muted_fg)
                .hover(|s| s.bg(c.secondary).text_color(c.fg)),
        })
}

/// The icon at the start of a row, in the accent colour when the row is selected.
fn row_icon(name: &'static str, selected: bool, c: Colors) -> Icon {
    icon(name)
        .size_4()
        .text_color(if selected { c.link } else { c.muted_fg })
}

/// One line in the project's right-click menu. `hover` is its colour under the pointer.
fn menu_item(
    id: &'static str,
    icon_name: &'static str,
    text: &'static str,
    hover: Hsla,
    c: Colors,
) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .items_center()
        .gap_2()
        .px_2()
        .py_1p5()
        .text_xs()
        .text_color(c.fg)
        .cursor_pointer()
        .occlude()
        .hover(|s| s.bg(c.accent).text_color(hover))
        .child(icon(icon_name).size_3p5())
        .child(text)
}

/// A section heading. A click folds the section to its heading or opens it again.
fn heading(
    id: &'static str,
    text: &'static str,
    collapsed: bool,
    c: Colors,
    cx: &App,
) -> Stateful<Div> {
    div()
        .id(id)
        .flex()
        .flex_shrink_0()
        .items_center()
        .gap_1()
        .px_4()
        .pt_4()
        .pb_1p5()
        .cursor_pointer()
        .child(label(text, c, cx))
        .child(
            icon(if collapsed {
                "chevron-right"
            } else {
                "chevron-down"
            })
            .size_3()
            .text_color(c.muted_fg),
        )
}

impl Mdk {
    pub(super) fn sidebar(&self, c: Colors, cx: &mut Context<Self>) -> Stateful<Div> {
        let mono = cx.theme().mono_font_family.clone();
        let counts = self.board.open_counts();
        let selected = self.board.selected_project.clone();

        let projects = self.board.projects.iter().map(|project| {
            let id = project.id.clone();
            let is_selected = selected.as_deref() == Some(id.as_str());
            let ok = project.status == ProjectStatus::Ok;
            let problem = match &project.status {
                ProjectStatus::Ok => None,
                ProjectStatus::Missing => Some("Folder not found".to_string()),
                ProjectStatus::Unreadable(why) => Some(format!("Can't read folder: {why}")),
            };
            let (select, menu) = (id.clone(), id.clone());

            div()
                .flex()
                .flex_col()
                .child(
                    row(SharedString::from(format!("project-{id}")), is_selected, c)
                        .child(row_icon("folder-open", is_selected, c))
                        .child(div().flex_1().truncate().child(project.name.clone()))
                        .when(!ok, |d| {
                            d.child(icon("alert-triangle").size_3p5().text_color(c.danger))
                        })
                        .child(
                            div()
                                .font_family(mono.clone())
                                .text_size(px(11.))
                                .font_weight(FontWeight::NORMAL)
                                .text_color(c.muted_fg)
                                .child(counts.get(id.as_str()).copied().unwrap_or(0).to_string()),
                        )
                        .on_click(cx.listener(move |this, _, _, cx| {
                            this.select_project(Some(select.clone()), cx)
                        }))
                        .on_mouse_down(
                            MouseButton::Right,
                            cx.listener(move |this, ev: &MouseDownEvent, _, cx| {
                                this.project_menu = Some((menu.clone(), ev.position));
                                cx.notify();
                            }),
                        ),
                )
                .when_some(problem, |d, problem| {
                    d.child(
                        div()
                            .px_4()
                            .pt_1()
                            .text_size(px(11.))
                            .text_color(c.danger)
                            .child(problem),
                    )
                })
        });
        let projects: Vec<_> = projects.collect();

        let workspaces = self.board.workspaces();
        let current = self.board.selected_workspace.clone();
        let folder = |name: Option<String>, cx: &mut Context<Self>| {
            let text = name.clone().unwrap_or_else(|| "all".into());
            let selected = current == name;
            // A folder can be called "all", so the All row's id can't be built from its text.
            let id = name
                .as_ref()
                .map_or("all-folders".into(), |n| format!("folder-{n}"));
            row(SharedString::from(id), selected, c)
                .child(row_icon("layout-grid", selected, c))
                .child(div().flex_1().truncate().child(text))
                .on_click(
                    cx.listener(move |this, _, _, cx| this.select_workspace(name.clone(), cx)),
                )
        };
        let folders: Vec<_> = std::iter::once(None)
            .chain(workspaces.iter().cloned().map(Some))
            .map(|name| folder(name, cx))
            .collect();
        let setting = |name: &'static str, switch: Stateful<Div>| {
            div()
                .flex()
                .items_center()
                .gap_2()
                .text_xs()
                .text_color(c.muted_fg)
                .child(div().flex_1().child(name))
                .child(switch)
        };
        let watching = self.prefs.auto_refresh;
        let projects_open = !self.prefs.projects_collapsed;
        let has_folders = !workspaces.is_empty();
        let folders_open = has_folders && !self.prefs.folders_collapsed;
        // With both lists open the project list keeps its dragged height; otherwise the
        // open list takes the room. Each list scrolls on its own.
        let split = projects_open && folders_open;

        div()
            .id("sidebar")
            .flex()
            .flex_col()
            .flex_shrink_0()
            .w(px(self.prefs.sidebar_width))
            .h_full()
            .bg(c.sidebar)
            .border_r_1()
            .border_color(c.border)
            .child(
                div()
                    .flex()
                    .flex_shrink_0()
                    .items_center()
                    .gap_2p5()
                    .h(px(48.))
                    .px_4()
                    .border_b_1()
                    .border_color(c.border)
                    .child(icon("layout-kanban").size_5().text_color(c.link))
                    .child(
                        div()
                            .flex_1()
                            .text_size(px(20.))
                            .font_weight(FontWeight::SEMIBOLD)
                            .child("mdk"),
                    )
                    .child(
                        div()
                            .font_family(mono.clone())
                            .text_size(px(10.))
                            .text_color(c.muted_fg)
                            .child(env!("CARGO_PKG_VERSION")),
                    ),
            )
            .child(
                heading("projects-heading", "PROJECTS", !projects_open, c, cx).on_click(
                    cx.listener(|this, _, _, cx| {
                        this.prefs.projects_collapsed = !this.prefs.projects_collapsed;
                        this.prefs_changed(cx);
                    }),
                ),
            )
            .when(projects_open, |d| {
                d.child(
                    div()
                        .id("projects-list")
                        .flex()
                        .flex_col()
                        .min_h(px(60.))
                        .overflow_y_scroll()
                        .map(|d| match split {
                            true => d.h(px(self.prefs.projects_height)).flex_shrink(1.),
                            false => d.flex_1(),
                        })
                        .on_drag_move(cx.listener(
                            |this, ev: &DragMoveEvent<SplitProjects>, window, cx| {
                                let top = f32::from(ev.bounds.top());
                                let room = f32::from(window.viewport_size().height);
                                this.prefs.projects_height = (f32::from(ev.event.position.y) - top)
                                    .clamp(60., (room - top - 200.).max(60.));
                                this.prefs_changed(cx);
                            },
                        ))
                        .child(
                            row("all-projects", selected.is_none(), c)
                                .child(row_icon("folders", selected.is_none(), c))
                                .child("All projects")
                                .on_click(
                                    cx.listener(|this, _, _, cx| this.select_project(None, cx)),
                                ),
                        )
                        .children(projects)
                        .child(
                            row("connect", false, c)
                                .child(icon("plus").size_4())
                                .child("Add project")
                                .on_click(cx.listener(|this, _, _, cx| this.connect_folder(cx))),
                        ),
                )
            })
            .when(split, |d| {
                d.child(
                    div()
                        .id("split-projects")
                        .flex_shrink_0()
                        .h(px(6.))
                        .mb(px(-6.))
                        .cursor_row_resize()
                        .border_b_1()
                        .border_color(c.border)
                        .hover(|s| s.bg(c.link.opacity(0.4)))
                        .on_mouse_down(MouseButton::Left, |_, _, cx| {
                            gpui_kit::component::GlobalState::suppress_text_selection(cx)
                        })
                        .on_drag(SplitProjects, |_, _, _, cx| cx.new(|_| SplitProjects)),
                )
            })
            .when(has_folders, |d| {
                d.child(
                    heading("folders-heading", "WORK FOLDERS", !folders_open, c, cx).on_click(
                        cx.listener(|this, _, _, cx| {
                            this.prefs.folders_collapsed = !this.prefs.folders_collapsed;
                            this.prefs_changed(cx);
                        }),
                    ),
                )
            })
            .when(folders_open, |d| {
                d.child(
                    div()
                        .id("folders-list")
                        .flex()
                        .flex_col()
                        .flex_1()
                        .min_h(px(60.))
                        .overflow_y_scroll()
                        .children(folders),
                )
            })
            .when(!projects_open && !folders_open, |d| d.child(div().flex_1()))
            .child(
                div()
                    .flex()
                    .flex_col()
                    .flex_shrink_0()
                    .gap_2()
                    .px_4()
                    .py_3()
                    .border_t_1()
                    .border_color(c.border)
                    .child(setting(
                        "Auto-refresh folders",
                        toggle("auto-refresh", watching, c).on_click(cx.listener(
                            |this, _, _, cx| this.set_auto_refresh(!this.prefs.auto_refresh, cx),
                        )),
                    ))
                    .child(setting(
                        "Vim keys in editor",
                        toggle("vim-keys", self.prefs.vim_keys, c).on_click(cx.listener(
                            |this, _, _, cx| this.set_vim_keys(!this.prefs.vim_keys, cx),
                        )),
                    ))
                    .child(
                        div()
                            .flex()
                            .items_center()
                            .gap_2()
                            .pt_1()
                            .child(div().size_2().bg(status_color(
                                if watching {
                                    Status::Done
                                } else {
                                    Status::Backlog
                                },
                                c.dark,
                            )))
                            .child(
                                div()
                                    .font_family(mono)
                                    .text_size(px(10.))
                                    .text_color(c.muted_fg)
                                    .child(format!(
                                        "{} · CHECKED {}",
                                        if watching { "WATCHING" } else { "NOT WATCHING" },
                                        self.last_check.as_deref().unwrap_or("NEVER")
                                    )),
                            ),
                    ),
            )
            .when_some(self.project_menu_view(c, cx), |d, menu| d.child(menu))
    }

    /// The right-click menu of a project: rescan (or locate, when the folder is gone) and
    /// remove.
    fn project_menu_view(&self, c: Colors, cx: &mut Context<Self>) -> Option<impl IntoElement> {
        let (id, position) = self.project_menu.clone()?;
        let project = self.board.project(&id)?;
        let ok = project.status == ProjectStatus::Ok;
        let (rescan, locate, remove) = (id.clone(), id.clone(), id);
        let menu = div()
            .id("project-menu")
            .occlude()
            .w(px(160.))
            .p_1()
            .bg(c.card)
            .border_1()
            .border_color(c.strong)
            .on_mouse_down_out(cx.listener(|this, _, _, cx| {
                this.project_menu = None;
                cx.notify();
            }))
            .map(|d| match ok {
                true => d.child(
                    menu_item("project-rescan", "refresh", "Rescan", c.accent_fg, c).on_click(
                        cx.listener(move |this, _, _, cx| {
                            this.project_menu = None;
                            this.rescan(&rescan, cx)
                        }),
                    ),
                ),
                false => d.child(
                    menu_item(
                        "project-locate",
                        "folder-open",
                        "Locate folder",
                        c.accent_fg,
                        c,
                    )
                    .on_click(cx.listener(move |this, _, _, cx| {
                        this.project_menu = None;
                        this.locate_folder(locate.clone(), cx)
                    })),
                ),
            })
            .child(
                menu_item("project-remove", "plug-off", "Remove project", c.danger, c).on_click(
                    cx.listener(move |this, _, _, cx| {
                        this.project_menu = None;
                        this.remove_project(&remove, cx)
                    }),
                ),
            );
        Some(deferred(anchored().position(position).snap_to_window().child(menu)).with_priority(1))
    }
}
