//! App struct, state machine, and data management for the TUI.

use leanspec_core::{
    search_specs, DependencyGraph, SpecInfo, SpecLoader, SpecPriority, SpecStats, SpecStatus,
};
use ratatui::layout::Rect;
use std::collections::HashSet;
use std::error::Error;

/// Which mode the app is in (affects keybinding dispatch).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum AppMode {
    Normal,
    Search,
    Help,
    Filter,
    Toc,
}

/// Sort order for the spec list.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum SortOption {
    #[default]
    IdDesc,
    IdAsc,
    PriorityDesc,
    TitleAsc,
    UpdatedDesc,
}

impl SortOption {
    pub fn label(self) -> &'static str {
        match self {
            SortOption::IdDesc => "ID ↓",
            SortOption::IdAsc => "ID ↑",
            SortOption::PriorityDesc => "Priority ↓",
            SortOption::TitleAsc => "Title A-Z",
            SortOption::UpdatedDesc => "Updated ↓",
        }
    }

    pub fn next(self) -> SortOption {
        match self {
            SortOption::IdDesc => SortOption::IdAsc,
            SortOption::IdAsc => SortOption::PriorityDesc,
            SortOption::PriorityDesc => SortOption::TitleAsc,
            SortOption::TitleAsc => SortOption::UpdatedDesc,
            SortOption::UpdatedDesc => SortOption::IdDesc,
        }
    }
}

/// Active filter state for the spec list.
#[derive(Debug, Clone, Default)]
pub struct FilterState {
    pub statuses: Vec<SpecStatus>,
    pub priorities: Vec<SpecPriority>,
    pub tags: Vec<String>,
}

impl FilterState {
    pub fn is_empty(&self) -> bool {
        self.statuses.is_empty() && self.priorities.is_empty() && self.tags.is_empty()
    }

    pub fn matches(&self, spec: &SpecInfo) -> bool {
        if !self.statuses.is_empty() && !self.statuses.contains(&spec.frontmatter.status) {
            return false;
        }
        if !self.priorities.is_empty() {
            match spec.frontmatter.priority {
                Some(ref p) if !self.priorities.contains(p) => return false,
                None => return false,
                _ => {}
            }
        }
        if !self.tags.is_empty() && !self.tags.iter().any(|t| spec.frontmatter.tags.contains(t)) {
            return false;
        }
        true
    }
}

/// Ordered statuses shown in the filter popup.
pub const FILTER_STATUSES: &[SpecStatus] = &[
    SpecStatus::InProgress,
    SpecStatus::Planned,
    SpecStatus::Draft,
    SpecStatus::Complete,
    SpecStatus::Archived,
];

/// Ordered priorities shown in the filter popup.
pub const FILTER_PRIORITIES: &[SpecPriority] = &[
    SpecPriority::Critical,
    SpecPriority::High,
    SpecPriority::Medium,
    SpecPriority::Low,
];

/// One row in the tree view of the list pane.
#[derive(Debug, Clone)]
pub struct TreeRow {
    pub spec_idx: usize,
    pub depth: usize,
    pub has_children: bool,
    pub is_collapsed: bool,
}

/// Which view is shown in the left pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PrimaryView {
    Board,
    List,
}

/// Which pane has focus.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum FocusPane {
    Left,
    Right,
}

/// What to show in the right pane.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum DetailMode {
    Content,
    Dependencies,
}

/// A group of specs sharing the same status, used for the board view.
#[derive(Debug, Clone)]
pub struct BoardGroup {
    pub status: SpecStatus,
    pub label: String,
    pub indices: Vec<usize>,
    pub collapsed: bool,
}

/// Core application state.
pub struct App {
    // Data
    pub specs: Vec<SpecInfo>,
    pub filtered_specs: Vec<usize>,
    pub selected_detail: Option<SpecInfo>,
    pub board_groups: Vec<BoardGroup>,
    pub dep_graph: DependencyGraph,
    pub stats: SpecStats,

    // Loader for lazy detail loading
    loader: SpecLoader,

    // State machine
    pub mode: AppMode,
    pub primary_view: PrimaryView,
    pub focus: FocusPane,
    pub detail_mode: DetailMode,
    pub should_quit: bool,

    // Board navigation
    pub board_group_idx: usize,
    pub board_item_idx: usize,

    // List navigation
    pub list_selected: usize,

    // Detail scroll
    pub detail_scroll: u16,
    /// Upper bound for detail_scroll estimated from content line count.
    pub detail_content_lines: u16,

    // Search
    pub search_query: String,
    pub search_results: Vec<usize>,

