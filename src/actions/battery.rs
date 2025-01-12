// #!/usr/bin/env python3
//
// import time
// import subprocess
//
// bat_status_arr = ["󰁺", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"]
//
//
// def get_bat_status():
//     acpi = subprocess.run(["acpi"], capture_output=True)
//
//     bat_status = acpi.stdout.decode("utf-8")
//     bat_status = bat_status.split(",")[1]
//
//     bat_status = bat_status.strip()
//     bat_status = bat_status[:-1]
//
//     bat_status = bat_status_arr[(int(bat_status) // 10) - 1]
//
//     return bat_status
//
//
// print(f"{get_bat_status()}")

use std::process::Command;

pub fn get_bat_status() -> String {
    let output = Command::new("acpi")
        .output()
        .expect("failed to execute process");

    let bat_status = String::from_utf8_lossy(&output.stdout).to_string();
    let bat_status = bat_status.split(",").collect::<Vec<&str>>()[1];
    let bat_status = bat_status.trim();
    let bat_status = bat_status[..bat_status.len() - 1].to_string();

    let bat_status_arr = ["󰁺", "󰁻", "󰁼", "󰁽", "󰁾", "󰁿", "󰂀", "󰂁", "󰂂", "󰁹"];
    let bat_status = bat_status_arr[((bat_status.parse::<u8>().unwrap() / 10) - 1) as usize];

    bat_status.to_string()
}
