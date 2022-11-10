mod mm;
use raylib::prelude::*;

fn make_cube() -> Vec<mm::Vec3> {
    let mut cube: Vec<mm::Vec3> = Vec::new();
    cube.push(mm::Vec3 {
        x: -1.0,
        y: 1.0,
        z: 1.0,
    });
    cube.push(mm::Vec3 {
        x: 1.0,
        y: 1.0,
        z: 1.0,
    });
    cube.push(mm::Vec3 {
        x: 1.0,
        y: -1.0,
        z: 1.0,
    });
    cube.push(mm::Vec3 {
        x: -1.0,
        y: -1.0,
        z: 1.0,
    });
    cube.push(mm::Vec3 {
        x: -1.0,
        y: 1.0,
        z: -1.0,
    });
    cube.push(mm::Vec3 {
        x: 1.0,
        y: 1.0,
        z: -1.0,
    });
    cube.push(mm::Vec3 {
        x: 1.0,
        y: -1.0,
        z: -1.0,
    });
    cube.push(mm::Vec3 {
        x: -1.0,
        y: -1.0,
        z: -1.0,
    });
    // multiply the verts by 0.5
    for vert in &mut cube {
        vert.x *= 0.5;
        vert.y *= 0.5;
        vert.z *= 0.5;
    }
    cube
}

fn main() {
    let gameboy_dims = mm::Vec2::new(160.0, 144.0) * 2.0;
    let dims = mm::Vec2::new(800.0, 600.0);
    let (mut rl, thread) = raylib::init()
        .size(dims.x as i32, dims.y as i32)
        //set resizable
        .resizable()
        .title("software renderer")
        .build();
    rl.set_target_fps(60);

    let cube = make_cube();

    let mut framebuffer = rl
        .load_render_texture(
            // pass in thread
            &thread,
            gameboy_dims.x as u32,
            gameboy_dims.y as u32,
        )
        .unwrap();

    //  set the virtual resolution so the moues position is correct
    let mouse_scale = gameboy_dims / dims;
    rl.set_mouse_scale(mouse_scale.x as f32, mouse_scale.y as f32);

    let cam_speed = 0.1;
    let cube_speed = 0.1;
    let mut cam_pos = mm::Vec3::new(0.0, 0.0, -10.0);
    let mut fov = 90.0;
    let mut near = 0.1;
    let mut far = 100.0;

    let mut mouse_center_normalized = mm::Vec2::new(0.0, 0.0);
    let mut look_at = mm::Vec3::new(0.0, 0.0, 1.0);

    let mut cube_pos = mm::Vec3 {
        x: 2.0,
        y: 1.0,
        z: 0.0,
    };

    while !rl.window_should_close() {
        let mut dt = rl.begin_drawing(&thread);
        {
            let mut d = dt.begin_texture_mode(&thread, &mut framebuffer);
            d.clear_background(Color::BLACK);

            // draw the mouse
            let mouse_pos = d.get_mouse_position();
            let mp: mm::Vec2 = mm::Vec2::new(mouse_pos.x, mouse_pos.y);
            mouse_center_normalized = mp / gameboy_dims - mm::Vec2::new(0.5, 0.5);

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

            // if q is pressed, move the camera forward
            if d.is_key_down(KeyboardKey::KEY_Q) {
                cam_pos.z += cam_speed;
            }
            // if e is pressed, move the camera backward
            if d.is_key_down(KeyboardKey::KEY_E) {
                cam_pos.z -= cam_speed;
            }

            // if z is pressed, zoom in
            if d.is_key_down(KeyboardKey::KEY_Z) {
                fov -= 1.0;
                if fov < 45.0 {
                    fov = 45.0;
                }
            }
            // if x is pressed, zoom out
            if d.is_key_down(KeyboardKey::KEY_X) {
                fov += 1.0;
                if fov > 120.0 {
                    fov = 120.0;
                }
            }
            // r to move cam forward
            if d.is_key_down(KeyboardKey::KEY_R) {
                cam_pos.z += cam_speed;
            }
            // f to move cam backward
            if d.is_key_down(KeyboardKey::KEY_F) {
                cam_pos.z -= cam_speed;
            }

            //arrow keys to move the cube
            if d.is_key_down(KeyboardKey::KEY_LEFT) {
                cube_pos.x -= cube_speed;
            }
            if d.is_key_down(KeyboardKey::KEY_RIGHT) {
                cube_pos.x += cube_speed;
            }
            if d.is_key_down(KeyboardKey::KEY_UP) {
                cube_pos.y += cube_speed;
            }
            if d.is_key_down(KeyboardKey::KEY_DOWN) {
                cube_pos.y -= cube_speed;
            }

            let proj = mm::Mat4::perspective(
                fov * std::f32::consts::PI / 180.0,
                gameboy_dims.x / gameboy_dims.y,
                near,
                far,
            );

            let cube_scale = mm::Vec3::one() * 10.0;

            // let time = d.get_time() as f32;
            // let rotation_speed = 10.0;
            // let rotation = mm::Vec3::one() * time * rotation_speed;

            let up_axis = mm::Vec3::new(0.0, 1.0, 0.0);

            let view_from_center = mm::Vec3::zero();
            let cam_dir = mm::Vec3::new(0.0, 0.0, 1.0);
            look_at = cam_pos + cam_dir;
            let view = mm::Mat4::look_at(view_from_center, cam_dir, up_axis);

            // get cube cam space
            let model = mm::Mat4::identity();
            let spin_axis = mm::Vec3::new(1.0, 1.0, 1.0);
            let model = model * mm::Mat4::translation(cube_pos - cam_pos);
            let model = model * mm::Mat4::rotation(d.get_time() as f32 * 3.0, spin_axis);
            let model = model * mm::Mat4::scale(cube_scale);

            // let cube_transform = proj * view * model;
            let cube_transform = view * proj * model;
            // let cube_transform = model * view * proj;

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
                d.draw_line_ex(rlv2_1, rlv2_2, 1.0, Color::WHITE);
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

        // draw the cam coords in the top left
        dt.draw_text(&format!("cam pos: {:?}", cam_pos), 0, 0, 20, Color::WHITE);
        // draw the look at point coords in the top left under cam
        dt.draw_text(&format!("look at: {:?}", look_at), 0, 20, 20, Color::WHITE);
        // draw the mouse normalized coords in the top left under look at
        dt.draw_text(
            &format!("mouse: {:?}", mouse_center_normalized),
            0,
            40,
            20,
            Color::WHITE,
        );
        // draw field of view in the top left under mouse
        dt.draw_text(&format!("fov: {}", fov), 0, 60, 20, Color::WHITE);
        // draw cube pos
        dt.draw_text(
            &format!("cube pos: {:?}", cube_pos),
            0,
            80,
            20,
            Color::WHITE,
        );
    }
}
