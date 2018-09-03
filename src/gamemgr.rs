
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use Display;
use Handler;
use Loader;
use text::{TextMgr, }; // RFontType, 
use texture::Texture;
use util::rmatrix::Matrix4f;

#[derive(Clone)]
pub struct GameMgr {
  pub handler: Arc<Mutex<Handler>>,
  pub loader: Arc<Mutex<Loader>>,
  pub display: Arc<Mutex<Display>>,
  pub textmgr: Option<Arc<Mutex<TextMgr>>>,
  pub textures: Arc<Mutex<HashMap<String, Arc<Texture>>>>,
  pub view_mat: Matrix4f, // not sure if I need this
}
impl GameMgr {
  pub fn new() -> Self {
    let loader = Arc::new(Mutex::new(Loader::new()));
    let handler = Arc::new(Mutex::new(Handler::new()));
    let display = Arc::new(Mutex::new(Display::new()));
    let textmgr = TextMgr::new();
    GameMgr {
      handler: handler,
      loader: loader,
      display: display,
      textmgr: Some(Arc::new(Mutex::new(textmgr))),
      textures: Arc::new(Mutex::new(HashMap::new())),
      view_mat: Matrix4f::new(),
    }
  }
  pub fn update_size(&mut self, dimensions: (u32, u32)) {
    {
      let mut d = self.display.lock().unwrap();
      d.update_size(dimensions);
    }
    let mgr = self.clone();
    let _textmgr = self.textmgr.take().unwrap();
    {
      let mut textmgr = _textmgr.lock().unwrap();
      textmgr.update_size(mgr);
    }
    self.textmgr = Some(_textmgr);
  }
  pub fn aspect_ratio(&self) -> f32 {
    let d = self.display.lock().unwrap();
    d.aspect_ratio
  }
  pub fn dimensions(&self) -> (u32, u32) {
    let d = self.display.lock().unwrap();
    d.dimensions()
  }
  pub fn handler_do<F>(&mut self, f: F)
    where F: Fn(&mut Handler) -> ()
  {
    let mut h = self.handler.lock().unwrap();
    f(&mut h);
  }
  pub fn loader_do<F>(&mut self, f: F)
    where F: Fn(&mut Loader) -> ()
  {
    let mut h = self.loader.lock().unwrap();
    f(&mut h);
  }
  pub fn new_texture(&mut self, name: &str) {
    let texture = {
      let _arc = self.loader.clone();
      let mut loader = _arc.lock().unwrap();
      loader.load_texture(name)
    };
    let _arc = self.textures.clone();
    let mut hm = _arc.lock().unwrap();
    // println!("texture: image<{}> tex_id<{}>", name, texture.tex_id);
    hm.insert(name.to_string(), Arc::new(texture));
  }
  pub fn texture(&self, name: &str) -> Arc<Texture> {
    let _arc = self.textures.clone();
    let mut hm = _arc.lock().unwrap();
    if hm.contains_key(name) {
      let out = hm.get_mut(name).unwrap();
      out.clone()
    } else { panic!("No Texture: {}", name) }
  }
  pub fn clean_up(&mut self) {
    let mut loader = self.loader.lock().unwrap();
    loader.clean_up();
  }
}
