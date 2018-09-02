
use std::sync::{Arc, Mutex};
// use glutin::VirtualKeyCode::*;
// use glutin::MouseButton as MB;

use input::Handler;
// use util::rmatrix::Matrix4f;
// use util::rvector::{RVec, Vector3f, XVEC, YVEC}; // , ZVEC
// use util::rvertex::RVertex;

pub struct Camera {
  pub handler: Arc<Mutex<Handler>>,
  pub dimensions: (u32, u32),
  pub mouse_rate: f32,
}

impl Camera {
  pub fn new(handler: Arc<Mutex<Handler>>) -> Self {
    Camera {
      handler: handler,
      dimensions: (0, 0),
      mouse_rate: 1.0_f32,
    }
  }
  
  pub fn update_size(&mut self, dimensions: (u32, u32)) {
    self.dimensions = dimensions;
  }
}
