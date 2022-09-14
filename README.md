# eml

## Event Modeling Language

DSLs and tooling for working with event models

## eml

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

Output: ![examples/eml_basic/model.svg](examples/eml_basic/model.svg)

## emlyml

`emlyml` compiles svg event models from a yaml-based dsl. This is the original
Python project.

```
usage: emlyml [-h] {compile,demo} ...

positional arguments:
  {compile,demo}  commands
    compile       compile a yaml model into svg
    demo          generate a demo model.yaml

options:
  -h, --help      show this help message and exit
```

### Example

```yaml
# Sample event model dsl using yaml
---
# Customer Entry
# -------------------------
- Form:
    id: CustomerForm
    text: |
      + Name: Bob
      + Age: 21
      + Email: bob@example.com

- Command:
    id: AddCustomer

- Event:
    id: CustomerAdded

- =>: { begin_at: CustomerForm, end_at: AddCustomer }
- =>: { begin_at: AddCustomer, end_at: CustomerAdded }

# Account Addition
# -------------------------
- View:
    id: AccountsToAdd
    text: |
      + CustomerId: 123
      + State: TODO

- Job:
    id: ProcessAccountsToAdd

- Command:
    id: AddAccount
    text: |
      + CustomerId: 123
      + Name: Bob
      + OpenDate: 2022-01-01

- Event:
    id: AccountAdded
    text: |
      + CustomerId: 123
      + Name: Bob

- =>: { begin_at: CustomerAdded, end_at: AccountsToAdd }
- =>: { begin_at: AccountsToAdd, end_at: ProcessAccountsToAdd }
- =>: { begin_at: ProcessAccountsToAdd, end_at: AddAccount }
- =>: { begin_at: AddAccount, end_at: AccountAdded }

# User Addition
# -------------------------
- View:
    id: UsersToAdd
    text: |
      + CustomerId: 123
      + State: TODO

- Job:
    id: ProcessUsersToAdd

- Command:
    id: AddUser
    text: |
      + Name: Bob
      + Login: bob

- Event:
    id: UserAdded

- =>: { begin_at: CustomerAdded, end_at: UsersToAdd }
- =>: { begin_at: UsersToAdd, end_at: ProcessUsersToAdd }
- =>: { begin_at: ProcessUsersToAdd, end_at: AddUser }
- =>: { begin_at: AddUser, end_at: UserAdded }
```

Output: ![examples/emlyml_python/model.svg](examples/emlyml_python/model.svg)
