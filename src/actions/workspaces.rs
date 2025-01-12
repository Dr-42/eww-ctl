// #!/usr/bin/env python3
//
// import sys
// import subprocess
// import json
//
// icons = ["󰄰", "󰄯", "󰄮"]
//
// has_window = [False] * 9
//
// hyprinfo = subprocess.run(
//     "hyprctl workspaces -j", shell=True, capture_output=True
// ).stdout.decode("utf-8")
// hyprinfo_json = json.loads(hyprinfo)
//
// for entry in hyprinfo_json:
//     if entry["id"]:
//         has_window[entry["id"] - 1] = True
//
// activeworkspace_info = subprocess.run(
//     "hyprctl activeworkspace -j", shell=True, capture_output=True
// ).stdout.decode("utf-8")
// activeworkspace_info_json = json.loads(activeworkspace_info)
//
// activeworkspace = activeworkspace_info_json["id"]
//
// window_icons = []
//
// for i in range(9):
//     if activeworkspace == i + 1:
//         window_icons.append(icons[1])
//     elif has_window[i]:
//         window_icons.append(icons[2])
//     else:
//         window_icons.append(icons[0])
//
//
// output = '(eventbox :onscroll "scripts/workspaces.py {}" (box :css getcss :class "workspaces" :halign "start" :spacing 8'
//
// for i in range(9):
//     output += (
//         '(button :onclick "hyprctl dispatch workspace '
//         + str(i + 1)
//         + '" "'
//         + window_icons[i]
//         + '")'
//     )
//
// output += "))"
//
// args = sys.argv
//
// if len(args) > 1:
//     arg = args[1]
//     if arg == "up":
//         subprocess.run("hyprctl dispatch workspace e+1", shell=True)
//     elif arg == "down":
//         subprocess.run("hyprctl dispatch workspace e-1", shell=True)
// else:
//     print(output)

use std::{fmt::Display, process::Command};

pub enum WorkspacesAction {
    Up,
    Down,
}

impl Display for WorkspacesAction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            WorkspacesAction::Up => write!(f, "up"),
            WorkspacesAction::Down => write!(f, "down"),
        }
    }
}

pub fn get_workspaces(action: Option<WorkspacesAction>) {
    let icons = ["󰄰", "󰄯", "󰄮"];
    let mut has_window = [false; 9];

    let hyprinfo = Command::new("hyprctl")
        .args(["workspaces", "-j"])
        .output()
        .expect("failed to execute process");

    let hyprinfo_json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&hyprinfo.stdout).as_ref()).unwrap();

    for entry in hyprinfo_json.as_array().unwrap() {
        if entry["id"].as_u64().unwrap() > 0 {
            has_window[entry["id"].as_u64().unwrap() as usize - 1] = true;
        }
    }

    let activeworkspace_info = Command::new("hyprctl")
        .args(["activeworkspace", "-j"])
        .output()
        .expect("failed to execute process");

    let activeworkspace_info_json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(&activeworkspace_info.stdout).as_ref())
            .unwrap();

    let activeworkspace = activeworkspace_info_json["id"].as_u64().unwrap() as usize;

    let mut window_icons = vec![""; 9];

    for i in 0..9 as usize {
        if activeworkspace == i + 1 {
            window_icons[i] = icons[1];
        } else if has_window[i] {
            window_icons[i] = icons[2];
        } else {
            window_icons[i] = icons[0];
        }
    }

    let mut output = "(eventbox :onscroll \"eww-ctl workspaces {}\" (box :css getcss :class \"workspaces\" :halign \"start\" :spacing 8".to_string();

    for i in 0..9 as usize {
        output += format!(
            "(button :onclick \"hyprctl dispatch workspace {}\" \"{}\")",
            i + 1,
            window_icons[i]
        )
        .as_str();
    }

    output += "))";

    if let Some(action) = action {
        match action {
            WorkspacesAction::Up => {
                Command::new("hyprctl")
                    .args(["dispatch", "workspace", "e+1"])
                    .output()
                    .expect("failed to execute process");
            }
            WorkspacesAction::Down => {
                Command::new("hyprctl")
                    .args(["dispatch", "workspace", "e-1"])
                    .output()
                    .expect("failed to execute process");
            }
        }
    } else {
        println!("{}", output)
    }
}
