//! Theme constants: colors, styles, and status symbols for the TUI.

use ratatui::style::{Color, Modifier, Style};

use leanspec_core::{SpecPriority, SpecStatus};

// Unicode symbols for status (single-cell-width, no emoji)
pub const STATUS_DRAFT: &str = "○";
pub const STATUS_PLANNED: &str = "·";
pub const STATUS_IN_PROGRESS: &str = "◑";
pub const STATUS_COMPLETE: &str = "●";
pub const STATUS_ARCHIVED: &str = "⊘";

pub fn status_symbol(status: &SpecStatus) -> &'static str {
    match status {
        SpecStatus::Draft => STATUS_DRAFT,
        SpecStatus::Planned => STATUS_PLANNED,
        SpecStatus::InProgress => STATUS_IN_PROGRESS,
        SpecStatus::Complete => STATUS_COMPLETE,
        SpecStatus::Archived => STATUS_ARCHIVED,
    }
}

pub fn status_color(status: &SpecStatus) -> Color {
    match status {
        SpecStatus::Draft => Color::Rgb(160, 220, 220),
        SpecStatus::Planned => Color::Rgb(100, 140, 255),
        SpecStatus::InProgress => Color::Rgb(255, 190, 50),
        SpecStatus::Complete => Color::Rgb(80, 200, 120),
        SpecStatus::Archived => Color::Rgb(90, 90, 90),
    }
}

pub fn status_style(status: &SpecStatus) -> Style {
    Style::default().fg(status_color(status))
}

pub fn priority_symbol(priority: Option<&SpecPriority>) -> &'static str {
    match priority {
        Some(SpecPriority::Critical) => "!!",
        Some(SpecPriority::High) => "! ",
        Some(SpecPriority::Medium) => "· ",
        Some(SpecPriority::Low) => "↓ ",
        None => "  ",
    }
}

// Common styles — RGB palette for modern look
pub fn title_style() -> Style {
    Style::default()
        .fg(Color::Rgb(220, 220, 255))
        .add_modifier(Modifier::BOLD)
}

pub fn selected_style() -> Style {
    Style::default()
        .bg(Color::Rgb(50, 50, 80))
        .add_modifier(Modifier::BOLD)
}

/// Dimmed selection style for when the sidebar is not focused but still shows which spec is active.
pub fn inactive_selected_style() -> Style {
    Style::default().bg(Color::Rgb(35, 35, 55))
}

pub fn header_style() -> Style {
    Style::default()
        .fg(Color::Rgb(220, 220, 255))
        .add_modifier(Modifier::BOLD)
}

pub fn dimmed_style() -> Style {
    Style::default().fg(Color::Rgb(100, 100, 120))
}

pub fn highlight_style() -> Style {
    Style::default()
        .bg(Color::Blue)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
}

pub fn status_bar_style() -> Style {
    Style::default().bg(Color::DarkGray).fg(Color::White)
}

pub fn border_focused_style() -> Style {
    Style::default().fg(Color::Rgb(100, 200, 255))
}

pub fn border_unfocused_style() -> Style {
    Style::default().fg(Color::Rgb(70, 70, 90))
}

pub fn project_name_style() -> Style {
    Style::default()
        .bg(Color::DarkGray)
        .fg(Color::Cyan)
        .add_modifier(Modifier::BOLD)
}

pub fn overlay_border_style() -> Style {
    Style::default().fg(Color::Green)
}

pub fn overlay_selected_style() -> Style {
    Style::default()
        .bg(Color::Blue)
        .fg(Color::White)
        .add_modifier(Modifier::BOLD)
}

pub fn favorite_style() -> Style {
    Style::default().fg(Color::Yellow)
}

pub fn error_style() -> Style {
    Style::default().fg(Color::Red)
}

pub fn success_style() -> Style {
    Style::default().fg(Color::Green)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_status_symbols() {
        assert_eq!(status_symbol(&SpecStatus::Draft), "○");
        assert_eq!(status_symbol(&SpecStatus::Planned), "·");
        assert_eq!(status_symbol(&SpecStatus::InProgress), "◑");
        assert_eq!(status_symbol(&SpecStatus::Complete), "●");
        assert_eq!(status_symbol(&SpecStatus::Archived), "⊘");
    }

    #[test]
    fn test_priority_symbols() {
        assert_eq!(priority_symbol(Some(&SpecPriority::Critical)), "!!");
        assert_eq!(priority_symbol(Some(&SpecPriority::High)), "! ");
        assert_eq!(priority_symbol(Some(&SpecPriority::Medium)), "· ");
        assert_eq!(priority_symbol(Some(&SpecPriority::Low)), "↓ ");
        assert_eq!(priority_symbol(None), "  ");
    }
}
