use clap::Parser;
use std::collections::HashMap;
use std::fs;

#[derive(Parser)]
struct Args {
    path: String,
}

fn main() {
    let args = Args::parse();
    let contents = fs::read_to_string(args.path).unwrap();
    let words = contents
            .lines()
            .map(|line| line.split_whitespace())
            .flatten()
            .filter(|s| !s.is_empty())
            .map(clean_str)
            .map(|s| s.to_lowercase())
            .fold(HashMap::new(), |mut m, s| {
                let word = m.entry(s).or_insert(0u32);
                *word += 1;
                m
            });
    
    let count_words = words
            .iter()
            .fold(HashMap::new(), |mut m, e| {
                let ne = m.entry(e.1).or_insert(Vec::new());
                ne.push(e.0);
                m
            });

    let mut counts = count_words.keys().collect::<Vec<_>>();
    counts.sort();
    counts.reverse();

    for i in 0..175 {
        print!("{}: ", counts[i]);
        for word in &count_words[counts[i]] {
            print!("{} ", word);
        }
        println!();
    }
}

fn clean_str(s: &str) -> String {
    let char_array: Vec<char> = s.chars().collect();

    let mut begin = 0;
    let mut end = char_array.len();

    if begin < end && char_array[begin] == '“' {
        begin += 1;
    }

    if end > begin && char_array[end - 1] == '.' {
        end -= 1;
    }

    if end > begin && char_array[end - 1] == ',' {
        end -= 1;
    }

    if end > begin && char_array[end - 1] == '?' {
        end -= 1;
    }

    if end > begin && char_array[end - 1] == '!' {
        end -= 1;
    }

    if end > begin && char_array[end - 1] == '”' {
        end -= 1;
    }

    String::from_iter(char_array[begin..end].iter())
}
