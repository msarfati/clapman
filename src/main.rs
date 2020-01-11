#[macro_use]
extern crate clap;
extern crate man;

use clap::{App, Arg, SubCommand};
use man::prelude::{Arg as ManArgs, Author, Flag, Env, Manual, Opt, Example, Section};
use std::fs;

const MAN_PAGE_FILEPATH: &str = "mastercp.1";

fn main() {
    let app = App::new("Master Control Program")
    .bin_name("mastercp")
    .version("1.0")
    .author(crate_authors!())
    .about("Controls all the programs masterfully")
    .arg(Arg::with_name("config")
         .short("c")
         .long("config")
         .value_name("FILE")
         .help("Sets a custom config file")
         .takes_value(true))
    .arg(Arg::with_name("INPUT")
         .help("Sets the input file to use")
         .required(true)
         .index(1))
    .arg(Arg::with_name("v")
         .short("v")
         .multiple(true)
         .help("Sets the level of verbosity"))
    .subcommand(SubCommand::with_name("test")
                .about("controls testing features")
                .version("1.3")
                .author("Someone E. <someone_else@other.com>")
                .arg(Arg::with_name("debug")
                    .short("d")
                    .help("print debug information verbosely")));

    // app.clone().get_matches();

    generate_manpage(app);
}

fn generate_manpage(app: App) {

    let mut manpage: Manual;

    // Check if there's a binary name, and use that to title them man page
    if let Some(bin_name) = app.get_bin_name() {
        manpage = Manual::new(bin_name);
    } else {
        manpage = Manual::new("untitled");
    }

    // Check about
    if let Some(about) = app.p.meta.about {
        manpage = manpage.about(about);
    }

    // Not all clap applications will use crate_authors
    let authors = crate_authors!("\n");
    println!("{}", authors);
    // manpage = manpage.author(Author::new("Alice Person").email("alice@person.com"));

    fs::write(MAN_PAGE_FILEPATH, manpage.render()).expect("Couldn't write manpage out.");

    // println!("{}", app.p.meta.name);
    // println!("{}", app.p.meta.author.unwrap());
    // manpage.about("ASdfwef");

    // let manpage = Manual::new("basic")
    //     .about("A basic example")
    //     .author(Author::new("Alice Person").email("alice@person.com"))
    //     .author(Author::new("Bob Human").email("bob@human.com"))
    //     .flag(
    //         Flag::new()
    //             .short("-d")
    //             .long("--debug")
    //             .help("Enable debug mode"),
    //     )
    //     .flag(
    //         Flag::new()
    //             .short("-v")
    //             .long("--verbose")
    //             .help("Enable verbose mode"),
    //     )
    //     .option(
    //         Opt::new("output")
    //             .short("-o")
    //             .long("--output")
    //             .help("The file path to write output to"),
    //     )
    //     .example(
    //         Example::new()
    //             .text("run basic in debug mode")
    //             .command("basic -d")
    //             .output("Debug Mode: basic will print errors to the console")
    //         )
    //     .custom(
    //         Section::new("usage note")
    //             .paragraph("This program will overwrite any file currently stored at the output path")
    //     )
    //     .render();

    // fs::write(MAN_PAGE_FILEPATH, manpage).expect("Couldn't write manpage out.");
}

    // let authors_l = vec![Author::new("Michael S").email("msarf@awefwef.com")];
    // let flags_l = vec![Flag::new().help("my_flag1")];
    // let opts_l = vec![Opt::new("my_opt")];
    // let envs_l = vec![Env::new("dev")];
    // let args_l = vec![ManArgs::new("my_arg1")];
    // let custom_sections_l = vec![Section::new("my_custom1")];
    // let examples_l = vec![Example::new().command("my mastercp command")];

    // let manpage = Manual {
    //     name: "myname".to_string(),
    //     about: Some("about".to_string()),
    //     description: Some("desc".to_string()),
    //     authors: authors_l,
    //     flags: flags_l,
    //     options: opts_l,
    //     environment: envs_l,
    //     arguments: args_l,
    //     custom_sections: custom_sections_l,
    //     examples: examples_l,
    // };
