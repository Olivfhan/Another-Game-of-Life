use glium::Surface;
use winit;

#[derive(Copy, Clone)]
struct Vertex {
    position: [f32; 2],
    tex_coords: [f32; 2],
}


pub fn circle_over_bg(circle_size:f32,spin_velocity:f32,n_of_triangles:u32,image_path:&str,bg_path:&str) {

    implement_vertex!(Vertex, position, tex_coords);
 
    let event_loop = winit::event_loop::EventLoopBuilder::new()
        .build()
        .expect("event loop building");
    let (window, display) = glium::backend::glutin::SimpleWindowBuilder::new()
        .with_title("")
        .build(&event_loop);
    
    let image = image::load(std::io::Cursor::new(&std::fs::read(image_path).unwrap()),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture = glium::texture::Texture2d::new(&display, image).unwrap();
    let image = image::load(std::io::Cursor::new(&std::fs::read(bg_path).unwrap()),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture2 = glium::texture::Texture2d::new(&display, image).unwrap();

    let image = image::load(std::io::Cursor::new(&std::fs::read("../resources/cv2.png").unwrap()),
                            image::ImageFormat::Png).unwrap().to_rgba8();
    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba_reversed(&image.into_raw(), image_dimensions);
    let texture3 = glium::texture::Texture2d::new(&display, image).unwrap();



    let mut vertex_vector = Vec::new();
    //let mut normals_vector = Vec::new();
    let mut indices_vector = Vec::new();
    vertex_vector.push(Vertex { position: [0.0,0.0], tex_coords: [0.5,0.5]});
    let alpha: f32 = std::f32::consts::PI*2.0/((n_of_triangles-2) as f32);
    let mut beta: f32 = 0.0;
    let mut x2: f32 = 0.0;
    let mut y2: f32 = 0.0;
    for i in 1u32..n_of_triangles {
        x2 = beta.sin();
        y2 = beta.cos();
        beta+=alpha;
        vertex_vector.push(Vertex { position: [x2,y2], tex_coords: [(x2+1.0)/2.0,(y2+1.0)/2.0]});
        
        indices_vector.push(0);
        indices_vector.push(i);
        indices_vector.push(i+1);
    }
    //vertex_vector.push(Vertex { position: [x2,y2], tex_coords: [x2,y2]});    
    
    let shape = vec![
        Vertex { position: [-1.0,-1.0], tex_coords: [0.0, 0.0] },
        Vertex { position: [ 1.0, -1.0], tex_coords: [1.0, 0.0] },
        Vertex { position: [ 1.0,1.0], tex_coords: [1.0, 1.0] },

        Vertex { position: [ 1.0,1.0], tex_coords: [1.0, 1.0] },
        Vertex { position: [-1.0,1.0], tex_coords: [0.0, 1.0] },
        Vertex { position: [-1.0,-1.0], tex_coords: [0.0, 0.0] },
    ];

    let positions = glium::VertexBuffer::new(&display, &vertex_vector).unwrap();
    //let normals = glium::VertexBuffer::new(&display, &normals_vector).unwrap();
    let indices = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                        &indices_vector).unwrap();
         //glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let positions2 = glium::VertexBuffer::new(&display, &shape).unwrap();
    let indices2 = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);

    let positions3 = glium::VertexBuffer::new(&display, &vertex_vector).unwrap();
    //let normals = glium::VertexBuffer::new(&display, &normals_vector).unwrap();
    let indices3 = glium::IndexBuffer::new(&display, glium::index::PrimitiveType::TrianglesList,
                                        &indices_vector).unwrap();
         //glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);


    let vertex_shader_src = r#"
        #version 140

        in vec2 position;
        in vec2 tex_coords;
        out vec2 v_tex_coords;
        uniform mat4 matrix_rot;
        uniform mat4 matrix_resize;

        void main() {
            v_tex_coords = tex_coords;
            gl_Position = matrix_resize * matrix_rot * vec4(position,0.0, 1.0);
        }
    "#;

    let fragment_shader_src = r#"
        #version 140
        in vec2 v_tex_coords;
        out vec4 color;
        uniform sampler2D tex;
        void main() {
            color = texture(tex,v_tex_coords);
        }
    "#;

    let program = glium::Program::from_source(&display, vertex_shader_src, fragment_shader_src,
                                            None).unwrap();
    let mut target = display.draw();
    target.clear_color(1.0, 1.0, 1.0, 1.0);

    let matrix = [
        [1.0, 0.0, 0.0, 0.0],
        [0.0, 1.0, 0.0, 0.0],
        [0.0, 0.0, 1.0, 0.0],
        [0.0, 0.0, 0.0, 1.0f32]
    ];

    let mut t:f32 = 0.0;

    let mut x = t.sin()*0.5;
    let mut y = t.cos()*0.5;
    let uniforms = uniform! {matrix_rot: matrix,matrix_resize: matrix,tex: &texture, };
    let uniforms2 = uniform! {matrix_rot: matrix,matrix_resize: matrix, tex:&texture2, };
    let uniforms3 = uniform! {matrix_rot: matrix,matrix_resize: matrix, tex:&texture3, };

    target.draw(&positions2, &indices2, &program, &uniforms2,
                                &Default::default()).unwrap();

    target.draw(&positions3, &indices3, &program, &uniforms3, &Default::default()).unwrap();
    target.draw(&positions, &indices, &program, &uniforms,
                &Default::default()).unwrap();
    target.finish().unwrap();
 
    event_loop.run(move |ev, window_target| {
        match ev {
            winit::event::Event::WindowEvent { event, .. } => match event {
                winit::event::WindowEvent::CloseRequested => {
                    window_target.exit();
                },
                // We now need to render everyting in response to a RedrawRequested event due to the animation
                winit::event::WindowEvent::RedrawRequested => {
                    let mut target = display.draw();
                    target.clear_color(1.0, 1.0, 1.0, 1.0);
                    t+=spin_velocity;
                    x=t.sin()*0.5;
                    y=t.cos()*0.5;
                    let matrix_rot = [
                        [1.0,0.0, 0.0, 0.0],
                        [0.0, 1.0, 0.0, 0.0],
                        [0.0, 0.0, 1.0, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32]
                    ];
                    let matrix_resize = [
                        [ circle_size, 0.0, 0.0, 0.0],
                        [0.0, circle_size, 0.0, 0.0],
                        [0.0, 0.0, circle_size, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32]
                    ];

                    let uniforms = uniform! { matrix_rot: matrix_rot,matrix_resize:matrix_resize,tex: &texture, };

                    let matrix_resize = [
                        [ circle_size+0.03, 0.0, 0.0, 0.0],
                        [0.0, circle_size+0.03, 0.0, 0.0],
                        [0.0, 0.0, circle_size+0.03, 0.0],
                        [0.0, 0.0, 0.0, 1.0f32]
                    ];

                    let uniforms3 = uniform! { matrix_rot: matrix_rot,matrix_resize:matrix_resize,tex: &texture3, };


                    target.draw(&positions2, &indices2, &program, &uniforms2,
                                &Default::default()).unwrap();

                    

                    target.draw(&positions3, &indices3, &program, &uniforms3, &Default::default()).unwrap();

                    target.draw(&positions, &indices, &program, &uniforms,
                                &Default::default()).unwrap();

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
