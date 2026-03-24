//! Mode-based input dispatch for keybindings.

use ratatui::crossterm::event::{KeyCode, KeyEvent, KeyModifiers, MouseEvent};

use super::app::{App, AppMode, FocusPane};

/// Handle a key event based on the current app mode.
pub fn handle_key(app: &mut App, key: KeyEvent) {
    match app.mode {
        AppMode::Normal => handle_normal(app, key),
        AppMode::Search => handle_search(app, key),
        AppMode::Help => handle_help(app, key),
        AppMode::Filter => handle_filter(app, key),
        AppMode::Toc => handle_toc(app, key),
    }
}

fn handle_normal(app: &mut App, key: KeyEvent) {
    let page_size = (app.layout_left.height.saturating_sub(4) as usize).max(5);

    match key.code {
        KeyCode::Char('q') => app.should_quit = true,
        KeyCode::Char('j') | KeyCode::Down => {
            if app.focus == FocusPane::Right {
                app.scroll_detail_down();
            } else {
                app.move_down();
            }
        }
        KeyCode::Char('k') | KeyCode::Up => {
            if app.focus == FocusPane::Right {
                app.scroll_detail_up();
            } else {
                app.move_up();
            }
        }
        KeyCode::Char('g') | KeyCode::Home => {
            if app.focus == FocusPane::Right {
                app.detail_scroll = 0;
            } else {
                app.move_first();
            }
        }
        KeyCode::Char('G') | KeyCode::End => {
            if app.focus == FocusPane::Right {
                app.detail_scroll = app.detail_scroll.saturating_add(999);
            } else {
                app.move_last();
            }
        }
        KeyCode::PageDown => {
            if app.focus == FocusPane::Right {
                for _ in 0..page_size {
                    app.scroll_detail_down();
                }
            } else {
                app.page_down(page_size);
            }
        }
        KeyCode::PageUp => {
            if app.focus == FocusPane::Right {
                for _ in 0..page_size {
                    app.scroll_detail_up();
                }
            } else {
                app.page_up(page_size);
            }
        }
        KeyCode::Char('h') | KeyCode::Left => app.focus_left(),
        KeyCode::Char('l') | KeyCode::Right | KeyCode::Enter => {
            if app.tree_mode && app.focus == FocusPane::Left {
                // Enter on a parent node in tree mode toggles expand/collapse
                let is_parent = app
                    .tree_rows
                    .get(app.list_selected)
                    .is_some_and(|r| r.has_children);
                if is_parent {
                    app.toggle_current_tree_node();
                    return;
                }
            }
            app.focus_right();
        }
        KeyCode::Char(' ') => {
            if app.tree_mode && app.focus == FocusPane::Left {
                app.toggle_current_tree_node();
            }
        }
        KeyCode::Tab => {
            if key.modifiers.contains(KeyModifiers::SHIFT) {
                app.prev_group();
            } else {
                app.next_group();
            }
        }
        KeyCode::BackTab => app.prev_group(),
        KeyCode::Char('1') => app.set_board_view(),
        KeyCode::Char('2') => app.set_list_view(),
        KeyCode::Char('d') => app.toggle_detail_mode(),
        KeyCode::Char('/') => app.enter_search(),
        KeyCode::Char('?') => app.enter_help(),
        KeyCode::Char('[') => app.sidebar_narrow(),
        KeyCode::Char(']') => app.sidebar_widen(),
        KeyCode::Char('\\') => app.sidebar_toggle_collapse(),
        KeyCode::Char('s') => app.cycle_sort(),
        KeyCode::Char('f') => app.open_filter(),
        KeyCode::Char('F') => app.clear_filters(),
        KeyCode::Char('t') => app.toggle_tree(),
        KeyCode::Char('z') => app.collapse_all(),
        KeyCode::Char('Z') => app.expand_all(),
        KeyCode::Char('c') => {
            if app.primary_view == super::app::PrimaryView::Board {
                app.toggle_current_board_group();
            }
        }
        KeyCode::Char('C') => {
            if app.primary_view == super::app::PrimaryView::Board {
                app.collapse_all_board_groups();
            }
        }
        KeyCode::Char('E') => {
            if app.primary_view == super::app::PrimaryView::Board {
                app.expand_all_board_groups();
            }
        }
        KeyCode::Char('T') => {
            if app.focus == FocusPane::Right {
                app.open_toc();
            }
        }
        KeyCode::Esc => app.focus_left(),
        _ => {}
    }
}

