extern crate zombie_shooter_lib;
extern crate cgmath;

#[test]
fn direction_test() {
  use cgmath::Point2;
  assert_eq!(0, zombie_shooter_lib::graphics::direction(Point2 {
    x: 1.0,
    y: 0.0,
  }, Point2 {
    x: 2.0,
    y: 0.0,
  }), "(1,0) to (2,0) should be 0deg");


  assert_eq!(90, zombie_shooter_lib::graphics::direction(Point2 {
    x: 0.0,
    y: 1.0,
  }, Point2 {
    x: 0.0,
    y: 2.0,
  }), "(0,1) to (0,2) should be 90deg");


  assert_eq!(26, zombie_shooter_lib::graphics::direction(Point2 {
    x: -2.0,
    y: 1.0,
  }, Point2 {
    x: 2.0,
    y: 3.0,
  }), "(-2,1) to (2,3) should be 26deg");

  assert_eq!(45, zombie_shooter_lib::graphics::direction(Point2 {
    x: -2.0,
    y: -2.0,
  }, Point2 {
    x: -1.0,
    y: -1.0,
  }), "(-2,-2) to (-1,-1) should be 45deg");

  assert_eq!(225, zombie_shooter_lib::graphics::direction(Point2 {
    x: -1.0,
    y: -2.0,
  }, Point2 {
    x: -3.0,
    y: -4.0,
  }), "(-1,-2) to (-3,-4) should be 225deg");

  assert_eq!(315, zombie_shooter_lib::graphics::direction(Point2 {
    x: -1.0,
    y: -2.0,
  }, Point2 {
    x: 1.0,
    y: -4.0,
  }), "(-1,-2) to (1,-4) should be 315deg");
}

#[test]
fn direction_movement_test() {
  use cgmath::Point2;

  assert_eq!(Point2 { x: 1.0, y: 0.0 },
             zombie_shooter_lib::graphics::direction_movement(
               zombie_shooter_lib::graphics::direction(Point2 {
                 x: 1.0,
                 y: 0.0,
               }, Point2 {
                 x: 2.0,
                 y: 0.0,
               })
             ), "(1,0) to (2,0) should be (1,0)");

  assert_eq!(Point2 { x: 0.0, y: 1.0 },
             zombie_shooter_lib::graphics::direction_movement(
               zombie_shooter_lib::graphics::direction(Point2 {
                 x: 0.0,
                 y: 1.0,
               }, Point2 {
                 x: 0.0,
                 y: 2.0,
               })
             ), "(0,1) to (0,2) should be (0,1)");

  assert_eq!(Point2 { x: 0.71, y: 0.71 }, // 0.71 = sqrt(2) / 2.0
             zombie_shooter_lib::graphics::direction_movement(
               zombie_shooter_lib::graphics::direction(Point2 {
                 x: -2.0,
                 y: -2.0,
               }, Point2 {
                 x: -1.0,
                 y: -1.0,
               })
             ), "(-2,-2) to (-1,-1) should be 45deg");

  assert_eq!(Point2 { x: -0.71, y: -0.71 }, // 0.71 = sqrt(2) / 2.0
             zombie_shooter_lib::graphics::direction_movement(
               zombie_shooter_lib::graphics::direction(Point2 {
                 x: -1.0,
                 y: -1.0,
               }, Point2 {
                 x: -2.0,
                 y: -2.0,
               })
             ), "(-1,-1) to (-2,-2) should be 225deg");
}
