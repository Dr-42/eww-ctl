// #!/usr/bin/env python3
//
// import time
// import subprocess
//
//
// def play_sound():
//     subprocess.Popen(
//         [
//             "/bin/sh",
//             "-c",
//             'echo "aplay assets/gong.wav" | at now',
//         ]
//     )
//
//
// def check_time():
//     current_time = time.localtime()
//     current_minute = current_time.tm_min
//     current_second = current_time.tm_sec
//     if current_minute == 0 and current_second == 0:
//         return True
//     else:
//         return False
//
//
// if check_time():
//     play_sound()

use chrono::Timelike;

fn check_time() -> bool {
    let now = chrono::Local::now();
    now.minute() == 0 && now.second() == 0
}

pub fn play_gong() {
    if !check_time() {
        return;
    }
    let sound = include_bytes!("../../assets/gong.wav");
    let (_stream, stream_handle) = rodio::OutputStream::try_default().unwrap();
    let sink = rodio::Sink::try_new(&stream_handle).unwrap();
    sink.append(rodio::Decoder::new(std::io::Cursor::new(sound)).unwrap());
    sink.sleep_until_end();
}
