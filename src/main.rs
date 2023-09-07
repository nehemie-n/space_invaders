use std::{error::Error, io, time::Duration};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;

enum Sounds {
    EXPLODE,
    LOSE,
    MOVE,
    PEW,
    STARTUP,
    WIN,
}
impl Sounds {
    fn name(&self) -> String {
        return match &self {
            Sounds::EXPLODE => String::from("explode"),
            Sounds::LOSE => String::from("lose"),
            Sounds::MOVE => String::from("move"),
            Sounds::PEW => String::from("pew"),
            Sounds::STARTUP => String::from("startup"),
            Sounds::WIN => String::from("win"),
        };
    }
}

fn main() -> Result<(), Box<dyn Error>> {
    // import audios
    let mut audio = Audio::new();

    audio.add(Sounds::EXPLODE.name(), "audio/explode.wav");
    audio.add(Sounds::LOSE.name(), "audio/lose.wav");
    audio.add(Sounds::MOVE.name(), "audio/move.wav");
    audio.add(Sounds::PEW.name(), "audio/pew.wav");
    audio.add(Sounds::STARTUP.name(), "audio/startup.wav");
    audio.add(Sounds::WIN.name(), "audio/win.wav");
    audio.play(Sounds::STARTUP.name());

    //
    let mut stdout = io::stdout();
    terminal::enable_raw_mode()?;
    stdout.execute(EnterAlternateScreen)?;
    stdout.execute(Hide)?;

    // 

    // main game loop

    'game: loop {
        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    event::KeyCode::Esc => {
                        audio.play(Sounds::LOSE.name());
                        break 'game;
                    }
                    _ => {}
                }
            }
        }
    }

    // audio clean up
    audio.wait();
    // terminal clean up
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    return Ok(());
}
