// #!/usr/bin/env python3
//
// import subprocess
//
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
// print(volume_status)

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

    volume.to_string().trim().to_string()
}