    // Layout / sidebar state
    pub sidebar_width_pct: u16,
    pub sidebar_collapsed: bool,
    pub drag_resize: bool,
    pub layout_left: Rect,
    pub layout_right: Rect,
    pub last_frame_width: u16,

    // Sort & filter
    pub sort_option: SortOption,
    pub filter: FilterState,
    /// Cursor position in the filter popup (0..FILTER_STATUSES.len() = status, then priorities).
    pub filter_cursor: usize,

    // Tree view
    pub tree_mode: bool,
    pub tree_collapsed: HashSet<String>,
    pub tree_rows: Vec<TreeRow>,

    // TOC overlay
    /// Headings extracted from the currently displayed spec: (line_idx, level, text)
    pub detail_toc: Vec<(usize, u8, String)>,
    /// Cursor position in the TOC overlay.
    pub toc_selected: usize,
}

fn priority_sort_key(p: Option<SpecPriority>) -> u8 {
    match p {
        Some(SpecPriority::Critical) => 0,
        Some(SpecPriority::High) => 1,
        Some(SpecPriority::Medium) => 2,
        Some(SpecPriority::Low) => 3,
        None => 4,
    }
}

impl App {
    pub fn new(specs_dir: &str, initial_view: PrimaryView) -> Result<Self, Box<dyn Error>> {
        let loader = SpecLoader::new(specs_dir);
        let specs = loader.load_all_metadata()?;
        let dep_graph = DependencyGraph::new(&specs);
        let stats = SpecStats::compute(&specs);

        let mut app = Self {
            filtered_specs: (0..specs.len()).collect(),
            specs,
            selected_detail: None,
            board_groups: Vec::new(),
            dep_graph,
            stats,
            loader,
            mode: AppMode::Normal,
            primary_view: initial_view,
            focus: FocusPane::Left,
            detail_mode: DetailMode::Content,
            should_quit: false,
            board_group_idx: 0,
            board_item_idx: 0,
            list_selected: 0,
            detail_scroll: 0,
            detail_content_lines: u16::MAX,
            search_query: String::new(),
            search_results: Vec::new(),
            sidebar_width_pct: 30,
            sidebar_collapsed: false,
            drag_resize: false,
            layout_left: Rect::default(),
            layout_right: Rect::default(),
            last_frame_width: 0,
            sort_option: SortOption::default(),
            filter: FilterState::default(),
            filter_cursor: 0,
            tree_mode: false,
            tree_collapsed: HashSet::new(),
            tree_rows: Vec::new(),
            detail_toc: Vec::new(),
            toc_selected: 0,
        };

        app.apply_filter_and_sort();
        app.load_selected_detail();

        Ok(app)
    }

    /// Apply current filter + sort to rebuild `filtered_specs`, board groups, and tree rows.
    pub fn apply_filter_and_sort(&mut self) {
        // 1. Filter
        self.filtered_specs = (0..self.specs.len())
            .filter(|&i| self.filter.matches(&self.specs[i]))
            .collect();

        // 2. Sort
        let specs = &self.specs;
        let sort = self.sort_option;
        self.filtered_specs.sort_by(|&a, &b| {
            let sa = &specs[a];
            let sb = &specs[b];
            match sort {
                SortOption::IdDesc => sb.number().cmp(&sa.number()),
                SortOption::IdAsc => sa.number().cmp(&sb.number()),
                SortOption::PriorityDesc => priority_sort_key(sb.frontmatter.priority)
                    .cmp(&priority_sort_key(sa.frontmatter.priority)),
                SortOption::TitleAsc => sa.title.to_lowercase().cmp(&sb.title.to_lowercase()),
                SortOption::UpdatedDesc => {
                    let ta = sa.frontmatter.updated_at.or(sa.frontmatter.created_at);
                    let tb = sb.frontmatter.updated_at.or(sb.frontmatter.created_at);
                    tb.cmp(&ta)
                }
            }
        });

        // 3. Rebuild board groups
        self.rebuild_board_groups_from_filtered();

        // 4. Rebuild tree rows
        self.rebuild_tree_rows();

        // 5. Clamp navigation indices
        let max_group = self.board_groups.len().saturating_sub(1);
        self.board_group_idx = self.board_group_idx.min(max_group);
        if let Some(group) = self.board_groups.get(self.board_group_idx) {
            self.board_item_idx = self
                .board_item_idx
                .min(group.indices.len().saturating_sub(1));
        }
        let max_list = self.visible_list_len().saturating_sub(1);
        self.list_selected = self.list_selected.min(max_list);
    }

