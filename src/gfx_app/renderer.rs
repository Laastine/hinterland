use gfx;
use std::sync::mpsc;

#[derive(Debug)]
pub struct EncoderQueue<D: gfx::Device> {
  pub sender: mpsc::Sender<gfx::Encoder<D::Resources, D::CommandBuffer>>,
  pub receiver: mpsc::Receiver<gfx::Encoder<D::Resources, D::CommandBuffer>>,
}

pub struct DeviceRenderer<D: gfx::Device> {
  queue: EncoderQueue<D>,
}

impl<D: gfx::Device> DeviceRenderer<D> {
  pub fn new(buffers: Vec<D::CommandBuffer>) -> (DeviceRenderer<D>, EncoderQueue<D>) {
    let (a_send, b_recv) = mpsc::channel();
    let (b_send, a_recv) = mpsc::channel();

    for cb in buffers {
      let encoder = gfx::Encoder::from(cb);
      a_send
        .send(encoder)
        .expect("command buffers send failed");
    }

    (DeviceRenderer {
      queue: EncoderQueue {
        sender: a_send,
        receiver: a_recv,
      },
    },
     EncoderQueue {
       sender: b_send,
       receiver: b_recv,
     })
  }

  pub fn draw(&mut self, device: &mut D) -> bool {
    match self.queue.receiver.recv() {
      Ok(mut encoder) => {
        encoder.flush(device);
        match self.queue.sender.send(encoder) {
          Ok(_) => {
            true
          },
          Err(e) => {
            false
            panic!("Unable to send {}", e);
          }
        }
      }
      Err(e) => {
        println!("Unable to receive {}", e);
        false
      }
    }
  }
}
