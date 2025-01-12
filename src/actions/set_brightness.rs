// #!/usr/bin/env python3
//
// import sys
// import os
//
//
// if args[1] == "up":
//     os.system("brightnessctl set +5%")
// elif args[1] == "down":
//     os.system("brightnessctl set 5%-")

pub enum BrightnessAction {
    Up,
    Down,
}

use std::process::Command;

pub fn set_brightness(action: BrightnessAction) {
    match action {
        BrightnessAction::Up => {
            let mut process = Command::new("brightnessctl")
                .args(["set", "5%+"])
                .spawn()
                .expect("failed to execute process");
            process.wait().unwrap();
        }
        BrightnessAction::Down => {
            let mut process = Command::new("brightnessctl")
                .args(["set", "5%-"])
                .spawn()
                .expect("failed to execute process");
            process.wait().unwrap();
        }
    }
}
