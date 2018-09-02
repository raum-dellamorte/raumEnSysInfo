
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

use camera::Camera;
use input::Handler;
use loader::Loader;
use text::{TextMgr, }; // RFontType, 
use texture::Texture;
use util::rmatrix::Matrix4f;

#[derive(Clone)]
pub struct GameMgr {
  pub handler: Arc<Mutex<Handler>>,
  pub loader: Arc<Mutex<Loader>>,
  pub camera: Arc<Mutex<Camera>>,
  pub textmgr: Option<Arc<Mutex<TextMgr>>>,
  pub textures: Arc<Mutex<HashMap<String, Arc<Texture>>>>,
  pub view_mat: Matrix4f, // not sure if I need this
}

impl GameMgr {
  pub fn new() -> Self {
    let loader = Arc::new(Mutex::new(Loader::new()));
    let handler = Arc::new(Mutex::new(Handler::new()));
    let camera = Arc::new(Mutex::new(Camera::new(handler.clone())));
    let textmgr = TextMgr::new();
    GameMgr {
      handler: handler,
      loader: loader,
      camera: camera,
      textmgr: Some(Arc::new(Mutex::new(textmgr))),
      textures: Arc::new(Mutex::new(HashMap::new())),
      view_mat: Matrix4f::new(),
    }
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
