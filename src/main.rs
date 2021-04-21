use std::mem;
use std::os::raw::c_void;
use std::time::Duration;

use c_str_macro::c_str;
use cgmath::perspective;
use cgmath::prelude::SquareMatrix;
use gl::types::{GLfloat, GLsizei, GLsizeiptr};
use sdl2::event::Event;
use sdl2::keyboard::Keycode;

use imgui;
use imgui_opengl_renderer;
use imgui_sdl2;

mod shader;
mod vertex;

use shader::Shader;
use vertex::Vertex;

#[allow(dead_code)]
type Point3 = cgmath::Point3<f32>;
#[allow(dead_code)]
type Vector3 = cgmath::Vector3<f32>;
#[allow(dead_code)]
type Matrix4 = cgmath::Matrix4<f32>;

const WINDOW_WIDTH: u32 = 640;
const WINDOW_HEIGHT: u32 = 480;
const FLOAT_NUM: usize = 3;
const VERTEX_NUM: usize = 3;
const BUF_LEN: usize = FLOAT_NUM * VERTEX_NUM;

fn main() {
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    {
        // バージョン情報の表示
        let gl_attr = video_subsystem.gl_attr();
        gl_attr.set_context_profile(sdl2::video::GLProfile::Core);
        gl_attr.set_context_version(3, 1);
        let (major, minor) = gl_attr.context_version();
        println!("OK: init OpenGL: version={}.{}", major, minor);
    }

    let window = video_subsystem
        .window("SDL", WINDOW_WIDTH, WINDOW_HEIGHT)
        .opengl()
        .position_centered()
        .build()
        .unwrap();

    let _gl_context = window.gl_create_context().unwrap();
    gl::load_with(|s| video_subsystem.gl_get_proc_address(s) as _);

    // シェーダーの読み込み
    let shader = Shader::new("rsc/shader/shader.vs", "rsc/shader/shader.fs");

    // set buffer
    #[rustfmt::skip]
    let buffer_array: [f32; BUF_LEN] = [
        -1.0, -1.0, 0.0,
        1.0, -1.0, 0.0,
        0.0, 1.0, 0.0,
    ];

    let vertex = Vertex::new(
        (BUF_LEN * mem::size_of::<GLfloat>()) as GLsizeiptr,
        buffer_array.as_ptr() as *const c_void,
        gl::STATIC_DRAW,
        vec![gl::FLOAT],
        vec![FLOAT_NUM as i32],
        FLOAT_NUM as i32 * mem::size_of::<GLfloat>() as GLsizei,
        VERTEX_NUM as i32,
    );

    // imguiの初期化
    // コンテキストデータの作成
    let mut imgui_context = imgui::Context::create();
    // 設定ファイルを作成しないようにする
    imgui_context.set_ini_filename(None);

    // imgui-sdl2の初期化
    // SDL2で作成したウィンドウでImguiを使えるようにする
    // imgui-sdl2のコンテキストデータを作成
    let mut imgui_sdl2_context = imgui_sdl2::ImguiSdl2::new(&mut imgui_context, &window);
    // 描画処理のためのレンダラーを作成
    let renderer = imgui_opengl_renderer::Renderer::new(&mut imgui_context, |s| {
        video_subsystem.gl_get_proc_address(s) as _
    });

    let mut event_pump = sdl_context.event_pump().unwrap();
    'running: loop {
        for event in event_pump.poll_iter() {
            // イベントがimguiによるものである場合、イベントをimgui側に渡し、無視する
            imgui_sdl2_context.handle_event(&mut imgui_context, &event);
            if imgui_sdl2_context.ignore_event(&event) {
                continue;
            }

            match event {
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }

        unsafe {
            // ビューポートの設定
            gl::Viewport(0, 0, WINDOW_WIDTH as i32, WINDOW_HEIGHT as i32);

            // 画面をクリア
            gl::ClearColor(1.0, 1.0, 1.0, 1.0);
            gl::Clear(gl::COLOR_BUFFER_BIT);

            // モデル行列、ビュー行列、射影行列の初期化

            // モデル行列
            // 平行移動、回転、拡大、縮小を行うときに使う
            let model_matrix = Matrix4::identity();
            // ビュー行列
            // 3次元空間でどこから何を見ているかを表した行列
            let view_matrix = Matrix4::look_at_rh(
                Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 5.0,
                },
                Point3 {
                    x: 0.0,
                    y: 0.0,
                    z: 0.0,
                },
                Vector3 {
                    x: 0.0,
                    y: 1.0,
                    z: 0.0,
                },
            );

            // 射影行列
            // 風景をどのように描画するかを指定する行列
            // perspective (透視投影法) と ortho (平行投影法)がある
            let projection_matrix: Matrix4 = perspective(
                cgmath::Deg(45.0f32),
                WINDOW_WIDTH as f32 / WINDOW_HEIGHT as f32,
                0.1,
                100.0,
            );

            // 行列に従ってshaderを適用する
            shader.use_program();
            shader.set_mat4(c_str!("uModel"), &model_matrix);
            shader.set_mat4(c_str!("uView"), &view_matrix);
            shader.set_mat4(c_str!("uProjection"), &projection_matrix);

            // 処理したデータに基づいて、描画を行う
            vertex.draw();

            // imguiのウィンドウを描画
            // SDL2とimgui間で画面サイズや入力などをよしなに調整する
            imgui_sdl2_context.prepare_frame(
                imgui_context.io_mut(),
                &window,
                &event_pump.mouse_state(),
            );

            // ウィジェット生成のためのUI構造体を作成
            let ui = imgui_context.frame();
            // imguiウィンドウ作成
            imgui::Window::new(imgui::im_str!("Information"))
                .size([300.0, 200.0], imgui::Condition::FirstUseEver)
                // .build() 第2引数にウィジェット追加のコードを書く
                .build(&ui, || {
                    ui.text(imgui::im_str!("Hello, World!"));
                    let mouse_pos = ui.io().mouse_pos;
                    ui.text(format!(
                        "Mouse Position: ({:.1}, {:.1})",
                        mouse_pos[0], mouse_pos[1]
                    ));
                    imgui::ProgressBar::new(0.6)
                        .size([200.0, 20.0])
                        .overlay_text(imgui::im_str!("Progress!"))
                        .build(&ui);
                    let arr = [0.6f32, 0.1f32, 1.0f32, 0.5f32, 0.92f32, 0.1f32, 0.2f32];

                    ui.plot_lines(imgui::im_str!("lines"), &arr)
                        .graph_size([200.0, 40.0])
                        .build();
                    ui.plot_histogram(imgui::im_str!("lines"), &arr)
                        .graph_size([200.0, 40.0])
                        .build();
                });

            // imguiウィンドウの描画
            // SDL2とimgui間で画面サイズや入力などをよしなに調整する
            imgui_sdl2_context.prepare_render(&ui, &window);
            // 描画
            renderer.render(ui);

            window.gl_swap_window();
        }

        ::std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 144));
    }
}
