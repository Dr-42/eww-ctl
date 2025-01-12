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
        .args([
            "get",
            "Master",
            "|",
            "grep",
            "-oP",
            "(?<=\\[).*?(?=\\])",
            "|",
            "tail",
            "-n",
            "1",
        ])
        .output()
        .expect("failed to execute process");

    let volume_status = String::from_utf8_lossy(&output.stdout);
    let volume_status = volume_status.trim().to_string();

    volume_status
}
