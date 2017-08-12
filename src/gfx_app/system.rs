use gfx_app::{ColorFormat, DepthFormat};
use gfx_app::renderer::EncoderQueue;
use gfx;
use terrain;
use character;
use specs;

pub struct DrawSystem<D: gfx::Device> {
  render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
  depth_stencil_view: gfx::handle::DepthStencilView<D::Resources, DepthFormat>,
  terrain_system: terrain::DrawSystem<D::Resources>,
  character_system: character::DrawSystem<D::Resources>,
  encoder_queue: EncoderQueue<D>,
}

impl<D: gfx::Device> DrawSystem<D> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<D::Resources, DepthFormat>,
                queue: EncoderQueue<D>)
                -> DrawSystem<D>
    where F: gfx::Factory<D::Resources>
  {
    DrawSystem {
      render_target_view: rtv.clone(),
      depth_stencil_view: dsv.clone(),
      terrain_system: terrain::DrawSystem::new(factory, rtv.clone(), dsv.clone()),
      character_system: character::DrawSystem::new(factory, rtv.clone(), dsv.clone()),
      encoder_queue: queue,
    }
  }
}

impl<D, C> specs::System<C> for DrawSystem<D>
  where D: gfx::Device,
        D::CommandBuffer: Send,
{
  fn run(&mut self, arg: specs::RunArg, _: C) {
    use specs::Join;
    let mut encoder = self.encoder_queue.receiver.recv().unwrap();
    let (mut terrain, character) = arg.fetch(|w| {
      (w.read::<terrain::Drawable>(),
       w.read::<character::Drawable>())
    });

    encoder.clear(&self.render_target_view, [16.0 / 256.0, 14.0 / 256.0, 22.0 / 256.0, 1.0]);
    encoder.clear_depth(&self.depth_stencil_view, 1.0);

    for t in (&mut terrain).join() {
      self.terrain_system.draw(t, &mut encoder);
    }

    for c in (&character).join() {
      self.character_system.draw(c, &mut encoder);
    }

    if let Err(e) = self.encoder_queue.sender.send(encoder) {
      println!("Disconnected, cannot return encoder to mpsc: {}", e);
    };
  }
}
