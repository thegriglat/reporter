use chrono::NaiveDateTime;
use oracle::Connection;
use std::env;

mod shift;
use shift::Shift;

mod requests;
use requests::print_shifter_info;

fn main() {
    let args: Vec<String> = env::args().collect();
    let start = NaiveDateTime::parse_from_str(&args[1], &"%d-%m-%Y %H:%M:%S");
    let end = NaiveDateTime::parse_from_str(&args[2], &"%d-%m-%Y %H:%M:%S");

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

    match start {
        Ok(_v) => {}
        Err(e) => {
            println!("Incorrect start date: {}", e);
        }
    }

    match end {
        Ok(_v) => {}
        Err(e) => {
            println!("Incorrect end date: {}", e);
        }
    }

    println!(
        "# PFG Report for the period from  {} until {}",
        start.unwrap(),
        end.unwrap()
    );

    let user = "user";
    let password = "pass";
    let host = "host";

    let conn = match Connection::connect(user, password, host) {
        Ok(v) => v,
        Err(e) => {
            println!("Cannot connect to database {} as {}", host, user);
            println!("{}", e);
            return;
        }
    };
    for shift in shifts {
        print_shifter_info(shift, &conn);
    }
}
