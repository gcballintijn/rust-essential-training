use rand::{thread_rng, Rng};
use std::error::Error;
use std::io::{self, Write};

fn main() -> Result<(), Box<dyn Error>> {
    println!("Higher or Lower");

    let mut random = thread_rng();
    let to_guess = random.gen_range(1..101);

    let mut buffer = String::new();
    let mut count = 1u32;
    loop {
        print!("Please provide a guess (guess {}): ", count);
        io::stdout().flush()?;

        buffer.clear();
        io::stdin().read_line(&mut buffer)?;
        let cleaned = buffer.trim();
        let guess = cleaned.parse::<i32>()?;

        match guess {
            guess if guess < to_guess => {
                println!("{} is too low.", guess);
            }
            guess if guess > to_guess => {
                println!("{} is too high.", guess);
            }
            guess => {
                println!("{} is right!", guess);
                break;
            }
        }

        count += 1;
    }

    Ok(())
}
