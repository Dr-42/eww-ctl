// #!/usr/bin/env python3
//
// import sys
// import os
//
// if args[1] == "up":
//     os.system("amixer sset Master 5%+")
// elif args[1] == "down":
//     os.system("amixer sset Master 5%-")

pub enum VolumeAction {
    Up,
    Down,
}

use std::process::Command;

pub fn set_volume(action: VolumeAction) {
    match action {
        VolumeAction::Up => {
            let mut process = Command::new("amixer")
                .args(["sset", "Master", "5%+"])
                .spawn()
                .expect("failed to execute process");
            process.wait().unwrap();
        }
        VolumeAction::Down => {
            let mut process = Command::new("amixer")
                .args(["sset", "Master", "5%-"])
                .spawn()
                .expect("failed to execute process");
            process.wait().unwrap();
        }
    }
}
