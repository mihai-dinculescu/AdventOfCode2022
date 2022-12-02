use anyhow::Context;

pub struct Strategy2 {
    opponent_choice: Choice,
    result: GameResult,
}

impl Strategy2 {
    pub fn new(value: &str) -> Result<Self, anyhow::Error> {
        let mut iter = value.split_whitespace();
        let opponent_choice =
            TryInto::<Choice>::try_into(iter.next().context("missing opponent choice")?)?;
        let result = TryInto::<GameResult>::try_into(iter.next().context("missing game result")?)?;

        Ok(Self {
            opponent_choice,
            result,
        })
    }

    pub fn score(&self) -> u64 {
        let mut score = self.result as u64;

        let player_choice = match self.result {
            GameResult::Win => match self.opponent_choice {
                Choice::Rock => Choice::Paper,
                Choice::Paper => Choice::Scissors,
                Choice::Scissors => Choice::Rock,
            },
            GameResult::Loss => match self.opponent_choice {
                Choice::Rock => Choice::Scissors,
                Choice::Paper => Choice::Rock,
                Choice::Scissors => Choice::Paper,
            },
            GameResult::Draw => self.opponent_choice,
        };

        score += player_choice as u64;

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
            "A" => Ok(Choice::Rock),
            "B" => Ok(Choice::Paper),
            "C" => Ok(Choice::Scissors),
            _ => Err(anyhow::anyhow!("Invalid choice")),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq)]
#[repr(u64)]
enum GameResult {
    Loss = 0,
    Draw = 3,
    Win = 6,
}

impl TryFrom<&str> for GameResult {
    type Error = anyhow::Error;

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "X" => Ok(GameResult::Loss),
            "Y" => Ok(GameResult::Draw),
            "Z" => Ok(GameResult::Win),
            _ => Err(anyhow::anyhow!("Invalid game result")),
        }
    }
}

#[cfg(test)]
pub mod tests {
    use super::*;

    #[test]
    fn test_game_1() {
        let game = Strategy2::new("A Y").unwrap();
        assert_eq!(game.score(), 4);
    }

    #[test]
    fn test_game_2() {
        let game = Strategy2::new("B X").unwrap();
        assert_eq!(game.score(), 1);
    }

    #[test]
    fn test_game_3() {
        let game = Strategy2::new("C Z").unwrap();
        assert_eq!(game.score(), 7);
    }
}
