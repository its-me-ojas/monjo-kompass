use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::state::AppState;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Header
            Constraint::Min(0),    // List
            Constraint::Length(1), // Footer
        ])
        .split(area);

    render_header(f, chunks[0], state);
    render_collection_list(f, chunks[1], state);
    render_footer(f, chunks[2]);
}

fn render_header(f: &mut Frame, area: Rect, state: &AppState) {
    let title = if let Some(db) = &state.current_database {
        format!(" Database: {} ", db)
    } else {
        " No database selected ".to_string()
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::BOTTOM));

    f.render_widget(header, area);
}

fn render_collection_list(f: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .collections
        .iter()
        .enumerate()
        .map(|(i, coll)| {
            let prefix = if i == state.selected_coll_index {
                "> "
            } else {
                "  "
            };

            let content = format!(
                "{}{} ({} documents, {} indexes)",
                prefix,
                coll.name,
                coll.document_count,
                coll.indexes.len()
            );

            let style = if i == state.selected_coll_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };
            ListItem::new(Line::from(Span::styled(content, style)))
        })
        .collect();
    
    let list = List::new(items)
        .block(Block::default().title(" Collections ").title_style(Style::default().fg(Color::Gray)))
        .style(Style::default().fg(Color::White));
        
    f.render_widget(list, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let footer_text = " [q] Quit  [↑/↓] Navigate  [Enter] View Docs  [Back] Go Back  [r] Refresh ";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray).bg(Color::Black));

    f.render_widget(footer, area);
}