    fn rebuild_board_groups_from_filtered(&mut self) {
        // Preserve existing collapsed state by status
        let prev_collapsed: std::collections::HashMap<SpecStatus, bool> = self
            .board_groups
            .iter()
            .map(|g| (g.status, g.collapsed))
            .collect();

        let statuses = [
            (SpecStatus::InProgress, "In Progress"),
            (SpecStatus::Planned, "Planned"),
            (SpecStatus::Draft, "Draft"),
            (SpecStatus::Complete, "Complete"),
            (SpecStatus::Archived, "Archived"),
        ];

        self.board_groups = statuses
            .iter()
            .filter_map(|(status, label)| {
                let indices: Vec<usize> = self
                    .filtered_specs
                    .iter()
                    .filter(|&&i| self.specs[i].frontmatter.status == *status)
                    .copied()
                    .collect();
                if indices.is_empty() {
                    None
                } else {
                    let collapsed = prev_collapsed.get(status).copied().unwrap_or(false);
                    Some(BoardGroup {
                        status: *status,
                        label: label.to_string(),
                        indices,
                        collapsed,
                    })
                }
            })
            .collect();
    }

    /// Rebuild tree_rows from filtered_specs and parent relationships.
    pub fn rebuild_tree_rows(&mut self) {
        // Build a path → index map for the filtered set
        let path_to_idx: std::collections::HashMap<&str, usize> = self
            .filtered_specs
            .iter()
            .map(|&i| (self.specs[i].path.as_str(), i))
            .collect();

        let mut children_map: std::collections::HashMap<String, Vec<usize>> =
            std::collections::HashMap::new();
        let mut root_indices: Vec<usize> = Vec::new();

        for &i in &self.filtered_specs {
            let spec = &self.specs[i];
            if let Some(parent_path) = spec.frontmatter.parent.as_deref() {
                if path_to_idx.contains_key(parent_path) {
                    children_map
                        .entry(parent_path.to_string())
                        .or_default()
                        .push(i);
                    continue;
                }
            }
            root_indices.push(i);
        }

        // DFS traversal to build tree_rows
        let mut rows: Vec<TreeRow> = Vec::new();
        for root_idx in root_indices {
            Self::dfs_tree(
                root_idx,
                0,
                &self.specs,
                &children_map,
                &self.tree_collapsed,
                &mut rows,
            );
        }
        self.tree_rows = rows;
    }

    fn dfs_tree(
        spec_idx: usize,
        depth: usize,
        specs: &[SpecInfo],
        children_map: &std::collections::HashMap<String, Vec<usize>>,
        collapsed: &HashSet<String>,
        rows: &mut Vec<TreeRow>,
    ) {
        let path = &specs[spec_idx].path;
        let children = children_map
            .get(path)
            .map_or(&[] as &[usize], |v| v.as_slice());
        let has_children = !children.is_empty();
        let is_collapsed = collapsed.contains(path);

        rows.push(TreeRow {
            spec_idx,
            depth,
            has_children,
            is_collapsed,
        });

        if !is_collapsed {
            for &child_idx in children {
                Self::dfs_tree(child_idx, depth + 1, specs, children_map, collapsed, rows);
            }
        }
    }

    /// Length of the visible list (flat or tree).
    fn visible_list_len(&self) -> usize {
        if self.tree_mode {
            self.tree_rows.len()
        } else {
            self.filtered_specs.len()
        }
    }

    // -- Sort & filter --

    pub fn cycle_sort(&mut self) {
        self.sort_option = self.sort_option.next();
        self.apply_filter_and_sort();
        self.load_selected_detail();
    }

    pub fn open_filter(&mut self) {
        self.mode = AppMode::Filter;
    }

    pub fn close_filter(&mut self) {
        self.mode = AppMode::Normal;
        self.apply_filter_and_sort();
        self.load_selected_detail();
    }

    pub fn clear_filters(&mut self) {
        self.filter = FilterState::default();
        self.apply_filter_and_sort();
        self.load_selected_detail();
    }

    /// Move cursor down in the filter popup.
    pub fn filter_cursor_down(&mut self) {
        let total = FILTER_STATUSES.len() + FILTER_PRIORITIES.len();
        if self.filter_cursor + 1 < total {
            self.filter_cursor += 1;
        }
    }

    /// Move cursor up in the filter popup.
    pub fn filter_cursor_up(&mut self) {
        self.filter_cursor = self.filter_cursor.saturating_sub(1);
    }

    /// Toggle the item at the current filter cursor position.
    pub fn filter_toggle_current(&mut self) {
        let n_statuses = FILTER_STATUSES.len();
        if self.filter_cursor < n_statuses {
            let status = FILTER_STATUSES[self.filter_cursor];
            if let Some(pos) = self.filter.statuses.iter().position(|&s| s == status) {
                self.filter.statuses.remove(pos);
            } else {
                self.filter.statuses.push(status);
            }
        } else {
            let pri_idx = self.filter_cursor - n_statuses;
            if let Some(&priority) = FILTER_PRIORITIES.get(pri_idx) {
                if let Some(pos) = self.filter.priorities.iter().position(|&p| p == priority) {
                    self.filter.priorities.remove(pos);
                } else {
                    self.filter.priorities.push(priority);
                }
            }
        }
    }

