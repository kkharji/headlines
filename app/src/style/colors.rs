macro_rules! colorscheme {
    ($($name:ident => { $($mode:ident => $color:expr),* $(,)? }),* $(,)?) => {
        paste::paste! {
            use eframe::epaint::Color32;
            use rgba_simple::Hex;

            $(impl crate::App {
                $(fn [<$name _ $mode>](&self) -> Color32 {
                    let c = rgba_simple::RGBA::from_hex($color).unwrap();
                    Color32::from_rgba_unmultiplied(c.red, c.green, c.blue, c.alpha)
                })+

                pub fn $name(&self) -> Color32 {
                    if self.state.mode.is_dark() { self.[<$name _dark>]() } else { self.[<$name _light>]() }
                }
            })+
        }
    };
}

colorscheme! {
    background => {
        dark => "#0d1117",
        light => "#f6f6f7",
    },
    background_2 => {
        dark => "#161C22",
        light => "#eceeef",
    },
    foreground_dark => {
        dark => "#4d5566",
        light => "#424242",
    },
    foreground_gutter => {
        dark => "#c5c5c5",
        light => "#24292e"
    },
    foreground_light => {
        dark => "#b3b1ad",
        light => "#24292e"
    },
    border => {
        dark => "#b3b1ad",
        light => "#24292e"
    },
    error => {
        dark => "#f85149",
        light => "#f85149"
    },
    warning => {
        dark => "#f0883e",
        light => "#f0883e"
    },
    black => {
        dark => "#484f58",
        light => "#24292f"
    },
    bright_black => {
        dark => "#6e7681",
        light => "#57606a"
    },
    white => {
        dark => "#b1bac4",
        light => "#6e7781"
    },
    bright_white => {
        dark => "#f0f6fc",
        light => "#8c959f"
    },
    red => {
        dark => "#ff7b72",
        light => "#cf222e"
    },
    bright_red => {
        dark => "#ffa198",
        light => "#a40e26"
    },
    green => {
        dark => "#3fb950",
        light => "#116329"
    },
    bright_green => {
        dark => "#56d364",
        light => "#1a7f37"
    },
    yellow => {
        dark => "#d29922",
        light => "#4d2d00"
    },
    bright_yellow => {
        dark => "#e3b341",
        light => "#633c01"
    },
    blue => {
        dark => "#58a6ff",
        light => "#0969da"
    },
    bright_blue => {
        dark => "#79c0ff",
        light => "#218bff"
    },
    magenta => {
        dark => "#bc8cff",
        light => "#8250df"
    },
    bright_magenta => {
        dark => "#d2a8ff",
        light => "#a475f9"
    },
    cyan => {
        dark => "#39c5cf",
        light => "#1b7c83"
    },
    bright_cyan => {
        dark => "#56d4dd",
        light => "#3192aa"
    },
}
