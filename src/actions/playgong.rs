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

use std::process::Command;

use chrono::Timelike;

fn check_time() -> bool {
    let now = chrono::Local::now();
    if now.minute() == 0 && now.second() == 0 {
        true
    } else {
        false
    }
}

pub fn play_gong() {
    if !check_time() {
        return;
    }
    let mut command = Command::new("/bin/sh")
        .args(["-c", "echo \"aplay assets/gong.wav\" | at now"])
        .spawn()
        .unwrap();

    command.wait().unwrap();
}
