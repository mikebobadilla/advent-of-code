use std::fs;
use std::error::Error;

pub struct Config {
    pub filename: String,
}

impl Config {
    pub fn build(args: &[String]) -> Result<Config, &'static str> {
        if args.len() < 2 {
            return Err("Not enough arguments")
        }

        let filename = args[1].clone();

        Ok(Config { filename })
    }
}

pub fn run(config: Config) -> Result<(), Box<dyn Error>> {
    let content = fs::read_to_string(config.filename)?;
    let answer = parse_content(&content);

    println!("Answer is: {}", answer);

    Ok(())
}


pub fn parse_content<'a>(content: &'a str) -> u32 {
    let mut answer = 0;
    let mut temp_answer = 0;

    let mut elves = Vec::new();

    for line in content.lines() {
        let number = line.parse::<u32>();

        if number.is_ok() {
            temp_answer = number.unwrap() + temp_answer;

            if temp_answer > answer {
                answer = temp_answer;
            }
        } else {
            elves.push(answer);
            temp_answer = 0;
            answer = 0;
        }
    }

    elves.push(answer);
    elves.sort();

    let wealthiest_elves = &elves[elves.len()-3..];

    for elf in elves.iter() {
        println!("{}", elf);
    }
    
    wealthiest_elves.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn largest_numbers() {
        let content = "\
7
5

3
2

5
5

7
7";

        assert_eq!(36, parse_content(content));
    }
}
