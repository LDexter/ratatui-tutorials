use crossterm::event::{self, Event, KeyCode, KeyEvent, KeyEventKind};
use ratatui::{
    prelude::*,
    symbols::border,
    widgets::{block::*, *},
};

use color_eyre::{
    eyre::{bail, WrapErr},
    Result,
};

// Add errors module
mod errors;

// Define tui module for terminal setup
mod tui;

// Contain app state within struct (enum instead if state is more complex)
#[derive(Debug, Default)]
pub struct App {
    counter: u8,
    exit: bool,
}

// App implementation
impl App {
    // Run app's main loop until user quits
    pub fn run(&mut self, terminal: &mut tui::Tui) -> Result<()> {
	while !self.exit {
	    // Draw terminal frames and handle events
	    terminal.draw(|frame| self.render_frame(frame))?;
	    self.handle_events().wrap_err("handle events failed")?;
	}
	Ok(())
    }

    // Render
    fn render_frame(&self, frame: &mut Frame) {
	// Full-size frame
	frame.render_widget(self, frame.size());
    }

    // Update app state based on user input
    // DOES NOT HANDLE OTHER TASKS, INSTEAD USE "event::poll"
    fn handle_events(&mut self) -> Result<()> {
	match event::read()? {
	    // Only listen to press
	    Event::Key(key_event) if key_event.kind == KeyEventKind::Press => {
		self.handle_key_event(key_event).wrap_err_with(|| {
		    format!("handling key event failed:\n{key_event:#?}")
		})
	    }
	    _ => Ok(())
	}
    }

    // Specific keypresses
    fn handle_key_event(&mut self, key_event: KeyEvent) -> Result<()> {
	match key_event.code {
	    KeyCode::Char('q') => self.exit(),
	    KeyCode::Left => self.decrement_counter()?,
	    KeyCode::Right => self.increment_counter()?,
	    _ => {}
	}
	Ok(())
    }

    // Handle state updates
    fn exit(&mut self) {
	self.exit = true;
    }
    fn decrement_counter(&mut self) -> Result<()> {
	self.counter -= 1;
	Ok(())
    }
    fn increment_counter(&mut self) -> Result<()> {
	self.counter += 1;
	if self.counter > 2 {
	    bail!("counter overflow");
	}
	Ok(())
    }
}

impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
	let title = Title::from(" Counter App Tutorial ".bold());
	
	let instructions = Title::from(Line::from(vec![
	    " Decrement ".into(),
	    "<Left>".blue().bold(),
	    " Increment ".into(),
	    "<Right>".blue().bold(),
	    " Quit ".into(),
	    "<Q> ".blue().bold(),
	]));
	
	let block = Block::default()
	    .title(title.alignment(Alignment::Center))
	    .title(
		instructions
		    .alignment(Alignment::Center)
		    .position(Position::Bottom)
	    )
	    .borders(Borders::ALL)
	    .border_set(border::THICK);

	let counter_text = Text::from(vec![Line::from(vec![
	    "Value: ".into(),
	    self.counter.to_string().yellow(),
	])]);

	Paragraph::new(counter_text)
	    .centered()
	    .block(block)
	    .render(area, buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    // Test rendering
    #[test]
    fn render() {
	let app = App::default();
	let mut buf = Buffer::empty(Rect::new(0, 0, 50, 4));

	app.render(buf.area, &mut buf);

	let mut expected = Buffer::with_lines(vec![
	    "┏━━━━━━━━━━━━━ Counter App Tutorial ━━━━━━━━━━━━━┓",
            "┃                    Value: 0                    ┃",
            "┃                                                ┃",
            "┗━ Decrement <Left> Increment <Right> Quit <Q> ━━┛",
        ]);
	let title_style = Style::new().bold();
	let counter_style = Style::new().yellow();
	let key_style = Style::new().blue().bold();
	expected.set_style(Rect::new(14, 0, 22, 1), title_style);
	expected.set_style(Rect::new(28, 1, 1, 1), counter_style);
        expected.set_style(Rect::new(13, 3, 6, 1), key_style);
        expected.set_style(Rect::new(30, 3, 7, 1), key_style);
        expected.set_style(Rect::new(43, 3, 4, 1), key_style);

	// note ratatui also has an assert_buffer_eq! macro that can be used to
        // compare buffers and display the differences in a more readable way
	assert_eq!(buf, expected);
    }

    // Test keyboard input
    #[test]
    fn handle_key_event() -> io::Result<()> {
        let mut app = App::default();
        app.handle_key_event(KeyCode::Right.into()).unwrap();
        assert_eq!(app.counter, 1);

        app.handle_key_event(KeyCode::Left.into()).unwrap();
        assert_eq!(app.counter, 0);

        let mut app = App::default();
        app.handle_key_event(KeyCode::Char('q').into()).unwrap();
        assert_eq!(app.exit, true);

        Ok(())
    }

    #[test]
    #[should_panic(expected = "attempt to subtract with overflow")]
    fn handle_key_event_panic() {
        let mut app = App::default();
        let _ = app.handle_key_event(KeyCode::Left.into());
    }

    #[test]
    fn handle_key_event_overflow() {
        let mut app = App::default();
        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
        assert!(app.handle_key_event(KeyCode::Right.into()).is_ok());
        assert_eq!(
            app.handle_key_event(KeyCode::Right.into())
                .unwrap_err()
                .to_string(),
            "counter overflow"
        );
    }
}

fn main() -> Result<()> {
    // Error handling
    errors::install_hooks()?;
    
    // Terminal setup
    let mut terminal = tui::init()?;

    // Create and run app with default state (0 and false for App struct)
    App::default().run(&mut terminal)?;

    // Restore terminal
    tui::restore()?;
    
    Ok(())
}
