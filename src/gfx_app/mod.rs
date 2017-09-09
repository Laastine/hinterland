use glutin;
use gfx;
use gfx_device_gl;
use gfx_window_glutin;
use glutin::GlContext;
use std::process;
use game::constants::{RESOLUTION_X, RESOLUTION_Y};

pub mod init;
pub mod renderer;
pub mod system;
pub mod controls;
pub mod graphics;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct GlutinWindow {
  window: glutin::GlWindow,
  controls: Option<controls::TilemapControls>,
  events_loop: glutin::EventsLoop,
  device: gfx_device_gl::Device,
  factory: gfx_device_gl::Factory,
  render_target_view: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
  depth_stencil_view: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
  mouse_pos: (f64, f64),
}

impl GlutinWindow {
  pub fn new() -> GlutinWindow {
    let builder = glutin::WindowBuilder::new()
      .with_title("Zombie shooter")
      .with_dimensions(RESOLUTION_X, RESOLUTION_Y);

    let events_loop = glutin::EventsLoop::new();
    let context = glutin::ContextBuilder::new()
      .with_pixel_format(24, 8)
      .with_gl(glutin::GlRequest::GlThenGles {
        opengles_version: (3, 0),
        opengl_version: (4, 1),
      });

    let (window, device, factory, rtv, dsv) = gfx_window_glutin::init::<ColorFormat,
      DepthFormat>(builder, context, &events_loop);

    GlutinWindow {
      window,
      controls: None,
      events_loop,
      device,
      factory,
      render_target_view: rtv,
      depth_stencil_view: dsv,
      mouse_pos: (0.0, 0.0)
    }
  }
}

#[allow(dead_code)]
#[derive(Debug, PartialEq, Eq)]
pub enum GameStatus {
  Render,
  Quit,
}


pub trait Window<D: gfx::Device, F: gfx::Factory<D::Resources>> {
  fn swap_window(&mut self);
  fn poll_events(&mut self) -> Option<GameStatus>;

  fn create_buffers(&mut self, count: usize) -> Vec<D::CommandBuffer>;
  fn set_controls(&mut self, controls: controls::TilemapControls);

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

  fn set_controls(&mut self, controls: controls::TilemapControls) {
    self.controls = Some(controls);
  }

  fn get_viewport_size(&mut self) -> (u32, u32) {
    self.window
      .get_inner_size_pixels()
      .unwrap_or((RESOLUTION_X, RESOLUTION_Y))
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
    use glutin::KeyboardInput;
    use glutin::MouseButton;
    use glutin::WindowEvent::{Resized, Closed, MouseMoved, MouseInput};
    use glutin::ElementState::{Pressed, Released};
    use glutin::VirtualKeyCode::{Escape, Minus, Equals, W, A, S, D, Up, Down, Left, Right};

    let controls = match self.controls {
      Some(ref mut c) => c,
      None => panic!("Terrain controls have not been initialized"),
    };
    let ref w = self.window;
    let ref mut rtv = self.render_target_view;
    let ref mut dsv = self.depth_stencil_view;
    let ref mut m_pos = self.mouse_pos;

    self.events_loop.poll_events(|event| {
      match event {
        glutin::Event::WindowEvent { event, .. } => match event {
          glutin::WindowEvent::KeyboardInput { input, .. } => match input {
            KeyboardInput { state: _, virtual_keycode: Some(Escape), modifiers: _, scancode: _ } => process::exit(0),
            KeyboardInput { state: Pressed, scancode: _, modifiers: _, virtual_keycode: Some(Minus) } => controls.zoom_out(),
            KeyboardInput { state: Pressed, scancode: _, modifiers: _, virtual_keycode: Some(Equals) } => controls.zoom_in(),
            KeyboardInput { state: Released, scancode: _, modifiers: _, virtual_keycode: Some(Minus) } |
            KeyboardInput { state: Released, scancode: _, modifiers: _, virtual_keycode: Some(Equals) } => controls.zoom_stop(),
            KeyboardInput { state: Pressed, scancode: _, modifiers: _, virtual_keycode: Some(Up) } => controls.move_map_up(),
            KeyboardInput { state: Pressed, scancode: _, modifiers: _, virtual_keycode: Some(Down) } => controls.move_map_down(),
            KeyboardInput { state: Released, scancode: _, modifiers: _, virtual_keycode: Some(Up) } |
            KeyboardInput { state: Released, scancode: _, modifiers: _, virtual_keycode: Some(Down) } => controls.stop_map_y(),
            KeyboardInput { state: Pressed, scancode: _, modifiers: _, virtual_keycode: Some(Right) } => controls.move_map_right(),
            KeyboardInput { state: Pressed, scancode: _, modifiers: _, virtual_keycode: Some(Left) } => controls.move_map_left(),
            KeyboardInput { state: Released, scancode: _, modifiers: _, virtual_keycode: Some(Right) } |
            KeyboardInput { state: Released, scancode: _, modifiers: _, virtual_keycode: Some(Left) } => controls.stop_map_x(),
            KeyboardInput { state: Pressed, scancode: _, modifiers: _, virtual_keycode: Some(W) } => controls.move_character_up(),
            KeyboardInput { state: Pressed, scancode: _, modifiers: _, virtual_keycode: Some(S) } => controls.move_character_down(),
            KeyboardInput { state: Released, scancode: _, modifiers: _, virtual_keycode: Some(W) } |
            KeyboardInput { state: Released, scancode: _, modifiers: _, virtual_keycode: Some(S) } => controls.stop_character_y(),
            KeyboardInput { state: Pressed, scancode: _, modifiers: _, virtual_keycode: Some(A) } => controls.move_character_left(),
            KeyboardInput { state: Pressed, scancode: _, modifiers: _, virtual_keycode: Some(D) } => controls.move_character_right(),
            KeyboardInput { state: Released, scancode: _, modifiers: _, virtual_keycode: Some(A) } |
            KeyboardInput { state: Released, scancode: _, modifiers: _, virtual_keycode: Some(D) } => controls.stop_character_x(),
            _ => (),
          },
          MouseInput {device_id: _, state: Pressed, button: MouseButton::Left} => {
            println!("Click {:?}", m_pos);
          },
          MouseMoved {device_id: _, position} => {
            *m_pos = position;
          },
          Closed => process::exit(0),
          Resized(_, _) => gfx_window_glutin::update_views(w, rtv, dsv),
          _ => (),
        },
        _ => (),
      }
    });
    None
  }
}