/// Handle mouse events.
pub fn handle_mouse(app: &mut App, mouse: MouseEvent) {
    use ratatui::crossterm::event::{MouseButton, MouseEventKind};
    // Always consume scroll events — never let them propagate to the outer terminal.
    match mouse.kind {
        MouseEventKind::ScrollDown => {
            if app.sidebar_collapsed || mouse.column >= app.layout_right.x {
                app.scroll_detail_down();
            } else {
                app.move_down();
            }
        }
        MouseEventKind::ScrollUp => {
            if app.sidebar_collapsed || mouse.column >= app.layout_right.x {
                app.scroll_detail_up();
            } else {
                app.move_up();
            }
        }
        MouseEventKind::Down(MouseButton::Left) => {
            let col = mouse.column;
            let row = mouse.row;
            // Check if near split boundary (drag handle)
            let split_col = if app.last_frame_width > 0 {
                (app.last_frame_width as u32 * app.sidebar_width_pct as u32 / 100) as u16
            } else {
                0
            };
            if !app.sidebar_collapsed
                && app.last_frame_width > 0
                && (col == split_col || col == split_col.saturating_sub(1) || col == split_col + 1)
            {
                app.drag_resize = true;
            } else if !app.sidebar_collapsed
                && app.layout_left.width > 0
                && col < app.layout_left.x + app.layout_left.width
            {
                app.click_sidebar(row);
            } else if col >= app.layout_right.x {
                app.focus = FocusPane::Right;
            }
        }
        MouseEventKind::Up(MouseButton::Left) => {
            app.drag_resize = false;
        }
        MouseEventKind::Drag(MouseButton::Left) => {
            if app.drag_resize {
                app.resize_drag_to(mouse.column);
            }
        }
        _ => {}
    }
}

fn handle_search(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.exit_search(),
        KeyCode::Enter => app.search_select(),
        KeyCode::Backspace => app.search_backspace(),
        KeyCode::Char(c) => app.search_type_char(c),
        _ => {}
    }
}

fn handle_help(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('?') | KeyCode::Char('q') => app.exit_overlay(),
        _ => {}
    }
}

fn handle_filter(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc => app.close_filter(),
        KeyCode::Char('j') | KeyCode::Down => app.filter_cursor_down(),
        KeyCode::Char('k') | KeyCode::Up => app.filter_cursor_up(),
        KeyCode::Char(' ') | KeyCode::Enter => app.filter_toggle_current(),
        KeyCode::Char('F') => {
            app.clear_filters();
            app.mode = super::app::AppMode::Filter; // stay in filter popup
        }
        _ => {}
    }
}

fn handle_toc(app: &mut App, key: KeyEvent) {
    match key.code {
        KeyCode::Esc | KeyCode::Char('T') => app.close_toc(),
        KeyCode::Char('j') | KeyCode::Down => app.toc_move_down(),
        KeyCode::Char('k') | KeyCode::Up => app.toc_move_up(),
        KeyCode::Enter => app.toc_jump(),
        _ => {}
    }
}

#[cfg(test)]
mod tests {
    use super::super::app::{AppMode, DetailMode, FocusPane, PrimaryView};
    use super::*;

    fn make_test_app() -> App {
        App::empty_for_test()
    }

    fn key(code: KeyCode) -> KeyEvent {
        KeyEvent::new(code, KeyModifiers::NONE)
    }

    #[test]
    fn test_q_quits() {
        let mut app = make_test_app();
        handle_key(&mut app, key(KeyCode::Char('q')));
        assert!(app.should_quit);
    }

    #[test]
    fn test_slash_enters_search() {
        let mut app = make_test_app();
        handle_key(&mut app, key(KeyCode::Char('/')));
        assert_eq!(app.mode, AppMode::Search);
    }

    #[test]
    fn test_question_mark_enters_help() {
        let mut app = make_test_app();
        handle_key(&mut app, key(KeyCode::Char('?')));
        assert_eq!(app.mode, AppMode::Help);
    }

    #[test]
    fn test_f_enters_filter() {
        let mut app = make_test_app();
        handle_key(&mut app, key(KeyCode::Char('f')));
        assert_eq!(app.mode, AppMode::Filter);
    }

