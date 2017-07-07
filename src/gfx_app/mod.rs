use winit;
use glutin;
use gfx;
use gfx_device_gl;
use gfx_window_glutin;
use std;
use glutin::WindowEvent::*;
use glutin::VirtualKeyCode::*;
use glutin::ElementState::*;

pub mod init;
pub mod renderer;
pub mod system;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;
pub type DefaultResources = gfx_device_gl::Resources;

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

#[derive(Debug,PartialEq,Eq)]
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
    use glutin::Event::*;
    use glutin::VirtualKeyCode::*;
    use glutin::ElementState::*;

    self.events_loop.poll_events(|event| {
      match event {
        _ => ()
      }
    });
    None
  }
}

pub struct WindowTargets<R: gfx::Resources> {
  pub color: gfx::handle::RenderTargetView<R, ColorFormat>,
  pub depth: gfx::handle::DepthStencilView<R, DepthFormat>,
  pub aspect_ratio: f32,
}

struct Harness {
  start: std::time::Instant,
  num_frames: f64,
}

impl Harness {
  fn new() -> Harness {
    Harness {
      start: std::time::Instant::now(),
      num_frames: 0.0,
    }
  }
  fn bump(&mut self) {
    self.num_frames += 1.0;
  }
}

impl Drop for Harness {
  fn drop(&mut self) {
    let time_end = self.start.elapsed();
    println!("Avg frame time: {} ms",
             ((time_end.as_secs() * 1000) as f64 +
               (time_end.subsec_nanos() / 1000_000) as f64) / self.num_frames);
  }
}

pub trait Factory<R: gfx::Resources>: gfx::Factory<R> {
  type CommandBuffer: gfx::CommandBuffer<R>;
  fn create_encoder(&mut self) -> gfx::Encoder<R, Self::CommandBuffer>;
}

pub trait ApplicationBase<R: gfx::Resources, C: gfx::CommandBuffer<R>> {
  fn new<F>(&mut F, WindowTargets<R>) -> Self where F: Factory<R, CommandBuffer=C>;
  fn render<D>(&mut self, &mut D) where D: gfx::Device<Resources=R, CommandBuffer=C>;
  fn on(&mut self, winit::WindowEvent);
  fn on_resize<F>(&mut self, &mut F, WindowTargets<R>) where F: Factory<R, CommandBuffer=C>;
}


impl Factory<gfx_device_gl::Resources> for gfx_device_gl::Factory {
  type CommandBuffer = gfx_device_gl::CommandBuffer;
  fn create_encoder(&mut self) -> gfx::Encoder<gfx_device_gl::Resources, Self::CommandBuffer> {
    self.create_command_buffer().into()
  }
}

pub trait Application<R: gfx::Resources>: Sized {
  fn new<F: gfx::Factory<R>>(&mut F, WindowTargets<R>) -> Self;
  fn render<C: gfx::CommandBuffer<R>>(&mut self, &mut gfx::Encoder<R, C>);

  fn on_resize(&mut self, WindowTargets<R>) {}

  fn on_resize_ext<F: gfx::Factory<R>>(&mut self, _factory: &mut F, targets: WindowTargets<R>) {
    self.on_resize(targets);
  }

  fn on(&mut self, _event: winit::WindowEvent) {}
}

pub struct Wrap<R: gfx::Resources, C, A> {
  encoder: gfx::Encoder<R, C>,
  app: A,
}

impl<R, C, A> ApplicationBase<R, C> for Wrap<R, C, A>
  where R: gfx::Resources,
        C: gfx::CommandBuffer<R>,
        A: Application<R> {
  fn new<F>(factory: &mut F, window_targets: WindowTargets<R>) -> Self
    where F: Factory<R, CommandBuffer=C> {
    Wrap {
      encoder: factory.create_encoder(),
      app: A::new(factory, window_targets),
    }
  }

  fn render<D>(&mut self, device: &mut D)
    where D: gfx::Device<Resources=R, CommandBuffer=C> {
    self.app.render(&mut self.encoder);
    self.encoder.flush(device);
  }

  fn on(&mut self, event: winit::WindowEvent) {
    self.app.on(event)
  }

  fn on_resize<F>(&mut self, factory: &mut F, window_targets: WindowTargets<R>)
    where F: Factory<R, CommandBuffer=C> {
    self.app.on_resize_ext(factory, window_targets);
  }
}
