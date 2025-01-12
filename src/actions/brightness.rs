// #!/usr/bin/env python3
//
// import subprocess
// from math import ceil
//
// brightness = subprocess.run(
//     ["brightnessctl", "get", "-P"], capture_output=True
// ).stdout.decode("utf-8")
//
// icons = [" ", " ", " ", " ", " ", " ", " ", " ", " "]
// brightness = int(brightness)
// index = ceil(brightness * (len(icons) - 1) / 100)
// if index < 0:
//     index = 0
// icon = icons[index]
// print(icon)

use std::process::Command;

pub fn get_brightness() -> String {
    let output = Command::new("brightnessctl")
        .args(["get", "-P"])
        .output()
        .expect("failed to execute process");

    let brightness = String::from_utf8_lossy(&output.stdout);
    let brightness = brightness.trim().to_string();

    let icons = [" ", " ", " ", " ", " ", " ", " ", " ", " "];
    let brightness = brightness.parse::<f32>().unwrap();
    let index = (brightness * (icons.len() - 1) as f32 / 100.0) as usize;
    let icon = icons[index];
    icon.to_string()
}
