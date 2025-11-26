use ratatui::{
    Frame,
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Wrap},
};

use crate::app::state::AppState;

pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(30), Constraint::Percentage(70)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2), // Header
            Constraint::Length(3), // Filter
            Constraint::Min(0),    // List
            Constraint::Length(1), // Footer
        ])
        .split(chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Content
            Constraint::Length(1), // Footer
        ])
        .split(chunks[1]);

    render_header(f, left_chunks[0], state);
    render_filter_input(f, left_chunks[1], state);
    render_document_list(f, left_chunks[2], state);
    render_footer(f, left_chunks[3]);
    
    render_document_content(f, right_chunks[0], state);
    render_content_footer(f, right_chunks[1]);
}

fn render_header(f: &mut Frame, area: Rect, state: &AppState) {
    let title = if let (Some(db), Some(coll)) = (&state.current_database, &state.current_collection)
    {
        format!(" {}.{} ", db, coll)
    } else {
        " No collection selected ".to_string()
    };

    let header = Paragraph::new(title)
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .block(Block::default().borders(Borders::BOTTOM));

    f.render_widget(header, area);
}

fn render_filter_input(f: &mut Frame, area: Rect, state: &AppState) {
    let (style, title, text) = if state.query_mode {
        (
            Style::default().fg(Color::Magenta),
            " Query (JSON) ",
            state.query_input.as_str(),
        )
    } else if state.filter_mode {
        (
            Style::default().fg(Color::Yellow),
            " Search ",
            state.filter_input.as_str(),
        )
    } else if state.filter.is_some() {
        (
            Style::default().fg(Color::Green),
            " Active Filter ",
            "...", 
        )
    } else {
        (
            Style::default().fg(Color::DarkGray),
            " Filter ",
            "Press 'f' or '/'",
        )
    };

    let filter_widget = Paragraph::new(text)
        .style(style)
        .block(Block::default().borders(Borders::BOTTOM).title(title).title_style(Style::default().fg(Color::Gray)));

    f.render_widget(filter_widget, area);
}

fn render_document_list(f: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .documents
        .iter()
        .enumerate()
        .map(|(i, doc)| {
            let id = doc
                .get("_id")
                .map(|v| format!("{}", v))
                .unwrap_or_else(|| format!("Doc {}", i + 1));

            let content = if id.len() > 25 {
                format!("{}...", &id[..22])
            } else {
                id
            };
            
            let prefix = if i == state.selected_doc_index {
                "> "
            } else {
                "  "
            };

            let style = if i == state.selected_doc_index {
                Style::default()
                    .fg(Color::Yellow)
                    .add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(content, style),
            ]))
        })
        .collect();

    let title = format!(" Documents ({}) ", state.documents.len());
    let list = List::new(items)
        .block(Block::default().title(title).title_style(Style::default().fg(Color::Gray)))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

fn render_document_content(f: &mut Frame, area: Rect, state: &AppState) {
    let content = if let Some(doc) = state.get_selected_document() {
        match serde_json::to_string_pretty(&doc) {
            Ok(json) => json,
            Err(_) => format!("{:?}", doc),
        }
    } else {
        "No document selected".to_string()
    };

    let lines: Vec<Line> = content
        .lines()
        .skip(state.doc_scroll_offset)
        .map(|line| Line::from(line.to_string()))
        .collect();

    let paragraph = Paragraph::new(lines)
        .style(Style::default().fg(Color::White))
        .block(
            Block::default()
                .borders(Borders::LEFT)
                .title(" Content ")
                .title_style(Style::default().fg(Color::Gray)),
        )
        .wrap(Wrap { trim: false });

    f.render_widget(paragraph, area);
}

fn render_footer(f: &mut Frame, area: Rect) {
    let footer_text = " [q] Quit  [↑/↓] Nav  [f] Filter ";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray).bg(Color::Black));

    f.render_widget(footer, area);
}

fn render_content_footer(f: &mut Frame, area: Rect) {
    let footer_text = " [PgUp/PgDn] Scroll  [r] Refresh ";
    let footer = Paragraph::new(footer_text)
        .style(Style::default().fg(Color::DarkGray).bg(Color::Black))
        .block(Block::default().borders(Borders::LEFT)); // Match content border

    f.render_widget(footer, area);
}
