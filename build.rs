extern crate dirs;
use std::fs;

fn main() {
    let config_location = format!(
        "{}/.config/buacshell-rs/",
        dirs::home_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_str()
            .unwrap(),
    );

    fs::create_dir_all(&config_location).unwrap();
    fs::copy("./resources/figlet", format!("{}/figlet", &config_location)).unwrap();
}
