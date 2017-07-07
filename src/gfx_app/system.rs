use gfx_app::{ColorFormat, DepthFormat};
use gfx_app::renderer::EncoderQueue;
use gfx;
use terrain;
use specs;

pub struct DrawSystem<D: gfx::Device> {
  render_target_view: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
  terrain_system: terrain::DrawSystem<D::Resources>,
  encoder_queue: EncoderQueue<D>,
}

impl<D: gfx::Device> DrawSystem<D> {
  pub fn new<F>(factory: &mut F,
                rtv: gfx::handle::RenderTargetView<D::Resources, ColorFormat>,
                dsv: gfx::handle::DepthStencilView<D::Resources, DepthFormat>,
                queue: EncoderQueue<D>,
                terrain: &terrain::terrain::Terrain)
                -> DrawSystem<D>
    where F: gfx::Factory<D::Resources>
  {
    DrawSystem {
      render_target_view: rtv.clone(),
      terrain_system: terrain::DrawSystem::new(factory, rtv.clone(), dsv.clone(), terrain),
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
    let terrain =
      arg.fetch(|w| {
         w.read::<terrain::Drawable>()
      });

    encoder.clear(&self.render_target_view, [0.0, 0.0, 0.0, 1.0]);

    for t in (&terrain).join() {
      self.terrain_system.draw(t, &mut encoder);
    }

    if let Err(e) = self.encoder_queue.sender.send(encoder) {
      println!("Disconnected, cannot return encoder to mpsc: {}", e);
    };
  }
}
