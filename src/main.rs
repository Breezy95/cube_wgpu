use std::sync::Arc;
use std::time;

use wgpu::SurfaceError;
use winit::dpi::PhysicalSize;
use winit::event_loop::{ControlFlow, EventLoop};
use winit::event::*;
//#[cfg(target_arch = "wasm32")]
//use winit::platform::pump_events::EventLoopExtPumpEvents;
use winit::window::WindowBuilder;

mod state;
pub mod wasm_driver;
pub mod wgpu_helpers;
mod camera;





fn main() {
    env_logger::init();
    let event_loop: EventLoop<()> = EventLoop::new().unwrap();
    let window = Arc::new(WindowBuilder::new().build(&event_loop).unwrap());
    window.set_title("cube with distinct face colors");
    let window_clone = window.clone();
    let mut state = pollster::block_on(state::State::new(&window));
    let render_start_time = time::Instant::now();
    let _ = event_loop.run(move |event, control_flow| {
        match event {
            Event::AboutToWait => {window_clone.request_redraw();}
            Event::WindowEvent {
                ref event,
                window_id,
            } if window_id == window_clone.id() => {
                if !state.input(event) {
                    match event { WindowEvent::CloseRequested
                        | WindowEvent::KeyboardInput {..} => {
                            control_flow.exit();
                        }
                        WindowEvent::Resized(physical_size) => {
                            state.resize(*physical_size);
                        }
                        WindowEvent::ScaleFactorChanged { scale_factor,.. } => {
                            // Use scale factor to adjust the size and request the inner size change
                            let new_size = PhysicalSize {
                                width: (*scale_factor * state.size.width as f64) as u32,
                                height: (*scale_factor * state.size.height as f64) as u32,
                            };
                            state.resize(new_size);
                            
                        }
                        WindowEvent::RedrawRequested => {
                            let current_time = std::time::Instant::now();
                            let time_diff = current_time - render_start_time;
                            state.update(time_diff);
                            match state.render() {
                                Ok(_) => {}
                                Err(SurfaceError::Lost) => state.resize(state.size),
                                Err(SurfaceError::OutOfMemory) => {
                                    control_flow.exit();
                                }
                                Err(e) => eprintln!("{:?}", e),
                            }
                        }
                        _ => {}
                    }
                }
            }
            _ => {}
        }
    }

);
}