    // -- Tree view --

    pub fn toggle_tree(&mut self) {
        self.tree_mode = !self.tree_mode;
        self.list_selected = 0;
        self.load_selected_detail();
    }

    pub fn collapse_all(&mut self) {
        // Collect paths of all specs that have children in the filtered set
        let children_parents: HashSet<String> = self
            .filtered_specs
            .iter()
            .filter_map(|&i| self.specs[i].frontmatter.parent.clone())
            .collect();
        for path in children_parents {
            self.tree_collapsed.insert(path);
        }
        self.rebuild_tree_rows();
        self.list_selected = self
            .list_selected
            .min(self.tree_rows.len().saturating_sub(1));
    }

    pub fn expand_all(&mut self) {
        self.tree_collapsed.clear();
        self.rebuild_tree_rows();
    }

    pub fn toggle_current_tree_node(&mut self) {
        if !self.tree_mode {
            return;
        }
        if let Some(row) = self.tree_rows.get(self.list_selected) {
            if !row.has_children {
                return;
            }
            let path = self.specs[row.spec_idx].path.clone();
            if self.tree_collapsed.contains(&path) {
                self.tree_collapsed.remove(&path);
            } else {
                self.tree_collapsed.insert(path);
            }
            self.rebuild_tree_rows();
            self.list_selected = self
                .list_selected
                .min(self.tree_rows.len().saturating_sub(1));
        }
    }

    /// Get the currently selected spec index based on the active view.
    pub fn selected_spec_index(&self) -> Option<usize> {
        match self.primary_view {
            PrimaryView::Board => {
                let group = self.board_groups.get(self.board_group_idx)?;
                if group.collapsed {
                    return None;
                }
                group.indices.get(self.board_item_idx).copied()
            }
            PrimaryView::List => {
                if self.tree_mode {
                    self.tree_rows.get(self.list_selected).map(|r| r.spec_idx)
                } else {
                    self.filtered_specs.get(self.list_selected).copied()
                }
            }
        }
    }

    /// Lazily load the full content of the currently selected spec.
    pub fn load_selected_detail(&mut self) {
        if let Some(idx) = self.selected_spec_index() {
            let Some(spec) = self.specs.get(idx) else {
                self.selected_detail = None;
                self.detail_toc = Vec::new();
                return;
            };
            let path = &spec.path;
            if let Ok(Some(full)) = self.loader.load(path) {
                self.detail_content_lines = full.content.lines().count() as u16;
                self.detail_toc = Self::extract_headings_inner(&full.content);
                self.selected_detail = Some(full);
            } else {
                self.selected_detail = None;
                self.detail_content_lines = u16::MAX;
                self.detail_toc = Vec::new();
            }
        } else {
            self.selected_detail = None;
            self.detail_content_lines = u16::MAX;
            self.detail_toc = Vec::new();
        }
        self.detail_scroll = 0;
        self.toc_selected = 0;
    }

    /// Extract ## and ### headings from markdown content.
    /// Returns (line_index, level, heading_text).
    fn extract_headings_inner(content: &str) -> Vec<(usize, u8, String)> {
        let mut headings: Vec<(usize, u8, String)> = Vec::new();
        let mut line_idx: usize = 0;
        let mut in_code_block = false;
        let mut code_len: usize = 0;

        for raw_line in content.lines() {
            let trimmed = raw_line.trim_end();

            if trimmed.starts_with("```") {
                if in_code_block {
                    // End of code block — count its rendered lines
                    line_idx += code_len + 2;
                    in_code_block = false;
                    code_len = 0;
                } else {
                    in_code_block = true;
                }
                continue;
            }

            if in_code_block {
                code_len += 1;
                continue;
            }

            if let Some(rest) = trimmed.strip_prefix("## ") {
                headings.push((line_idx, 2, rest.to_string()));
            } else if let Some(rest) = trimmed.strip_prefix("### ") {
                headings.push((line_idx, 3, rest.to_string()));
            }

            line_idx += 1;
        }

        headings
    }

    // -- Navigation --

