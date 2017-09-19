use std::fs::File;
use std::io::BufReader;
use rodio;
use rodio::Sink;

pub struct CharacterAudio {
  sink: Sink,
}

impl CharacterAudio {
  pub fn new() -> CharacterAudio {
    let endpoint = rodio::get_default_endpoint().unwrap();
    CharacterAudio {
      sink: Sink::new(&endpoint),
    }
  }

  pub fn play_pistol(&self) {
    if self.sink.empty() {
      let file = File::open("./assets/audio/pistol.ogg").unwrap();
      let pistol_data = rodio::Decoder::new(BufReader::new(file)).unwrap();
      self.sink.append(pistol_data);
    }
  }
}
