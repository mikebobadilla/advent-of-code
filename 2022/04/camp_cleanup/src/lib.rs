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
        let (vec_1, vec_2) = build_list_of_numbers(Some(line));

        let mut vec_1_is_all_in_vec_2 = true;
        for num in vec_1.iter() {
            if !vec_2.contains(num) {
                vec_1_is_all_in_vec_2 = false;
            } 
        }

        let mut vec_2_is_all_in_vec_1 = true;
        for num in vec_2.iter() {
            if !vec_1.contains(num) {
                vec_2_is_all_in_vec_1 = false;
            } 
        }

        if vec_1_is_all_in_vec_2 || vec_2_is_all_in_vec_1 {
            answer += 1;
        }
    }

    return answer;

}

pub fn part_two<'a>(content: &'a str) -> u32 {
    let mut answer = 0;

    for line in content.lines() {
        let (vec_1, vec_2) = build_list_of_numbers(Some(line));

        let mut overlaps = false;
        for num in vec_1.iter() {
            if vec_2.contains(num) {
                overlaps = true;
            } 
        }

        if overlaps {
            answer += 1;
        }
    }

    return answer;

}

fn build_list_of_numbers(line: Option<&str>) -> (Vec<u32>, Vec<u32>) {
    let lines: Vec<&str> = line.expect("works").split(',').collect();
    let limits_1: Vec<&str> = lines[0].split('-').collect();
    let limits_2: Vec<&str> = lines[1].split('-').collect();

    let start_1 = limits_1[0].parse::<u32>().unwrap();
    let end_1 = limits_1[1].parse::<u32>().unwrap();
    let start_2 = limits_2[0].parse::<u32>().unwrap();
    let end_2 = limits_2[1].parse::<u32>().unwrap();

    let vec_1: Vec<u32> = (start_1..end_1 + 1).collect();
    let vec_2: Vec<u32> = (start_2..end_2 + 1).collect();

    return (vec_1, vec_2);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cleanup_1() {
        let content = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(2, part_one(content));
    }

    #[test]
    fn cleanup_2() {
        let content = "\
2-4,6-8
2-3,4-5
5-7,7-9
2-8,3-7
6-6,4-6
2-6,4-8";

        assert_eq!(4, part_two(content));
    }

    #[test]
    fn it_should_return_two_vectors() {
        let content = "\
2-4,6-8";

        let mut line = content.lines();

        assert_eq!(((2..5).collect(), (6..9).collect()), build_list_of_numbers(line.next()));
    }

}
