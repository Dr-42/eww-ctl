use crate::utils::{extract_pallete, Color};

fn make_readable(fg: &Color, bg: &Color) -> (Color, Color) {
    let fg_luminnance = fg.luminance();
    let bg_luminance = bg.luminance();

    let (mut light, dark) = if fg_luminnance < bg_luminance {
        (fg.clone(), bg.clone())
    } else {
        (bg.clone(), fg.clone())
    };

    let ratio = (bg_luminance + 0.05) / (fg_luminnance + 0.05);

    if ratio < 4.5 {
        let fg_lum_new = (bg_luminance + 0.05) / 4.5 - 0.05;
        light = Color {
            r: light.r * fg_lum_new / fg_luminnance,
            g: light.g * fg_lum_new / fg_luminnance,
            b: light.b * fg_lum_new / fg_luminnance,
        };
    }

    (light, dark)
}

pub fn getcss(image_path: &str) -> String {
    let mut colors = extract_pallete(image_path, 6, 7).unwrap();
    colors.sort_by(|a, b| a.luminance().partial_cmp(&b.luminance()).unwrap());

    let dial_color = colors[0].invert();
    let hour_color = &colors[1];
    let minute_color = &colors[2];
    let second_color = &colors[3];

    let marker_filled_color = colors[4].with_alpha(0.2);
    let marker_empty_color = colors[4].with_alpha(0.1);

    let text_color = &colors[5];
    let text_bg_color = colors[5].invert();

    let (text_color, text_bg_color) = make_readable(text_color, &text_bg_color);

    let bar_main_bg = colors[3].with_alpha(0.9);
    let bar_bg = colors[0].with_alpha(0.9);
    let bar_fg = colors[0].invert();

    let bar_border = colors[1].invert().with_alpha(0.9);
    let bar_shadow = colors[2].invert().with_alpha(0.9);

    let css = format!(
        r#"
.bar {{
    background-color: {bar_main_bg};
}}

.workspaces {{
    background-color: {bar_bg};
    color: {bar_fg};
    border: 1px solid {bar_border};
    padding: 6px 20px 4px 20px;
    font-size: 16px;
    box-shadow: 2px 2px 2px {bar_shadow};
}}

.time_battery {{
    background: {bar_bg};
    color: {bar_fg};
    font-size: 16px;
    padding: 5px 15px 5px 14px; 
    border: 1px solid {bar_border};
    box-shadow: 2px 2px 2px {bar_shadow};
    text-shadow:
        0.07em 0 black,
        0 0.07em black,
        -0.07em 0 black,
        0 -0.07em black;
}}

.clockmarkerfilled {{
    color: {marker_filled_color};
    background-color: rgba(255, 0, 0, 0);
}}

.clockmarkerempty {{
    color: {marker_empty_color};
    background-color: rgba(255, 0, 0, 0);
}}

.hour {{
    background-color: {hour_color};
    color: {dial_color};
}}

.minute {{
    background-color: {minute_color};
    color: {dial_color};
}}

.second {{
    background-color: {second_color};
    color: {dial_color};
}}

.clocktext {{
    margin: 0;
    padding: 0;
    font-size: 20px;
    color: {text_color};
    background-color: {text_bg_color};
    border-radius: 150px 150px 0 0;
}}

.clockdate {{
    margin: 0;
    padding: 0;
    font-size: 13px;
    color: {text_color};
    background-color: {text_bg_color};
    border-radius: 0 0 150px 150px;
}}

.clockmarker {{
    color: #ff0000;
}}

.powertray {{
    background-color: {bar_bg};
    border-radius: 10px;
    color: {bar_fg};
    border: 1px solid {bar_border};
    padding: 6px 20px 4px 20px;
    font-size: 16px;
    box-shadow: 2px 2px 2px {bar_shadow};
    min-width: 300px;
    min-height: 80px;
}}

.powertray:hover {{
    background-color: {bar_bg};
    border-radius: 10px;
    color: {bar_fg};
    border: 1px solid {bar_border};
    padding: 6px 20px 4px 20px;
    font-size: 16px;
    box-shadow: 2px 2px 2px {bar_shadow};
    min-width: 300px;
    min-height: 80px;
}}

.powertray-button {{
    background-color: {bar_bg};
    border-radius: 10px;
    color: {bar_fg};
    border: 1px solid {bar_border};
    padding: 8px 20px 4px 20px;
    font-size: 22px;
    box-shadow: 2px 2px 2px {bar_shadow};
    min-width: 40px;
    min-height: 60px;
}}

.powertray-button:hover {{
    background-color: {bar_fg};
    border-radius: 10px;
    color: {bar_bg};
    border: 1px solid {bar_border};
    padding: 8px 20px 4px 20px;
    font-size: 22px;
    box-shadow: 2px 2px 2px {bar_shadow};
    min-width: 40px;
    min-height: 60px;
}}

.end-default-notification-box-low {{
    background-color: {bar_bg};
    padding: 12px;
    padding-left: 8px;
    margin: 12px;
    border-radius: 10px;
    border: 1px dashed blue;
}}

.end-default-notification-box-normal {{
    background-color: {bar_bg};
    padding: 12px;
    padding-left: 8px;
    margin: 12px;
    border-radius: 10px;
}}

.end-default-notification-box-critical {{
    background-color: {bar_bg};
    padding: 12px;
    padding-left: 8px;
    margin: 12px;
    border-radius: 10px;
    border: 2px solid #ff0000;
}}

.end-default-notification-title-bar {{
    background-color: {bar_bg};
    margin: 3px;
    border-bottom: 1px solid {bar_border};
}}

.end-default-notification-appicon {{
    margin-right: 10px;
    margin-bottom: 5px;
}}

.end-default-notification-appname {{
    color: {bar_fg};
    margin-right: 12px;
    font-family: 'JetBrainsMono Nerd Font';
    font-weight: bold;
    font-size: 15px;
}}

.end-default-notification-body-box {{
    margin-top: 12px;
}}

.notification-text {{
    color: {bar_fg};
    font-family: 'JetBrainsMono Nerd Font';
}}

.notification-title {{
  font-weight: bold;
  font-size: 1em;
}}

.notification-content {{
  font-size: .8em;
}}

.end-notification-buttons {{
    margin-top: 12px;
}}

.end-notification-button {{
    background-color: {bar_bg};
    color: {bar_fg};
    padding: 8px;
    border-radius: 10px;
    margin-right: 12px;
    border: 1px solid {bar_border};
}}

.end-notification-reply {{
    background-color: {bar_bg};
    color: {bar_fg};
    padding: 8px;
    border-radius: 10px;
    margin-right: 12px;
    border: 1px solid {bar_border};
}}

.end-notification-reply:hover:focus {{
    background-color: {bar_fg};
    color: {hour_color};
    padding: 8px;
    border-radius: 10px;
    margin-right: 12px;
    border: 1px solid {bar_border};
}}

.end-notification-button:hover {{
    background-color: {bar_fg};
    color: {bar_bg};
    padding: 8px;
    border-radius: 10px;
    margin-right: 12px;
    border: 1px solid {bar_border};
}}

.end-history-frame {{
    background-color: {bar_bg};
    padding: 12px;
    padding-left: 8px;
    border-radius: 10px;
}}

.end-history-box {{
    background-color: {bar_bg};
    padding: 12px;
    padding-left: 8px;
    border: 1px solid {bar_border};
}}

.end-history-title-bar {{
    background-color: {bar_bg};
    margin: 1px;
    border-bottom: 1px solid {bar_border};
}}

.end-history-appicon {{
    margin-right: 5px;
    margin-bottom: 2px;
}}

.end-history-appname {{
    color: {bar_fg};
    margin-right: 5px;
    font-family: 'JetBrainsMono Nerd Font';
    font-weight: bold;
    font-size: 10px;
}}

.end-history-body-box {{
    margin-top: 3px;
}}

.content-box {{
    margin-left: 12px;
}}

.battery-icon {{
    color: rgb(255, 0, 0);
    font-size: 2.1em;
    margin-right: 48px;
    margin-left: 24px;
}}

.evd-icon {{
  margin: 5px;
}}

.evd-box {{
  background-color: {bar_bg};
  border-radius: 10px;
}}

.evd-scale {{
  background-color: {hour_color};
  margin: 4px;
  border-radius: 4px;
}}

.evd-scale progress {{
  background-color: {second_color};
  border-radius: 4px;
}}

.evd-text {{
  color: {bar_fg};
  background-color: {bar_bg};
  font-size: 15px;
  margin: 4px;
}}
        "#,
    );

    css
}
