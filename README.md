# rust-chores
Manage houshold chores using Rust.
Experimenting with Rust.
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
`./chores report`
```
╭───────────────┬──────────────────────┬───────┬───────────┬────────────┬────────────╮
│ name          │ description          │ level │ frequency │ last       │ next       │
├───────────────┼──────────────────────┼───────┼───────────┼────────────┼────────────┤
│ kasia         │ naczynia,            │ 2     │ 1         │ 2024-05-12 │ 2024-05-13 │
│ ziuta         │ porządki,            │ 2     │ 1         │ 2024-05-02 │ 2024-05-03 │
│ wiktor        │ mycie szafek,        │ 2     │ 7         │ 2024-04-29 │ 2024-05-06 │
│ kotek mruczek │ mycie zlewu w kuchni │ 2     │ 7         │ 2024-05-02 │ 2024-05-09 │
╰───────────────┴──────────────────────┴───────┴───────────┴────────────┴────────────╯
```
### Usage of clap library:
```
Managing household chores with ease

Usage: chores [OPTIONS] <COMMAND>

Commands:
  person      A command that manages persons in the household
  chore       A command that manages all chores
  report      Prints report for all persons, chores and assignments
  assignment  Assigns a person to a chore
  task
  help        Print this message or the help of the given subcommand(s)

Options:
  -d, --dbpath <DBPATH>  [default: test.db]
  -h, --help             Print help
  -V, --version          Print version
```

# CLI API
* person 
    * add --name NAME
    * remove --index INDEX
    * list 
    * help 
* chore
    * add --descrioption DESCRIPTION --level LEVEL -f FREQ_DAYS
    * remove --index INDEX
    * list 
    * help 
* assignment 
    * add --person PERSON --chore CHORE
    * remove --index INDEX
    * list 
    * help 
* report 
* task --person PERSON --chore CHORE --date DATE 
* help



# Polishing
- [ ] make logic and db operations part of separate lib create
- [ ] make CLI an application create use the lib
- [ ] when called without arguments, let the app start in interactive mode
- [ ] make autocompletion work for CLI version
- [ ] create web version

# Functional additions
These are currently out of scope (but nice to have):
- generate printable (pdf) for each person with their montly calendar
- allow moving chores from one person to other
  - forever
  - only temporarily
- generate historical statistics
    - expected/done ratio
    - delays
    - moves
    - workload per person
    - time spent (can use `level` column of `chore` table)

## To think
- how to design interactive flow in declarative way (so that the flow can be freely edited with minimal disruption and changes to the code?)
