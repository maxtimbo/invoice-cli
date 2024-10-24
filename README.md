# Invoice-CLI Work in Progress

I wanted to learn how to write in Rust. I'm using this project to practice. I don't recommend you use this until this readme is more fleshed out as the project will likely be closer to completion. In the meantime, have a look at the src to view my progress.

## Roadmap
- [x] Basic CLI elements using clap
- [x] Database generation
- [x] Basic CRUD for SQlite using rusqlite
- [x] More complex logic for generating invoices
- [x] Create tables by inputting a json
- [x] Figure out how to handle logo images for the company table
- [x] Generate HTML using Tera
- [x] Refine default.html
- [x] Refactor how currency is handled
- [ ] Fix template editor
- [x] Fix generate invoice notes editor
- [x] Add item creation during invoice generation
- [ ] Add configuration settings
- [ ] Add automatic emails
- [x] Generate PDF from HTML
- [ ] ?

## Refactor Roadmap
- [ ] Fix how the CRUD system is implemented
- [ ] Remove the CachedStmt struct(?) and [x] all the traits associated with it
- [ ] Cleaner code...

## Maybe Features?
- Automatically send generated pdf to company email and client email
- Check if Invoice has been paid
- Send reminders as due date closes

## Installation

1. Clone the repo and `cd` to the dir:

```
$ git clone https://github.com/maxtimbo/invoice-cli.git
$ cd invoice-cli
```
> [!NOTE]
> Running this app will automatically create the following directory tree in `~/.local/share`
> ```
> ~/.local/share/invoice-cli
> ├── imgs
> ├── invoice-cli.db
> └── templates
>     └── default.html
> ```


2. Build with cargo:

```
$ cargo build
```
Or just run:
```
$ cargo run
```

## Brief Usage

Check out the `example.json` to see how a json file should be structured. Optional fields should be ommitted. You can create as many entities as you want with one json file.
