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
extern crate systemstat;
// extern crate subprocess;

use {
  gl::*,
  glutin::{
    // dpi::*,
    event::{Event, WindowEvent, Event::DeviceEvent, },
    event_loop::{ControlFlow, EventLoop, },
  },
  sysinfo::SystemExt,
  systemstat::{System, Platform},
  // crate::{
  //   // render::{ * },
  //   // shader::{ Shader, },
  // },
};

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
  
  // Specify OpenGL version
  let gl_request = glutin::GlRequest::Specific(glutin::Api::OpenGl, (4, 3));
  let gl_profile = glutin::GlProfile::Core;
  // Create a window
  let el = EventLoop::new();
  let wb = glutin::window::WindowBuilder::new()
    .with_title("RaumEn SysInfo")
    .with_inner_size(glutin::dpi::LogicalSize::new(1024.0, 768.0))
    .with_maximized(false);
  let windowed_context = glutin::ContextBuilder::new()
    .with_gl(gl_request)
    .with_gl_profile(gl_profile)
    .build_windowed(wb, &el)
    .unwrap();
  
  let windowed_context = unsafe { windowed_context.make_current().unwrap() };
  // Set up OpenGL
  unsafe {
    load_with(|symbol| windowed_context.context().get_proc_address(symbol) as *const _);
    ClearColor(0.0, 1.0, 0.0, 1.0);
  }
  
  let mut render_mgr = RenderMgr::new();
  let mut mgr = render_mgr.mgr.clone();
  
  let mut system = sysinfo::System::new();
  
  let cpu = cpu_name();
  let ram = get_ram_total(&mut system);
  let cpu_ram = mk_cpu_ram_str(&cpu, &ram, &mut system);
  let hdd = get_hdd();
  
  let mut fps: f32 = 30.0;
  let mut sec = 0.0;
  
  { // Here, we're getting the size of the window in pixels
    // and passing it to the update_size() method. It in turn
    // updates the Projection Matrix and passes that to 
    // ALL THE SHADERS, so if you add a SHADER, you need
    // to REMEMBER to add that shader to the update_size()
    // method near the bottom of this file.
    // let dpi = windowed_context.window().get_hidpi_factor();
    let size: glutin::dpi::PhysicalSize<u32> = windowed_context.window().inner_size();
    mgr.update_size(size.into());
  }
  {
    let _textmgr = mgr.clone().textmgr.take().unwrap();
    let mut textmgr = _textmgr.lock().unwrap();
    textmgr.add_font(mgr.clone(), "pirate");
    textmgr.add_font(mgr.clone(), "sans");
    textmgr.new_text(mgr.clone(), "Title", "SysInfo", "pirate", 4.0, 0.0, 0.0, 1.0, true, true);
    textmgr.new_text(mgr.clone(), "CPU RAM HDD", &[cpu_ram, hdd.clone()].join(""), "sans", 2.0, 0.0, 0.4, 1.0, true, true);
    textmgr.new_text(mgr.clone(), "FPS", "FPS: 0.0", "sans", 1.5, 0.0, 0.0, 0.3, false, true);
  }
  
  // Game loop!
  println!("Starting main loop.");
  el.run(move |event, _, control_flow| {
    // *control_flow = ControlFlow::Wait;
    {
      mgr.handler_do(|handler| {
        handler.timer.tick();
        handler.reset_delta();
      });
    }
    
    match event {
      Event::LoopDestroyed => {
        println!("Cleaning Up...");
        // Clean up
        render_mgr.clean_up();
        return
      }
      Event::WindowEvent { event, .. } => match event {
        WindowEvent::CloseRequested => {
          *control_flow = ControlFlow::Exit
        },
        WindowEvent::Resized(size) => {
          windowed_context.resize(size);
          mgr.update_size(size.into());
        },
        _ => { mgr.handler_do(|handler| { handler.window_event(&event); }); }
      },
      DeviceEvent{ event, ..} => { mgr.handler_do(|handler| { handler.device_event(&event); }); }
      Event::NewEvents( _time ) => {
        // Emitted when new events arrive from the OS to be processed.
        // 
        // This event type is useful as a place to put code that should be done before you start processing events, such as 
        // updating frame timing information for benchmarking or checking the StartCause][crate::event::StartCause] to see 
        // if a timer set by [ControlFlow::WaitUntil has elapsed.
      }
      Event::MainEventsCleared => {
        // Emitted when all of the event loop's input events have been processed and redraw processing is about to begin.
        
        // This event is useful as a place to put your code that should be run after all state-changing events have been 
        // handled and you want to do stuff (updating state, performing calculations, etc) that happens as the "main body" 
        // of your event loop. 
        // If your program draws graphics, it's usually better to do it in response to Event::RedrawRequested, which gets 
        // emitted immediately after this event.
      }
      Event::RedrawRequested(_) => {
        // Emitted after MainEventsCleared when a window should be redrawn.
        
        // This gets triggered in two scenarios:
        
        // - The OS has performed an operation that's invalidated the window's contents (such as resizing the window).
        // - The application has explicitly requested a redraw via Window::request_redraw.
        
        // During each iteration of the event loop, Winit will aggregate duplicate redraw requests into a single event, 
        // to help avoid duplicating rendering work.
      }
      Event::RedrawEventsCleared => {
        // Emitted after all RedrawRequested events have been processed and control flow is about to be taken away from 
        // the program. If there are no RedrawRequested events, it is emitted immediately after MainEventsCleared.
        
        // This event is useful for doing any cleanup or bookkeeping work after all the rendering tasks have been completed.
      }
      e => println!("Other Event:\n{:?}", e)
    }
    // *** Do per frame calculations such as movement
    {
      let handler = mgr.handler.lock().unwrap();
      fps = handler.timer.fps;
      sec += handler.timer.delta;
    }
    if sec >= 1.0 {
      sec -= 1.0;
      let cpu_ram = mk_cpu_ram_str(&cpu, &ram, &mut system);
      let _textmgr = mgr.clone().textmgr.take().unwrap();
      let mut textmgr = _textmgr.lock().unwrap();
      textmgr.update_text(mgr.clone(), "CPU RAM HDD", &[cpu_ram, hdd.clone()].join(""));
      textmgr.update_text(mgr.clone(), "FPS", &format!("FPS: {:.3}", (fps * 1000.0).round() / 1000.0 ) );
    }
    
    // *** Drawing phase
    render_mgr.render();
    
    // _fbo_final.blit_to_screen(&world);
    
    
    // Write the new frame to the screen!
    windowed_context.swap_buffers().unwrap();
  });
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
  // use cupid;
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

fn get_hdd() -> String {
  let sys = System::new();
  let mut out = String::new();
  match sys.mounts() {
    Ok(mounts) => {
      for mount in mounts.iter() {
        out = format!("{}\n{} Size: {}; Free: {}",
          out, mount.fs_mounted_on, mount.total, mount.avail);
      }
    }
    Err(x) => println!("\nMounts: error: {}", x)
  }
  out
}
