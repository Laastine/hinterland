use game::constants::PISTOL_AUDIO_PATH;
use gfx_app::mouse_controls::MouseInputState;
use rodio;
use rodio::Sink;
use specs;
use specs::{ReadStorage, WriteStorage};
use std::fs::File;
use std::io::BufReader;
use std::sync::mpsc;

pub enum Effects {
  PISTOL_FIRE,
  NONE
}

pub struct AudioData {
  sink: Sink,
}

impl AudioData {
  fn new() -> AudioData {
    let endpoint = rodio::default_endpoint().unwrap();

    AudioData {
      sink: Sink::new(&endpoint),
    }
  }

  pub fn play_effect(&mut self) {
    let file = File::open(PISTOL_AUDIO_PATH).unwrap();
    let pistol_data = rodio::Decoder::new(BufReader::new(file)).unwrap();
    self.sink.append(pistol_data);
  }
}

impl specs::Component for AudioData {
  type Storage = specs::HashMapStorage<AudioData>;
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
      effects: Effects::NONE,
      queue: rx,
    }, tx)
  }
}

impl<'a> specs::System<'a> for AudioSystem {
  type SystemData = (ReadStorage<'a, MouseInputState>,
                     WriteStorage<'a, AudioData>);

  fn run(&mut self, (mouse_input, mut audio_data): Self::SystemData) {
    use specs::Join;

    while let Ok(effect) = self.queue.try_recv() {
      match effect {
        Effects::PISTOL_FIRE => self.effects = Effects::PISTOL_FIRE,
        _ => self.effects = Effects::NONE,
      }
    }

    for (mi, audio) in (&mouse_input, &mut audio_data).join() {
      if mi.left_click_point.is_some() {
        audio.play_effect();
      }
    }
  }
}
