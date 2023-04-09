use std::{env, fs, process};

fn main() {
    let args: Vec<String> = env::args().collect();
    let mut ending = ".disabled".to_owned();
    let mut new_file = "".to_owned();

    // check it only contains a single argument (the file), or 3 (+ending)
    if args.len() != 2 && args.len() != 3 {
        println!("improper amount of arguments");
        println!("disable file [ending[.disabled]]");
        println!("--help");
        println!("\tfile - The file to change");
        println!("\tending [.disabled] - The ending to look for. Should contain the '.'");
        process::exit(1);
    }

    // set the ending
    if args.len() == 3 {
        ending = args[2].to_string();
    }

    // get the new file name (nice way of doing it)
    // this whole part looks ugly
    if args[1].ends_with(&ending) {
        new_file = args[1].trim_end_matches(&ending).to_string()
    } else {
        new_file.push_str(args[1].as_str());
        new_file.push_str(&ending)
    }

    // copy the file, then delete it
    // this is safer than moving (what if something happens)
    fs::copy(args[1].as_str(), new_file).unwrap_or_else(|err| {
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
