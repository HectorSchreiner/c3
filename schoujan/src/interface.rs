use crossterm::{
    event::{self, Event, KeyCode},
    execute,
    style::{Color, Print, SetForegroundColor},
    terminal::{Clear, ClearType},
    ExecutableCommand,
};
use std::io::{stdout, Stdout, Result};

use crate::{menu::Menu, server::{self, *}};

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
        
        for item in menu.items.iter() {
            self.stdout.execute(Print(&item.name))?;
        }

        Ok(())
    }

    pub fn handle_input(&mut self) -> Result<()> {
        loop {
            if event::poll(std::time::Duration::from_millis(500))? {
                if let Event::Key(key_event) = event::read()? {
                    match key_event.code {
                        KeyCode::Char('1') => {
                            
                        },
                        KeyCode::Esc => {
                            self.display_message("Exiting...")?;
                            break;
                        }
                        _ => {}
                    }
                }
            }
        }
        Ok(())
    }

    pub fn display_message(&mut self, message: &str) -> Result<()> {
        self.stdout.execute(Clear(ClearType::All))?;
        self.stdout.execute(SetForegroundColor(Color::Yellow))?;
        self.stdout.execute(Print(format!("{}\n", message)))?;
        self.stdout.execute(SetForegroundColor(Color::Reset))?;
        self.stdout.execute(Print("Press any key to return to the menu..."))?;

        loop {
            if event::poll(std::time::Duration::from_millis(500))? {
                if let Event::Key(_) = event::read()? {
                    break;
                }
            }
        }        
    }

    pub fn run(&mut self, menu: &Menu) -> Result<()> {
        self.display_menu(menu)?;
        self.handle_input()?;
        Ok(())
    }
}