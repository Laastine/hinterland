use game::constants::PISTOL_AUDIO_PATH;
use rodio;
use rodio::Sink;
use specs;
use specs::WriteStorage;
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Effects {
  PistolFire,
  None
}

pub struct AudioData {
  sink: Sink,
}

impl AudioData {
  pub fn new() -> AudioData {
    let endpoint = rodio::default_endpoint().unwrap();

    AudioData {
      sink: Sink::new(&endpoint),
    }
  }

  pub fn play_effect(&mut self) {
    let file = File::open(PISTOL_AUDIO_PATH).unwrap();
    let pistol_data = rodio::Decoder::new(BufReader::new(file)).unwrap();
    if self.sink.empty() {
      self.sink.append(pistol_data);
    }
  }
}

impl specs::Component for AudioData {
  type Storage = specs::VecStorage<AudioData>;
}

pub struct AudioSystem {
  effects: Effects,
  queue: mpsc::Receiver<Effects>
}

impl AudioSystem {
  pub fn new() -> (AudioSystem, mpsc::Sender<Effects>) {
    #[allow(deprecated)]
    let (tx, rx) = mpsc::channel();

    (AudioSystem {
      effects: Effects::None,
      queue: rx,
    }, tx)
  }
}

impl<'a> specs::System<'a> for AudioSystem {
  type SystemData = WriteStorage<'a, AudioData>;

  fn run(&mut self, mut audio_data: Self::SystemData) {
    use specs::Join;

    while let Ok(effect) = self.queue.try_recv() {
      match effect {
        Effects::PistolFire => self.effects = Effects::PistolFire,
        _ => self.effects = Effects::None,
      }
    }

    for audio in (&mut audio_data).join() {
      if let Effects::PistolFire = self.effects { audio.play_effect() }
    }
  }
}
