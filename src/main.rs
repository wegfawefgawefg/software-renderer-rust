mod mm;
use raylib::prelude::*;

fn main() {
    let gameboy_dims = mm::Vec2::new(160.0, 144.0);
    let dims = gameboy_dims * 5.0;


    //  setup raylib window
    let (mut rl, thread) = raylib::init()
        .size(dims.x as i32, dims.y as i32)
        //set resizable
        .resizable()
        .title("software renderer")
        .build();

    //  make low res texture as render target
    let mut target = rl.load_render_texture(
        // pass in thread
        &thread,
        gameboy_dims.x as u32, gameboy_dims.y as u32);
    



    rl.set_target_fps(60);

    let mut perspective = mm::Mat4::perspective(
        45.0 * std::f32::consts::PI / 180.0,
        800.0 / 450.0,
        0.01,
        100.0,
    );

    // create a cube of size 1 by adding all 8 verticies of a cube
    let mut cube: Vec<mm::Vec3> = Vec::new();
    cube.push(mm::Vec3 { x: -1.0, y: 1.0, z: 1.0 });
    cube.push(mm::Vec3 { x: 1.0, y: 1.0, z: 1.0 });
    cube.push(mm::Vec3 { x: 1.0, y: -1.0, z: 1.0 });
    cube.push(mm::Vec3 { x: -1.0, y: -1.0, z: 1.0 });
    cube.push(mm::Vec3 { x: -1.0, y: 1.0, z: -1.0 });
    cube.push(mm::Vec3 { x: 1.0, y: 1.0, z: -1.0 });
    cube.push(mm::Vec3 { x: 1.0, y: -1.0, z: -1.0 });
    cube.push(mm::Vec3 { x: -1.0, y: -1.0, z: -1.0 });


    
    // make run loop
    while !rl.window_should_close() {
        // just clear the screen
        let mut d = rl.begin_drawing(&thread);
        d.clear_background(Color::BLACK);
        
        // draw the mouse
        let mouse_pos = d.get_mouse_position();
        d.draw_circle_v(mouse_pos, 10.0, Color::RED);
        // quit if escape is pressed
        if d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
            break;
        }

        let cam_pos = mm::Vec3::new(0.0, 0.0, -10.0);
        let proj = mm::Mat4::perspective(
            45.0 * std::f32::consts::PI / 180.0,
            800.0 / 450.0,
            0.01,
            100.0,
        );

        let cube_pos = mm::Vec3 { 
            x: 5.0, 
            y: 5.0, 
            z: 10.0, 
        };
        let cube_size = 20.0;
        let cube_scale = mm::Vec3 { 
            x: cube_size, 
            y: cube_size, 
            z: cube_size, 
        };

        let cube_transform = mm::Mat4::identity();
        let cube_transform = cube_transform * mm::Mat4::rotation_y(mouse_pos.x * 0.01);
        let cube_transform = cube_transform * mm::Mat4::rotation_x(mouse_pos.y * 0.01);
        let cube_transform = cube_transform * mm::Mat4::translation(cube_pos);
        let cube_transform = cube_transform * mm::Mat4::translation(-cam_pos);
        let cube_transform = cube_transform * mm::Mat4::scale(cube_scale);
        let cube_transform = cube_transform * proj;
        // make new transformed cube vertices
        let mut cube_vertices: Vec<mm::Vec3> = Vec::new();
        for i in 0..cube.len() {
            cube_vertices.push(cube[i] * cube_transform);
        }

        // // draw the cube, one dot at a time
        for vert in &cube_vertices {
            let rlffiv2p = raylib::ffi::Vector2 {
                x: vert.x + dims.x / 2.0,
                y: vert.y + dims.y / 2.0,
            };
            d.draw_circle_v(rlffiv2p, 2.0, Color::RED);
        }

        // draw the cube, one line at a time
        for i in 0..cube_vertices.len() {
            let vert1 = cube_vertices[i];
            let vert2 = cube_vertices[(i + 1) % cube_vertices.len()];
            let rlffiv2p1 = raylib::ffi::Vector2 {
                x: vert1.x + dims.x / 2.0,
                y: vert1.y + dims.y / 2.0,
            };
            let rlffiv2p2 = raylib::ffi::Vector2 {
                x: vert2.x + dims.x / 2.0,
                y: vert2.y + dims.y / 2.0,
            };
            d.draw_line_v(rlffiv2p1, rlffiv2p2, Color::RED);
        }

        // draw circle on render target
        d.clear_background(Color::BLACK);
        d.draw_circle_v(mouse_pos, 10.0, Color::RED);

        // draw render target to screen
        d.draw_texture_rec(
            target,
            raylib::ffi::Rectangle {
                x: 0.0,
                y: 0.0,
                width:  gameboy_dims.x,
                height: gameboy_dims.y,
            },
            raylib::ffi::Vector2 { x: 0.0, y: 0.0 },
            Color::WHITE,
        );


    }



}
