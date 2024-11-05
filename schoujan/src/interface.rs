use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::{io::{stdout, Result, Stdout, Write}, process::Command};

use crate::{C2Command, C2};

pub struct Interface {
    stdout: Stdout,
}

impl Interface {
    pub fn new() -> Self {
        Self { stdout: stdout() }
    }

    pub fn display_menu(&mut self, menu: &Menu) -> Result<()> {
        self.stdout.execute(Clear(ClearType::All))?;
        self.stdout.execute(SetForegroundColor(Color::Cyan))?;

        self.stdout
            .execute(Print(format!("=== {} ===\n", menu.header)))?;

        for (iter, item) in menu.items.iter().enumerate() {
            self.stdout
                .execute(Print(format!("{}: {}\n", iter, &item.name)))?;
        }

        self.stdout.execute(SetForegroundColor(Color::Reset))?;
        self.stdout.execute(Print(
            "Select an option by number, or press 'Esc' to exit.\n",
        ))?;
        Ok(())
    }

    pub fn handle_input(&mut self, menu: &Menu, c2: &mut C2) -> Result<()> {
        loop {
            if event::poll(std::time::Duration::from_millis(500))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char(c) if c.is_ascii_digit() => {
                            let index = c.to_digit(10).unwrap() as usize;
                            if index > 0 && index <= menu.items.len() {
                                self.send_to_c2(&menu.items[index - 1].command, c2);
                            } else {
                                self.display_message("Invalid option")?;
                            }
                        }
                        KeyCode::Esc => {
                            self.display_message("Exiting...")?;
                            break;
                        }
                        _ => {
                            self.display_message("Invalid input, try again.")?;
                        }
                    }
                }
            }
        }
        Ok(())
    }

    pub fn display_message(&mut self, message: &str) -> Result<()> {
        self.stdout.execute(Clear(ClearType::All))?;
        self.stdout.execute(Print(format!("{}\n", message)))?;
        self.stdout.flush()?;  
        Ok(())
    }

    fn send_to_c2(&mut self, command: &C2Command, c2: &mut C2) {
        c2.add_command(&command);
    }

    pub fn run(&mut self, menu: &Menu, c2: &mut C2) -> Result<()> {
        self.display_menu(menu)?;
        self.handle_input(menu, c2)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct Menu {
    pub header: String,
    pub items: Vec<Item>,
}

#[derive(Debug, Clone)]
pub struct Item {
    pub name: String,
    pub command: C2Command,
}

impl Item {
    pub fn new(name: String, command: C2Command) -> Self {
        Self { name, command }
    }
}

impl Menu {
    pub fn new(header: String) -> Self {
        Self {
            header,
            items: Vec::new(),
        }
    }

    pub fn add_item(&mut self, item: &Item) {
        self.items.push(item.clone());
    }
}

