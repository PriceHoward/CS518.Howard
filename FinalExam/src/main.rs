mod enums;
mod reader;
mod ast;
mod parser;
mod interpret;
mod display;

use pixels::{Pixels, SurfaceTexture};
use winit::{
    event::{Event, WindowEvent},
    event_loop::{ControlFlow, EventLoop},
    window::WindowBuilder,
};

const WIDTH: u32 = 640;
const HEIGHT: u32 = 480;

fn main() {
    let source = "set x 50\npen 1\ndotimes 4 {\n  forward x\n  turn 90\n}";

    let tokens = reader::Reader::new(source).tokenize().unwrap();
    let ast = parser::Parser::new(tokens).parse().unwrap();

    let mut turtle = interpret::TurtleState::new(320.0, 240.0);
    let mut disp = display::Display::new(WIDTH, HEIGHT);

    {
        let mut draw_line = |x1: f64, y1: f64, x2: f64, y2: f64| {
            disp.draw_line(x1, y1, x2, y2);
        };
        interpret::interpret(&ast, &mut turtle, &mut draw_line).unwrap();
    }


    let event_loop = EventLoop::new();
    let window = WindowBuilder::new()
        .with_title("Turtle Graphics")
        .with_inner_size(winit::dpi::LogicalSize::new(WIDTH, HEIGHT))
        .build(&event_loop)
        .unwrap();

    let mut pixels = {
        let size = window.inner_size();
        let surface = SurfaceTexture::new(size.width, size.height, &window);
        Pixels::new(WIDTH, HEIGHT, surface).unwrap()
    };

    event_loop.run(move |event, _, control_flow| {
        *control_flow = ControlFlow::Wait;

        match event {
            Event::RedrawRequested(_) => {
                disp.blit(pixels.frame_mut());

                if pixels.render().is_err() {
                    *control_flow = ControlFlow::Exit;
                }
            }

            Event::WindowEvent {
                event: WindowEvent::CloseRequested,
                ..
            } => {
                *control_flow = ControlFlow::Exit;
            }

            Event::MainEventsCleared => {
                window.request_redraw();
            }

            _ => {}
        }
    });
}