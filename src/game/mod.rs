#[macro_use]
mod events;
pub mod data;
pub mod gfx;
pub mod constants;

use sdl2::image::{INIT_PNG};
use sdl2::mixer::{INIT_OGG, AUDIO_S16LSB};
use sdl2::render::Renderer;

struct_events! {
  keyboard: {
    key_escape: Escape,
    key_up: W,
    key_down: S,
    key_left: A,
    key_right: D,
    zoom_in: KpPlus,
    zoom_out: KpMinus
  },
  else: {
    quit: Quit { .. }
  }
}

pub struct Game<'window> {
  pub events: Events,
  pub renderer: Renderer<'window>,
  allocated_channels: i32,
}

impl<'window> Game<'window> {
  fn new(events: Events, renderer: Renderer<'window>) -> Game<'window> {
    let allocated_channels = 32i32;
    ::sdl2::mixer::allocate_channels(allocated_channels);
    Game {
      events: events,
      renderer: renderer,
      allocated_channels: allocated_channels,
    }
  }

  pub fn output_size(&self) -> (f64, f64) {
    let (w, h) = self.renderer.output_size().unwrap();
    (w as f64 * 2.0, h as f64 * 2.0)
  }

  pub fn play_sound(&mut self, sound: &::sdl2::mixer::Chunk) {
    match ::sdl2::mixer::Channel::all().play(sound, 0) {
      Err(_) => {
        self.allocated_channels *= 2;
        ::sdl2::mixer::allocate_channels(self.allocated_channels);
        self.play_sound(sound);
      },

      _ => {}
    }
  }
}

pub enum ViewAction {
  None,
  Quit,
  ChangeView(Box<View>),
}

pub trait View {
  fn render(&mut self, context: &mut Game, elapsed: f64) -> ViewAction;
}

pub fn spawn<F>(title: &str, init: F) where F: Fn(&mut Game) -> Box<View> {
  let sdl_context = ::sdl2::init().unwrap();
  let video = sdl_context.video().unwrap();
  let mut timer = sdl_context.timer().unwrap();
  let _ = ::sdl2::image::init(INIT_PNG).unwrap();
  let _ = ::sdl2::mixer::init(INIT_OGG).unwrap();

  let frequency = 44100;
  let format = AUDIO_S16LSB;
  let channels = 2;
  let chunk_size = 1024;
  let _ = ::sdl2::mixer::open_audio(frequency, format, channels, chunk_size).unwrap();

  let window = video.window(title, 1280, 720)
    .position_centered().opengl().resizable()
    .build().unwrap();

  let mut context = Game::new(
    Events::new(sdl_context.event_pump().unwrap()),
    window.renderer()
      .accelerated()
      .build().unwrap());

  let mut current_view = init(&mut context);

  let interval = 1_000 / 60;
  let mut before = timer.ticks();
  let mut last_second = timer.ticks();
  let mut fps = 0u16;

  loop {
    let now = timer.ticks();
    let dt = now - before;
    let elapsed = dt as f64 / 1_000.0;

    if dt < interval {
      timer.delay(interval - dt);
      continue;
    }

    before = now;
    fps += 1;

    if now - last_second > 1_000 {
      println!("FPS: {}", fps);
      last_second = now;
      fps = 0;
    }

    context.events.pump(&mut context.renderer);

    match current_view.render(&mut context, elapsed) {
      ViewAction::None => context.renderer.present(),
      ViewAction::Quit => break,
      ViewAction::ChangeView(new_view) => current_view = new_view,
    }
  }
}
