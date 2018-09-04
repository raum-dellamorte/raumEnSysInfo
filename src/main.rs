#![recursion_limit="128"]
//#![allow(unused_imports)]
//#![allow(dead_code)]

extern crate gl;
extern crate glutin;
#[macro_use] extern crate nom;
extern crate image;
extern crate time;
extern crate cupid;
extern crate sysinfo;
// extern crate subprocess;

use gl::*;
// use std::os::raw::c_void;
use glutin::dpi::*;
use glutin::GlContext;

use sysinfo::SystemExt;

// const CVOID: *const c_void = 0 as *const c_void;

// in project stuff
pub mod display; // I think I still need this for storing window dimensions
pub mod gamemgr;
pub mod input;
pub mod loader; // Can be simplified
pub mod render;
pub mod shader;
pub mod text;
pub mod texture; // needed for font atlas but needed things can be ported out
pub mod timer;
pub mod util;

pub use display::Display;
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
  
  let mut _system = sysinfo::System::new();
  let system = &mut _system;
  
  let cpu = &cpu_name();
  let ram = &get_ram_total(system);
  let mut cpu_ram = mk_cpu_ram_str(cpu, ram, system);
  
  let mut fps: f32;
  let mut sec = 0.0;
  
  {
    let dpi = gl_window.get_hidpi_factor();
    let size = gl_window.get_inner_size().unwrap().to_physical(dpi);
    mgr.update_size(size.into());
  }
  {
    let _textmgr = mgr.clone().textmgr.take().unwrap();
    let mut textmgr = _textmgr.lock().unwrap();
    textmgr.add_font(mgr.clone(), "pirate");
    textmgr.add_font(mgr.clone(), "sans");
    textmgr.new_text(mgr.clone(), "Title", "raumEn SysInfo", "pirate", 4.0, 0.0, 0.0, 1.0, true, true);
    textmgr.new_text(mgr.clone(), "CPU RAM", &cpu_ram, "sans", 2.0, 0.0, 0.4, 1.0, true, true);
    textmgr.new_text(mgr.clone(), "FPS", "FPS: 0.0", "sans", 1.5, 0.0, 0.0, 0.3, false, true);
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
            mgr.update_size(size.into());
          },
          _ => { mgr.handler_do(|handler| { handler.window_event(&event); }); }
        },
        glutin::Event::DeviceEvent{ event, ..} => {
          mgr.handler_do(|handler| { handler.device_event(&event); });
        }
        e => println!("Other Event:\n{:?}", e)
      }
    });
    {
      let handler = mgr.handler.lock().unwrap();
      fps = handler.timer.fps;
      sec += handler.timer.delta;
    }
    if sec >= 1.0 {
      sec -= 1.0;
      cpu_ram = mk_cpu_ram_str(cpu, ram, system);
      let _textmgr = mgr.clone().textmgr.take().unwrap();
      let mut textmgr = _textmgr.lock().unwrap();
      textmgr.update_text(mgr.clone(), "CPU RAM", &cpu_ram);
      textmgr.update_text(mgr.clone(), "FPS", &format!("FPS: {:.3}", (fps * 1000.0).round() / 1000.0 ) );
    }
    
    render_mgr.render();
    
    gl_window.swap_buffers().unwrap();
  }
  render_mgr.clean_up();
}

pub const EOF: &str = "\04";

pub fn eof(string: &str) -> String {
  [string, EOF].join("")
}

// pub fn call_cmd(cmd: &str) -> Result<String, String> {
//     use subprocess::{Exec,Redirection};
//     let out = Exec::shell(cmd)
//         .stdout(Redirection::Pipe)
//         .capture().map_err(|e|e.to_string())?
//         .stdout_str();
//     return Ok(out.trim().to_owned());
// }
// use nom::{multispace, rest_s};
// named!(_cpu_name<&str, String>,
//   do_parse!(
//     tag!("Name") >> multispace >> out: rest_s >>
//     ( out.to_owned() )
//   )
// );

fn mk_cpu_ram_str(cpu: &str, ram: &str, system: &mut sysinfo::System) -> String {
  let ram_used = get_ram_used(system);
  [cpu.to_owned(), ram.to_owned(), ram_used].join("\n")
}

fn cpu_name() -> String {
  use cupid;
  let info = cupid::master();
  match info {
    Some(x) => {
      match x.brand_string() {
        Some(x) => { ["CPU: ".to_owned(), x.to_owned()].join("") }
        _ => { "Could not get CPU Name".to_owned() }
      }
    }
    _ => { "Could not get CPU Name".to_owned() }
  }
}

fn get_ram_total(system: &mut sysinfo::System) -> String {
  system.refresh_all();
  let ram_total = ((system.get_total_memory() as f32 / 1024.0) / 1024.0).round();
  format!("Total Memory: {} GB", ram_total )
}

fn get_ram_used(system: &mut sysinfo::System) -> String {
  system.refresh_all();
  let ram_used = (((system.get_used_memory() as f32 / 1024.0) / 1024.0) * 1000.0).round() / 1000.0;
  format!("Used Memory : {:.3} GB", ram_used )
}
