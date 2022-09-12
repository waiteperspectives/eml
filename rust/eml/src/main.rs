mod eventmodel;
mod ingest;
mod parse;
mod svg;
mod utils;

use clap::Parser;
use indoc::indoc;
use parse::parse;
use std::fs::File;
use std::io;
use svg::*;
use utils::newid;

fn demo() {
    let mut doc = SvgDocument {
        id: newid(),
        width: 1000f64,
        height: 1000f64,
        cards: Vec::new(),
        arrows: Vec::new(),
        swimlane: Swimlane::new(),
    };
    let input = indoc! {r#"
        # eml: 0.0.1

        form CustomerForm {
            name: "Bob"
            age: "21"
            email: "bob@example.com"
        }

        command AddCustomer {
            name: "Bob"
            age: "21"
            email: "bob@example.com"
        }

        event CustomerAdded {
            name: "Bob"
            age: "21"
            email: "bob@example.com"
        }


        view AccountsToAdd {
          | CustomerId | state  |
          |------------|--------|
          | 123        | done   |
          | 456        | todo   |
          | 789        | todo   |
        }

        flow { CustomerForm => AddCustomer => CustomerAdded }

        job ProcessAccountsToAdd {}

        command AddAccount {
            CustomerId: "123"
            Name: "Bob"
        }

        event AccountAdded {
            CustomerId: "123"
            Name: "Bob"
        }

        flow { CustomerAdded => AccountsToAdd }
        flow { AccountsToAdd => ProcessAccountsToAdd => AddAccount => AccountAdded }

        view UsersToAdd {
          | CustomerId | state  |
          |------------|--------|
          | 123        | done   |
          | 456        | todo   |
          | 789        | todo   |
        }

        job ProcessUsersToAdd {}

        command AddUser {
            Name: "Bob"
            Login: "Bob"
        }

        event UserAdded {
            Name: "Bob"
            Login: "Bob"
        }

        flow { CustomerAdded => UsersToAdd }
        flow { UsersToAdd => ProcessUsersToAdd => AddUser => UserAdded }


    "#};
    let model = parse(input).unwrap();
    doc.ingest_expressions(model.expressions);
    let config = SvgConfig {
        pad: 150f64,
        card_width: 300f64,
        card_height: 150f64,
    };
    doc.set_dimensions(&config);
    let svg_string = doc.render();
    println!("{}", svg_string);
}

fn process(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut doc = SvgDocument {
        id: newid(),
        width: 1000f64,
        height: 1000f64,
        cards: Vec::new(),
        arrows: Vec::new(),
        swimlane: Swimlane::new(),
    };
    let model = parse(input)?;
    doc.ingest_expressions(model.expressions);
    let config = SvgConfig {
        pad: 150f64,
        card_width: 300f64,
        card_height: 150f64,
    };
    doc.set_dimensions(&config);
    let svg_string = doc.render();
    Ok(svg_string)
}

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// eml input or filepath
    #[clap(value_parser, default_value = "-")]
    input: String,
}

fn read_input(input: &str) -> Result<String, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    let mut rdr: Box<dyn io::Read> = match input {
        "-" => Box::new(io::stdin()),
        _ => Box::new(File::open(input)?),
    };
    rdr.read_to_string(&mut buf)?;
    Ok(buf)
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();
    let input = read_input(args.input.as_str())?;
    let output = process(&input)?;
    println!("{}", output);
    Ok(())
}
