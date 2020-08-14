use libknox::VaultContext;
use select::document::Document;
use select::predicate::{Class, Name};
use std::fmt;
use colorful::{Color, Colorful};

static BUA_URL: &str = "http://gaudi.ua.es";
static BUA_LOGIN: &str = "/uhtbin/cgisirsi/0/0/0/29/124/X/3";
static USER_AGENT: &str = "github.com/ecomaikgolf/buacshell-rs";

static TIMEOUT: u64 = 10;

pub struct Book {
    // Beter with &str/Box<str>
    name: String,
    author: String,
    code: String,
    date: String,
}

impl fmt::Display for Book {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}\n{}\n{}\n{}\n",
            self.name[..].color(Color::Orange1),
            self.author,
            self.code,
            self.date[..].color(Color::Green1)
        )
    }
}

/// Returns true/false depending if dni/nip is correct
///
/// # Arguments
///
/// * `dni` - BUA login DNI (starting zero and no letter)
/// * `nip` - BUA login password (numerical)
pub fn check_credentials(dni: &str, nip: &str) -> bool {
    let token = self::get_random_token();
    let request = minreq::post(&format!("{}{}", BUA_URL, &token[..]))
        .with_body(format!{"user_id={}&password={}", dni, nip})
        .with_header("User-Agent", USER_AGENT)
        .with_timeout(TIMEOUT)
        .send()
        .unwrap();

    let html_string = String::from_utf8_lossy(request.as_bytes()).to_string();
    return !html_string.contains("<p><strong>Acceso inv");
}

/// Prints current loans and expiration dates
pub fn print_loans() {
    let credentials = self::get_credentials();
    // cleaner with lambda
    for book in self::get_loans(&credentials.0[..], &credentials.1[..]) {
        println!("{}", book);
    }
}

/// Renews all non-reserved pending loans
///
/// # Return
/// The number of non-renewed books (0 means everything went well)
pub fn renew_all_loans() {
    let credentials = self::get_credentials();
    let loans = self::renew_loans(&credentials.0[..], &credentials.1[..]);
    match loans {
        Ok(_) => println!("{}", "Loans renewed".color(Color::Green)),
        Err(n) => {
            println!(
                "{}",
                format!("{} book(s) reserved", n).color(Color::Red)
            )
        }
    }
}

/// Requests initial random token
///
/// # Return
/// Random token (in the form of URL path)
fn get_random_token() -> String {
    let request = minreq::get(&format!("{}{}", BUA_URL, BUA_LOGIN)[..])
        .with_header("User-Agent", USER_AGENT)
        .with_timeout(TIMEOUT)
        .send()
        .unwrap();

    let html_string = String::from_utf8_lossy(request.as_bytes()).to_string();
    let parser = Document::from(&html_string[..]);
    return parser
        .find(select::predicate::Attr("name", "accessform"))
        .next()
        .unwrap()
        .attr("action")
        .unwrap()
        .to_string();
}

/// Requests user credentials to libknox
///
/// # Return
/// * `(dni, _)` - BUA login DNI
/// * `(_, nip)` - BUA login password
fn get_credentials() -> (String, String) {
    let libknox_location = format!(
        "{}{}",
        dirs::home_dir()
            .unwrap_or(std::path::PathBuf::new())
            .to_str()
            .unwrap(),
        super::CREDENTIALS_LOCATION
    );

    let vault = VaultContext::open(libknox_location).expect("FAIL");
    let entry = vault.read_entry("bua").expect("FAIL");

    return (
        String::from(
            entry
                .get_attributes()
                .get_key_value("dni")
                .unwrap()
                .1
                .get_value(),
        ),
        String::from(
            entry
                .get_attributes()
                .get_key_value("nip")
                .unwrap()
                .1
                .get_value(),
        ),
    );
}

/// Renews all non-reserved loans
///
/// # Arguments
///
/// * `dni` - BUA login DNI (starting zero and no letter)
/// * `nip` - BUA login password (numerical)
fn renew_loans(dni: &str, nip: &str) -> Result<(), u8> {
    let mut token = self::get_random_token();
    minreq::post(&format!("{}{}", BUA_URL, &token[..]))
        .with_body(format!{"user_id={}&password={}", dni, nip})
        .with_header("User-Agent", USER_AGENT)
        .with_timeout(TIMEOUT)
        .send()
        .unwrap();

    token.push('3');
    let auth = minreq::post(&format!("{}{}", BUA_URL, &token[..]))
        .with_body(format!{"user_id={}&selection_type=all", dni})
        .with_header("User-Agent", USER_AGENT)
        .with_timeout(TIMEOUT)
        .send()
        .unwrap();

    let html_string = String::from_utf8_lossy(auth.as_bytes()).to_string();
    let non_renewed = html_string.matches("El documento tiene reserva").count() as u8;

    return match non_renewed {
        0 => Ok(()),
        _ => Err(non_renewed),
    };
}

/// Returns user pending loans
///
/// # Arguments
///
/// * `dni` - BUA login DNI (starting zero and no letter)
/// * `nip` - BUA login password (numerical)
///
/// # Return
/// Vector of Books (see Book struct)
fn get_loans(dni: &str, nip: &str) -> Vec<Book> {
    let token = self::get_random_token();
    let request = minreq::post(&format!("{}{}", BUA_URL, &token[..]))
        .with_body(format!{"user_id={}&password={}", dni, nip})
        .with_header("User-Agent", USER_AGENT)
        .with_timeout(TIMEOUT)
        .send()
        .unwrap();

    let html_string = String::from_utf8_lossy(request.as_bytes()).to_string();
    let parser = Document::from(&html_string[..]);

    let mut book_data: Vec<Book> = Vec::new();
    for node in parser.find(Name("tr")) {
        for book in node.find(select::predicate::Predicate::or(
            Class("itemlisting2"),
            Class("itemlisting"),
        )).take(1)
        {
            let mut iter = book.children()
                .skip(1)
                .next()
                .unwrap()
                .attr("name")
                .unwrap()
                .split("^")
                .skip(2);

            book_data.push(
                Book {
                    code: iter.next().unwrap().to_string(),
                    author: iter.clone().skip(1).next().unwrap().to_string(),
                    name: iter.skip(2).next().unwrap().to_string(),
                    date: node.find(Name("strong")).next().unwrap().text(),
                });
        }
    }
    book_data.remove(0); //Trash
    return book_data;
}
