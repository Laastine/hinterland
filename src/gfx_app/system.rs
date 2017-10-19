use gfx_app::{ColorFormat, DepthFormat};
use gfx_app::renderer::EncoderQueue;
use gfx;
use terrain;
use character;
use specs;
use std::time::Instant;
use character::character::CharacterSprite;
use graphics::orientation::Stance;

pub type Delta = f32;

pub struct DrawSystem<D: gfx::Device> {
  render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
  depth_stencil_view: gfx::handle::DepthStencilView<D::Resources, DepthFormat>,
  terrain_system: terrain::DrawSystem<D::Resources>,
  character_system: character::DrawSystem<D::Resources>,
  encoder_queue: EncoderQueue<D>,
  game_time: Instant,
  frames: u32,
  cool_down: f32,
  fire_cool_down: f32,
}

impl<D: gfx::Device> DrawSystem<D> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<D::Resources, DepthFormat>,
                encoder_queue: EncoderQueue<D>)
                -> DrawSystem<D>
    where F: gfx::Factory<D::Resources>
  {
    DrawSystem {
      render_target_view: rtv.clone(),
      depth_stencil_view: dsv.clone(),
      terrain_system: terrain::DrawSystem::new(factory, rtv.clone(), dsv.clone()),
      character_system: character::DrawSystem::new(factory, rtv.clone(), dsv.clone()),
      encoder_queue,
      game_time: Instant::now(),
      frames: 0,
      cool_down: 1.0,
      fire_cool_down: 1.0
    }
  }
}

impl<D> specs::System<Delta> for DrawSystem<D>
  where D: gfx::Device,
        D::CommandBuffer: Send,
{
  fn run(&mut self, arg: specs::RunArg, delta: Delta) {
    use specs::Join;
    let mut encoder = self.encoder_queue.receiver.recv().unwrap();
    let (mut terrain, mut character, mut sprite) = arg.fetch(|w| {
      if self.cool_down == 0.0 {
        self.cool_down += 0.07;
      }
      if self.fire_cool_down == 0.0 {
        self.fire_cool_down += 0.2;
      }
      self.cool_down = (self.cool_down - delta).max(0.0);
      self.fire_cool_down = (self.fire_cool_down - delta).max(0.0);
      (w.write::<terrain::Drawable>(),
       w.write::<character::CharacterDrawable>(),
       w.write::<CharacterSprite>())
    });

    let current_time = Instant::now();
    self.frames += 1;
    if cfg!(feature = "dev") && (current_time.duration_since(self.game_time).as_secs()) >= 1 {
      println!("{:?} ms/frames", 1000.0 / self.frames as f32);
      self.frames = 0;
      self.game_time = Instant::now();
    }

    encoder.clear(&self.render_target_view, [16.0 / 256.0, 16.0 / 256.0, 20.0 / 256.0, 1.0]);
    encoder.clear_depth(&self.depth_stencil_view, 1.0);

    for (t, c, s) in (&mut terrain, &mut character, &mut sprite).join() {
      self.terrain_system.draw(t, &mut encoder);
      if self.cool_down == 0.0 && c.stance == Stance::Normal {
        s.update_run();
      } else if self.fire_cool_down == 0.0 && c.stance == Stance::Firing {
        s.update_fire();
      }
      self.character_system.draw(c, s, &mut encoder);
    }

    if let Err(e) = self.encoder_queue.sender.send(encoder) {
      println!("Disconnected, cannot return encoder to mpsc: {}", e);
    };
  }
}
