use std::{io::{self, stdout}, time::{Duration, Instant}};

use crossterm::event::{self, Event, KeyCode, KeyEventKind};
use ratatui::{backend::{Backend, CrosstermBackend}, Terminal};
use tic_of_war::{
    app::{
        Game,
        Movement},
    ui};

fn main() -> io::Result<()> {
    ui::startup()?;

    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    let mut game = Game::new();

    let res = run(&mut terminal, &mut game);
    ui::shutdown()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

fn run<B: Backend>(terminal: &mut Terminal<B>, game: &mut Game) -> io::Result<()> {
    let mut last_redraw = Instant::now();

    loop {
        // handle all keyboard events
        if event::poll(Duration::from_millis(100))? {
            if let Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    match key.code {
                        KeyCode::Char('q') => break,
                        KeyCode::Char('r') => game.reset(),
    
                        KeyCode::Up => game.move_position(&Movement::Up),
                        KeyCode::Down => game.move_position(&Movement::Down),
                        KeyCode::Left => game.move_position(&Movement::Left),
                        KeyCode::Right => game.move_position(&Movement::Right),
    
                        KeyCode::Enter => game.enter_position(),
    
                        _ => ()
                    }
                }
            }
        }

        // check if 100 milliseconds have elapsed to avoid frequent redraws
        if last_redraw.elapsed() >= Duration::from_millis(100) {
            terminal.draw(|frame| ui::render(frame, game))?;
            last_redraw = Instant::now();
        }
    }

    Ok(())
}
