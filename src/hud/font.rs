use cgmath::Point2;
use rusttype::{Font, point, Scale};

pub fn draw_text(font: &Font, font_size: f32, text: &str) -> (Point2<i32>, Vec<u8>) {
  let scale = Scale {
    x: font_size,
    y: font_size,
  };
  let v_metrics = font.v_metrics(scale);
  let offset = point(0.0, v_metrics.ascent);
  let glyphs: Vec<_> = font.layout(text, scale, offset).collect();
  let pixel_height = font_size.ceil() as usize;
  let width = glyphs
    .iter()
    .rev()
    .map(|g| g.position().x as f32 + g.unpositioned().h_metrics().advance_width)
    .next()
    .unwrap_or(0.0)
    .ceil() as usize;

  let mut pixel_data = vec![0_u8; 4 * width * pixel_height];
  let mapping_scale = 255.0;
  for g in glyphs {
    if let Some(bb) = g.pixel_bounding_box() {
      g.draw(|x, y, v| {
        let v = (v * mapping_scale + 0.5) as u8;
        let x = x as i32 + bb.min.x;
        let y = y as i32 + bb.min.y;
        if v > 0 && x >= 0 && x < width as i32 && y >= 0 && y < pixel_height as i32 {
          let i = (x as usize + y as usize * width) * 4;
          pixel_data[i] = 255;
          pixel_data[i + 1] = 255;
          pixel_data[i + 2] = 255;
          pixel_data[i + 3] = v;
        }
      });
    }
  }
  let size = Point2 {
    x: width as i32,
    y: pixel_height as i32,
  };
  (size, pixel_data)
}
