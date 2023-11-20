extern crate rand;
extern crate unic_ucd_category;
extern crate unic_ucd_normal;

use rand::distributions::Alphanumeric;
use rand::Rng;

fn generate_flag(title: &str) -> String {
    let leet_mapping: [(char, &str); 7] = [
        ('a', "4"),
        ('e', "3"),
        ('g', "9"),
        ('i', "1"),
        ('o', "0"),
        ('s', "$"),
        ('t', "7"),
    ];

    let leet_title: String = title
        .chars()
        .flat_map(|c| {
            match leet_mapping
                .iter()
                .find(|&&tuple| tuple.0 == c.to_ascii_lowercase())
            {
                Some(&(_, value)) => value.chars().collect::<Vec<_>>(),
                None => match c {
                    '-' => vec!['_'],
                    '\'' => vec![],
                    ' ' => vec!['_'],
                    _ => vec![c],
                },
            }
        })
        .collect();

    // Generate a random hex value of length 6
    let hex_value: String = rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .filter(|c| c.is_ascii_hexdigit())
        .map(char::from)
        .take(6)
        .collect();

    format!("flag-{}_{}", leet_title, hex_value.to_lowercase())
}

fn main() {
    let titles = [
        "L'initiation d'Eugene",
        "Le progres d'Eugene",
        "Les secrets du seminaire",
        "La recommandation de l'abbe",
        "L'avant-garde de la Polytechnique",
        "La conscience d'Eugene",
    ];

    for title in titles.iter() {
        println!("{}", generate_flag(title));
    }
}
