use crossbeam_channel as channel;
use gfx;

pub struct EncoderQueue<D: gfx::Device> {
  pub sender: channel::Sender<gfx::Encoder<D::Resources, D::CommandBuffer>>,
  pub receiver: channel::Receiver<gfx::Encoder<D::Resources, D::CommandBuffer>>,
}

pub struct DeviceRenderer<D: gfx::Device> {
  queue: EncoderQueue<D>,
}

impl<D: gfx::Device> DeviceRenderer<D> {
  pub fn new(buffers: Vec<D::CommandBuffer>) -> (DeviceRenderer<D>, EncoderQueue<D>) {
    let (a_send, b_recv) = channel::unbounded();
    let (b_send, a_recv) = channel::unbounded();

    for cb in buffers {
      let encoder = gfx::Encoder::from(cb);
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

  pub fn draw(&mut self, device: &mut D) {
    let _ = self.queue.receiver.recv()
      .map(|mut encoder| {
        encoder.flush(device);
        let _ = self.queue.sender.send(encoder);
      });
  }
}
