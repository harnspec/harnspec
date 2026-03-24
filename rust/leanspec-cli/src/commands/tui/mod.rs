//! Terminal UI (TUI) for interactive spec management.
//!
//! Provides a terminal-based interface for browsing, searching,
//! and viewing specs using ratatui + crossterm.

mod app;
mod board;
mod deps;
mod detail;
mod filter;
mod help;
mod keybindings;
mod list;
mod markdown;
mod search;
mod theme;
mod toc;

use std::error::Error;

use ratatui::{
    crossterm::event::{self, Event},
    crossterm::event::{DisableMouseCapture, EnableMouseCapture},
    crossterm::execute,
    layout::{Constraint, Layout, Rect},
    DefaultTerminal, Frame,
};

use app::{App, AppMode, DetailMode, PrimaryView};

/// Parse the --view CLI argument into a PrimaryView.
fn parse_view(view: &str) -> PrimaryView {
    match view {
        "board" => PrimaryView::Board,
        _ => PrimaryView::List,
    }
}

/// Entry point for the TUI command.
pub fn run(specs_dir: &str, view: &str) -> Result<(), Box<dyn Error>> {
    let initial_view = parse_view(view);
    let mut app = App::new(specs_dir, initial_view)?;

    // Install custom panic hook to restore terminal on panic
    let original_hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        ratatui::restore();
        original_hook(info);
    }));

    let mut terminal = ratatui::init();
    execute!(std::io::stdout(), EnableMouseCapture)?;
    let result = run_event_loop(&mut terminal, &mut app);
    execute!(std::io::stdout(), DisableMouseCapture).ok();
    ratatui::restore();
    result
}

fn run_event_loop(terminal: &mut DefaultTerminal, app: &mut App) -> Result<(), Box<dyn Error>> {
    loop {
        terminal.draw(|frame| draw(frame, app))?;

        if app.should_quit {
            break;
        }

        match event::read()? {
            Event::Key(key) => keybindings::handle_key(app, key),
            Event::Mouse(mouse) => keybindings::handle_mouse(app, mouse),
            _ => {}
        }
    }
    Ok(())
}

fn draw(frame: &mut Frame, app: &mut App) {
    let area = frame.area();

    // Main layout: content + status bar
    let main_chunks = Layout::vertical([Constraint::Min(1), Constraint::Length(1)]).split(area);

    let content_area = main_chunks[0];
    let status_area = main_chunks[1];

    // Responsive layout: split pane for wide terminals, single pane for narrow
    if content_area.width >= 80 {
        draw_split_pane(frame, content_area, app);
    } else {
        draw_single_pane(frame, content_area, app);
    }

    draw_status_bar(frame, status_area, app);

    // Draw overlays on top
    match app.mode {
        AppMode::Search => search::render(area, frame.buffer_mut(), app),
        AppMode::Help => help::render(area, frame.buffer_mut()),
        AppMode::Filter => filter::render(area, frame.buffer_mut(), app),
        AppMode::Toc => toc::render(area, frame.buffer_mut(), app),
        AppMode::Normal => {}
    }
}

fn draw_split_pane(frame: &mut Frame, area: Rect, app: &mut App) {
    app.last_frame_width = area.width;

    let left_constraint = if app.sidebar_collapsed {
        Constraint::Length(0)
    } else {
        Constraint::Percentage(app.sidebar_width_pct)
    };
    let chunks = Layout::horizontal([left_constraint, Constraint::Min(1)]).split(area);
    app.layout_left = chunks[0];
    app.layout_right = chunks[1];

    // Left pane: Board or List
    if !app.sidebar_collapsed {
        match app.primary_view {
            PrimaryView::Board => board::render(chunks[0], frame.buffer_mut(), app),
            PrimaryView::List => list::render(chunks[0], frame.buffer_mut(), app),
        }
    }

    // Right pane: Detail or Dependencies
    match app.detail_mode {
        DetailMode::Content => detail::render(chunks[1], frame.buffer_mut(), app),
        DetailMode::Dependencies => deps::render(chunks[1], frame.buffer_mut(), app),
    }
}

fn draw_single_pane(frame: &mut Frame, area: Rect, app: &mut App) {
    app.last_frame_width = area.width;
    app.layout_left = area;
    app.layout_right = area;

    match app.focus {
        app::FocusPane::Left => match app.primary_view {
            PrimaryView::Board => board::render(area, frame.buffer_mut(), app),
            PrimaryView::List => list::render(area, frame.buffer_mut(), app),
        },
        app::FocusPane::Right => match app.detail_mode {
            DetailMode::Content => detail::render(area, frame.buffer_mut(), app),
            DetailMode::Dependencies => deps::render(area, frame.buffer_mut(), app),
        },
    }
}

fn draw_status_bar(frame: &mut Frame, area: Rect, app: &App) {
    use ratatui::text::{Line, Span};

    let mode_str = match app.mode {
        AppMode::Normal => "NORMAL",
        AppMode::Search => "SEARCH",
        AppMode::Help => "HELP",
        AppMode::Filter => "FILTER",
        AppMode::Toc => "TOC",
    };

    let view_str = match app.primary_view {
        PrimaryView::Board => "Board",
        PrimaryView::List => "List",
    };

    let detail_str = match app.detail_mode {
        DetailMode::Content => "Content",
        DetailMode::Dependencies => "Deps",
    };

    let completion = app.stats.completion_percentage();

    // Selected spec path
    let selected_path = app
        .selected_detail
        .as_ref()
        .map(|s| format!(" {} ", s.path))
        .unwrap_or_default();

    let status_line = Line::from(vec![
        Span::styled(format!(" {} ", mode_str), theme::highlight_style()),
        Span::styled(
            format!(" {} | {} ", view_str, detail_str),
            theme::status_bar_style(),
        ),
        Span::styled(
            format!(" {} specs | {:.0}% complete ", app.stats.total, completion),
            theme::status_bar_style(),
        ),
        Span::styled(selected_path, theme::status_bar_style()),
        Span::styled(
            " q:quit  /:search  ?:help  1/2:view  s:sort  f:filter  t:tree  d:deps  [/]:sidebar ",
            theme::status_bar_style(),
        ),
    ]);

    let paragraph = ratatui::widgets::Paragraph::new(status_line).style(theme::status_bar_style());
    frame.render_widget(paragraph, area);
}
