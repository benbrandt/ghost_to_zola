use clap::clap_app;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Ben Brandt <benjamin.j.brandt@gmail.com>")
        (about: "Migrates a Ghost export file to Zola markdown content format.")
        (@arg INPUT: +required "Sets the input file to use")
    )
    .get_matches();

    let file = matches.value_of("INPUT").unwrap();
    println!("The file passed is: {}", file);

    Ok(())
}
