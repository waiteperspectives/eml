# eml

## Event Modeling Language

DSLs and tooling for working with event models

## emlyml

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

Output: ![examples/model.svg](examples/model.svg)
