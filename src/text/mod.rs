pub mod guitext;
pub mod metafile;
pub mod rtmc;
pub mod textmgr;

pub use {
  crate::{
    text::{
      textmgr::TextMgr,
    },
  },
};

use {
  crate::{
    shader::Shader,
    text::{
      guitext::GuiTextVals,
      rtmc::RTextMeshCreator,
      // metafile::MetaFile,
    },
    util::{
      Vector2f , Vector3f,
    },
  },
};

pub const SPACE_ASCII: u32 = 32;
pub const NEWLINE_ASCII: u32 = 10;
pub const LINE_HEIGHT: f32 = 0.03;

#[derive(Debug)]
pub struct RFontType {
  pub tex_atlas: String,
  pub rtmc: RTextMeshCreator,
}
impl RFontType {
  pub fn new(aspect_ratio: f32, font: &str) -> Self {
    Self {
      tex_atlas: font.to_owned(),
      rtmc: RTextMeshCreator::new(aspect_ratio, font),
    }
  }
  pub fn load_text(&mut self, text: &mut GuiTextVals) -> RTextMesh {
    self.rtmc.create_text_mesh(text)
  }
  pub fn update_size(&mut self, aspect_ratio: f32) {
    self.rtmc.update_size(aspect_ratio);
  }
}

#[derive(Debug, Clone)]
pub struct RFontEffect {
  pub offset: Vector2f,
  pub colour: Vector3f,
  pub colour_border: Vector3f,
  pub width: f32,
  pub edge: f32,
  pub width_border: f32,
  pub edge_border: f32,
  delta: f32,
  pub timer_r: f32,
  pub timer_g: f32,
  pub timer_b: f32,
  pub max_r: f32,
  pub max_g: f32,
  pub max_b: f32,
}
impl RFontEffect {
  pub fn new() -> Self {
    Self {
      offset: Vector2f::blank(),
      colour: Vector3f::blank(),
      colour_border: Vector3f::new_isize(1, 1, 1),
      width: 0.5,
      edge: 0.05,
      width_border: 0.4,
      edge_border: 0.3,
      delta: 0.0,
      timer_r: 0.0,
      timer_g: 0.0,
      timer_b: 0.0,
      max_r: 7.0,
      max_g: 5.0,
      max_b: 3.0,
    }
  }
  pub fn load_to_shader(&self, shader: &Shader) {
    shader.load_vec_2f("offset", &self.offset);
    shader.load_vec_3f("colour", &self.colour);
    shader.load_vec_3f("colourBorder", &self.colour_border);
    shader.load_float("width", self.width);
    shader.load_float("edge", self.edge);
    shader.load_float("widthBorder", self.width_border);
    shader.load_float("edgeBorder", self.edge_border);
  }
  pub fn anim_timer(&mut self, delta: f32) {
    self.delta = delta;
    self.timer_r += self.delta;
    self.timer_g += self.delta;
    self.timer_b += self.delta;
    if self.timer_r >= self.max_r { self.timer_r -= self.max_r; }
    if self.timer_g >= self.max_g { self.timer_g -= self.max_g; }
    if self.timer_b >= self.max_b { self.timer_b -= self.max_b; }
  }
  pub fn anim_border_colour(&mut self) {
    let r = self.timer_r / self.max_r;
    let g = self.timer_g / self.max_g;
    let b = self.timer_b / self.max_b;
    let anim_r = ((360.0 * r) + 0.0).to_radians().cos().abs();
    let anim_g = ((360.0 * g) + 0.0).to_radians().cos().abs();
    let anim_b = ((360.0 * b) + 0.0).to_radians().cos().abs();
    self.colour_border.from_f32(anim_r, anim_g, anim_b)
  }
}

#[derive(Debug)]
pub struct RTextMesh {
  pub verts: Vec<f32>,
  pub tex_coords: Vec<f32>,
  pub vert_count: u32,
}
impl RTextMesh {
  pub fn new(verts: Vec<f32>, tex_coords: Vec<f32>) -> Self {
    let count = verts.len() / 2;
    Self {
      verts: verts,
      tex_coords: tex_coords,
      vert_count: count as u32,
    }
  }
}

#[derive(Debug)]
pub struct RLine {
  pub words: Vec<RWord>,
  pub line_length: f32,
  pub max_length: f32,
  pub space_size: f32,
}
impl RLine {
  pub fn new(space_width: f32, font_size: f32, max_length: f32) -> Self {
    Self {
      words: Vec::new(),
      line_length: 0.0,
      max_length: max_length,
      space_size: space_width * font_size,
    }
  }
  pub fn try_add_word(&mut self, word: &mut Option<RWord>) -> Option<RWord> {
    let word = word.take().unwrap();
    let mut plus_length = (&word).width;
    if !self.words.is_empty() { plus_length += self.space_size; }
    // println!("size: {} trying to add word: {:?}, ", plus_length, word);
    if self.line_length + plus_length <= self.max_length {
      self.words.push(word);
      self.line_length += plus_length;
      None
    } else {
      Some(word)
    }
  }
}

#[derive(Debug)]
pub struct RWord {
  pub font_size: f32,
  pub chars: Vec<RChar>,
  pub width: f32,
}
impl RWord {
  pub fn new(size: f32) -> Self {
    Self {
      font_size: size,
      chars: Vec::new(),
      width: 0.0,
    }
  }
  pub fn add_char(&mut self, char: Option<&RChar>) {
    if char.is_some() {
      let char = char.unwrap();
      self.width += char.x_advance * self.font_size;
      self.chars.push((*char).clone());
    }
  }
}

#[derive(Clone, Debug)]
pub struct RChar {
  pub id: u32,
  pub x_tex: f32, pub y_tex: f32,
  pub x_tex_max: f32, pub y_tex_max: f32,
  pub x_offset: f32, pub y_offset: f32,
  pub x_size: f32, pub y_size: f32,
  pub x_advance: f32,
}
impl RChar {
  pub fn new(
    id: u32,
    x_tex: f32, y_tex: f32,
    x_tex_size: f32, y_tex_size: f32,
    x_offset: f32, y_offset: f32,
    x_size: f32, y_size: f32,
    x_advance: f32,
  ) -> Self {
    Self {
      id: id,
      x_tex: x_tex, y_tex: y_tex,
      x_tex_max: x_tex_size + x_tex, y_tex_max: y_tex_size + y_tex,
      x_offset: x_offset, y_offset: y_offset,
      x_size: x_size, y_size: y_size,
      x_advance: x_advance,
    }
  }
}