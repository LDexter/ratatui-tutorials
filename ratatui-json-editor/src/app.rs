pub enum CurrentScreen {
    Main,
    Editing,
    Exiting,
}

pub enum CurrentlyEditing {
    Key,
    Value,
}

pub struct App {
    pub key_input: String,                            // JSON key that is currently being edited
    pub value_input: String,                          // JSON value that is currently being edited
    pub pairs: HashMap<String, String>,               // Key-value pairs with serde Serialise support
    pub current_screen: CurrentScreen,                // Current screen being rendered
    pub currently_editing: Option<CurrentlyEditing>,  // Which pair is being edited, if at all
}

// App struct implementation
impl App {
    // Universal state defaults for creation of state
    pub fn new() -> App {
	App {
	    key_input: String::new(),
	    value_input: String::new(),
	    pairs: HashMap::new(),
	    current_screen: CurrentScreen::Main,
	    currently_editing: None,
	}
    }

    // Save key-value pair that is currently being edited
    pub fn save_key_value(&mut self) {
	// Add stored inputs in HashMap
	self.pairs
	    .insert(self.key_input.clone(), self.value_input.clone());

	// Reset editing state
	self.key_input = String::new();
	self.value_input = String::new();
	self.currently_editing = None;
    }

    // Swap between editing either key or value
    pub fn toggle_editing(&mut self) {
	// Only if currently editing
	if let Some(edit_mode) = &self.currently_editing {
	    // Determine if editing key or value
	    match edit_mode {
		CurrentlyEditing::Key => {    // If key, change to value
		    self.currently_editing = Some(CurrentlyEditing::Value)
		}
		CurrentlyEditing::Value => {  // If value, change to key
		    self.currently_editing = Some(CurrentlyEditing::Key)
		}
	    };
	// If not editing, make key
	} else {
	    self.currently_editing = Some(CurrentlyEditing::Key)
	}
    }

    // Print serialised JSON from all key-value pairs
    pub fn print_json(&self) -> Result<()> {
	// Serialise pairs to string
	let output = serde_json::to_string(&self.pairs)?;
	println!("{}", output);
	Ok(())
    }
