# rust-chores
Manage houshold chores using Rust

# Goal
1. The primary goal is to refresh Rust a bit and create a very basic
* commandline application ([clap](https://github.com/clap-rs/clap)) that uses 
* sqlite ([rusqlite](https://github.com/rusqlite/rusqlite)) and 
* formats tables to stdout ([tabled](https://github.com/zhiburt/tabled)).

2. The seconday goal is to create a basic commandline application for managing:
- what chores exist 🌊
- how often they should be done 🕒
- who is responsible for what chore 🧚🏼 
- when the chores needs to be scheduled in someone's calendar 📅

3. And just have fun 😀

# Non-goal
* Make full-blown app
* Be perfect
* Show off


## How does it lok like:
### Output
```
╭───────┬─────────────┬───────┬───────────┬────────────╮
│ name  │ description │ level │ frequency │ last       │
├───────┼─────────────┼───────┼───────────┼────────────┤
│ kasia │ naczynia,   │ 2     │ 1         │ 2024-05-02 │
│ ziuta │ pranie,     │ 2     │ 1         │ 2024-05-02 │
╰───────┴─────────────┴───────┴───────────┴────────────╯
╭────┬───────────────╮
│ id │ name          │
├────┼───────────────┤
│ 1  │ kasia         │
│ 2  │ ziuta         │
│ 5  │ tomasz        │
│ 6  │ wiktor        │
│ 7  │ dziadek piotr │
│ 9  │ babcia marta  │
│ 10 │ kotek mruczek │
╰────┴───────────────╯
╭────┬──────────────────────┬───────┬───────────╮
│ id │ description          │ level │ frequency │
├────┼──────────────────────┼───────┼───────────┤
│ 1  │ pranie,              │ 2     │ 1         │
│ 2  │ naczynia,            │ 2     │ 1         │
│ 3  │ porządki,            │ 2     │ 1         │
│ 4  │ zmywarka             │ 2     │ 1         │
│ 5  │ odkurzanie,          │ 2     │ 7         │
│ 6  │ prysznic,            │ 2     │ 7         │
│ 7  │ mycie szafek,        │ 2     │ 7         │
│ 8  │ mycie zlewu w kuchni │ 2     │ 7         │
╰────┴──────────────────────┴───────┴───────────╯
╭────┬───────────┬──────────╮
│ id │ person_id │ chore_id │
├────┼───────────┼──────────┤
│ 1  │ 1         │ 2        │
│ 2  │ 2         │ 1        │
│ 3  │ 2         │ 3        │
╰────┴───────────┴──────────╯

```

### Usage of clap library:
Managing household chores with ease
```
Usage: chores <COMMAND>

Commands:
  person     Perform operation on a person
  add-chore  Adds a chore with description, level and frequency
  report     Prints report for all persons, chores and assignments
  assign     Assigns a person to a chore
  help       Print this message or the help of the given subcommand(s)

Options:
  -h, --help     Print help
  -V, --version  Print version
➜  chores git:(main) ✗ ./chores person
Perform operation on a person

Usage: chores person <COMMAND>

Commands:
  add     add a new person
  remove  remove a person
  help    Print this message or the help of the given subcommand(s)

Options:
  -h, --help  Print help

```
