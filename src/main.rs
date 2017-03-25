// Copyright 2017 Nathan Sizemore <nathanrsizemore@gmail.com>
//
// This Source Code Form is subject to the terms of the Mozilla Public
// License, v. 2.0. If a copy of the MPL was not distributed with this file,
// you can obtain one at http://mozilla.org/MPL/2.0/.


extern crate docopt;
extern crate rustc_serialize;


use docopt::Docopt;


const USAGE: &'static str = "
surl - Converts Spotify share links from URL to URI

Usage:
    surl <url>
    surl (-h | --help)
    surl --version

Options:
    -h --help       Show this screen.
    --version       Show version.
";

const VERSION_STR: &'static str = "surl 0.1.0

Copyright (c) 2017 Nathan Sizemore <nathanrsizemore@gmail.com>
License: MPL-2.0 https://www.mozilla.org/en-US/MPL/2.0
This is free software: you are free to change and redistribute it.";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub arg_url: String,
    pub flag_version: bool
}


fn main() {
    let args: Args = Docopt::new(USAGE)
        .and_then(|d| d.decode())
        .unwrap_or_else(|e| e.exit());

    let exit_code = match handle_input(&args) {
        Ok(_) => 0,
        Err(_) => -1
    };

    std::process::exit(exit_code);
}

fn handle_input(args: &Args) -> Result<(), ()> {
    if args.flag_version {
        println!("{}", VERSION_STR);
        return Ok(());
    }

    let uri = try!(convert_url(&args.arg_url));
    println!("{}", uri);

    return Ok(());
}

// https://open.spotify.com/track/1yCkCjXNm8UI3qMkg0YQIh
// to
// spotify:track:0fTQdOS8FucxBQilLTp2ha
fn convert_url(url: &String) -> Result<String, ()> {
    let mut uri = String::with_capacity(50);
    let chunks: Vec<&str> = url.split("/").collect();

    // for chunk in chunks.iter() { println!("{}", chunk); }

    // The URL should split into 5 chunks
    // https:
    //
    // open.spotify.com
    // track
    // 1yCkCjXNm8UI3qMkg0YQIh
    if chunks.len() < 5 { return Err(()); }

    uri.push_str("spotify:");
    uri.push_str(chunks[3]);
    uri.push_str(":");
    uri.push_str(chunks[4]);

    return Ok(uri);
}
