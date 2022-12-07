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
    let answer = part_two(&content);
    //let answer = part_one(&content);

    println!("Answer is: {}", answer);

    Ok(())
}

pub fn part_one<'a>(content: &'a str) -> u32 {
    let mut answer = 0;

    for line in content.lines() {
        let mut n = line.split_whitespace();
        let opponent = n.next();
        let me = n.next();

        let my_hand_score = get_char_value(me.unwrap());
        let opp_hand_score = get_char_value(opponent.unwrap());

        // Rock
        if opp_hand_score == 1 {
            // Rock
            if my_hand_score == 1 {
                answer += 3;
            }
            // Paper
            if my_hand_score == 2 {
                answer += 6;
            }
        }

        // Paper
        if opp_hand_score == 2 {
            // Paper
            if my_hand_score == 2 {
                answer += 3;
            }
            // Scissors
            if my_hand_score == 3 {
                answer += 6;
            }

        }

        // Scissors
        if opp_hand_score == 3 {
            // Rock
            if my_hand_score == 1 {
                answer += 6;
            }
            // Scissors
            if my_hand_score == 3 {
                answer += 3;
            }

        }

        answer += my_hand_score;
    }

    return answer;

}

pub fn part_two<'a>(content: &'a str) -> u32 {
    let mut answer = 0;

    for line in content.lines() {
        let mut n = line.split_whitespace();
        let opponent = n.next();
        let me = n.next();

        let my_hand_score = get_char_value(me.unwrap());
        let opp_hand_score = get_char_value(opponent.unwrap());

        // Rock
        if opp_hand_score == 1 {
            // Lose 
            if my_hand_score == 1 {
                answer += 3;
            }
            // Tie
            if my_hand_score == 2 {
                answer += 4;
            }
            // Win
            if my_hand_score == 3 {
                answer += 8;
            }
        }

        // Paper
        if opp_hand_score == 2 {
            // Lose 
            if my_hand_score == 1 {
                answer += 1;
            }
            // Tie
            if my_hand_score == 2 {
                answer += 5;
            }
            // Win
            if my_hand_score == 3 {
                answer += 9;
            }
        }

        // Scissors
        if opp_hand_score == 3 {
            // Lose 
            if my_hand_score == 1 {
                answer += 2;
            }
            // Tie
            if my_hand_score == 2 {
                answer += 6;
            }
            // Win
            if my_hand_score == 3 {
                answer += 7;
            }
        }
    }

    return answer;

}

fn get_char_value(player_hand: &str) -> u32 {
    if player_hand == "A" || player_hand == "X" {
        return 1;
    }

    if player_hand == "B" || player_hand == "Y" {
        return 2;
    }

    if player_hand == "C" || player_hand == "Z" {
        return 3;
    }

    return 0;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn rock_paper_scissors() {
        let content = "\
A Y
B X
C Z
C Y
A X
A Z";

        assert_eq!(24, part_one(content));
    }

    #[test]
    fn rock_paper_scissors_2() {
        let content = "\
A Y
B X
C Z
C Y
A X
A Z";

        assert_eq!(29, part_two(content));
    }

    #[test]
    fn test_get_char_value() {
        assert_eq!(1, get_char_value("A"));
        assert_eq!(1, get_char_value("X"));
        
        assert_eq!(2, get_char_value("B"));
        assert_eq!(2, get_char_value("Y"));

        assert_eq!(3, get_char_value("C"));
        assert_eq!(3, get_char_value("Z"));
    }
}
