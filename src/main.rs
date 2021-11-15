use chrono::{Duration, NaiveDateTime, Utc};
use oracle::Connection;

#[macro_use]
extern crate clap;
use clap::App;

mod shift;
use shift::Shift;

mod requests;
use requests::*;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let matches = App::from_yaml(yaml).get_matches();

    let shifts: Vec<Shift> = vec![
        Shift {
            system: "ECAL",
            shift: "DOC",
        },
        Shift {
            system: "ECAL",
            shift: "DG Lieutenant",
        },
        Shift {
            system: "ECAL",
            shift: "PFG expert",
        },
        Shift {
            system: "ECAL",
            shift: "trigger expert on call",
        },
    ];

    let start = match NaiveDateTime::parse_from_str(
        matches.value_of("start").unwrap_or(""),
        &"%d-%m-%Y %H:%M:%S",
    ) {
        Ok(v) => v,
        Err(_e) => (Utc::now() - Duration::days(1)).naive_utc(),
    };
    let end = match NaiveDateTime::parse_from_str(
        matches.value_of("end").unwrap_or(""),
        &"%d-%m-%Y %H:%M:%S",
    ) {
        Ok(v) => v,
        Err(_e) => Utc::now().naive_utc(),
    };

    println!(
        "# PFG Report for the period from  {} until {}",
        start.format("%d-%m-%Y %H:%M:%S"),
        end.format("%d-%m-%Y %H:%M:%S")
    );

    // allow unwrap as they are required options
    let user = matches.value_of("user").unwrap();
    let password = matches.value_of("password").unwrap();
    let host = matches.value_of("host").unwrap();

    let conn = match Connection::connect(user, password, host) {
        Ok(v) => v,
        Err(e) => {
            println!("Cannot connect to database {} as {}", host, user);
            println!("{}", e);
            return;
        }
    };

    // shifters
    for shift in shifts {
        print_shifter_info(shift, &conn);
    }

    // LHC fills
    print_lhc_fills(&conn, start, end);
}
