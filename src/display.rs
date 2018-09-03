
use gl::Viewport;

pub struct Display {
  pub w: u32,
  pub h: u32,
  pub aspect_ratio: f32,
}

impl Display {
  pub fn new() -> Self {
    Self {
      w: 640,
      h: 480,
      aspect_ratio: 1.333334,
    }
  }
  pub fn dimensions(&self) -> (u32, u32) {
    (self.w, self.h)
  }
  pub fn update_size(&mut self, dimensions: (u32, u32)) {
    let (w, h) = dimensions;
    unsafe { Viewport(0, 0, w as i32, h as i32); }
    self.w = w;
    self.h = h;
    self.aspect_ratio = w as f32 / h as f32;
  }
}
