use crossbeam_channel as channel;
use wgpu;

pub struct EncoderQueue {
  pub sender: channel::Sender<wgpu::CommandEncoder>,
  pub receiver: channel::Receiver<wgpu::CommandEncoder>,
}

pub struct DeviceRenderer {
  queue: EncoderQueue,
}

impl DeviceRenderer {
  pub fn new(buffers: Vec<wgpu::CommandEncoder>, device: &mut wgpu::Device) -> (DeviceRenderer, EncoderQueue) {
    let (a_send, b_recv) = channel::unbounded();
    let (b_send, a_recv) = channel::unbounded();

    for cb in buffers {
      let encoder = device.create_command_encoder(&wgpu::CommandEncoderDescriptor { todo: 0 });
      let _ = a_send
        .send(encoder)
        .map_err(|e| panic!("Device renderer error {:?}", e));
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

  pub fn draw(&mut self, device: &mut wgpu::Device) {
    let _ = self.queue.receiver.recv()
      .map(|mut encoder| {
        let _ = self.queue.sender.send(encoder.finish());
      });
  }
}
