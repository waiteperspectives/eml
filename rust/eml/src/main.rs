mod eventmodel;
mod ingest;
mod parse;
mod svg;
mod utils;

use indoc::indoc;
use parse::parse;
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

fn main() {
    demo()
}
