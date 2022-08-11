use rand::{prelude::SliceRandom, thread_rng, Rng};
use std::{fs, io::Error, str};

fn capitalize(s: &str) -> String {
    let mut c = s.chars();
    match c.next() {
        None => String::new(),
        //Capitalize first letter, then memcpy
        Some(f) => f.to_uppercase().collect::<String>() + c.as_str(),
    }
}

const DELIMS: [&str; 5] = [",", ".", ";", "/", "-"];
pub fn get_words(quant: u8, filename: &str) -> Result<String, Error> {
    let mut rng = rand::thread_rng();

    let file = fs::read_to_string(filename)?;

    //need it to be a Vec, bcs shuffle
    let mut vec_file: Vec<&str> = file.split("\n").collect();
    vec_file.shuffle(&mut thread_rng());

    let contents: String = vec_file
        .into_iter()
        .take(quant as usize)
        .map(|word| match rng.gen::<usize>() % 40 {
            index @ (1..=4 | 9..=12) => word.to_string() + DELIMS[index % 5] + " ",
            5..=10 | 29..=35 => capitalize(word) + " ",
            13 => capitalize(word) + DELIMS[rng.gen::<usize>() % 5] + " ",
            _ => word.to_string() + " ",
        })
        .collect();

    Ok(contents)
}
