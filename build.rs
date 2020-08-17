extern crate dirs;
use std::fs;

fn main() {
    let figlet_location = format!(
        "{}/.config/buacshell-rs/figlet",
        dirs::home_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_str()
            .unwrap(),
    );

    println!("{}",figlet_location);

    fs::copy("./resources/figlet", figlet_location).unwrap();
}
