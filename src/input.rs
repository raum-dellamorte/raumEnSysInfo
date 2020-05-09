
use {
  glutin::{
    dpi::PhysicalPosition,
    event::{
      WindowEvent as WEvent,
      DeviceEvent as DEvent,
      KeyboardInput as KB,
      MouseButton as MB,
      ElementState::{Pressed, Released},
      VirtualKeyCode as VKC,
      // ModifiersState as MKS,
    },
  },
  crate::{
    timer::Timer,
    util::HashMap,
  },
};

pub struct Handler {
  pub timer: Timer,
  pub kb: HashMap<String, bool>,
  pub mouse: HashMap<MB, bool>,
  pub cursor_pos: Option<(f64, f64)>,
  pub cursor_delta: Option<(f64, f64)>,
}

impl Handler {
  pub fn new() -> Self {
    let mut timer = Timer::new();
    timer.tick();
    Handler { timer: timer, kb: HashMap::new(), mouse: HashMap::new(), cursor_pos: None, cursor_delta: None }
  }
  pub fn reset_delta(&mut self) {
    self.cursor_delta = None;
  }
  pub fn window_event(&mut self, event: &WEvent) {
    match event {
      WEvent::CursorMoved { position: pos, ..} => {
        self.cursor_pos = Some((pos.x,pos.y));
      }
      WEvent::CursorEntered { device_id: _, } => {
        // mouse moved, probably
      }
      WEvent::CursorLeft { device_id: _, } => {
        // mouse moved, probably
      }
      WEvent::AxisMotion { device_id: _, axis: _, value: _, } => {
        // mouse moved, probably
      }
      WEvent::MouseInput { state: Pressed, button: bttn, ..} => {
        self.mouse.insert(*bttn, true);
      }
      WEvent::MouseInput { state: Released, button: bttn, ..} => {
        self.mouse.insert(*bttn, false);
      }
      WEvent::Moved(PhysicalPosition { x: _, y: _, }) => {
        // Window moved
      }
      WEvent::ModifiersChanged { .. } => {  }
      WEvent::KeyboardInput { input: KB { virtual_keycode: Some(bttn), state: Pressed, ..}, ..} => {
        // print!("{:?}-{}-{}-{}-{}", bttn, modkey.shift, modkey.ctrl, modkey.alt, modkey.logo);
        self.kb.insert(key_code(&bttn), true);
      }
      WEvent::KeyboardInput { input: KB { virtual_keycode: Some(bttn), state: Released, ..}, ..} => {
        self.kb.insert(key_code(&bttn), false);
      }
      e => println!("Window Event:\n  {:?}", e)
      // _ => ()
    }
  }
  pub fn device_event(&mut self, event: &DEvent) {
    match event.clone() {
      DEvent::MouseMotion { delta: dxdy} => {
        self.cursor_delta = Some(dxdy);
      }
      DEvent::Button { button: bttn, state: Pressed } => {
        println!("Button pressed: {}", bttn);
      }
      DEvent::Button { button: bttn, state: Released } => {
        println!("Button released: {}", bttn);
      }
      DEvent::Motion {..} => {}
      // e => println!("Device Event:\n{:?}", e)
      _ => ()
    }
  }
  pub fn read_kb_single(&mut self, kc: KeyCode) -> bool {
    match self.kb.insert(key_code(&kc.key), false) {
      Some(tf) => { return tf; },
      None     => { return false; },
    }
  }
  pub fn read_kb_multi(&self, kc: KeyCode) -> bool {
    match self.kb.get(&key_code(&kc.key)) {
      Some(&tf) => { return tf; },
      None      => { return false; },
    }
  }
  pub fn read_kb_multi_any_of(&self, kcs: KeyCodes) -> bool {
    for kc in kcs.keys {
      if self.read_kb_multi(kc) { return true; }
    }
    false
  }
  pub fn clear_kb_bttns(&mut self) {
    self.kb.clear();
  }
  pub fn read_mouse_single(&mut self, key: MB) -> bool {
    match self.mouse.insert(key, false) {
      Some(tf) => { return tf; },
      None     => { return false; },
    }
  }
  pub fn read_mouse_multi(&self, key: MB) -> bool {
    match self.mouse.get(&key) {
      Some(&tf) => { return tf; },
      None      => { return false; },
    }
  }
  pub fn clear_mouse_bttns(&mut self) {
    self.mouse.clear();
  }
}
pub struct KeyCode {
  pub key: VKC,
}
impl KeyCode {
  pub fn new(key: VKC) -> Self {
    KeyCode {
      key: key,
    }
  }
  pub fn to_str(&self) -> String {
    key_code(&self.key)
  }
}
pub struct KeyCodes {
  keys: Vec<KeyCode>,
}
impl KeyCodes {
  pub fn new(keys: &[VKC]) -> Self {
    let mut out = KeyCodes { keys: Vec::new() };
    for key in keys {
      out.keys.push(KeyCode::new(*key));
    }
    out
  }
  pub fn to_str_vec(&self) -> Vec<String> {
    let mut out = Vec::new();
    for key in &self.keys {
      out.push(key_code(&key.key));
    }
    out
  }
}
pub fn key_code(bttn: &VKC) -> String {
  format!("{:?}", bttn)
}
