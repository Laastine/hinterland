use glutin;
use gfx;
use gfx_device_gl;
use gfx_window_glutin;

pub mod init;
pub mod renderer;
pub mod system;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct GlutinWindow {
  window: glutin::Window,
  events_loop: glutin::EventsLoop,
  device: gfx_device_gl::Device,
  factory: gfx_device_gl::Factory,
  render_target_view: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
  depth_stencil_view: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
}

impl GlutinWindow {
  pub fn new() -> GlutinWindow {
    let builder = glutin::WindowBuilder::new()
      .with_title("Zombie shooter")
      .with_pixel_format(24, 8)
      .with_gl(glutin::GlRequest::GlThenGles {
        opengles_version: (3, 0),
        opengl_version: (4, 1),
      });

    let events_loop = glutin::EventsLoop::new();

    let (window, device, factory, rtv, dsv) = gfx_window_glutin::init::<ColorFormat,
      DepthFormat>(builder, &events_loop);

    GlutinWindow {
      window: window,
      events_loop: events_loop,
      device: device,
      factory: factory,
      render_target_view: rtv,
      depth_stencil_view: dsv,
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameStatus {
  Render,
  Quit,
}

pub trait Window<D: gfx::Device, F: gfx::Factory<D::Resources>> {
  fn swap_window(&mut self);
  fn poll_events(&mut self) -> Option<GameStatus>;

  fn create_buffers(&mut self, count: usize) -> Vec<D::CommandBuffer>;

  fn get_viewport_size(&mut self) -> (u32, u32);
  fn get_device(&mut self) -> &mut D;
  fn get_factory(&mut self) -> &mut F;
  fn get_render_target_view(&mut self) -> gfx::handle::RenderTargetView<D::Resources, ColorFormat>;
  fn get_depth_stencil_view(&mut self) -> gfx::handle::DepthStencilView<D::Resources, DepthFormat>;
}

impl Window<gfx_device_gl::Device, gfx_device_gl::Factory> for GlutinWindow {
  fn swap_window(&mut self) {
    use gfx::Device;
    self.window
      .swap_buffers()
      .expect("Unable to swap buffers");
    self.device.cleanup();
  }

  fn create_buffers(&mut self, count: usize) -> Vec<gfx_device_gl::CommandBuffer> {
    let mut bufs = Vec::new();
    for _ in 0..count {
      bufs.push(self.factory.create_command_buffer());
    }
    bufs
  }

  fn get_viewport_size(&mut self) -> (u32, u32) {
    self.window
      .get_inner_size_pixels()
      .unwrap_or((1280, 720))
  }

  fn get_device(&mut self) -> &mut gfx_device_gl::Device {
    &mut self.device
  }

  fn get_factory(&mut self) -> &mut gfx_device_gl::Factory {
    &mut self.factory
  }

  fn get_render_target_view(&mut self) -> gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat> {
    self.render_target_view.clone()
  }

  fn get_depth_stencil_view(&mut self) -> gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat> {
    self.depth_stencil_view.clone()
  }

  fn poll_events(&mut self) -> Option<GameStatus> {
    self.events_loop.poll_events(|event| {
      match event {
        glutin::Event::WindowEvent { event, .. } =>
          match event {
            glutin::WindowEvent::Closed => self.events_loop.interrupt(),
            _ => (),
          },
      }
    });
    None
  }
}