    pub fn move_down(&mut self) {
        match self.primary_view {
            PrimaryView::Board => {
                let group_len = self
                    .board_groups
                    .get(self.board_group_idx)
                    .map(|g| if g.collapsed { 0 } else { g.indices.len() })
                    .unwrap_or(0);
                if self.board_item_idx + 1 < group_len {
                    self.board_item_idx += 1;
                } else if self.board_group_idx + 1 < self.board_groups.len() {
                    // Move to next group
                    self.board_group_idx += 1;
                    self.board_item_idx = 0;
                }
            }
            PrimaryView::List => {
                let max = self.visible_list_len().saturating_sub(1);
                if self.list_selected < max {
                    self.list_selected += 1;
                }
            }
        }
        self.load_selected_detail();
    }

    pub fn move_up(&mut self) {
        match self.primary_view {
            PrimaryView::Board => {
                if self.board_item_idx > 0 {
                    self.board_item_idx -= 1;
                } else if self.board_group_idx > 0 {
                    // Move to previous group
                    self.board_group_idx -= 1;
                    let prev_len = self
                        .board_groups
                        .get(self.board_group_idx)
                        .map(|g| if g.collapsed { 0 } else { g.indices.len() })
                        .unwrap_or(0);
                    self.board_item_idx = prev_len.saturating_sub(1);
                }
            }
            PrimaryView::List => {
                self.list_selected = self.list_selected.saturating_sub(1);
            }
        }
        self.load_selected_detail();
    }

    /// Jump to the first item.
    pub fn move_first(&mut self) {
        match self.primary_view {
            PrimaryView::Board => {
                self.board_item_idx = 0;
            }
            PrimaryView::List => {
                self.list_selected = 0;
            }
        }
        self.load_selected_detail();
    }

    /// Jump to the last item.
    pub fn move_last(&mut self) {
        match self.primary_view {
            PrimaryView::Board => {
                if let Some(group) = self.board_groups.get(self.board_group_idx) {
                    self.board_item_idx = group.indices.len().saturating_sub(1);
                }
            }
            PrimaryView::List => {
                self.list_selected = self.visible_list_len().saturating_sub(1);
            }
        }
        self.load_selected_detail();
    }

    /// Move down by page_size rows.
    pub fn page_down(&mut self, page_size: usize) {
        match self.primary_view {
            PrimaryView::Board => {
                if let Some(group) = self.board_groups.get(self.board_group_idx) {
                    self.board_item_idx = (self.board_item_idx + page_size)
                        .min(group.indices.len().saturating_sub(1));
                }
            }
            PrimaryView::List => {
                let max = self.visible_list_len().saturating_sub(1);
                self.list_selected = (self.list_selected + page_size).min(max);
            }
        }
        self.load_selected_detail();
    }

    /// Move up by page_size rows.
    pub fn page_up(&mut self, page_size: usize) {
        match self.primary_view {
            PrimaryView::Board => {
                self.board_item_idx = self.board_item_idx.saturating_sub(page_size);
            }
            PrimaryView::List => {
                self.list_selected = self.list_selected.saturating_sub(page_size);
            }
        }
        self.load_selected_detail();
    }

    pub fn next_group(&mut self) {
        if self.primary_view == PrimaryView::Board && !self.board_groups.is_empty() {
            self.board_group_idx = (self.board_group_idx + 1) % self.board_groups.len();
            self.board_item_idx = 0;
            self.load_selected_detail();
        }
    }

    pub fn prev_group(&mut self) {
        if self.primary_view == PrimaryView::Board && !self.board_groups.is_empty() {
            if self.board_group_idx == 0 {
                self.board_group_idx = self.board_groups.len() - 1;
            } else {
                self.board_group_idx -= 1;
            }
            self.board_item_idx = 0;
            self.load_selected_detail();
        }
    }

    /// Toggle collapse/expand of the current board group.
    pub fn toggle_current_board_group(&mut self) {
        if let Some(group) = self.board_groups.get_mut(self.board_group_idx) {
            group.collapsed = !group.collapsed;
            // If now collapsed, reset item cursor to 0 so it doesn't point past end
            if group.collapsed {
                self.board_item_idx = 0;
            }
        }
    }

    /// Collapse all board groups.
    pub fn collapse_all_board_groups(&mut self) {
        for group in &mut self.board_groups {
            group.collapsed = true;
        }
        self.board_item_idx = 0;
    }

    /// Expand all board groups.
    pub fn expand_all_board_groups(&mut self) {
        for group in &mut self.board_groups {
            group.collapsed = false;
        }
    }

    pub fn scroll_detail_down(&mut self) {
        if self.detail_scroll < self.detail_content_lines {
            self.detail_scroll = self.detail_scroll.saturating_add(1);
        }
    }

    pub fn scroll_detail_up(&mut self) {
        self.detail_scroll = self.detail_scroll.saturating_sub(1);
    }

