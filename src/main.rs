
#[macro_use]
extern crate glium;

use glium::{Surface, glutin::{event::{Event, WindowEvent, VirtualKeyCode, ElementState, MouseButton, MouseScrollDelta}, event_loop::{ControlFlow, EventLoop}, dpi::{PhysicalPosition, PhysicalSize, LogicalSize}, window::WindowBuilder, ContextBuilder}, VertexBuffer, IndexBuffer, index::PrimitiveType, Program, DrawParameters, Display, texture::{MipmapsOption, SrgbTexture1d, SrgbFormat}, uniforms::SamplerWrapFunction};



#[derive(Copy, Clone)]
pub struct Vec2 { pub p: [f32; 2] }
impl Vec2 {
	pub fn new(x: f32, y: f32) -> Self { Self { p: [x, y] }}
}
implement_vertex!(Vec2, p);



fn main() {
	let event_loop = EventLoop::new();
	let wb = WindowBuilder::new().with_inner_size(LogicalSize::new(1024.0, 768.0));
	let cb = ContextBuilder::new();
	let display = Display::new(wb, cb, &event_loop).unwrap();
	let PhysicalSize { mut width, mut height } = display.gl_window().window().inner_size();
	
	
	let default_vertex_shader = "
		#version 150
		in vec2 p;
		out vec2 pos;
		void main() {
			pos = (p + 1.0) * 0.5;
			gl_Position = vec4(p, 0.0, 1.0);
		}
	";
	
	let mandelbrot_single = Program::from_source(&display, &default_vertex_shader, include_str!("frag1.glsl"), None).unwrap();
	let mandelbrot_double = Program::from_source(&display, &default_vertex_shader, include_str!("frag2.glsl"), None).unwrap();
	let julia_single = Program::from_source(&display, &default_vertex_shader, include_str!("frag3.glsl"), None).unwrap();
	let julia_double = Program::from_source(&display, &default_vertex_shader, include_str!("frag4.glsl"), None).unwrap();
	
	let default_vertex_buffer = VertexBuffer::new(&display, &[Vec2::new(-1.0, -1.0), Vec2::new(1.0, -1.0), Vec2::new(1.0, 1.0), Vec2::new(-1.0, 1.0)]).unwrap();
	let default_index_buffer = IndexBuffer::new(&display, PrimitiveType::TrianglesList, &[0u16, 1, 2, 0, 2, 3]).unwrap();
	
	
	//let mut main_texture = SrgbTexture2d::empty(&display, width, height).unwrap();
	
	
	let colors = [
		(0.0   ,	  0.0,	  7.0,	100.0),
		(0.16  ,	 32.0,	107.0,	203.0),
		(0.42  ,	237.0,	255.0,	255.0),
		(0.6425,	255.0,	170.0,	  0.0),
		(0.8575,	  0.0,	  2.0,	  0.0),
		(1.0   ,	  0.0,	  7.0,	100.0),
	];
	
	let cycle_iters = 100;
	
	let color_lut: Vec<(u8, u8, u8)> = (0..cycle_iters).map(|x| {
		let t = x as f32 / cycle_iters as f32;
		for i in 1..colors.len() {
			if t < colors[i].0 {
				let u = (t - colors[i-1].0) / (colors[i].0 - colors[i-1].0);
				let s = u;
				return (
					(colors[i-1].1 + (colors[i].1 - colors[i-1].1) * s) as u8,
					(colors[i-1].2 + (colors[i].2 - colors[i-1].2) * s) as u8,
					(colors[i-1].3 + (colors[i].3 - colors[i-1].3) * s) as u8,
				);
			}
		}
		(0, 0, 0)
	}).collect();
	
	let gradient_texture = SrgbTexture1d::with_format(&display, color_lut, SrgbFormat::U8U8U8, MipmapsOption::NoMipmap).unwrap();
	
	
	
	let mut previous_mouse_pos = PhysicalPosition::<f64>::new(0.0, 0.0);
	let mut mouse_down = false;
	
	
	
	let mut x = 0.0f64;
	let mut y = 0.0f64;
	let mut z = 4.0f64;
	
	let mut mx = x;
	let mut my = y;
	let mut mz = z;
	
	let mut iterations = 250;
	let mut double_precision = false;
	let mut julia_mode = false;
	
	
	event_loop.run(move |ev, _, control_flow| {
		match ev {
			Event::WindowEvent { event, .. } => match event {
				WindowEvent::KeyboardInput { input, .. } => {
					if let Some(code) = input.virtual_keycode {
						let state = match input.state {
							ElementState::Pressed => true,
							ElementState::Released => false
						};
						match code {
							VirtualKeyCode::Up => if state { z /= 1.25 }
							VirtualKeyCode::Down => if state { z *= 1.25 }
							VirtualKeyCode::W => if state { y += z * 0.05 }
							VirtualKeyCode::S => if state { y -= z * 0.05 }
							VirtualKeyCode::A => if state { x -= z * 0.05 }
							VirtualKeyCode::D => if state { x += z * 0.05 }
							
							VirtualKeyCode::Space => if state {
								x += (previous_mouse_pos.x / width as f64 - 0.5) * z;
								y -= (previous_mouse_pos.y - 0.5 * height as f64) / width as f64 * z;
							}
							
							VirtualKeyCode::Key1 => iterations = 250,
							VirtualKeyCode::Key2 => iterations = 500,
							VirtualKeyCode::Key3 => iterations = 1000,
							VirtualKeyCode::Key4 => iterations = 2000,
							VirtualKeyCode::Key5 => iterations = 4000,
							VirtualKeyCode::Key6 => iterations = 8000,
							VirtualKeyCode::Key7 => iterations = 16000,
							VirtualKeyCode::Key8 => iterations = 32000,
							VirtualKeyCode::Key9 => iterations = 64000,
							VirtualKeyCode::Key0 => iterations = 128000,
							
							VirtualKeyCode::Comma => double_precision = false,
							VirtualKeyCode::Period => double_precision = true,
							VirtualKeyCode::J => if state && !julia_mode {
								julia_mode = true;
								mx = x + (previous_mouse_pos.x / width as f64 - 0.5) * z;
								my = y - (previous_mouse_pos.y - 0.5 * height as f64) / width as f64 * z;
								mz = z;
								x = 0.0;
								y = 0.0;
								z = 4.0;
							}
							VirtualKeyCode::M => if state && julia_mode {
								julia_mode = false;
								x = mx;
								y = my;
								z = mz;
							}
							
							_ => ()
						}
						
						if state {
							display.gl_window().window().request_redraw();
						}
						
					}
				}
				WindowEvent::MouseInput { state, button, .. } => {
					if let MouseButton::Left = button {
						mouse_down = match state {
							ElementState::Pressed => true,
							ElementState::Released => false
						};
					}
				}
				WindowEvent::CursorMoved { position, .. } => {
					if mouse_down {
						let dx = position.x - previous_mouse_pos.x;
						let dy = position.y - previous_mouse_pos.y;
						x -= dx * z / width as f64;
						y += dy * z / width as f64;
						display.gl_window().window().request_redraw();
					}
					previous_mouse_pos = position;
				}
				WindowEvent::MouseWheel { delta, .. } => {
					let distance = match delta {
						MouseScrollDelta::LineDelta(_, dy) => dy as f64,
						MouseScrollDelta::PixelDelta(PhysicalPosition { y, .. }) => y / 20.0
					};
					
					x += (previous_mouse_pos.x / width as f64 - 0.5) * z;
					y -= (previous_mouse_pos.y - 0.5 * height as f64) / width as f64 * z;
					z *= 1.0 - 0.1 * distance;
					x -= (previous_mouse_pos.x / width as f64 - 0.5) * z;
					y += (previous_mouse_pos.y - 0.5 * height as f64) / width as f64 * z;
					
					display.gl_window().window().request_redraw();
				}
				WindowEvent::Resized(new_size) => {
					width = new_size.width;
					height = new_size.height;
					//main_texture = SrgbTexture2d::empty(&display, width, height).unwrap();
				}
				WindowEvent::CloseRequested => {
					*control_flow = ControlFlow::Exit;
				}
				_ => ()
			}
			Event::RedrawEventsCleared => {
				//display.gl_window().window().request_redraw();
			}
			Event::RedrawRequested(_) => {
				
				let gradient_sampler = gradient_texture.sampled().wrap_function(SamplerWrapFunction::Repeat);
				
				let mut target = display.draw();
				target.clear_color_and_depth((0.0, 0.0, 0.0, 1.0), 1.0);
				
				match (double_precision, julia_mode) {
					(false, false) => target.draw(&default_vertex_buffer, &default_index_buffer, &mandelbrot_single, &uniform! {
						aspect_ratio: height as f32 / width as f32,
						x: x as f32, y: y as f32, zoom: z as f32,
						iterations: iterations, cycle_iters: cycle_iters, gradient: gradient_sampler
					}, &DrawParameters::default()).unwrap(),
					
					(true, false) => target.draw(&default_vertex_buffer, &default_index_buffer, &mandelbrot_double, &uniform! {
						aspect_ratio: height as f32 / width as f32,
						x: x, y: y, zoom: z,
						iterations: iterations, cycle_iters: cycle_iters, gradient: gradient_sampler
					}, &DrawParameters::default()).unwrap(),
					
					(false, true) => target.draw(&default_vertex_buffer, &default_index_buffer, &julia_single, &uniform! {
						aspect_ratio: height as f32 / width as f32,
						x: x as f32, y: y as f32, zoom: z as f32,
						iterations: iterations, cycle_iters: cycle_iters, gradient: gradient_sampler,
						cx: mx as f32, cy: my as f32
					}, &DrawParameters::default()).unwrap(),
					
					(true, true) => target.draw(&default_vertex_buffer, &default_index_buffer, &julia_double, &uniform! {
						aspect_ratio: height as f32 / width as f32,
						x: x, y: y, zoom: z,
						iterations: iterations, cycle_iters: cycle_iters, gradient: gradient_sampler,
						cx: mx, cy: my
					}, &DrawParameters::default()).unwrap(),
				}
				
				
				target.finish().unwrap();
				
			}
			_ => ()
		}
	});
}
