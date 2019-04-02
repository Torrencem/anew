extern crate clap;
use clap::{Arg, App, SubCommand};

mod lib;

use std::process::exit;
use std::path::PathBuf;
use glob::glob;

fn main() {
    let matches = App::new(env!("CARGO_PKG_NAME"))
                        .version(env!("CARGO_PKG_VERSION"))
                        .author(env!("CARGO_PKG_AUTHORS"))
                        .about(env!("CARGO_PKG_DESCRIPTION"))
                        .arg(Arg::with_name("NAME")
                                .help("The name of the template to copy"))
                        .arg(Arg::with_name("DIRECTORY")
                                .help("The directory to clone into")
                                .default_value("."))
                        .subcommand(SubCommand::with_name("create")
                                    .about("Create a new template")
                                    .arg(Arg::with_name("no-copy")
                                            .short("nc")
                                            .long("no-copy")
                                            .help("Don't create a copy of the files in the template (use a link instead)"))
                                    .arg(Arg::with_name("NAME")
                                            .help("The name of the template to create")
                                            .required(true))
                                    .arg(Arg::with_name("FILES")
                                            .help("The files or folders to add to the template")
                                            .multiple(true)))
                        .subcommand(SubCommand::with_name("remove")
                                    .about("Remove a template")
                                    .arg(Arg::with_name("NAME")
                                            .help("The name of the template to remove")
                                            .required(true)))
                        .subcommand(SubCommand::with_name("ls")
                                    .about("List available templates"))
                        .subcommand(SubCommand::with_name("dir")
                                    .about("Alias for ls"))
                        .get_matches();

    (match matches.subcommand_name() {
        None => {
            let mut chosen_path = PathBuf::new();
            chosen_path.push(matches.value_of("DIRECTORY").unwrap());
            let name = matches.value_of("NAME").unwrap_or_else(|| {
                eprintln!("error: Expected argument NAME");
                eprintln!("note: You must choose a template to apply");
                exit(1);
            });
            let chosen_path = chosen_path.canonicalize().unwrap_or_else(|e| {
                eprintln!("destination directory file error:");
                eprintln!("{}", e);
                exit(1);
            });
            lib::apply_template(&name.to_string(), chosen_path)
        },
        Some("create") => {
            let matches = matches.subcommand_matches("create").unwrap();
            let files = matches.values_of("FILES")
                .unwrap().map(|fpath| {
                    glob(fpath).expect("Invalid path or glob pattern")
                        .map(|path| {
                            path.unwrap_or_else(|e| {
                                eprintln!("Error opening file\n");
                                eprintln!("{}", e);
                                exit(1);
                            }).canonicalize().unwrap()
                        })
            }).flatten().collect();
            lib::create_template(&matches.value_of("NAME").unwrap().to_string(), files, matches.is_present("no-copy"))
        },
        Some("remove") => {
            let matches = matches.subcommand_matches("remove").unwrap();
            lib::remove_template(&matches.value_of("NAME").unwrap().to_string())
        },
        Some("ls") => lib::list_templates(),
        Some("dir") => lib::list_templates(),
        Some(_) => unreachable!()
    }).unwrap_or_else(|e| {
        eprintln!("io error:");
        eprintln!("{}", e);
        exit(1);
    });
}