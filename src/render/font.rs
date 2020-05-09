
use gl::*;
// use std::collections::{HashMap, HashSet};

use gamemgr::GameMgr;
use shader::gen_font_shader;
use shader::Shader;

pub struct RenderFont {
  pub shader: Shader,
}

impl RenderFont {
  pub fn new() -> Self {
    Self {
      shader: gen_font_shader(),
    }
  }
  pub fn render(&mut self, mgr: GameMgr) {
    let mut mgr = mgr;
    unsafe {
      Enable(BLEND);
      BlendFunc(SRC_ALPHA, ONE_MINUS_SRC_ALPHA);
      Disable(DEPTH_TEST);
    }
    // println!("Running Text Render Pass");
    let _textmgr = mgr.textmgr.take().unwrap();
    let mut textmgr = _textmgr.lock().unwrap();
    self.shader.start();
    let mut fonts = Vec::new();
    for (font, _) in &textmgr.active_text {
      fonts.push(font.to_owned());
    }
    for font in fonts {
      let tex_id = match textmgr.fonts.get_mut(&font) {
        Some(x) => {
          let texs = mgr.textures.lock().unwrap();
          match texs.get(&x.tex_atlas) {
            Some(tid) => { tid.tex_id }
            _ => { println!("No font atlas texture {}", &x.tex_atlas); continue }
          }
        }
        _ => { println!("No ftype {}", font); continue }
      };
      // println!("tex_id: {}", tex_id);
      unsafe {
        ActiveTexture(TEXTURE0);
        BindTexture(TEXTURE_2D, tex_id);
      }
      let mut gtstrs = Vec::new();
      for gtexts in textmgr.active_text.get(&font) { for gtstr in gtexts {
        gtstrs.push(gtstr.to_owned());
      }}
      for gtstr in gtstrs {
        for gtext in textmgr.texts.get_mut(&gtstr) {
          gtext.effect.anim_timer(mgr.delta());
          gtext.effect.anim_border_colour();
          unsafe {
            BindVertexArray(gtext.text_mesh_vao);
            EnableVertexAttribArray(0);
            EnableVertexAttribArray(1);
            gtext.effect.load_to_shader(&self.shader);
            self.shader.load_vec_2f("translation", &gtext.position);
            DrawArrays(TRIANGLES, 0, gtext.vertex_count as i32);
            DisableVertexAttribArray(0);
            DisableVertexAttribArray(1);
            BindVertexArray(0);
          }
        }
      }
    }
    self.shader.stop();
    unsafe {
      Disable(BLEND);
      Enable(DEPTH_TEST);
    }
  }
  pub fn clean_up(&mut self) {
    self.shader.clean_up();
  }
}
