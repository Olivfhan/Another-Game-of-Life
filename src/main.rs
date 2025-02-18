#[macro_use]
extern crate glium;

use std::{thread, time};
use clearscreen::clear;
use glium::Surface;
use winit;

pub mod gameoflifeparsing;
pub mod gameoflife;


struct Shape{
    vertex_vector: Vec<Vertex>,
    indices_vector: Vec<u32>,
}/*

pub enum Cell {
   shape: Shape,
   no_shape: bool,
}*/

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32;2],
}


fn offset_square(x:f32,y:f32) -> Vec<f32> {
    let offset_vector = vec!(1.0-x/16.0,1.0-y/16.0);
    return offset_vector
}

fn to_list_of_squares(chunk: [u32;32]) -> Vec<Vec<f32>> {
    let mut chunk_shapes: Vec<Vec<f32>> = vec!();
    let mut row_counter: u32 = 0;
    for row in chunk {
        for i in 0..32 {
            if (row>>i)&1==1 {
                chunk_shapes.push(offset_square(i as f32, row_counter as f32));
            }
        }
        row_counter+=1;
    }
    return chunk_shapes
}


fn main() {

    implement_vertex!(Vertex, position);
    //movement_keys::movement_keys(-0.1,50,"../resources/zippy2.png","../resources/zippy2.png");
    //circle_over_bg::circle_over_bg(0.5,-0.1,50,"../resources/yo.png","../resources/cv.png");
    
    let twenty_millis = time::Duration::from_millis(200);
    let mut initial: [u32;32] = gameoflifeparsing::format_initial();
    //gameoflifeparsing::output_to_terminal(initial);
 
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("")
        .build(&event_loop);
    
    let vertex_vector = vec![
        Vertex { position: [-1.0,-1.0]},
        Vertex { position: [ 1.0, -1.0]},
        Vertex { position: [ 1.0,1.0] },

        Vertex { position: [ 1.0,1.0]},
        Vertex { position: [-1.0,1.0]},
        Vertex { position: [-1.0,-1.0]},
    ];

    let positions = glium::VertexBuffer::new(&display, &vertex_vector).unwrap();
    //let normals = glium::VertexBuffer::new(&display, &normals_vector).unwrap();
    let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

                                        //glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                        //&indices_vector).unwrap();
         
    
    let vertex_shader_src = r#"
        #version 400

        in vec2 position;
        uniform float x;
        uniform float y;
        uniform float resize_factor;
        out vec2 vertex_color;
        void main() {

            vertex_color = vec2(position.x,position.y);
            gl_Position = vec4(position.x*resize_factor+x,position.y*resize_factor+y,0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 400
        out vec4 color;
        in vec2 vertex_color;
        void main() {
            color = vec4(1.0, 1.0,1.0, 1.0);

        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                            None).unwrap();
    let mut target = display.draw();
    target.clear_color(0.0, 0.0, 0.0, 1.0);
    target.finish().unwrap();
    
    let resize_factor: f32 = 1.0/32.0;

    event_loop.run(move |ev, window_target| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                // We now need to render everyting in response to a RedrawRequested event due to the animation
                winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color(0.0, 0.0, 0.0, 1.0);
                    thread::sleep(twenty_millis);
                    initial = gameoflife::evolve(initial);
                    let list_offsets = to_list_of_squares(initial);
                    for offset in &list_offsets {
                        let uniforms = uniform! { x: offset[0], y: offset[1], resize_factor:resize_factor };
                        target.draw(&positions, &indices, &program, &uniforms,
                                    &Default::default()).unwrap();
                    }
                    target.finish().unwrap();
                },
                // Because glium doesn't know about windows we need to resize the display
                // when the window's size has changed.
                winit::event::WindowEvent::Resized(window_size) => {
                    display.resize(window_size.into());
                },
                _ => (),
            },
            // By requesting a redraw in response to a AboutToWait event we get continuous rendering.
            // For applications that only change due to user input you could remove this handler.
            winit::event::Event::AboutToWait => {
                window.request_redraw();
            },
            _ => (),
        }
    })
    .unwrap();


}
