use ratatui::{
    layout::{Constraint, Rect},
    text::Line,
    widgets::{Block, Borders, Cell, Paragraph, Row, Table, TableState},
    Frame,
};

use crate::{fs::entry::FsEntry, ui::theme};

pub fn render(
    frame: &mut Frame,
    area: Rect,
    title: &str,
    entries: &[FsEntry],
    selected: Option<usize>,
    scroll: usize,
    dim_content: bool,
    active: bool,
) {
    let border_style = if active {
        theme::active_border()
    } else {
        theme::border()
    };

    if entries.is_empty() {
        let empty = Paragraph::new("(empty)").block(
            Block::default()
                .borders(Borders::ALL)
                .border_style(border_style)
                .title(title),
        );
        frame.render_widget(empty, area);
        return;
    }

    // STEP 1: Determine the base text style based on the dim_content flag
    let text_style = if dim_content {
        theme::dim()
    } else {
        ratatui::style::Style::default()
    };

    // STEP 2: Convert FsEntry data into Table Rows instead of ListItems
    let rows: Vec<Row> = entries
        .iter()
        .map(|entry| {
            // TO MODIFY COLUMNS: 
            // You will need to replace the placeholder strings ("--") below with the 
            // actual methods from your `FsEntry` struct (e.g., `entry.size()`, `entry.modified_date()`).
            Row::new(vec![
                Cell::from(entry.display_name()).style(text_style), // Column 1: Name
                Cell::from(entry.display_size()).style(text_style), // Column 2: Size
                Cell::from(entry.display_date()).style(text_style), // Column 3: Date
            ])
            // Optional: You can add height/margin to rows here if needed
            // .height(1) 
        })
        .collect();

    // STEP 3: Build the Table Widget
    // We pass our rows and define the width of each column using Constraints
    let table = Table::new(
        rows,
        // TO MODIFY COLUMN WIDTHS:
        // Adjust these constraints to fit your data. 
        // Example: Name takes up remaining space (Min), Size is exactly 10 chars, Date is exactly 12 chars.
        [
            Constraint::Min(20),      // Name column
            Constraint::Length(10),   // Size column
            Constraint::Length(12),   // Date column
        ],
    )
    // Optional: Add a header row to your table
    .header(
        Row::new(vec!["Name", "Size", "Date"])
            .style(theme::dim()) // Style the header differently if you want
            .bottom_margin(1),   // Adds a blank space below the header
    )
    .block(
        Block::default()
            .borders(Borders::ALL)
            .border_style(border_style)
            .title(title),
    )
    .highlight_style(theme::selection())
    .highlight_symbol("  ");

    // STEP 4: Manage State (Identical to ListState!)
    let mut state = TableState::default();
    state.select(selected);
    state = state.with_offset(scroll);

    frame.render_stateful_widget(table, area, &mut state);
}