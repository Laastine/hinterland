extern crate zombie_shooter_lib;
extern crate cgmath;

#[test]
fn direction_test() {
  use cgmath::Point2;
  assert_eq!(90, zombie_shooter_lib::graphics::direction(Point2 {
    x: 0.0,
    y: 0.0,
  }, Point2 {
    x: 0.0,
    y: 1.0,
  }));
}
