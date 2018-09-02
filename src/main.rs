#![recursion_limit="128"]
//#![allow(unused_imports)]
//#![allow(dead_code)]

extern crate gl;
extern crate glutin;
#[macro_use] extern crate nom;
extern crate image;
extern crate time;
extern crate sysinfo;

use gl::*;
// use std::os::raw::c_void;
use glutin::dpi::*;
use glutin::GlContext;

// const CVOID: *const c_void = 0 as *const c_void;

// in project stuff
pub mod camera; // I think I still need this for storing window dimensions
pub mod gamemgr;
pub mod input;
pub mod loader; // Can be simplified
pub mod render;
pub mod shader;
pub mod text;
pub mod texture; // needed for font atlas but needed things can be ported out
pub mod timer;
pub mod util;

pub use camera::Camera;
pub use input::Handler;
pub use loader::Loader;
pub use render::{RenderMgr, };
pub use shader::Shader;
pub use timer::Timer;

fn main() {
  // Test code for parsing fnt files
  // use text::metafile::test_noms;
  // test_noms();
  
  let mut events_loop = glutin::EventsLoop::new();
  let window = glutin::WindowBuilder::new()
    .with_title("RaumEn SysInfo")
    .with_dimensions(LogicalSize::new(640.0, 480.0));
  let context = glutin::ContextBuilder::new()
    .with_vsync(true);
  let gl_window = glutin::GlWindow::new(window, context, &events_loop).unwrap();
  
  unsafe {
    gl_window.make_current().unwrap();
  }
  
  unsafe {
    load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
    ClearColor(0.0, 1.0, 0.0, 1.0);
  }
  
  let mut render_mgr = RenderMgr::new();
  let mut mgr = render_mgr.mgr.clone();
  
  {
    let dpi = gl_window.get_hidpi_factor();
    let size = gl_window.get_inner_size().unwrap().to_physical(dpi);
    let mut camera = mgr.camera.lock().unwrap();
    camera.update_size(size.into());
  }
  {
    let _textmgr = mgr.clone().textmgr.take().unwrap();
    let mut textmgr = _textmgr.lock().unwrap();
    textmgr.add_font(mgr.clone(), "pirate");
    textmgr.add_font(mgr.clone(), "sans");
    textmgr.new_text(mgr.clone(), "Title", "raumEn SysInfo", "sans", 4.0, 0.0, 0.0, 1.0, true, true);
  }
  println!("Starting game loop.");
  let mut running = true;
  while running {
    mgr.handler_do(|handler| {
      handler.timer.tick();
      handler.reset_delta();
    });
    events_loop.poll_events(|event| {
      match event {
        glutin::Event::WindowEvent{ event, .. } => match event {
          glutin::WindowEvent::CloseRequested => running = false,
          glutin::WindowEvent::Resized(logical_size) => {
            let dpi = gl_window.get_hidpi_factor();
            let size = logical_size.to_physical(dpi);
            gl_window.resize(size);
            let mut camera = mgr.camera.lock().unwrap();
            camera.update_size(size.into());
          },
          _ => { mgr.handler_do(|handler| { handler.window_event(&event); }); }
        },
        glutin::Event::DeviceEvent{ event, ..} => {
          mgr.handler_do(|handler| { handler.device_event(&event); });
        }
        e => println!("Other Event:\n{:?}", e)
      }
    });
    render_mgr.render();
    
    gl_window.swap_buffers().unwrap();
  }
  render_mgr.clean_up();
}

pub const EOF: &str = "\04";

pub fn eof(string: &str) -> String {
  [string, EOF].join("")
}
