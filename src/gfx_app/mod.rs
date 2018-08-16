use character::controls::CharacterControl;
use game::constants::{RESOLUTION_X, RESOLUTION_Y};
use gfx;
use gfx_app::controls::{Control, TilemapControls};
use gfx_device_gl;
use gfx_window_glutin;
use glutin;
use glutin::{GlContext, KeyboardInput, MouseButton};
use glutin::ElementState::{Pressed, Released};
use glutin::VirtualKeyCode::{Escape, Z, X, W, A, S, D, R};

pub mod init;
pub mod renderer;
pub mod system;
pub mod controls;
pub mod mouse_controls;

pub type ColorFormat = gfx::format::Rgba8;
pub type DepthFormat = gfx::format::DepthStencil;

pub struct WindowContext {
  window: glutin::GlWindow,
  controls: Option<controls::TilemapControls>,
  events_loop: glutin::EventsLoop,
  device: gfx_device_gl::Device,
  factory: gfx_device_gl::Factory,
  render_target_view: gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat>,
  depth_stencil_view: gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat>,
  mouse_pos: (f64, f64),
}

impl WindowContext {
  pub fn new() -> WindowContext {
    let events_loop = glutin::EventsLoop::new();

    let window_title = glutin::WindowBuilder::new()
      .with_title("Hinterland");

    let builder = if cfg!(feature = "windowed") {
      window_title
        .with_dimensions(RESOLUTION_X, RESOLUTION_Y)
        .with_min_dimensions(RESOLUTION_X, RESOLUTION_Y)
        .with_max_dimensions(RESOLUTION_X, RESOLUTION_Y)
    } else {
      let monitor = {
        events_loop.get_available_monitors().nth(0).expect("Please enter a valid ID")
      };
      window_title.with_fullscreen(Some(monitor))
                  .with_dimensions(RESOLUTION_X, RESOLUTION_Y)
                  .with_min_dimensions(RESOLUTION_X, RESOLUTION_Y)
                  .with_max_dimensions(RESOLUTION_X, RESOLUTION_Y)
    };

    let context = glutin::ContextBuilder::new()
      .with_vsync(true)
      .with_pixel_format(24, 8)
      .with_gl(glutin::GlRequest::GlThenGles {
        opengles_version: (3, 0),
        opengl_version: (4, 1),
      });

    let (window, device, factory, rtv, dsv) =
      gfx_window_glutin::init::<ColorFormat, DepthFormat>(builder, context, &events_loop);

    WindowContext {
      window,
      controls: None,
      events_loop,
      device,
      factory,
      render_target_view: rtv,
      depth_stencil_view: dsv,
      mouse_pos: (0.0, 0.0),
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum WindowStatus {
  Open,
  Close,
}

pub trait Window<D: gfx::Device, F: gfx::Factory<D::Resources>> {
  fn swap_window(&mut self);
  fn create_buffers(&mut self, count: usize) -> Vec<D::CommandBuffer>;
  fn set_controls(&mut self, controls: controls::TilemapControls);
  fn get_viewport_size(&mut self) -> (f32, f32);
  fn get_device(&mut self) -> &mut D;
  fn get_factory(&mut self) -> &mut F;
  fn get_hidpi_factor(&mut self) -> f32;
  fn get_render_target_view(&mut self) -> gfx::handle::RenderTargetView<D::Resources, ColorFormat>;
  fn get_depth_stencil_view(&mut self) -> gfx::handle::DepthStencilView<D::Resources, DepthFormat>;
  fn poll_events(&mut self) -> WindowStatus;
}

impl Window<gfx_device_gl::Device, gfx_device_gl::Factory> for WindowContext {
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

  fn get_viewport_size(&mut self) -> (f32, f32) {
    (RESOLUTION_X as f32, RESOLUTION_Y as f32)
  }

  fn get_device(&mut self) -> &mut gfx_device_gl::Device {
    &mut self.device
  }

  fn get_factory(&mut self) -> &mut gfx_device_gl::Factory {
    &mut self.factory
  }

  fn get_hidpi_factor(&mut self) -> f32 {
    self.window.hidpi_factor() as f32
  }

  fn get_render_target_view(&mut self) -> gfx::handle::RenderTargetView<gfx_device_gl::Resources, ColorFormat> {
    self.render_target_view.clone()
  }

  fn get_depth_stencil_view(&mut self) -> gfx::handle::DepthStencilView<gfx_device_gl::Resources, DepthFormat> {
    self.depth_stencil_view.clone()
  }

  fn poll_events(&mut self) -> WindowStatus {
    use glutin::WindowEvent::{CursorMoved, CloseRequested, MouseInput};

    let controls = match self.controls {
      Some(ref mut c) => c,
      None => panic!("Terrain controls have not been initialized"),
    };

    let m_pos = &mut self.mouse_pos;

    let mut game_status = WindowStatus::Open;

    self.events_loop.poll_events(|event| {
      game_status = if let glutin::Event::WindowEvent { event, .. } = event {
        match event {
          glutin::WindowEvent::KeyboardInput { input, .. } => {process_keyboard_input(input, controls)},
          MouseInput { state: Pressed, button: MouseButton::Left, .. } => {
            controls.mouse_left_click(Some(*m_pos));
            WindowStatus::Open
          }
          MouseInput { state: Released, button: MouseButton::Left, .. } => {
            controls.mouse_left_click(None);
            WindowStatus::Open
          }
          MouseInput { state: Pressed, button: MouseButton::Right, .. } => {
            controls.mouse_right_click(Some(*m_pos));
            WindowStatus::Open
          }
          MouseInput { state: Released, button: MouseButton::Right, .. } => {
            controls.mouse_right_click(None);
            WindowStatus::Open
          }
          CursorMoved { position, .. } => {
            *m_pos = position;
            WindowStatus::Open
          }
          CloseRequested => WindowStatus::Close,
          _ => WindowStatus::Open,
        }
      } else {
        WindowStatus::Open
      };
    });
    game_status
  }
}

fn process_keyboard_input(input: glutin::KeyboardInput, controls: &mut TilemapControls) -> WindowStatus {
  match input {
    KeyboardInput { virtual_keycode: Some(Escape), .. } => WindowStatus::Close,
    KeyboardInput { state: Pressed, virtual_keycode: Some(Z), .. } => {
      controls.zoom(&Control::Negative);
      WindowStatus::Open
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(X), .. } => {
      controls.zoom(&Control::Plus);
      WindowStatus::Open
    }
    KeyboardInput { state: Released, virtual_keycode: Some(Z), .. } |
    KeyboardInput { state: Released, virtual_keycode: Some(X), .. } => {
      controls.zoom(&Control::Released);
      WindowStatus::Open
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(W), .. } => {
      controls.move_character(CharacterControl::Up);
      WindowStatus::Open
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(S), .. } => {
      controls.move_character(CharacterControl::Down);
      WindowStatus::Open
    }
    KeyboardInput { state: Released, virtual_keycode: Some(W), .. } |
    KeyboardInput { state: Released, virtual_keycode: Some(S), .. } => {
      controls.move_character(CharacterControl::YMoveStop);
      WindowStatus::Open
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(A), .. } => {
      controls.move_character(CharacterControl::Left);
      WindowStatus::Open
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(D), .. } => {
      controls.move_character(CharacterControl::Right);
      WindowStatus::Open
    }
    KeyboardInput { state: Released, virtual_keycode: Some(A), .. } |
    KeyboardInput { state: Released, virtual_keycode: Some(D), .. } => {
      controls.move_character(CharacterControl::XMoveStop);
      WindowStatus::Open
    }
    KeyboardInput { state: Pressed, virtual_keycode: Some(R), .. } => {
      controls.reload_weapon(true);
      WindowStatus::Open
    },
    KeyboardInput { state: Released, virtual_keycode: Some(R), .. } => {
      controls.reload_weapon(false);
      WindowStatus::Open
    },
    KeyboardInput { state: Pressed, modifiers, .. } => {
      if modifiers.ctrl {
        controls.ctrl_pressed(true);
      }
      WindowStatus::Open
    }
    KeyboardInput { state: Released, modifiers, .. } => {
      if !modifiers.ctrl {
        controls.ctrl_pressed(false);
      }
      WindowStatus::Open
    }
  }
}

