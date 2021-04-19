use sdl2::event::Event;
use sdl2::keyboard::Keycode;
use sdl2::pixels::Color;
use std::time::Duration;

fn main() {
    // SDLの初期化
    let sdl_context = sdl2::init().unwrap();
    let video_subsystem = sdl_context.video().unwrap();

    // match文でエラー処理をする
    let window = match video_subsystem
        .window("SDL", 640, 480)
        .position_centered()
        .build()
    {
        // build()は Result<Window, WindowBuildError>を返す
        Ok(window) => window,
        Err(err) => panic!("failed to build window: {:?}", err),
    };

    // WindowCanvas構造体の取得
    let mut canvas = window.into_canvas().build().unwrap();
    // 塗りつぶしの色指定
    canvas.set_draw_color(Color::RGB(255, 255, 255));
    // set_draw_colorで指定した色で画面全体を塗りつぶす
    canvas.clear();
    // 描画状況を画面にレンダリング
    canvas.present();

    //イベントループ
    let mut event_pump = sdl_context.event_pump().unwrap();
    // ループ"running"を開始
    'running: loop {
        // event_pumpにたまったイベントキューを1つずつ処理
        for event in event_pump.poll_iter() {
            match event {
                // eventの種類が「終了イベント」or「Esc押下イベント」だったら ループ"running"を抜ける
                Event::Quit { .. }
                | Event::KeyDown {
                    keycode: Some(Keycode::Escape),
                    ..
                } => break 'running,
                _ => {}
            }
        }
        canvas.present();
        std::thread::sleep(Duration::new(0, 1_000_000_000u32 / 60))
    }
}
