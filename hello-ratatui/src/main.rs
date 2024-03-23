use crossterm::{
    event::{self, KeyCode, KeyEventKind},
    terminal::{
	disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen,
    },
    ExecutableCommand,
};
use ratatui::{
    prelude::{CrosstermBackend, Stylize, Terminal},
    widgets::Paragraph,
};
use std::io::{stdout, Result};

// Main function for running application
fn main() -> Result<()> {
    // Allow app to render what it needs without disturbing shell output
    stdout().execute(EnterAlternateScreen)?;

    // Turn off input and output processing
    enable_raw_mode()?;

    // Create backend and clear screen
    let mut terminal = Terminal::new(CrosstermBackend::new(stdout()))?;
    terminal.clear()?;

    // Main loop to maintain app until quit
    loop {
	// Main interation point for Ratatui
	terminal.draw(|frame| {
	    // Make full size of terminal window
	    let area = frame.size();

	    // Render paragraph widget
	    frame.render_widget(
		Paragraph::new("Hello Ratatui!")
		    // White foreground on blue background
		    .white()
		    .on_blue(),
		area,
	    );
	})?;

	// Event handling with allowance for 60fps
	if event::poll(std::time::Duration::from_millis(16))? {
	    // Reading keyboard
	    if let event::Event::Key(key) = event::read()? {
		// Only on press
		if key.kind == KeyEventKind::Press
		    // Check for lower and upper "q"
		    && (key.code == KeyCode::Char('q')
		    || key.code == KeyCode::Char('Q'))
		{
		    // Exit application
		    break;
		}
	    }
	}
    }
    
    // Revert modifications to terminal
    stdout().execute(LeaveAlternateScreen)?;
    disable_raw_mode()?;
    Ok(())
}
    
