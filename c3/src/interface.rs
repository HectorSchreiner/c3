use crossterm::{
    cursor, event::{self, read, Event, KeyCode, KeyEvent, KeyEventKind}, execute, queue, style::{self, Color, Print, SetForegroundColor}, terminal::{self, Clear, ClearType}, ExecutableCommand, QueueableCommand
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

    pub fn display_menu(&mut self) -> Result<()> {
        self.stdout.execute(SetForegroundColor(Color::Cyan))?;

        //terminal::enable_raw_mode()?;

        const MENU: &str = r#"
<==== Command & Control Server ====><

Options:
    1. List Clients
    2. Encrypt Drives
    3. Send shell command

Select option to run ('1', '2', ...) or hit <CTRL-C> to exit
__________________________________________________________
        "#;

        Ok(loop {
            queue!(
                self.stdout,
                style::ResetColor,
                terminal::Clear(ClearType::All),
                cursor::Hide,
                cursor::MoveTo(1, 1)
            )?;
    
            for line in MENU.split('\n') {
                queue!(self.stdout, style::Print(line), cursor::MoveToNextLine(1))?;
            }            
    
            self.stdout.flush()?;
        
            match Self::read_char()? {
                '1' => {
                    self.stdout.execute(Print(format!("Listing Clients\n- Windows 1\n- Windows 2\n- Ubuntu\n")))?;
                    Self::read_enter(&mut self.stdout)?;
                    self.display_menu()?;
                },
                '2' => {
                    self.stdout.execute(Print(format!("Encrypting C Drive\n")))?;
                    Self::read_enter(&mut self.stdout)?;
                    self.display_menu()?;
                }
                _ => {
                    self.stdout.execute(Print(format!("Invalid input\n")))?;
                    Self::read_enter(&mut self.stdout)?;
                    self.display_menu()?;
                }
            };
        })         
        //terminal::disable_raw_mode()
    }

    pub fn display_message(&mut self, message: &str) -> Result<()> {
        //self.stdout.execute(Clear(ClearType::All))?;
        self.stdout.execute(Print(format!("{}\n", message)))?;
        self.stdout.flush()?;  
        Ok(())
    }

    fn send_to_c2(&mut self, command: &C2Command, c2: &mut C2) {
        c2.add_command(&command);
    }

    pub fn run(&mut self, c2: &mut C2) -> Result<()> {
        //self.display_menu(menu)?;
        Ok(())
    }

    pub fn read_char() -> std::io::Result<char> {
        loop {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Char(c),
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) = event::read()
            {
                return Ok(c);
            }
        }
    }

    pub fn read_enter(stdout: &mut Stdout) -> std::io::Result<()> {
        stdout.execute(Print(format!("_______________________\n")))?;
        stdout.execute(Print(format!("press enter to continue\n")))?;
        loop {
            if let Ok(Event::Key(KeyEvent {
                code: KeyCode::Enter,
                kind: KeyEventKind::Press,
                modifiers: _,
                state: _,
            })) = event::read()
            {
                return Ok(());
            }
        }
    }
}


