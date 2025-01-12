// #!/usr/bin/env python3
//
// import subprocess
// import json
//
// hypr_ws = (
//     subprocess.run("hyprctl activeworkspace -j", shell=True, capture_output=True)
//     .stdout.decode("utf-8")
//     .strip()
// )
// curpos = (
//     subprocess.run("hyprctl cursorpos -j", shell=True, capture_output=True)
//     .stdout.decode("utf-8")
//     .strip()
// )
// hypr_wins = (
//     subprocess.run("hyprctl clients -j", shell=True, capture_output=True)
//     .stdout.decode("utf-8")
//     .strip()
// )
//
// hypr_ws = json.loads(hypr_ws)
// curpos = json.loads(curpos)
// hypr_wins = json.loads(hypr_wins)
//
//
// def get_window_res(active_ws, y_thresh):
//     for window in hypr_wins:
//         if window["workspace"]["id"] == active_ws:
//             if window["at"][1] < y_thresh:
//                 return False
//
//     return True
//
//
// if curpos["y"] < 30:
//     print("true")
// elif hypr_ws["windows"] == 0:
//     print("true")
// elif get_window_res(hypr_ws["id"], 40):
//     print("true")
// else:
//     print("false")

use std::process::Command;

fn get_window_res(active_ws: u64, y_thresh: u64) -> bool {
    let output = Command::new("hyprctl")
        .args(["clients", "-j"])
        .output()
        .expect("failed to execute process");

    let hypr_wins = String::from_utf8_lossy(&output.stdout);
    let hypr_wins_json: serde_json::Value = serde_json::from_str(&hypr_wins).unwrap();

    for window in hypr_wins_json.as_array().unwrap() {
        if window["workspace"]["id"] == active_ws && window["at"][1] < y_thresh {
            return false;
        }
    }

    return true;
}

pub fn show_bar() -> String {
    let output = Command::new("hyprctl")
        .args(["activeworkspace", "-j"])
        .output()
        .expect("failed to execute process");

    let hypr_ws = String::from_utf8_lossy(&output.stdout);
    let hypr_ws_json: serde_json::Value = serde_json::from_str(&hypr_ws).unwrap();

    let output = Command::new("hyprctl")
        .args(["cursorpos", "-j"])
        .output()
        .expect("failed to execute process");

    let curpos = String::from_utf8_lossy(&output.stdout);
    let curpos_json: serde_json::Value = serde_json::from_str(&curpos).unwrap();

    let output = Command::new("hyprctl")
        .args(["clients", "-j"])
        .output()
        .expect("failed to execute process");

    let hypr_wins = String::from_utf8_lossy(&output.stdout);
    let hypr_wins_json: serde_json::Value = serde_json::from_str(&hypr_wins).unwrap();

    if curpos_json["y"].as_u64().unwrap() < 30 {
        return "true".to_string();
    } else if hypr_ws_json["windows"].as_u64().unwrap() == 0 {
        return "true".to_string();
    } else if get_window_res(hypr_ws_json["id"].as_u64().unwrap(), 40) {
        return "true".to_string();
    } else {
        return "false".to_string();
    }
}