    // -- View switching --

    pub fn set_board_view(&mut self) {
        self.primary_view = PrimaryView::Board;
        self.focus = FocusPane::Left;
    }

    pub fn set_list_view(&mut self) {
        self.primary_view = PrimaryView::List;
        self.focus = FocusPane::Left;
    }

    pub fn toggle_detail_mode(&mut self) {
        self.detail_mode = match self.detail_mode {
            DetailMode::Content => DetailMode::Dependencies,
            DetailMode::Dependencies => DetailMode::Content,
        };
    }

    pub fn focus_left(&mut self) {
        self.focus = FocusPane::Left;
    }

    pub fn focus_right(&mut self) {
        self.focus = FocusPane::Right;
        self.detail_scroll = 0;
    }

    // -- Mode transitions --

    pub fn enter_search(&mut self) {
        self.mode = AppMode::Search;
        self.search_query.clear();
        self.search_results.clear();
    }

    pub fn exit_search(&mut self) {
        self.mode = AppMode::Normal;
    }

    pub fn enter_help(&mut self) {
        self.mode = AppMode::Help;
    }

    pub fn exit_overlay(&mut self) {
        self.mode = AppMode::Normal;
    }

    pub fn open_toc(&mut self) {
        if !self.detail_toc.is_empty() {
            // Position cursor at currently visible section
            self.toc_selected = self.current_toc_section();
            self.mode = AppMode::Toc;
        }
    }

    pub fn close_toc(&mut self) {
        self.mode = AppMode::Normal;
    }

    pub fn toc_move_down(&mut self) {
        if self.toc_selected + 1 < self.detail_toc.len() {
            self.toc_selected += 1;
        }
    }

    pub fn toc_move_up(&mut self) {
        self.toc_selected = self.toc_selected.saturating_sub(1);
    }

    /// Jump to the TOC entry at `toc_selected` and close the overlay.
    pub fn toc_jump(&mut self) {
        if let Some(&(line_idx, _, _)) = self.detail_toc.get(self.toc_selected) {
            self.detail_scroll = line_idx as u16;
        }
        self.close_toc();
    }

    /// Return the index into `detail_toc` of the section currently visible
    /// (last heading whose line_idx <= detail_scroll).
    pub fn current_toc_section(&self) -> usize {
        let scroll = self.detail_scroll as usize;
        let mut best = 0;
        for (i, &(line_idx, _, _)) in self.detail_toc.iter().enumerate() {
            if line_idx <= scroll {
                best = i;
            }
        }
        best
    }

    // -- Search --

    pub fn search_type_char(&mut self, c: char) {
        self.search_query.push(c);
        self.update_search_results();
    }

    pub fn search_backspace(&mut self) {
        self.search_query.pop();
        self.update_search_results();
    }

    pub fn search_select(&mut self) {
        if let Some(&idx) = self.search_results.first() {
            // Navigate to the found spec
            match self.primary_view {
                PrimaryView::Board => {
                    // Find the board group and item for this spec index
                    for (gi, group) in self.board_groups.iter().enumerate() {
                        if let Some(ii) = group.indices.iter().position(|&i| i == idx) {
                            self.board_group_idx = gi;
                            self.board_item_idx = ii;
                            break;
                        }
                    }
                }
                PrimaryView::List => {
                    if let Some(pos) = self.filtered_specs.iter().position(|&i| i == idx) {
                        self.list_selected = pos;
                    }
                }
            }
            self.load_selected_detail();
        }
        self.exit_search();
    }

    // -- Sidebar width / collapse --

    pub fn sidebar_widen(&mut self) {
        self.sidebar_width_pct = (self.sidebar_width_pct + 5).min(60);
    }

    pub fn sidebar_narrow(&mut self) {
        self.sidebar_width_pct = self.sidebar_width_pct.saturating_sub(5).max(15);
    }

    pub fn sidebar_toggle_collapse(&mut self) {
        self.sidebar_collapsed = !self.sidebar_collapsed;
    }

