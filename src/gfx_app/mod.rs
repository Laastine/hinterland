use game::constants::{RESOLUTION_X, RESOLUTION_Y};
use gfx;
use gfx_device_gl;
use gfx_window_glutin;
use glutin;
use glutin::GlContext;

pub mod init;
pub mod renderer;
pub mod system;
pub mod controls;
pub mod mouse_controls;

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
    let events_loop = glutin::EventsLoop::new();

    let window_title = glutin::WindowBuilder::new()
      .with_title("Hinterland");

    let builder = if cfg!(feature = "windowed") {
      window_title.with_dimensions(RESOLUTION_X, RESOLUTION_Y)
    } else {
      let monitor = {
        events_loop.get_available_monitors().nth(0).expect("Please enter a valid ID")
      };
      window_title.with_fullscreen(Some(monitor))
                  .with_dimensions(RESOLUTION_X, RESOLUTION_Y)
    };

    let context = glutin::ContextBuilder::new()
      .with_vsync(true)
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
      mouse_pos: (0.0, 0.0),
    }
  }
}

#[derive(Debug, PartialEq, Eq)]
pub enum GameStatus {
  Quit,
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
  fn poll_events(&mut self) -> Option<GameStatus>;
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

  fn get_viewport_size(&mut self) -> (f32, f32) {
    let viewport = self.window
        .get_inner_size()
        .unwrap_or((RESOLUTION_X, RESOLUTION_Y));
    ((viewport.0 as f32), (viewport.1 as f32))
  }

  fn get_device(&mut self) -> &mut gfx_device_gl::Device {
    &mut self.device
  }

  fn get_factory(&mut self) -> &mut gfx_device_gl::Factory {
    &mut self.factory
  }

  fn get_hidpi_factor(&mut self) -> f32 {
    self.window.hidpi_factor()
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
    use glutin::WindowEvent::{Resized, Closed, CursorMoved, MouseInput};
    use glutin::ElementState::{Pressed, Released};
    use glutin::VirtualKeyCode::{Escape, Z, X, W, A, S, D};

    let controls = match self.controls {
      Some(ref mut c) => c,
      None => panic!("Terrain controls have not been initialized"),
    };
    let w = &self.window;
    let m_rtv = &mut self.render_target_view;
    let m_dsv = &mut self.depth_stencil_view;
    let m_pos = &mut self.mouse_pos;

    let mut game_status: Option<GameStatus> = None;

    self.events_loop.poll_events(|event| {
      game_status = if let glutin::Event::WindowEvent { event, .. } = event {
        match event {
          glutin::WindowEvent::KeyboardInput { input, .. } => match input {
            KeyboardInput { virtual_keycode: Some(Escape), .. } => Some(GameStatus::Quit),
            KeyboardInput { state: Pressed, virtual_keycode: Some(Z), .. } => {
              controls.zoom_out();
              None
            }
            KeyboardInput { state: Pressed, virtual_keycode: Some(X), .. } => {
              controls.zoom_in();
              None
            }
            KeyboardInput { state: Released, virtual_keycode: Some(Z), .. } |
            KeyboardInput { state: Released, virtual_keycode: Some(X), .. } => {
              controls.zoom_stop();
              None
            }
            KeyboardInput { state: Pressed, virtual_keycode: Some(W), .. } => {
              controls.move_character_up();
              None
            }
            KeyboardInput { state: Pressed, virtual_keycode: Some(S), .. } => {
              controls.move_character_down();
              None
            }
            KeyboardInput { state: Released, virtual_keycode: Some(W), .. } |
            KeyboardInput { state: Released, virtual_keycode: Some(S), .. } => {
              controls.stop_character_y();
              None
            }
            KeyboardInput { state: Pressed, virtual_keycode: Some(A), .. } => {
              controls.move_character_left();
              None
            }
            KeyboardInput { state: Pressed, virtual_keycode: Some(D), .. } => {
              controls.move_character_right();
              None
            }
            KeyboardInput { state: Released, virtual_keycode: Some(A), .. } |
            KeyboardInput { state: Released, virtual_keycode: Some(D), .. } => {
              controls.stop_character_x();
              None
            }
            _ => None,
          },
          MouseInput { state: Pressed, button: MouseButton::Left, .. } => {
            controls.mouse_left_click(Some(*m_pos));
            None
          }
          MouseInput { state: Released, button: MouseButton::Left, .. } => {
            controls.mouse_left_click(None);
            None
          }
          MouseInput { state: Pressed, button: MouseButton::Right, .. } => {
            controls.mouse_right_click(Some(*m_pos));
            None
          }
          MouseInput { state: Released, button: MouseButton::Right, .. } => {
            controls.mouse_right_click(None);
            None
          }
          CursorMoved { position, .. } => {
            *m_pos = position;
            None
          }
          Closed => Some(GameStatus::Quit),
          Resized(_, _) => {
            gfx_window_glutin::update_views(w, m_rtv, m_dsv);
            Some(GameStatus::Quit)
          }
          _ => None,
        }
      } else {
        None
      };
    });
    game_status
  }
}


