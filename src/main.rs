extern crate winapi;
extern crate enigo;

use enigo::{ MouseControllable, Enigo};
use std::thread;
use std::time::{Duration};
use winapi::um::winuser::{GetCursorPos, SetCursorPos};
use winapi::shared::windef::POINT;
use chrono::Local;

fn main() {
    let mut last_position: POINT = POINT { x: 0, y: 0 };
    let mut last_time = Local::now();
    let mut current_position = POINT { x: 0, y: 0 };
    let wait_time = 30;
    let mut enigo = Enigo::new();

    loop {
        unsafe { GetCursorPos(&mut current_position) };
        let now = Local::now();

        if current_position.x != last_position.x || current_position.y != last_position.y {
            println!("{}，检测到鼠标主动移动，忽略", now.format("%H:%M:%S:%f"));
        } else if now.signed_duration_since(last_time).num_seconds() >= wait_time  {
            println!("{}，检测到鼠标未移动，随机移动鼠标", now.format("%H:%M:%S:%f"));
            let new_x = rand::random::<i32>() % 1920; // 随机生成新的 X 坐标
            let new_y = rand::random::<i32>() % 1080; // 随机生成新的 Y 坐标
            enigo.mouse_move_to(new_x, new_y);
            last_time = now;
        }
        last_position = current_position;

        thread::sleep(Duration::from_secs(wait_time.try_into().unwrap()));
    }
}
