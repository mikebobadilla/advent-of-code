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
    //let answer = part_one(&content);
    let answer = part_two(&content);

    println!("Answer is: {}", answer);

    Ok(())
}

pub fn part_one<'a>(content: &'a str) -> u32 {
    let mut answer = 0;

    for line in content.lines() {
        let (first, second) = line.split_at(line.len() / 2);
        let matched_letter = get_matching_letter(first, second);
        if matched_letter != '0' {
            let value = get_letter_value(matched_letter);
            answer += value;
        }
    }

    return answer;

}

pub fn part_two<'a>(content: &'a str) -> u32 {
    let mut answer = 0;

    let mut lines = content.lines();
    let mut ruck_1 = lines.next();
    let mut ruck_2 = lines.next();
    let mut ruck_3 = lines.next();

    while ruck_1 != None {
        let matched_letter = get_unique_letter(ruck_1.unwrap(), ruck_2.unwrap(), ruck_3.unwrap());

        if matched_letter != '0' {
            let value = get_letter_value(matched_letter);
            answer += value;
        }

        ruck_1 = lines.next();
        ruck_2 = lines.next();
        ruck_3 = lines.next();
    }

    return answer;

}

fn get_matching_letter<'a>(a: &'a str, b: &'a str) -> char{

    let chars = a.chars();

    for letter in chars {
        if b.contains(letter) {
            return letter;
        }

    }
    return '0';
}


fn get_unique_letter<'a>(a: &'a str, b: &'a str, c: &'a str) -> char{

    let chars = a.chars();

    let mut b_letters = Vec::new();
    let mut c_letters = Vec::new();

    for letter in chars {
        if b.contains(letter) {
            b_letters.push(letter);
        }
        if c.contains(letter) {
            c_letters.push(letter);
        }
    }

    for answer in &b_letters {
        if c_letters.contains(answer) {
            return *answer;
        }

    }
    println!("{:?}", b_letters);
    println!("{:?}", c_letters);
    return '0';
}

fn get_letter_value(a: char) -> u32 {
    let mut value = 0;
    let is_uppercase = a.is_uppercase();

    for lowercase_value in a.to_lowercase() {
        if lowercase_value == 'a' {
            value = 1;
        }
        if lowercase_value == 'b' {
            value = 2;
        }
        if lowercase_value == 'c' {
            value = 3;
        }
        if lowercase_value == 'd' {
            value = 4;
        }
        if lowercase_value == 'e' {
            value = 5;
        }
        if lowercase_value == 'f' {
            value = 6;
        }
        if lowercase_value == 'g' {
            value = 7;
        }
        if lowercase_value == 'h' {
            value = 8;
        }
        if lowercase_value == 'i' {
            value = 9;
        }
        if lowercase_value == 'j' {
            value = 10;
        }
        if lowercase_value == 'k' {
            value = 11;
        }
        if lowercase_value == 'l' {
            value = 12;
        }
        if lowercase_value == 'm' {
            value = 13;
        }
        if lowercase_value == 'n' {
            value = 14;
        }
        if lowercase_value == 'o' {
            value = 15;
        }
        if lowercase_value == 'p' {
            value = 16;
        }
        if lowercase_value == 'q' {
            value = 17;
        }
        if lowercase_value == 'r' {
            value = 18;
        }
        if lowercase_value == 's' {
            value = 19;
        }
        if lowercase_value == 't' {
            value = 20;
        }
        if lowercase_value == 'u' {
            value = 21;
        }
        if lowercase_value == 'v' {
            value = 22;
        }
        if lowercase_value == 'w' {
            value = 23;
        }
        if lowercase_value == 'x' {
            value = 24;
        }
        if lowercase_value == 'y' {
            value = 25;
        }
        if lowercase_value == 'z' {
            value = 26;
        }
    }

    if is_uppercase {
        return value + 26;
    }

    return value;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rucksack_1() {
        let content = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(157, part_one(content));
    }

    #[test]
    fn rucksack_2() {
        let content = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw";

        assert_eq!(70, part_two(content));
    }

    #[test]
    fn it_has_matching_letter() {
        let a = "vyx";
        let b = "rbv";

        assert_eq!('v', get_matching_letter(a, b));
    }

    #[test]
    fn it_finds_unique_letter() {
        let a = "vyx";
        let b = "rbv";
        let c = "dvy";

        assert_eq!('v', get_unique_letter(a, b, c));
    }

    #[test]
    fn it_returns_value_for_letter() {
        assert_eq!(1, get_letter_value('a'));
        assert_eq!(2, get_letter_value('b'));
        assert_eq!(28, get_letter_value('B'));
        assert_eq!(38, get_letter_value('L'));
    }
}