    #[test]
    fn test_esc_in_search_returns_to_normal() {
        let mut app = make_test_app();
        app.mode = AppMode::Search;
        handle_key(&mut app, key(KeyCode::Esc));
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_esc_in_help_returns_to_normal() {
        let mut app = make_test_app();
        app.mode = AppMode::Help;
        handle_key(&mut app, key(KeyCode::Esc));
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_esc_in_filter_returns_to_normal() {
        let mut app = make_test_app();
        app.mode = AppMode::Filter;
        handle_key(&mut app, key(KeyCode::Esc));
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_1_2_switch_views() {
        let mut app = make_test_app();
        handle_key(&mut app, key(KeyCode::Char('2')));
        assert_eq!(app.primary_view, PrimaryView::List);

        handle_key(&mut app, key(KeyCode::Char('1')));
        assert_eq!(app.primary_view, PrimaryView::Board);
    }

    #[test]
    fn test_d_toggles_detail_mode() {
        let mut app = make_test_app();
        assert_eq!(app.detail_mode, DetailMode::Content);

        handle_key(&mut app, key(KeyCode::Char('d')));
        assert_eq!(app.detail_mode, DetailMode::Dependencies);
    }

    #[test]
    fn test_h_l_switch_focus() {
        let mut app = make_test_app();
        handle_key(&mut app, key(KeyCode::Char('l')));
        assert_eq!(app.focus, FocusPane::Right);

        handle_key(&mut app, key(KeyCode::Char('h')));
        assert_eq!(app.focus, FocusPane::Left);
    }

    #[test]
    fn test_bracket_keys_resize_sidebar() {
        let mut app = make_test_app();
        assert_eq!(app.sidebar_width_pct, 30);

        handle_key(&mut app, key(KeyCode::Char(']')));
        assert_eq!(app.sidebar_width_pct, 35);

        handle_key(&mut app, key(KeyCode::Char('[')));
        assert_eq!(app.sidebar_width_pct, 30);
    }

    #[test]
    fn test_backslash_toggles_sidebar_collapse() {
        let mut app = make_test_app();
        assert!(!app.sidebar_collapsed);

        handle_key(&mut app, key(KeyCode::Char('\\')));
        assert!(app.sidebar_collapsed);

        handle_key(&mut app, key(KeyCode::Char('\\')));
        assert!(!app.sidebar_collapsed);
    }

    #[test]
    fn test_s_cycles_sort() {
        use super::super::app::SortOption;
        let mut app = make_test_app();
        assert_eq!(app.sort_option, SortOption::IdDesc);

        handle_key(&mut app, key(KeyCode::Char('s')));
        assert_eq!(app.sort_option, SortOption::IdAsc);

        handle_key(&mut app, key(KeyCode::Char('s')));
        assert_eq!(app.sort_option, SortOption::PriorityDesc);
    }

    #[test]
    fn test_t_toggles_tree() {
        let mut app = make_test_app();
        assert!(!app.tree_mode);

        handle_key(&mut app, key(KeyCode::Char('t')));
        assert!(app.tree_mode);

        handle_key(&mut app, key(KeyCode::Char('t')));
        assert!(!app.tree_mode);
    }

    #[test]
    fn test_home_end_list() {
        let mut app = make_test_app();
        app.set_list_view();
        app.filtered_specs = vec![0, 1, 2, 3, 4];
        app.list_selected = 2;

        handle_key(&mut app, key(KeyCode::Home));
        assert_eq!(app.list_selected, 0);

        handle_key(&mut app, key(KeyCode::End));
        assert_eq!(app.list_selected, 4);
    }

    #[test]
    fn test_search_mode_typing() {
        let mut app = make_test_app();
        app.mode = AppMode::Search;

        handle_key(&mut app, key(KeyCode::Char('t')));
        handle_key(&mut app, key(KeyCode::Char('e')));
        handle_key(&mut app, key(KeyCode::Char('s')));
        assert_eq!(app.search_query, "tes");

        handle_key(&mut app, key(KeyCode::Backspace));
        assert_eq!(app.search_query, "te");
    }

    #[test]
    fn test_filter_j_k_moves_cursor() {
        let mut app = make_test_app();
        app.mode = AppMode::Filter;
        assert_eq!(app.filter_cursor, 0);

        handle_key(&mut app, key(KeyCode::Char('j')));
        assert_eq!(app.filter_cursor, 1);

        handle_key(&mut app, key(KeyCode::Char('k')));
        assert_eq!(app.filter_cursor, 0);
    }
}
