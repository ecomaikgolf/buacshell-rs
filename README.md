# BUACShell [Deprecated]
This project is a Rust port from [BUACShell](https://github.com/ecomaikgolf/BUACShell) with a better password handling

## Installation
```
cargo install --git https://github.com/ecomaikgolf/buacshell-rs
```

Requirements:
 * GPG Key

Operating Systems:
 * GNU/Linux
   * Archlinux (Tested)

## Features
* Fast & Secure (Rust)
* Passwords protected with [libknox](https://github.com/apognu/knox/tree/master/libknox)
* Easy build from source (cargo)
* Scriptable
* Open Source!

## Usage
```
BUACShell 1.0
Ernesto Martínez García <ecomaikgolf@protonmail.com>
Unofficial terminal application for the University of Alicante library

USAGE:
    buacshell-rs [SUBCOMMAND]

FLAGS:
    -h, --help       Prints help information
    -V, --version    Prints version information

SUBCOMMANDS:
    help      Prints this message or the help of the given subcommand(s)
    loans     List library loans and due dates
    reinit    Re-Initialise the app DB
    renew     Renew library loans
 ```

### Examples

```
$ buacshell-rs loans

C++ crash course : a fast-paced Introduction
Lospinoso, Josh
POE 004.43/C++/LOS/C++
27/10/2020,21:00

Practical malware analysis : the hands-on guide to dissecting malicious software
Sikorski, Michael
POE 004.056/SIK/PRA
27/10/2020,21:00

C++ standard library practical tips
Reese, Greg
POE 004.43/ C++/REE/C++
27/10/2020,21:00

Fundamentos de algoritmia
Brassard, Gilles
POE 004.421/BRA/FUN
27/10/2020,21:00

Estructura de datos en C++
Joyanes Aguilar, Luis
POE 004.43/C++/JOY/EST
27/10/2020,21:00
```

```
$ buacshell-rs renew

Loans renewed
```

```
$ buacshell-rs renew loans

Loans renewed

C++ crash course : a fast-paced Introduction
Lospinoso, Josh
POE 004.43/C++/LOS/C++
27/10/2020,21:00

Practical malware analysis : the hands-on guide to dissecting malicious software
Sikorski, Michael
POE 004.056/SIK/PRA
27/10/2020,21:00

C++ standard library practical tips
Reese, Greg
POE 004.43/ C++/REE/C++
27/10/2020,21:00

Fundamentos de algoritmia
Brassard, Gilles
POE 004.421/BRA/FUN
27/10/2020,21:00

Estructura de datos en C++
Joyanes Aguilar, Luis
POE 004.43/C++/JOY/EST
27/10/2020,21:00
```

```
$ buacshell-rs reinit

*Prompt for BUA credentials, etc*
```

## TODO
- [ ] Better strings (&str or Box<str> instead of String in bua.rs)
- [ ] Check if GPG Key exists in initialisation.rs
- [ ] Why `get_loans(dni: &str, nip: &str` in bua.rs is reading one book (iteration) of "garbage" (line 218)
- [ ] Change GPG Key selector to a numerical selection ¿?
