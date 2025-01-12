// #!/usr/bin/env python3
//
// import subprocess
// import json
//
// hypr_info = subprocess.run(
//     ["hyprctl", "clients", "-j"], capture_output=True
// ).stdout.decode("utf-8")
//
// hypr_info_json = json.loads(hypr_info)
//
// for client in hypr_info_json:
//     if client["focusHistoryID"] == 0:
//         print(client["title"])
//         break

use std::process::Command;

pub fn get_active_window() -> String {
    let output = Command::new("hyprctl")
        .args(["clients", "-j"])
        .output()
        .expect("failed to execute process");

    let hypr_info = String::from_utf8_lossy(&output.stdout);
    let hypr_info_json: serde_json::Value = serde_json::from_str(&hypr_info).unwrap();

    for client in hypr_info_json.as_array().unwrap() {
        if client["focusHistoryID"] == 0 {
            return client["title"].as_str().unwrap().to_string();
        }
    }
    "".to_string()
}
