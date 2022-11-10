mod mm;
use raylib::prelude::*;

fn main() {
    let gameboy_dims = mm::Vec2::new(160.0, 144.0);
    let dims = gameboy_dims * 5.0;
    let (mut rl, thread) = raylib::init()
        .size(dims.x as i32, dims.y as i32)
        //set resizable
        .resizable()
        .title("software renderer")
        .build();
    rl.set_target_fps(60);

    let mut _perspective = mm::Mat4::perspective(
        45.0 * std::f32::consts::PI / 180.0,
        800.0 / 450.0,
        0.01,
        100.0,
    );

    // create a cube of size 1
    let mut cube: Vec<mm::Vec3> = Vec::new();
    cube.push(mm::Vec3 { x: -1.0, y: 1.0, z: 1.0 });
    cube.push(mm::Vec3 { x: 1.0, y: 1.0, z: 1.0 });
    cube.push(mm::Vec3 { x: 1.0, y: -1.0, z: 1.0 });
    cube.push(mm::Vec3 { x: -1.0, y: -1.0, z: 1.0 });
    cube.push(mm::Vec3 { x: -1.0, y: 1.0, z: -1.0 });
    cube.push(mm::Vec3 { x: 1.0, y: 1.0, z: -1.0 });
    cube.push(mm::Vec3 { x: 1.0, y: -1.0, z: -1.0 });
    cube.push(mm::Vec3 { x: -1.0, y: -1.0, z: -1.0 });

    let mut framebuffer = rl
        .load_render_texture(
            // pass in thread
            &thread,
            gameboy_dims.x as u32, gameboy_dims.y as u32)
        .unwrap();

    //  set the virtual resolution so the moues position is correct
    let mouse_scale =  gameboy_dims / dims;
    rl.set_mouse_scale(
        mouse_scale.x as f32,
        mouse_scale.y as f32,
    );

        
    let cam_speed = 1.0;
    let mut cam_pos = mm::Vec3::new(0.0, 0.0, -10.0);

    while !rl.window_should_close() {

        let mut dt = rl.begin_drawing(&thread);
        {
            let mut d = dt.begin_texture_mode(&thread, &mut framebuffer);
            d.clear_background(Color::BLACK);
            
            // draw the mouse
            let mouse_pos = d.get_mouse_position();
            let mp: mm::Vec2 = mm::Vec2::new(mouse_pos.x, mouse_pos.y);
            d.draw_circle_v(mouse_pos, 2.0, Color::RED);
            // quit if escape is pressed
            if d.is_key_pressed(KeyboardKey::KEY_ESCAPE) {
                break;
            }
            // if a key is pressed, move the camera left
            if d.is_key_down(KeyboardKey::KEY_A) {
                cam_pos.x -= cam_speed;
            }
            // if d is pressed, move the camera right
            if d.is_key_down(KeyboardKey::KEY_D) {
                cam_pos.x += cam_speed;
            }
            // if w is pressed, move the camera up
            if d.is_key_down(KeyboardKey::KEY_W) {
                cam_pos.y += cam_speed;
            }
            // if s is pressed, move the camera down
            if d.is_key_down(KeyboardKey::KEY_S) {
                cam_pos.y -= cam_speed;
            }



            let proj = mm::Mat4::perspective(
                45.0 * std::f32::consts::PI / 180.0,
                gameboy_dims.x / gameboy_dims.y,
                0.01,
                100.0,
            );

            let cube_pos = mm::Vec3 { 
                x: 0.0, 
                y: 5.0, 
                z: 10.0, 
            };
            let cube_size = 10.0;
            let cube_scale = mm::Vec3 { 
                x: cube_size, 
                y: cube_size, 
                z: cube_size, 
            };


            let rotation_speed = 10.0;
            let time = d.get_time() as f32;
            let rotation = mm::Vec3 {
                x: time * rotation_speed,
                y: time * rotation_speed,
                z: time * rotation_speed,
            };


            let up_axis = mm::Vec3::new(0.0, 1.0, 0.0);
            let spin_axis = mm::Vec3::new(1.0, 1.0, 1.0);

            // move cam around the cube with mouse
            let mouse_from_center = mp - gameboy_dims / 2.0;
            // let cam_pos = cam_pos + -mm::Vec3::new(mouse_from_center.x, -mouse_from_center.y, 0.0) * 0.1;
            let view = mm::Mat4::look_at(
                cam_pos,
                cam_pos + mm::Vec3::new(0.0, 0.0, 1.0),
                up_axis,
            );

            let cube_transform = mm::Mat4::identity();
            // let cube_transform = cube_transform * mm::Mat4::rotation_y(mouse_pos.x * 0.01);
            // let cube_transform = cube_transform * mm::Mat4::rotation_x(d.get_time() as f32 * 0.1);
            // let cube_transform = cube_transform * mm::Mat4::rotation(d.get_time() as f32 * 1.0, spin_axis);
            // let cube_transform = cube_transform * mm::Mat4::translation(cube_pos);
            let cube_transform = cube_transform * mm::Mat4::scale(cube_scale);
            let cube_transform = cube_transform * proj;
            let cube_transform = cube_transform * view;
            let cube_transform = cube_transform * mm::Mat4::translation(-cam_pos);
            

            // make new transformed cube vertices
            let mut cube_vertices: Vec<mm::Vec3> = Vec::new();
            for i in 0..cube.len() {
                cube_vertices.push(cube[i] * cube_transform);
            }

            //get all possible pairs of edges
            let mut edges: Vec<(usize, usize)> = Vec::new();
            for i in 0..cube_vertices.len() {
                for j in i + 1..cube_vertices.len() {
                    edges.push((i, j));
                }
            }

            let half_screen = gameboy_dims / 2.0;
            
            // draw the edges
            for edge in edges {
                let v1 = cube_vertices[edge.0];
                let v2 = cube_vertices[edge.1];
                let rlv2_1 = raylib::ffi::Vector2 {
                    x: v1.x + half_screen.x,
                    y: v1.y + half_screen.y,
                };
                let rlv2_2 = raylib::ffi::Vector2 {
                    x: v2.x + half_screen.x,
                    y: v2.y + half_screen.y,
                };
                d.draw_line_ex(
                    rlv2_1,
                    rlv2_2,
                    1.0,
                    Color::WHITE,
                );
            }


            // // draw the cube, one dot at a time
            for vert in &cube_vertices {
                let rlffiv2p = raylib::ffi::Vector2 {
                    x: vert.x + dims.x / 2.0,
                    y: vert.y + dims.y / 2.0,
                };
                d.draw_circle_v(rlffiv2p, 2.0, Color::BLUE);
            }

            // // draw the cube, one line at a time
            // for i in 0..cube_vertices.len() {
            //     let vert1 = cube_vertices[i];
            //     let vert2 = cube_vertices[(i + 1) % cube_vertices.len()];
            //     let rlffiv2p1 = 
            //     let rlffiv2p2 = raylib::ffi::Vector2 {
            //         x: vert2.x + gameboy_dims.x / 2.0,
            //         y: vert2.y + gameboy_dims.y / 2.0,
            //     };
            //     d.draw_line_v(rlffiv2p1, rlffiv2p2, Color::RED);
            // }

        }
        // draw render target to screen
        // render target is 160x144
        // screen is 800x450
        dt.draw_texture_pro(
            framebuffer.texture(),
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: framebuffer.texture.width as f32,
                height: -framebuffer.texture.height as f32,
            },
            Rectangle {
                x: 0.0,
                y: 0.0,
                width: dims.x,
                height: dims.y,
            },
            Vector2 { x: 0.0, y: 0.0 },
            0.0,
            Color::WHITE,

        );

    }



}
