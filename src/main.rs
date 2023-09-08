use std::{
    error::Error,
    io,
    sync::mpsc::channel,
    thread,
    time::{Duration, Instant},
};

use crossterm::{
    cursor::{Hide, Show},
    event::{self, Event},
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};
use rusty_audio::Audio;
use space_invaders::{frame::Frame, player::Player};
use space_invaders::{
    frame::{new_frame, Drawable},
    render,
};

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
    //
    let (render_tx, render_rx) = channel::<Frame>();
    let handle = thread::spawn(move || {
        let mut prev_frame: Vec<Vec<&str>> = new_frame();
        let mut stdout = io::stdout();
        render::render(&mut stdout, &prev_frame, &prev_frame, true);
        loop {
            let curr_frame = match render_rx.recv() {
                Ok(frame) => frame,
                Err(_) => break,
            };
            render::render(&mut stdout, &prev_frame, &curr_frame, false);
            prev_frame = curr_frame;
        }
    });

    // player
    let mut player = Player::new();
    let mut instant = Instant::now();

    // main game loop
    'game: loop {
        let delta = instant.elapsed();
        instant = Instant::now();
        // per frame init
        let mut curr_frame = new_frame();

        while event::poll(Duration::default())? {
            if let Event::Key(key_event) = event::read()? {
                match key_event.code {
                    event::KeyCode::Right => player.move_right(),
                    event::KeyCode::Left => player.move_left(),
                    event::KeyCode::Char(' ') => {
                        if player.shoot() {
                            audio.play(Sounds::PEW.name());
                        };
                    }
                    event::KeyCode::Esc => {
                        audio.play(Sounds::LOSE.name());
                        break 'game;
                    }
                    _ => {}
                }
            }
        }
        //
        player.update(delta);

        // draw and render the frame
        player.draw(&mut curr_frame);
        let _ = render_tx.send(curr_frame);
        thread::sleep(Duration::from_millis(1));
    }

    // audio clean up
    drop(render_tx);
    handle.join().unwrap();
    audio.wait();
    // terminal clean up
    stdout.execute(Show)?;
    stdout.execute(LeaveAlternateScreen)?;
    terminal::disable_raw_mode()?;

    return Ok(());
}
