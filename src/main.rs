use clap::clap_app;
use ghost_to_zola::{ghost, zola};
use rayon::prelude::*;
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

    let posts = ghost::read_posts_from_file(path)?;
    posts
        .into_par_iter()
        .map(|p| zola::Page::from(p))
        .for_each(|p| p.write_to_file());

    Ok(())
}
