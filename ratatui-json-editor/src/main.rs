// Imports for terminal setup
use crossterm::event::EnableMouseCapture;
use crossterm::execute;
use crossterm::terminal::{enable_raw_mode, EnterAlternateScreen};
use std::io;

// Imports for restoring terminal
use crossterm::event::DisableMouseCapture;
use crossterm::terminal::{disable_raw_mode, LeaveAlternateScreen};

// Main function for startup, main loop, and cleanup
fn main() -> Result<(), Box<dyn Error>> {
    // Setup terminal
    enable_raw_mode()?;

    // Allow user to pipe output into external programs like ratatui > output.json
    // Otherwide using stdout would be fine
    let mut stderr = io::stderr();
    execute!(stderr, EnterAlternateScreen, EnableMouseCapture)?;

    // Create backend
    let backend = CrosstermBackend::new(stderr);
    let mut terminal = Terminal::new(backend)?;

    // Create application instance and run it
    let mut app = App::new();
    let res = run_app(&mut terminal, &mut app);

    // Restore terminal
    disable_raw_mode()?;
    execute!(
	terminal.backend_mut(),
	LeaveAlternateScreen,
	DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    // If run_app returned Ok state, check if JSON should be printed
    // If run_app returned an error, print error
    if let Ok(do_print) = res {
	if do_print {
	    app.print_json()?;
	}
    } else if let Err(err) = err {
	println!("{err:?}");
    }

    Ok(())
}

// Application logic
fn run_app<B: Backend>(          // Start method signature across ratatui::backend::Backend
    terminal: &mut Terminal<B>,  // Terminal object, which implements the ratatui::backend::Backend trait
    app: &mut App,               // Mutable borrow to application state object
) -> io::Result<bool> {          // Return whether was io error with Err state and an Ok(bool) to know if printing JSON

    // Event/UI loop update
    loop {
	// Pass f: <Frame> into ui function to be drawn
	// Also pass immutable borrow of app state
	terminal.draw(|f| ui(f, app))?;

	// Polling for keyboard events
	// Alternatively could use more complex "counter" tutorial method
	if let Event::Key(key) = event::read()? {
	    if key.kind == event::KeyEventKind:Release {
		// Skip events that are not press
		continue;
	    }

	    // Test for main screen
	    match app.current_screen {
		// Main screen only has two keys to match 
		CurrentScreen::Main => match key.code {
		    // Edit action
		    KeyCode::Char('e') => {
			// Switch to edit screen
			app.current_screen = CurrentScreen::Editing;
			// Update editing state, starting user on key side
			app.currently_editing = Some(CurrentlyEditing::Key);
		    }
		    // Quit action
		    KeyCode::Char('q') => {
			app.current_screen = CurrentScreen::Exiting;
		    }
		    _ => {}
		},

		// Exiting prompt for outputting JSON
		CurrentScreen::Exiting => match key.code {
		    // Confirm print
		    KeyCode::Char('y') => {
			return Ok(true);
		    }
		    // Decline print
		    KeyCode::Char('n') | KeyCode::Char('q') => {
			return Ok(false);
		    }
		    _ => {}
		},

		// Handle enter key for moving through edit mode and returning to main
		CurrentScreen::Editing if key.kind == KeyEventKind::Press => {
		    match key.code {
			// Check for Enter key
			KeyCode::Enter => {
			    // Check for editing
			    if let Some(editing) = &app.currently_editing {
				match editing {
				    // When on key, move to value
				    CurrentlyEditing::Key => {
					app.currently_editing = Some(CurrentlyEditing::Value);
				    }
				    // When on value, save pair and return to main screen
				    CurrentlyEditing::Value => {
					app.save_key_value();
					app.current_screen = CurrentScreen::Main;
				    }
				}
			    }
			}

			// Handle Backspace key for deleting characters
			KeyCode::Backspace => {
			    // Check for editing
			    if let Some(editing) = &app.currently_editing {
				match editing {
				    // Delete end characters off either key or value strings
				    CurrentlyEditing::Key => {
					app.key_input.pop();
				    }
				    CurrentlyEditing::Key => {
					app.value_input.pop();
				    }
				}
			    }
			}

			// Handle Escape key to exit edit mode
			KeyCode::Esc => {
			    app.current_screen = CurrentScreen::Main;
			    app.currently_editing = None;
			}
			
			// Swap between key and value
			KeyCode::Tab => {
			    app.toggle_editing();
			}

			// Handle typing valid characters by capturing value of char
			KeyCode::Char(value) => {
			    if let Some(editing) = &app.currently_editing {
				match editing {
				    CurrentlyEditing::Key => {
					app.key_input.push(value);
				    }
				    CurrentlyEditing::Value => {
					app.value_input.push(value);
				    }
				}
			    }
			}
			_ => {}
		    }
		}
		_ => {}
	    }
	}
    }
}
