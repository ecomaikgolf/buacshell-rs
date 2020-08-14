use clap::{App};

mod initialisation;
mod bua;

static CREDENTIALS_LOCATION:&'static str = "/.config/buacshell-rs/credentials.libknox";

fn main() {
    let matches = App::new("BUACShell")
        .version("1.0")
        .author("Ernesto Martínez García <ecomaikgolf@protonmail.com>")
        .about("Unofficial terminal application for the University of Alicante library")
        .subcommand(App::new("renew")
             .about("Renew library loans")
             .subcommand(App::new("loans")
                         .about("Print loans after renewing")))
        .subcommand(App::new("loans")
             .about("List library loans and due dates"))
        .subcommand(App::new("reinit")
             .about("Re-Initialise the app DB"))
        .get_matches();

    initialisation::auto_init();

    match matches.subcommand() {
        ("reinit", Some(_args)) => initialisation::re_initialise(),
        ("renew",  Some(args)) => {
            bua::renew_all_loans();
            match args.subcommand() {
                ("loans",  Some(_args)) => {
                    println!("");
                    bua::print_loans();
                },
                (_, _) => return,
            }
        },
        ("loans",  Some(_args)) => bua::print_loans(),
        (_, _) =>  return,
    }
}
