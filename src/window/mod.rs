use winit;

pub struct Window {
  events_loop: winit::EventsLoop,
  window: winit::Window,
}

impl Window {
  pub fn new() -> Window {
    let mut events_loop = winit::EventsLoop::new();
    let window = winit::Window::new(&events_loop).expect("Window creation failed");
    Window {
      events_loop,
      window,
    }
  }

  pub fn run(&mut self) {
    self.events_loop.run_forever(|event| {
      match event {
        winit::Event::WindowEvent {
          event: winit::WindowEvent::CloseRequested,
          ..
        } => winit::ControlFlow::Break,
        _ => winit::ControlFlow::Continue,
      }
    });
  }
}
