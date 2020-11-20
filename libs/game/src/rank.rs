use std::{convert::TryFrom, fmt::Display};

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum Rank {
    First,
    Second,
    Third,
    Fourth,
    Fifth,
    Sixth,
    Seventh,
    Eighth,
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", i32::from(self))
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        i32::from(self).partial_cmp(&i32::from(other))
    }
}

impl TryFrom<char> for Rank {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let rank = match value {
            '1' => Rank::First,
            '2' => Rank::Second,
            '3' => Rank::Third,
            '4' => Rank::Fourth,
            '5' => Rank::Fifth,
            '6' => Rank::Sixth,
            '7' => Rank::Seventh,
            '8' => Rank::Eighth,
            _ => return Err(()),
        };
        Ok(rank)
    }
}

impl TryFrom<i32> for Rank {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let output = match value {
            1 => Rank::First,
            2 => Rank::Second,
            3 => Rank::Third,
            4 => Rank::Fourth,
            5 => Rank::Fifth,
            6 => Rank::Sixth,
            7 => Rank::Seventh,
            8 => Rank::Eighth,
            _ => return Err(()),
        };
        Ok(output)
    }
}

impl From<&Rank> for i32 {
    fn from(rank: &Rank) -> Self {
        match rank {
            Rank::First => 1,
            Rank::Second => 2,
            Rank::Third => 3,
            Rank::Fourth => 4,
            Rank::Fifth => 5,
            Rank::Sixth => 6,
            Rank::Seventh => 7,
            Rank::Eighth => 8,
        }
    }
}

pub struct RankIter {
    current: i32,
}

impl RankIter {
    pub fn new(start: Rank) -> Self {
        Self {
            current: i32::from(&start),
        }
    }
}

impl Iterator for RankIter {
    type Item = Rank;
    fn next(&mut self) -> Option<Self::Item> {
        let current_before = self.current;
        if current_before > 8 {
            None
        } else {
            self.current += 1;
            Some(Rank::try_from(current_before).unwrap())
        }
    }
}

impl DoubleEndedIterator for RankIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current_before = self.current;
        if current_before == 0 {
            None
        } else {
            self.current -= 1;
            Some(Rank::try_from(current_before).unwrap())
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rank_iter() {
        assert_eq!(
            RankIter::new(Rank::First).collect::<Vec<_>>(),
            vec![
                Rank::First,
                Rank::Second,
                Rank::Third,
                Rank::Fourth,
                Rank::Fifth,
                Rank::Sixth,
                Rank::Seventh,
                Rank::Eighth,
            ]
        );

        assert_eq!(
            RankIter::new(Rank::Third,).collect::<Vec<_>>(),
            vec![
                Rank::Third,
                Rank::Fourth,
                Rank::Fifth,
                Rank::Sixth,
                Rank::Seventh,
                Rank::Eighth,
            ]
        );

        assert_eq!(
            RankIter::new(Rank::Fourth).rev().collect::<Vec<_>>(),
            vec![Rank::Fourth, Rank::Third, Rank::Second, Rank::First]
        );
    }
}
