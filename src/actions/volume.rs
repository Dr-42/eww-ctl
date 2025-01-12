// #!/usr/bin/env python3
//
// import subprocess
//
// icons = ["󰕾 ", "󰖁 "]
//
// # Get current volume status
// volume_status = (
//     subprocess.run(
//         "amixer get Master | grep -oP '(?<=\[).*?(?=%\])' | tail -n 1",
//         shell=True,
//         capture_output=True,
//     )
//     .stdout.decode("utf-8")
//     .strip()
// )
//
// vol = int(volume_status)
//
// is_muted = subprocess.run(
//     "amixer get Master | grep -oP '(?<=\[).*?(?=\])' | tail -1",
//     shell=True,
//     capture_output=True,
// ).stdout.decode("utf-8")
//
// if "off" in is_muted:
//     print(icons[1])
// else:
//     print(icons[0])

use std::process::Command;

pub fn get_volume() -> String {
    let output = Command::new("amixer")
        .args(["get", "Master"])
        .output()
        .expect("failed to execute process");

    let volume_details = String::from_utf8_lossy(&output.stdout);

    let mut volume = 0;
    for line in volume_details.lines() {
        if line.contains("%") {
            let vol = line.split_once("[").unwrap().1.split_once("%").unwrap().0;
            volume = vol.parse::<u32>().unwrap();
            break;
        }
    }

    let icons = ["󰕾 ", "󰖁 "];

    if volume == 0 {
        icons[1].to_string()
    } else {
        icons[0].to_string()
    }
}
