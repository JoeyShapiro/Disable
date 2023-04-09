use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();

    // check it only contains a single argument (the file)
    if args.len() != 2 {
        println!("improper amount of arguments");
        process::exit(1);
    }

    // copy the file, then delete it
    // this is safer than moving (what if something happens)
    fs::copy(args[1].as_str(), args[1].as_str().to_owned()+".disabled").unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    // delete the old file
    fs::remove_file(args[1].as_str()).unwrap_or_else(|err| {
        eprintln!("{err}");
        process::exit(1);
    });

    // all good :D
    process::exit(0);
}
