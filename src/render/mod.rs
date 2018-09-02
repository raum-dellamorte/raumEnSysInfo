
pub mod font;

use render::font::RenderFont;

use gl::*;
// use gl::types::{GLuint, }; // GLfloat, GLenum, GLint, GLchar, GLsizeiptr, GLboolean, 
// use glutin::dpi::PhysicalSize;

use gamemgr::GameMgr;

pub fn prepare() { unsafe {
  Clear(COLOR_BUFFER_BIT|DEPTH_BUFFER_BIT);
  ClearColor(0.2, 0.2, 0.3, 1.0);
}}

pub struct RenderMgr {
  pub mgr: GameMgr,
  pub ren_font: RenderFont,
}

impl RenderMgr {
  pub fn new() -> Self {
    RenderMgr {
      mgr: GameMgr::new(),
      ren_font: RenderFont::new(),
    }
  }
  pub fn render(&mut self) { 
    prepare();
    self.ren_font.render(self.mgr.clone());
    unsafe { BindVertexArray(0); }
  }
  pub fn clean_up(&mut self) {
    self.mgr.clean_up();
    self.ren_font.clean_up();
  }
}