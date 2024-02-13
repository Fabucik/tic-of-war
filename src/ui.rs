use std::io::{self, stdout};

use crossterm::{
    event::{DisableMouseCapture, EnableMouseCapture},
    execute,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen}
    };
use ratatui::{layout::{Alignment, Constraint, Direction, Layout, Rect}, style::{Color, Style}, text::{Line, Span}, widgets::{Block, Borders, Padding, Paragraph}, Frame};

use crate::app::{Game, Player};

pub fn startup() -> io::Result<()> {
    enable_raw_mode()?;
    execute!(stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    Ok(())
}

pub fn shutdown() -> io::Result<()> {
    disable_raw_mode()?;
    execute!(stdout(), LeaveAlternateScreen, DisableMouseCapture)?;

    Ok(())
}

pub fn render(frame: &mut Frame, game: &Game) {
    // split screen horizontally
    let area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(frame.size());

    let game_block = Block::default()
        .title("Tic of War")
        .title_alignment(ratatui::layout::Alignment::Center)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightBlue));

    // renders game and players
    render_game_board(frame, game_block.inner(area[0]), game);
    // renders game information
    render_info_board(frame, game_block.inner(area[1]), game);

    frame.render_widget(game_block, frame.size());
}

fn render_game_board(frame: &mut Frame, area: Rect, game: &Game) {
    // creates 3 columns
    let columns = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [area.height / 3; 3]
        )
        .split(area);

    for (y, col) in columns.iter().enumerate() {

        // creates 3 rows
        let rows = Layout::default()
        .direction(Direction::Horizontal)
        .constraints(
            [col.width / 3; 3]
        )
        .split(*col);

        // iterates over every position in game board
        for (x, row) in rows.iter().enumerate() {
            // converts current player to ASCII art
            let current_drawable_player = game.board[x][y].to_ascii();

            // changes block color based on X or O
            let pos_block_color = if x == game.pos_x && y == game.pos_y {
                Color::Magenta
            } else {
                Color::LightYellow
            };

            // changes player color based on X or O
            let player_color = if game.board[x][y] == Player::Cross {
                Color::Blue
            } else {
                Color::Red
            };
            
            let pos_block = Block::default()
                .borders(Borders::ALL)
                .border_style(Style::default().fg(pos_block_color));

            let player_widget = Paragraph::new(current_drawable_player)
                .block(pos_block.padding(Padding::new(0, 0, row.height / 6, 0)))
                .centered()
                .style(Style::default().fg(player_color));

            frame.render_widget(player_widget, *row);
        }
    }
}

fn render_info_board(frame: &mut Frame, area: Rect, game: &Game) {
    let quit_line = Line::from("Press 'q' to quit");

    // borders around info
    let info_block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::LightMagenta))
        .title("Info")
        .title_alignment(Alignment::Center);

    // different info board if game is finished
    if game.finished {
        let winner_char = game.winner.to_char();

        let winner_line = if winner_char == ' ' {
            Line::from(
                Span::styled("Nobody won!", Style::default().fg(Color::LightRed)))
        } else {
            Line::from(
                Span::styled(format!("Winner is {winner_char}"), Style::default().fg(Color::LightRed)))
        };
        let game_over_line = Line::from("Game over!");
        let restart_line = Line::from("Press 'r' to restart");

        let info_widget = Paragraph::new(vec![winner_line, game_over_line, quit_line, restart_line])
            .centered()
            .block(info_block);

        frame.render_widget(info_widget, area);

        return
    }

    let current_player_char = game.current_player.to_char();

    let currently_playing_line = Line::from(format!("Player {current_player_char}'s turn"));
    let move_line = Line::from("Use arrow keys to move");
    let enter_line = Line::from("Press enter to place symbol");

    let info_widget = Paragraph::new(vec![currently_playing_line, move_line, enter_line, quit_line])
        .centered()
        .block(info_block);

    frame.render_widget(info_widget, area);
}

fn centered_rect(percent_x: u16, percent_y: u16, rect: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(100 - percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(rect);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}