use clap::clap_app;
use ghost_to_zola::read_posts_from_file;
use std::error::Error;

fn main() -> Result<(), Box<dyn Error>> {
    let matches = clap_app!(myapp =>
        (version: "1.0")
        (author: "Ben Brandt <benjamin.j.brandt@gmail.com>")
        (about: "Migrates a Ghost export file to Zola markdown content format.")
        (@arg INPUT: +required "Sets the input file to use")
    )
    .get_matches();

    let path = matches.value_of("INPUT").unwrap();

    let posts = read_posts_from_file(path)?;
    println!("Post Title: {}", posts.first().unwrap().title);

    Ok(())
}
