
use time::Instant;

pub struct Timer {
  pub delta: f32,
  pub fps: f32,
  pub last: Instant,
  pub now: Instant,
}

impl Timer {
  pub fn new() -> Self {
    let tmp = Instant::now();
    Timer {
      delta: 0.0667_f32,
      fps: 0_f32,
      last: tmp,
      now: Instant::now(),
    }
  }
  
  pub fn tick(&mut self) -> &Self {
    self.last = self.now;
    self.now = Instant::now();
    let dur = self.now - self.last;
    self.delta = (dur.whole_microseconds() as f32) / 1000000_f32;
    self.fps = match self.delta > 0_f32 {
      true => 1_f32 / self.delta,
      false => 0_f32,
    };
    self
  }
}
