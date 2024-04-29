// cargo build --release
#![allow(warnings)]

// use chrono::{DateTime, Duration, Utc}; // to do some date-time manipulation
// use clap::{Arg, Parser, Subcommand, ValueEnum}; // to parse command-line arguments
// use configparser::ini::Ini; // to handle configuration files
// use flate2::*; // to compresses everything using zlib
// use glob::glob; // to support .gitignore and match filenames against patterns
// use hashlib::*; // to use SHA-1 function
// use regex::Regex; // need a bit of regex
// use std::collections::HashMap; // to use as ordered dictionary (maybe need more)
// use std::fs::Metadata; // to access to owner-group ID of files
// use std::path::{Path, PathBuf}; // to provide some nice filesystem abstraction routines

// f64::ceil()
mod repo;

use repo::*;
use std::env; // to access actual command-line arguments

// TODO: add a decent cli parser!

fn cmd_init(path: &str) {
    repo::GitRepository::repo_create(path);
}

fn main() {
    // cargo run -- query content
    println!("Hello, world!");
    let args: Vec<String> = env::args().collect();

    let command: &str = &args[1];
    let content = &args[2];

    match command {
        "add" => cmd_add(content),
        "cat-file" => cmd_cat_file(content),
        "check-ignore" => cmd_check_ignore(content),
        "checkout" => cmd_checkout(content),
        "commit" => cmd_commit(content),
        "hash-object" => cmd_hash_object(content),
        "init" => cmd_init(content),
        "log" => cmd_log(content),
        "ls-files" => cmd_ls_files(content),
        "ls-tree" => cmd_ls_tree(content),
        "rev-parse" => cmd_rev_parse(content),
        "rm" => cmd_rem(content),
        "show-ref" => cmd_show_ref(content),
        "status" => cmd_status(content),
        "tag" => cmd_tag(content),
        _ => println!("Bad command."),
    }

    println!("Print arguments: {:?}", args);
    println!("Print query: {}", command);
    println!("Print query content: {}", content);
}
