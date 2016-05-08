#[macro_use]
mod events;
pub mod data;
pub mod gfx;

use sdl2::render::Renderer;

struct_events! {
  keyboard: {
    key_escape: Escape,
    key_up: Up,
    key_down: Down,
    key_left: Left,
    key_right: Right,
    key_space: Space
  },
  else: {
    quit: Quit { .. }
  }
}

pub struct Game<'window> {
  pub events: Events,
  pub renderer: Renderer<'window>,
}

impl<'window> Game<'window> {
  fn new(events: Events, renderer: Renderer<'window>) -> Game<'window> {
    Game {
      events: events,
      renderer: renderer,
    }
  }

  pub fn output_size(&self) -> (f64, f64) {
    let (w, h) = self.renderer.output_size().unwrap();
    (w as f64, h as f64)
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

  pub fn spawn<F>(title: &str, init: F)
  where F: Fn(&mut Game) -> Box<View> {
    let sdl_context = ::sdl2::init().unwrap();
    let video = sdl_context.video().unwrap();
    let mut timer = sdl_context.timer().unwrap();
    let _image_context = ::sdl2_image::init(::sdl2_image::INIT_PNG).unwrap();

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
        ViewAction::None =>
        context.renderer.present(),

        ViewAction::Quit =>
        break,

        ViewAction::ChangeView(new_view) =>
        current_view = new_view,
      }
    }
  }
