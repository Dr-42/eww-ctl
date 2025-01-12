// #!/usr/bin/env python3
//
// import os
// import sys
//
// arg = sys.argv[1]
//
// if arg == "sleep":
//     os.system("systemctl suspend")
// elif arg == "hibernate":
//     os.system("systemctl hibernate")
// elif arg == "shutdown":
//     os.system("shutdown now")
// elif arg == "restart":
//     os.system("reboot")
// elif arg == "logout":
//     os.system("hyprctl dispatch exit")
// else:
//     sys.exit(1)

use std::process::Command;

pub enum PowertrayAction {
    Sleep,
    Hibernate,
    Shutdown,
    Restart,
    Logout,
}

pub fn powertray(action: PowertrayAction) {
    match action {
        PowertrayAction::Sleep => {
            let mut process = Command::new("systemctl")
                .args(["suspend"])
                .spawn()
                .expect("failed to execute process");
            process.wait().unwrap();
        }
        PowertrayAction::Hibernate => {
            let mut process = Command::new("systemctl")
                .args(["hibernate"])
                .spawn()
                .expect("failed to execute process");
            process.wait().unwrap();
        }
        PowertrayAction::Shutdown => {
            let mut process = Command::new("shutdown")
                .args(["now"])
                .spawn()
                .expect("failed to execute process");
            process.wait().unwrap();
        }
        PowertrayAction::Restart => {
            let mut process = Command::new("reboot")
                .spawn()
                .expect("failed to execute process");
            process.wait().unwrap();
        }
        PowertrayAction::Logout => {
            let mut process = Command::new("hyprctl")
                .args(["dispatch", "exit"])
                .spawn()
                .expect("failed to execute process");
            process.wait().unwrap();
        }
    };
}
