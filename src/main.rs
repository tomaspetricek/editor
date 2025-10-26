use crossterm::{
    cursor,
    event::{self, Event, KeyCode, KeyEvent, KeyModifiers},
    execute,
    terminal::{ClearType, disable_raw_mode, enable_raw_mode, ScrollUp, SetSize, size},
};
use std::fs::{File, OpenOptions};
use std::io::{self, Read, Seek, SeekFrom, Write, stdin, stdout};
use std::path::PathBuf;
use std::env;

struct Editor {
    file: File,
    contents: String,
    keep_editing: bool,
    save_changes: bool,
}

impl Editor {
    fn get_file_path() -> io::Result<PathBuf> {
        println!("enter path to file to be loaded:");
        let mut file_path = String::new();
        stdin().read_line(&mut file_path)?;
        Ok(PathBuf::from(file_path.trim()))
    }

    fn open_file(path: &PathBuf) -> io::Result<File> {
        OpenOptions::new()
            .read(true)
            .write(true)
            .create(false)
            .open(path)
    }

    fn read_contents(file: &mut File) -> io::Result<String> {
        file.seek(SeekFrom::Start(0))?;
        let mut contents = String::new();
        file.read_to_string(&mut contents)?;
        Ok(contents)
    }

    fn open(file_path: &PathBuf) -> io::Result<Editor> {
        let mut file = Self::open_file(&file_path).map_err(|err| {
            eprintln!("failed to open file: {file_path:#?} due to: {err}");
            err
        })?;
        let contents = Self::read_contents(&mut file).map_err(|err| {
            eprintln!("failed to read file contents due to: {err}");
            err
        })?;
        Ok(Editor {
            file: file,
            contents: contents,
            keep_editing: true,
            save_changes: true,
        })
    }

    fn save_changes(&mut self) -> io::Result<()> {
        self.file.set_len(0)?;
        self.file.seek(SeekFrom::Start(0))?;
        self.file.write_all(self.contents.as_bytes())?;
        self.file.flush()?;
        Ok(())
    }

    fn display_content(&self) -> io::Result<()> {
        write!(stdout(), "{}", self.contents)
    }

    fn display_menu() -> io::Result<()> {
        write!(stdout(), "Press Ctrl+S to save, Ctrl+C to quit.")
    }

    fn read_key() -> io::Result<KeyEvent> {
        loop {
            if event::poll(std::time::Duration::from_millis(500))? {
                if let Event::Key(key_event) = event::read()? {
                    return Ok(key_event);
                }
            }
        }
    }

    fn process_key(&mut self, key_event: KeyEvent) -> Result<(), Box<dyn std::error::Error>> {
        if key_event.modifiers.contains(KeyModifiers::CONTROL) {
            if key_event.code == KeyCode::Char('s') {
                println!("Ctrl+S pressed!");
                self.keep_editing = false;
            } else if key_event.code == KeyCode::Char('c') {
                println!("Ctrl+C pressed!");
                self.keep_editing = false;
                self.save_changes = false;
            }
        } else {
            match key_event.code {
                KeyCode::Up => {
                    execute!(stdout(), cursor::MoveUp(1),)?;
                }
                KeyCode::Down => {
                    execute!(stdout(), cursor::MoveDown(1),)?;
                }
                KeyCode::Left => {
                    execute!(stdout(), cursor::MoveLeft(1),)?;
                }
                KeyCode::Right => {
                    execute!(stdout(), cursor::MoveRight(1),)?;
                }
                KeyCode::Char(value) => {
                    print!("{value}");
                }
                _ => {}
            }
        }
        Ok(())
    }

    pub fn run(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        enable_raw_mode()?;

        execute!(
            stdout(),
            crossterm::terminal::Clear(ClearType::All),
            crossterm::cursor::MoveTo(0, 0)
        )?;
        self.display_content()?;
        Self::display_menu()?;

        while self.keep_editing {
            let key_event = Self::read_key()?;
            self.process_key(key_event)?;
        }
        if self.save_changes {
            self.save_changes()?;
        }
        disable_raw_mode()?;
        Ok(())
    }
}

fn main() {
    let mut args = std::env::args();
    args.next();
    let file_path = match args.next() {
        Some(file_path) => PathBuf::from(file_path),
        None => {
            eprintln!("path to file not provided");
            std::process::exit(1);
        }
    };
    let mut editor = Editor::open(&file_path).unwrap_or_else(|err| {
        eprintln!("failed to open editor: {err}");
        std::process::exit(1);
    });

    if let Err(err) = editor.run() {
        eprintln!("editor failed: {err}");
        std::process::exit(1);
    }
}
