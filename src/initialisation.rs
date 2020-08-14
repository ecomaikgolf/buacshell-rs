use libknox::{VaultContext, Entry};
use rpassword;

use colorful::{Color, Colorful};

use std::process::Command;
use std::io::Write;
use std::io::prelude::Read;
use std::fs;
use dirs;

/// Inits the database if doesn't exist
pub fn auto_init() {
    let libknox_location = format!(
        "{}{}",
        dirs::home_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_str()
            .unwrap(),
        super::CREDENTIALS_LOCATION
    );

    if !std::fs::metadata(libknox_location).is_ok() {
        self::initialise_db();
    }
}

/// Deletes and re-initialises the database
pub fn re_initialise() {
    let libknox_location = format!(
        "{}{}",
        dirs::home_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_str()
            .unwrap(),
        super::CREDENTIALS_LOCATION
    );

    match std::fs::remove_dir_all(libknox_location) {
        Ok(_) => self::initialise_db(),
        Err(_) => self::initialise_db(),
    }
}

/// Initialises the database
fn initialise_db() {
    /* Clear screen  (better with termion) */
    print!("\x1B[2J\x1B[1;1H");

    /* Banner */
    let figlet = fs::read_to_string("resources/figlet").expect("error");
    println!(
        "{}",
        figlet.gradient_with_color(
            colorful::RGB::new(84, 51, 255),
            colorful::RGB::new(165, 254, 203),
        )
    );

    /* secure database location (relative to $PATH) */
    let libknox_location = format!(
        "{}{}",
        dirs::home_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_str()
            .unwrap(),
        super::CREDENTIALS_LOCATION
    );

    println!(
        "[*] Please provide an {} associated with your personal {} (user@domain.ext)",
        "email".color(Color::Orange1),
        "GnuPGP Key".color(Color::Orange1)
    );

    println!(
        "[*] Keys available: {}",
        "(gpg --list-secret-keys)".color(Color::Grey53)
    );

    let gpg_list = Command::new("sh")
        .arg("-c")
        .arg("gpg --list-secret-keys | grep \"@\" | sed 's/.*] / - /g'")
        .output()
        .expect("Failure");
    std::io::stdout().write_all(&gpg_list.stdout).unwrap();

    /* Read gpg_email */
    let mut gpg_email = String::new();
    print!("{}", "[>] Email: ".color(Color::Blue));
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut gpg_email).expect(
        "IO Error",
    );
    gpg_email.pop();

    /* TODO
    println!("[*] Checking if '{}' is valid", gpg_email.clone().color(Color::Orange1));
    let check_email = Command::new("sh")
        .arg("-c")
        .arg(format!("gpg --list-secret-keys | grep -x \"{}\"", gpg_email))
        .status()
        .expect("Failure");

    if check_email.success() {
        panic!("Invalid email, key not found");
    } else {
        println!("[*] '{}' is {}", gpg_email.clone().color(Color::Orange1), "valid".color(Color::Green));
    }
    */

    /* Create secure db */
    println!(
        "[*] Initialising secure libknox database at '{}'",
        libknox_location.clone().color(Color::Orange1)
    );

    let id = vec![gpg_email];
    let mut vault = VaultContext::create(&libknox_location, &id)
        .unwrap_or_else(|_err| panic!("libknox db exists?"));
    let mut bua = Entry::new();

    /* Store BUA credentials */
    let mut bua_dni = String::new();
    print!("{}", "[>] DNI (011111111): ".color(Color::Blue));
    std::io::stdout().flush().unwrap();
    std::io::stdin().read_line(&mut bua_dni).expect("IO Error");

    print!("{}", "[>] NIP: ".color(Color::Blue));
    std::io::stdout().flush().unwrap();
    let bua_nip = rpassword::read_password().unwrap();

    if !super::bua::check_credentials(&bua_dni[..], &bua_nip[..]) {
        println!("{}", "[!] Invalid credentials".color(Color::Red));
        std::process::exit(1);
    }

    bua.add_attribute("dni", &bua_dni[..]);
    bua.add_confidential_attribute("nip", &bua_nip[..]);
    vault.write_entry("bua", &bua).expect("FAIL");

    println!("{}", "[*] Finished ;)".color(Color::Green));

    write!(std::io::stdout(), "Press any key to continue... ").unwrap();
    std::io::stdout().flush().unwrap();
    std::io::stdin().read(&mut [0u8]).unwrap();

    /* Clear screen */
    print!("\x1B[2J\x1B[1;1H");
}
