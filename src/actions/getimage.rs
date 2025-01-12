// #!/usr/bin/env python3
//
// # Print the path to a random image in $HOME/dotfiles/Wallpapers
//
// import os
// import random
// import subprocess
//
// wallpapers_dir = os.path.expanduser("~/dotfiles/Wallpapers")
//
// wallpapers = os.listdir(wallpapers_dir)
// wallpaper = random.choice(wallpapers)
//
// complete_path = os.path.join(wallpapers_dir, wallpaper)
//
// # save the wallpaper path in a file
// with open(os.path.expanduser("~/.cache/.wallpaper"), "w") as f:
//     f.write(complete_path)
//     f.close()
//
// # extract colors from the wallpaper
// css = subprocess.run(["scripts/getcss.py"], capture_output=True).stdout.decode("utf-8")
//
// arg = f"getcss={css}"
//
// home = os.path.expanduser("~")
// eww_binary = os.path.join(home, ".local/bin/eww")
// subprocess.run([eww_binary, "update", arg])
//
// print(complete_path)

use rand::seq::SliceRandom;
use std::process::Command;

pub fn get_image() -> String {
    let wallpapers_dir = "/home/spandan/dotfiles/Wallpapers";
    let wallpapers = std::fs::read_dir(wallpapers_dir).unwrap();
    let wallpapers = wallpapers.filter_map(|f| f.ok()).collect::<Vec<_>>();
    let mut wallpapers = wallpapers
        .iter()
        .filter(|f| f.path().is_file())
        .collect::<Vec<_>>();
    let wallpaper = wallpapers
        .choose_mut(&mut rand::thread_rng())
        .unwrap()
        .path()
        .display()
        .to_string();

    let css = super::getcss::getcss(&wallpaper);

    let arg = format!("getcss={css}");

    let eww_binary = "eww";

    Command::new(eww_binary)
        .args(["update", &arg])
        .output()
        .expect("failed to updaate css");

    wallpaper
}
