use anyhow::Context;

pub struct Strategy1 {
    opponent_choice: Choice,
    player_choice: Choice,
}

impl Strategy1 {
    pub fn new(value: &str) -> Result<Self, anyhow::Error> {
        let mut iter = value.split_whitespace();
        let opponent_choice =
            TryInto::<Choice>::try_into(iter.next().context("missing opponent choice")?)?;
        let player_choice =
            TryInto::<Choice>::try_into(iter.next().context("missing player choice")?)?;

        Ok(Self {
            opponent_choice,
            player_choice,
        })
    }

    pub fn score(&self) -> u64 {
        let mut score = self.player_choice as u64;

        score += match (self.player_choice, self.opponent_choice) {
            (Choice::Rock, Choice::Scissors) => 6,
            (Choice::Scissors, Choice::Paper) => 6,
            (Choice::Paper, Choice::Rock) => 6,
            (a, b) if a == b => 3,
            _ => 0,
        };

        score
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
enum Choice {
    Rock = 1,
    Paper,
    Scissors,
}

impl TryFrom<&str> for Choice {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "A" | "X" => Ok(Choice::Rock),
            "B" | "Y" => Ok(Choice::Paper),
            "C" | "Z" => Ok(Choice::Scissors),
            _ => Err(anyhow::anyhow!("Invalid choice")),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_game_1() {
        let game = Strategy1::new("A Y").unwrap();
        assert_eq!(game.score(), 8);
    }

    #[test]
    fn test_game_2() {
        let game = Strategy1::new("B X").unwrap();
        assert_eq!(game.score(), 1);
    }

    #[test]
    fn test_game_3() {
        let game = Strategy1::new("C Z").unwrap();
        assert_eq!(game.score(), 6);
    }
}