    /// Select a spec in the sidebar by clicking at a terminal row.
    pub fn click_sidebar(&mut self, row: u16) {
        self.focus = FocusPane::Left;
        match self.primary_view {
            PrimaryView::List => {
                let content_row = row.saturating_sub(self.layout_left.y).saturating_sub(1);
                let item_row = content_row.saturating_sub(2) as usize;
                let visible_rows = self.layout_left.height.saturating_sub(4) as usize;
                let offset = if self.list_selected >= visible_rows {
                    self.list_selected - visible_rows + 1
                } else {
                    0
                };
                let new_idx = offset + item_row;
                if new_idx < self.filtered_specs.len() {
                    self.list_selected = new_idx;
                    self.load_selected_detail();
                }
            }
            PrimaryView::Board => {
                // Iterate groups counting rows to find which spec was clicked
                let mut current_row = self.layout_left.y + 1; // inside border
                'outer: for (gi, group) in self.board_groups.iter().enumerate() {
                    // Group header row
                    current_row += 1;
                    // Item rows
                    for (ii, _) in group.indices.iter().enumerate() {
                        if row == current_row {
                            self.board_group_idx = gi;
                            self.board_item_idx = ii;
                            self.load_selected_detail();
                            break 'outer;
                        }
                        current_row += 1;
                    }
                    // Blank line between groups
                    current_row += 1;
                }
            }
        }
    }

    /// Update sidebar width based on a mouse drag to column `col`.
    pub fn resize_drag_to(&mut self, col: u16) {
        if self.last_frame_width == 0 {
            return;
        }
        let new_pct = (col as u32 * 100 / self.last_frame_width as u32) as u16;
        self.sidebar_width_pct = new_pct.clamp(15, 60);
    }

    fn update_search_results(&mut self) {
        if self.search_query.is_empty() {
            self.search_results.clear();
            return;
        }
        let results = search_specs(&self.specs, &self.search_query, 20);
        self.search_results = results
            .iter()
            .filter_map(|r| self.specs.iter().position(|s| s.path == r.path))
            .collect();
    }

    /// Create an empty App for testing (no filesystem access needed).
    #[cfg(test)]
    pub fn empty_for_test() -> Self {
        App {
            specs: Vec::new(),
            filtered_specs: Vec::new(),
            selected_detail: None,
            board_groups: Vec::new(),
            dep_graph: DependencyGraph::new(&[]),
            stats: SpecStats::compute(&[]),
            loader: SpecLoader::new("/nonexistent"),
            mode: AppMode::Normal,
            primary_view: PrimaryView::List,
            focus: FocusPane::Left,
            detail_mode: DetailMode::Content,
            should_quit: false,
            board_group_idx: 0,
            board_item_idx: 0,
            list_selected: 0,
            detail_scroll: 0,
            detail_content_lines: u16::MAX,
            search_query: String::new(),
            search_results: Vec::new(),
            sidebar_width_pct: 30,
            sidebar_collapsed: false,
            drag_resize: false,
            layout_left: Rect::default(),
            layout_right: Rect::default(),
            last_frame_width: 0,
            sort_option: SortOption::default(),
            filter: FilterState::default(),
            filter_cursor: 0,
            tree_mode: false,
            tree_collapsed: HashSet::new(),
            tree_rows: Vec::new(),
            detail_toc: Vec::new(),
            toc_selected: 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn make_test_app() -> App {
        App::empty_for_test()
    }

    #[test]
    fn test_mode_transitions() {
        let mut app = make_test_app();
        assert_eq!(app.mode, AppMode::Normal);

        app.enter_search();
        assert_eq!(app.mode, AppMode::Search);

        app.exit_search();
        assert_eq!(app.mode, AppMode::Normal);

        app.enter_help();
        assert_eq!(app.mode, AppMode::Help);

        app.exit_overlay();
        assert_eq!(app.mode, AppMode::Normal);
    }

    #[test]
    fn test_view_switching() {
        let mut app = make_test_app();
        // Default view is now List
        assert_eq!(app.primary_view, PrimaryView::List);

        app.set_board_view();
        assert_eq!(app.primary_view, PrimaryView::Board);
        assert_eq!(app.focus, FocusPane::Left);

        app.set_list_view();
        assert_eq!(app.primary_view, PrimaryView::List);
        assert_eq!(app.focus, FocusPane::Left);
    }

    #[test]
    fn test_focus_toggle() {
        let mut app = make_test_app();
        assert_eq!(app.focus, FocusPane::Left);

        app.focus_right();
        assert_eq!(app.focus, FocusPane::Right);

        app.focus_left();
        assert_eq!(app.focus, FocusPane::Left);
    }

    #[test]
    fn test_detail_mode_toggle() {
        let mut app = make_test_app();
        assert_eq!(app.detail_mode, DetailMode::Content);

        app.toggle_detail_mode();
        assert_eq!(app.detail_mode, DetailMode::Dependencies);

        app.toggle_detail_mode();
        assert_eq!(app.detail_mode, DetailMode::Content);
    }

    #[test]
    fn test_board_navigation_empty() {
        let mut app = make_test_app();
        // Should not panic with empty groups
        app.move_down();
        app.move_up();
        app.next_group();
        app.prev_group();
        assert_eq!(app.board_group_idx, 0);
        assert_eq!(app.board_item_idx, 0);
    }

    #[test]
    fn test_list_navigation_empty() {
        let mut app = make_test_app();
        app.set_list_view();
        // Should not panic with empty list
        app.move_down();
        app.move_up();
        assert_eq!(app.list_selected, 0);
    }

    #[test]
    fn test_board_navigation_wraps_groups() {
        let mut app = make_test_app();
        app.primary_view = PrimaryView::Board;
        app.board_groups = vec![
            BoardGroup {
                status: SpecStatus::InProgress,
                label: "In Progress".to_string(),
                indices: vec![0],
                collapsed: false,
            },
            BoardGroup {
                status: SpecStatus::Draft,
                label: "Draft".to_string(),
                indices: vec![1],
                collapsed: false,
            },
        ];

        app.next_group();
        assert_eq!(app.board_group_idx, 1);

        app.next_group();
        assert_eq!(app.board_group_idx, 0); // wraps

        app.prev_group();
        assert_eq!(app.board_group_idx, 1); // wraps back
    }

    #[test]
    fn test_list_navigation_bounds() {
        let mut app = make_test_app();
        app.set_list_view();
        app.filtered_specs = vec![0, 1, 2];

        app.move_down();
        assert_eq!(app.list_selected, 1);
        app.move_down();
        assert_eq!(app.list_selected, 2);
        app.move_down();
        assert_eq!(app.list_selected, 2); // stays at end

        app.move_up();
        assert_eq!(app.list_selected, 1);
        app.move_up();
        assert_eq!(app.list_selected, 0);
        app.move_up();
        assert_eq!(app.list_selected, 0); // stays at start
    }

    #[test]
    fn test_detail_scroll() {
        let mut app = make_test_app();
        assert_eq!(app.detail_scroll, 0);

        app.scroll_detail_down();
        assert_eq!(app.detail_scroll, 1);
        app.scroll_detail_down();
        assert_eq!(app.detail_scroll, 2);

        app.scroll_detail_up();
        assert_eq!(app.detail_scroll, 1);
        app.scroll_detail_up();
        assert_eq!(app.detail_scroll, 0);
        app.scroll_detail_up();
        assert_eq!(app.detail_scroll, 0); // doesn't go negative
    }

    #[test]
    fn test_sidebar_widen_narrow() {
        let mut app = make_test_app();
        assert_eq!(app.sidebar_width_pct, 30);

        app.sidebar_widen();
        assert_eq!(app.sidebar_width_pct, 35);

        app.sidebar_widen();
        app.sidebar_widen();
        app.sidebar_widen();
        app.sidebar_widen();
        app.sidebar_widen(); // would be 60
        assert_eq!(app.sidebar_width_pct, 60);

        app.sidebar_widen(); // capped at 60
        assert_eq!(app.sidebar_width_pct, 60);

        app.sidebar_narrow();
        assert_eq!(app.sidebar_width_pct, 55);

        // Narrow to minimum
        for _ in 0..20 {
            app.sidebar_narrow();
        }
        assert_eq!(app.sidebar_width_pct, 15);
    }

    #[test]
    fn test_sidebar_toggle_collapse() {
        let mut app = make_test_app();
        assert!(!app.sidebar_collapsed);

        app.sidebar_toggle_collapse();
        assert!(app.sidebar_collapsed);

        app.sidebar_toggle_collapse();
        assert!(!app.sidebar_collapsed);
    }

    #[test]
    fn test_resize_drag_to() {
        let mut app = make_test_app();
        app.last_frame_width = 100;

        app.resize_drag_to(30);
        assert_eq!(app.sidebar_width_pct, 30);

        app.resize_drag_to(5); // below minimum, clamped to 15
        assert_eq!(app.sidebar_width_pct, 15);

        app.resize_drag_to(70); // above maximum, clamped to 60
        assert_eq!(app.sidebar_width_pct, 60);
    }

    #[test]
    fn test_resize_drag_zero_width_noop() {
        let mut app = make_test_app();
        app.last_frame_width = 0;
        app.sidebar_width_pct = 30;
        app.resize_drag_to(50);
        assert_eq!(app.sidebar_width_pct, 30); // unchanged
    }

    #[test]
    fn test_search_char_and_backspace() {
        let mut app = make_test_app();
        app.enter_search();
        assert_eq!(app.search_query, "");

        app.search_type_char('a');
        assert_eq!(app.search_query, "a");
        app.search_type_char('b');
        assert_eq!(app.search_query, "ab");

        app.search_backspace();
        assert_eq!(app.search_query, "a");
        app.search_backspace();
        assert_eq!(app.search_query, "");
        app.search_backspace(); // no panic on empty
        assert_eq!(app.search_query, "");
    }
}
