# eml

`eml` is a command line tool for compiling the event modeling language dsl into
a visual representation. The target output is svg, which scales well and plays
nice with both web and standalone documents. The tool is written in Rust and
leverages the [nom](https://github.com/Geal/nom) parser-combinator library for
it's parser and [clap](https://github.com/clap-rs/clap) for the command line
interface.

```
eml 0.0.1

USAGE:
    eml [ARGS]

ARGS:
    <INPUT>     eml input: either stdin or filepath [default: -]
    <OUTPUT>    eml output: either stdout or filepath [default: -]

OPTIONS:
    -h, --help       Print help information
    -V, --version    Print version information
```

### Example

```eml
# eml: 0.0.1

# Customer Entry
# -------------------------
form CustomerForm {
  Name: John
  Age: 21
  Email: jdoe@example.com
}

command AddCustomer { use CustomerForm }
event CustomerAdded { use CustomerForm }
flow { CustomerForm => AddCustomer => CustomerAdded }

# Account Addition
# -------------------------
view AccountsToAdd {
  |   CustomerId | State   |
  |--------------|---------|
  |          123 | DONE    |
  |          456 | TODO    |
}
job ProcessAccountsToAdd {}
command AddAccount {
  CustomerId: 456
  Name: John
}
event AccountAdded { use AddAccount }
flow { CustomerAdded => AccountsToAdd => ProcessAccountsToAdd => AddAccount => AccountAdded }

# Account Addition
# -------------------------
view UsersToAdd {
  |   CustomerId | State   |
  |--------------|---------|
  |          123 | DONE    |
  |          456 | TODO    |
}
job ProcessUsersToAdd {}
command AddUser { Name: John, Login: john }
event UserAdded { use AddUser }
flow { CustomerAdded => UsersToAdd => ProcessUsersToAdd => AddUser => UserAdded }
```
