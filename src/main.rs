extern crate nalgebra;
#[macro_use] extern crate glium;
extern crate time;


use glium::{DisplayBuild, Surface};
use nalgebra::{PerspectiveMatrix3, Vector3, Matrix4, Point3, Isometry3, ToHomogeneous, Translation};

#[derive(Clone, Copy, Debug)]
struct Vert {
    position: [f32;3],
}

implement_vertex!(Vert, position);

fn main() {
    let mut window = glium::glutin::WindowBuilder::new()
        .with_dimensions(1280, 720)
        .with_depth_buffer(24)
        .with_title(format!("Hello world"))
        .build_glium()
        .expect("Failed to open window");

    let tr = [1f32, 1.0, 0.0];
    let tl = [-1f32, 1.0, 0.0];
    let bl = [-1f32, -1.0, 0.0];
    let br = [1f32, -1.0, 0.0];

    let data = &[
        Vert { position: tr }, 
        Vert { position: tl }, 
        Vert { position: bl }, 
               
        Vert { position: tr }, 
        Vert { position: bl }, 
        Vert { position: br },
    ];
    let box_vb = glium::vertex::VertexBuffer::new(&window, data).unwrap();

    let v_shader = "
        #version 150
        
        uniform mat4 perspective;
        uniform mat4 modelview;

        in vec3 position;

        void main() {
            gl_Position = perspective * modelview * vec4(position, 1.0);
        }
    ";
    let f_shader = "
        #version 150

        void main() {
            gl_FragColor = vec4(0.0, 0.0, 0.0, 1.0);
        }
    ";

    let box_shader = glium::program::Program::from_source(&window, v_shader, f_shader, None).unwrap();

    let params = glium::DrawParameters {
        backface_culling: glium::draw_parameters::BackfaceCullingMode::CullClockwise,
        depth: glium::Depth {
            test: glium::draw_parameters::DepthTest::IfLess,
            write: true,
            .. Default::default()
        },
        .. Default::default()
    };


    use std::f32::consts::FRAC_PI_4;
    const PI_4: f32 = FRAC_PI_4;
    const UP: Vector3<f32> = Vector3 {x: 0.0, y: 1.0, z: 0.0};

    let position = Point3::new(0.0, 0.0, 10.0);
    let target = Point3::new(0.0, 0.0, 0.0);
    let view = Isometry3::look_at_rh(&position, &target, &UP).to_homogeneous();
    let perspective = PerspectiveMatrix3::new(1.78, PI_4, 1.0, 100.0).to_matrix();

    let mut model = Isometry3::new(Vector3::new(5.0, 0.0, 0.0), Vector3::new(0.0, 0.0, 0.0));

    let mut curr_t = time::PreciseTime::now();
    let mut prev_t: time::PreciseTime;
    let mut dt = time::Duration::milliseconds(2);

    let mut progress = 0.0;
    loop {
        if do_exit(&mut window) {
            break;
        }

        progress = progress + 0.001*(dt.num_milliseconds() as f32);
        model.set_translation(((1.0-progress)*Point3::new(5.0, 0.0, 0.0) + progress*Vector3::new(-5.0, 0.0, 0.0)).to_vector());

        let mut frame = window.draw();
        frame.clear_color_and_depth((0.0, 1.0, 0.0, 1.0), 1.0);

        let indices = glium::index::NoIndices(glium::index::PrimitiveType::TrianglesList);
        let uniforms = uniform! {
            perspective: perspective.as_ref().clone(),
            modelview: (model.to_homogeneous()*view).as_ref().clone(),
        };
        frame.draw(&box_vb, &indices, &box_shader, &uniforms, &params).unwrap();
        frame.finish().unwrap();
        window.swap_buffers().unwrap();

        prev_t = curr_t;
        curr_t = time::PreciseTime::now();
        dt = prev_t.to(curr_t);
    }
}

fn do_exit(window: &mut glium::Display) -> bool {
    for event in window.poll_events() {
        use glium::glutin::Event;
        use glium::glutin::VirtualKeyCode as KC;
        match event {
            Event::Closed => {
                return true;
            },
            Event::KeyboardInput(state, _, key) => {
                if state == glium::glutin::ElementState::Pressed {
                    match key.unwrap() {
                        KC::Escape => {
                            return true;
                        },
                        _ => ()
                    }
                }
            }
            _ => ()
        }
    }
    return false;
}
