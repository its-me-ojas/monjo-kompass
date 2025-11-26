use ratatui::{
    Frame,
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph},
};

use crate::app::state::AppState;


pub fn render(f: &mut Frame, area: Rect, state: &AppState) {
    // Vertical layout: Center the content block, with footer at bottom
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),         // Top spacer
            Constraint::Length(30),     // Content block (Logo + Input + History/Instructions)
            Constraint::Min(0),         // Bottom spacer
            Constraint::Length(1),      // Footer
        ])
        .split(area);

    // Center horizontally
    let center_area = |area: Rect, width: u16| -> Rect {
        let h_chunks = Layout::default()
            .direction(Direction::Horizontal)
            .constraints([
                Constraint::Min(0),
                Constraint::Length(width),
                Constraint::Min(0),
            ])
            .split(area);
        h_chunks[1]
    };

    let content_area = center_area(main_chunks[1], 60);

    // Split content area into Logo, Input, History
    let content_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(12),     // Logo
            Constraint::Length(3),      // Input
            Constraint::Length(15),     // History / Instructions
        ])
        .split(content_area);

    render_logo(f, content_chunks[0]);
    render_input(f, content_chunks[1], state);

    if state.show_history && !state.connection_history.is_empty() {
        render_history(f, content_chunks[2], state);
    } else {
        render_instructions(f, content_chunks[2]);
    }

    render_footer(f, main_chunks[3], state);
}

fn render_logo(f: &mut Frame, area: Rect) {
    let logo_text = vec![
        " __  __  ___  _   _     _  ___  ",
        "|  \\/  |/ _ \\| \\ | |   | |/ _ \\ ",
        "| |\\/| | | | |  \\| |_  | | | | |",
        "| |  | | |_| | |\\  | |_| | |_| |",
        "|_|  |_|\\___/|_| \\_|\\___/ \\___/ ",
        " _  __  ___   __  __  ____      _    ____   ____  ",
        "| |/ / / _ \\ |  \\/  ||  _ \\    / \\  / ___| / ___| ",
        "| ' / | | | || |\\/| || |_) |  / _ \\ \\___ \\ \\___ \\ ",
        "| . \\ | |_| || |  | ||  __/  / ___ \\ ___) | ___) |",
        "|_|\\_\\ \\___/ |_|  |_||_|    /_/   \\_\\|____/ |____/ ",
    ];

    let logo = Paragraph::new(logo_text.join("\n"))
        .style(Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD))
        .alignment(Alignment::Center);

    f.render_widget(logo, area);
}

fn render_instructions(f: &mut Frame, area: Rect) {
    let text = vec![
        Line::from(Span::styled("Enter MongoDB URI to connect", Style::default().fg(Color::DarkGray))),
        Line::from(Span::raw("")),
        Line::from(vec![
            Span::styled("Example: ", Style::default().fg(Color::DarkGray)),
            Span::styled("mongodb://localhost:27017", Style::default().fg(Color::Gray)),
        ]),
    ];
    
    let instructions = Paragraph::new(text)
        .alignment(Alignment::Center);
    f.render_widget(instructions, area);
}

fn render_input(f: &mut Frame, area: Rect, state: &AppState) {
    let input_style = if state.input_mode {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::White)
    };
    
    let prefix = if state.connection_input.is_empty() {
        "  > "
    } else {
        "  > "
    };

    let input_text = format!("{}{}", prefix, state.connection_input);
    
    let input = Paragraph::new(input_text)
        .style(input_style)
        .block(
            Block::default()
                .borders(Borders::BOTTOM)
                .border_style(Style::default().fg(if state.input_mode { Color::Yellow } else { Color::DarkGray })),
        );
    f.render_widget(input, area);
}

fn render_history(f: &mut Frame, area: Rect, state: &AppState) {
    let items: Vec<ListItem> = state
        .connection_history
        .iter()
        .enumerate()
        .map(|(i, uri)| {
            let prefix = if i == state.selected_history_index {
                "> "
            } else {
                "  "
            };
            
            let style = if i == state.selected_history_index {
                Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD)
            } else {
                Style::default().fg(Color::Gray)
            };
            
            ListItem::new(Line::from(vec![
                Span::styled(prefix, style),
                Span::styled(uri.clone(), style),
            ]))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Recent Connections").title_alignment(Alignment::Center))
        .style(Style::default().fg(Color::White));

    f.render_widget(list, area);
}

fn render_footer(f: &mut Frame, area: Rect, state: &AppState) {
    let text = if state.error.is_some() {
        format!("Error: {}", state.error.as_ref().unwrap())
    } else if state.loading {
        "Connecting...".to_string()
    } else {
        "[Enter] Connect  [Tab] History  [Esc] Clear  [Ctrl+C] Quit".to_string()
    };
    
    let footer = Paragraph::new(text)
        .style(Style::default().fg(Color::DarkGray))
        .alignment(Alignment::Center);

    f.render_widget(footer, area);
}
