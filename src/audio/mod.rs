use character::controls::CharacterInputState;
use crossbeam_channel as channel;
use game::constants::PISTOL_AUDIO_PATH;
use rodio;
use rodio::Sink;
use specs;
use specs::prelude::ReadStorage;
use std::{fs::File, io::BufReader};

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Effects {
  PistolFire,
  None
}

pub struct AudioSystem {
  effects: Effects,
  sink: Sink,
  queue: channel::Receiver<Effects>
}

impl AudioSystem {
  pub fn new() -> (AudioSystem, channel::Sender<Effects>) {
    #[allow(deprecated)]
    let (tx, rx) = channel::unbounded();
    let endpoint = rodio::default_output_device().unwrap();

    (AudioSystem {
      effects: Effects::None,
      sink: Sink::new(&endpoint),
      queue: rx,
    }, tx)
  }

  fn play_effect(&mut self) {
    let file = File::open(PISTOL_AUDIO_PATH).unwrap();
    let pistol_data = rodio::Decoder::new(BufReader::new(file)).unwrap();
    if self.sink.empty() {
      self.sink.append(pistol_data);
    }
  }
}

impl<'a> specs::prelude::System<'a> for AudioSystem {
  type SystemData = (ReadStorage<'a, CharacterInputState>);

  fn run(&mut self, character_input: Self::SystemData) {
    use specs::join::Join;

    while let Some(effect) = self.queue.try_recv() {
      match effect {
        Effects::PistolFire => self.effects = Effects::PistolFire,
        _ => self.effects = Effects::None,
      }
    }

    for ci in (&character_input).join() {
      if let Effects::PistolFire = self.effects { if ci.is_shooting { self.play_effect() } }
    }
  }
}
