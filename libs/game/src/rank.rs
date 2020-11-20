use std::{
    convert::TryFrom,
    fmt::Display,
    ops::{Add, Sub},
};

/// A chess rank (horizontal line)
#[derive(Debug, Copy, PartialEq, Clone, Eq)]
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

impl Add<u8> for Rank {
    type Output = Option<Rank>;

    fn add(self, rhs: u8) -> Self::Output {
        let mut v = u8::from(&self);

        v += rhs;

        match Rank::try_from(v) {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    }
}

impl Sub<u8> for Rank {
    type Output = Option<Rank>;

    fn sub(self, rhs: u8) -> Self::Output {
        let mut v = u8::from(&self);

        v = v.checked_sub(rhs)?;

        match Rank::try_from(v) {
            Ok(f) => Some(f),
            Err(_) => None,
        }
    }
}

impl Display for Rank {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", u8::from(self))
    }
}

impl PartialOrd for Rank {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        u8::from(self).partial_cmp(&u8::from(other))
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

impl TryFrom<u8> for Rank {
    type Error = ();
    fn try_from(value: u8) -> Result<Self, Self::Error> {
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

impl From<&Rank> for u8 {
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

/// An iterator over ranks
///
/// By default it iterates in increasing order (`First` -> `Second`, `Second` -> `Third`)
pub struct RankIter {
    current: u8,
}

impl RankIter {
    /// Start a new `RankIter` at `start`
    /// 
    /// # Example
    /// ```
    /// use game::{Rank, RankIter};
    /// 
    /// let mut rank_iter = RankIter::start_at(Rank::First);
    /// assert_eq!(rank_iter.next(), Some(Rank::First));
    /// assert_eq!(rank_iter.next(), Some(Rank::Second));
    ///
    /// let mut rank_iter = RankIter::start_at(Rank::Eighth);
    /// assert_eq!(rank_iter.next(), Some(Rank::Eighth));
    /// assert_eq!(rank_iter.next(), None);
    /// ```
    pub fn start_at(start: Rank) -> Self {
        Self {
            current: u8::from(&start),
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
            RankIter::start_at(Rank::First).collect::<Vec<_>>(),
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
            RankIter::start_at(Rank::Third,).collect::<Vec<_>>(),
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
            RankIter::start_at(Rank::Fourth).rev().collect::<Vec<_>>(),
            vec![Rank::Fourth, Rank::Third, Rank::Second, Rank::First]
        );
    }
}
