use crate::utils::{extract_pallete, Color, ColorA};

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

pub struct ColorData {
    dial_color: Color,
    hour_color: Color,
    minute_color: Color,
    second_color: Color,
    marker_filled_color: ColorA,
    marker_empty_color: ColorA,
    text_color: Color,
    text_bg_color: Color,
    bar_main_bg: ColorA,
    bar_bg: ColorA,
    bar_fg: Color,
    bar_border: ColorA,
    bar_shadow: ColorA,
}

impl ColorData {
    pub fn init(image_path: &str) -> Self {
        let mut colors = extract_pallete(image_path, 6, 7).unwrap();
        colors.sort_by(|a, b| a.luminance().partial_cmp(&b.luminance()).unwrap());

        let dial_color = colors[0].invert();
        let hour_color = colors[1].clone();
        let minute_color = colors[2].clone();
        let second_color = colors[3].clone();

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

        ColorData {
            dial_color,
            hour_color,
            minute_color,
            second_color,
            marker_filled_color,
            marker_empty_color,
            text_color,
            text_bg_color,
            bar_main_bg,
            bar_bg,
            bar_fg,
            bar_border,
            bar_shadow,
        }
    }
}

pub fn get_eww_css(color_data: &ColorData) -> String {
    let dial_color = &color_data.dial_color;
    let hour_color = &color_data.hour_color;
    let minute_color = &color_data.minute_color;
    let second_color = &color_data.second_color;
    let marker_filled_color = &color_data.marker_filled_color;
    let marker_empty_color = &color_data.marker_empty_color;
    let text_color = &color_data.text_color;
    let text_bg_color = &color_data.text_bg_color;
    let bar_main_bg = &color_data.bar_main_bg;
    let bar_bg = &color_data.bar_bg;
    let bar_fg = &color_data.bar_fg;
    let bar_border = &color_data.bar_border;
    let bar_shadow = &color_data.bar_shadow;
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

pub fn get_wofi_css(color_data: &ColorData) -> String {
    let hour_color = &color_data.hour_color;
    let bar_main_bg = &color_data.bar_main_bg;
    let bar_bg = &color_data.bar_bg;
    let bar_fg = &color_data.bar_fg;
    let bar_border = &color_data.bar_border;
    let bar_shadow = &color_data.bar_shadow;

    let css = format!(
        r#"
window {{
    margin: 0px;
    border: 5px solid 1e1e2e;
    background-color: {bar_main_bg};
    border-radius: 10px;
    box-shadow: 2px 2px 2px {bar_shadow};
}}

#input {{
    padding: 4px;
    margin: 4px;
    padding-left: 20px;
    border: none;
    color: #cdd6f4;
    font-weight: bold;
    background-color: {bar_bg};
    /* background: ; */
   	outline: none;
    border-radius: 10px;
    margin: 10px 13px;
    margin-bottom: 2px;
}}
#input:focus {{
    border: 0px solid {bar_border};
    margin-bottom: 0px;
}}

#inner-box {{
    margin: 4px;
    border: 10px solid {bar_border};
    color: {bar_fg};
    font-weight: bold;
    background-color: #1e1e2e;
    /* background-color: red; */
    border-radius: 15px;
}}

#outer-box {{
    margin: 0px;
    border: 1.5px solid #ffffff;
    border-radius: 10px;
    background-color: {bar_main_bg};
}}

#scroll {{
    margin-top: 5px;
    border: none;
    border-radius: 15px;
    margin-bottom: 5px;
}}

#img{{
    margin-right: 10px;
}}

#img:selected {{
    background-color: {hour_color};
    margin-right: 10px;
    border-radius: 10px;
}}

#text:selected {{
    color: {bar_fg};
    color: black;
    margin: 0px 0px;
    border: none;
    border-radius: 10px;
    background-color: {bar_bg};
}}

#entry {{
    margin: 0px 0px;
    border: none;
    border-radius: 15px;
    background-color: transparent;
}}

#entry:selected {{
    margin: 0px 0px;
    border: none;
    border-radius: 5px;
    background-color: {bar_main_bg};
}}"#
    );

    css
}
