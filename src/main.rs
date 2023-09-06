use std::error::Error;

use rusty_audio::Audio;

fn main() -> Result<(), Box<dyn Error>> {
    // import audios
    let audio = Audio::new();
    // audio.add("name", path);

    return Ok(());
}
