use actions::{
    powertray::PowertrayAction, set_brightness::BrightnessAction, workspaces::WorkspacesAction,
};
use std::error::Error;

mod actions;
mod utils;

fn main() -> Result<(), Box<dyn Error>> {
    let args: Vec<String> = std::env::args().collect();

    match args.get(1) {
        Some(action) => match action.as_str() {
            "active_window" => {
                let res = actions::active_window::get_active_window();
                println!("{}", res);
                Ok(())
            }
            "battery" => {
                let res = actions::battery::get_bat_status();
                println!("{}", res);
                Ok(())
            }
            "brightness" => {
                let res = actions::brightness::get_brightness();
                println!("{}", res);
                Ok(())
            }
            "get_volume" => {
                let res = actions::get_volume::get_volume();
                println!("{}", res);
                Ok(())
            }
            "getimage" => {
                let res = actions::getimage::get_image();
                println!("{}", res);
                Ok(())
            }
            "playgong" => {
                actions::playgong::play_gong();
                Ok(())
            }
            "powertray" => {
                let power_action = match args.get(2) {
                    Some(action) => match action.as_str() {
                        "sleep" => PowertrayAction::Sleep,
                        "hibernate" => PowertrayAction::Hibernate,
                        "shutdown" => PowertrayAction::Shutdown,
                        "restart" => PowertrayAction::Restart,
                        "logout" => PowertrayAction::Logout,
                        _ => {
                            println!("Invalid action");
                            return Ok(());
                        }
                    },
                    None => {
                        println!("No power action provided");
                        return Ok(());
                    }
                };
                actions::powertray::powertray(power_action);
                Ok(())
            }
            "set_brightness" => {
                let brightness_action = {
                    match args.get(2) {
                        Some(action) => match action.as_str() {
                            "up" => BrightnessAction::Up,
                            "down" => BrightnessAction::Down,
                            _ => {
                                println!("Invalid action");
                                return Ok(());
                            }
                        },
                        None => {
                            println!("No brightness action provided");
                            return Ok(());
                        }
                    }
                };
                actions::set_brightness::set_brightness(brightness_action);
                Ok(())
            }
            "set_volume" => {
                let volume_action = {
                    match args.get(2) {
                        Some(action) => match action.as_str() {
                            "up" => actions::set_volume::VolumeAction::Up,
                            "down" => actions::set_volume::VolumeAction::Down,
                            _ => {
                                println!("Invalid action");
                                return Ok(());
                            }
                        },
                        None => {
                            println!("No volume action provided");
                            return Ok(());
                        }
                    }
                };
                actions::set_volume::set_volume(volume_action);
                Ok(())
            }
            "showbar" => {
                let res = actions::showbar::show_bar();
                println!("{}", res);
                Ok(())
            }
            "volume" => {
                let res = actions::volume::get_volume();
                println!("{}", res);
                Ok(())
            }
            "workspaces" => {
                let workspaces_action = match args.get(2) {
                    Some(action) => match action.as_str() {
                        "up" => Some(WorkspacesAction::Up),
                        "down" => Some(WorkspacesAction::Down),
                        _ => {
                            println!("Invalid action");
                            return Ok(());
                        }
                    },
                    None => None,
                };
                actions::workspaces::get_workspaces(workspaces_action);
                Ok(())
            }
            _ => Ok(()),
        },
        None => Ok(()),
    }
}
