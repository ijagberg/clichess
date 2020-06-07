use std::{convert::TryFrom, fmt::Display};

#[derive(Debug, Copy, PartialEq, Clone)]
pub enum File {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
}

impl Display for File {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let output = char::from(self);
        write!(f, "{}", output)
    }
}

impl PartialOrd for File {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        i32::from(self).partial_cmp(&i32::from(other))
    }
}

impl From<&File> for char {
    fn from(file: &File) -> Self {
        match file {
            File::A => 'a',
            File::B => 'b',
            File::C => 'c',
            File::D => 'd',
            File::E => 'e',
            File::F => 'f',
            File::G => 'g',
            File::H => 'h',
        }
    }
}

impl TryFrom<char> for File {
    type Error = ();
    fn try_from(value: char) -> Result<Self, Self::Error> {
        let file = match value {
            'a' | 'A' => File::A,
            'b' | 'B' => File::B,
            'c' | 'C' => File::C,
            'd' | 'D' => File::D,
            'e' | 'E' => File::E,
            'f' | 'F' => File::F,
            'g' | 'G' => File::G,
            'h' | 'H' => File::H,
            _ => return Err(()),
        };

        Ok(file)
    }
}

impl TryFrom<i32> for File {
    type Error = ();
    fn try_from(value: i32) -> Result<Self, Self::Error> {
        let output = match value {
            1 => File::A,
            2 => File::B,
            3 => File::C,
            4 => File::D,
            5 => File::E,
            6 => File::F,
            7 => File::G,
            8 => File::H,
            _ => return Err(()),
        };
        Ok(output)
    }
}

impl From<&File> for i32 {
    fn from(file: &File) -> Self {
        match file {
            File::A => 1,
            File::B => 2,
            File::C => 3,
            File::D => 4,
            File::E => 5,
            File::F => 6,
            File::G => 7,
            File::H => 8,
        }
    }
}

pub struct FileIter {
    current: i32,
}

impl FileIter {
    pub fn new(start: File) -> Self {
        Self {
            current: i32::from(&start),
        }
    }
}

impl Iterator for FileIter {
    type Item = File;
    fn next(&mut self) -> Option<Self::Item> {
        let current_before = self.current;
        if self.current > 8 {
            None
        } else {
            self.current += 1;
            Some(File::try_from(current_before).unwrap())
        }
    }
}

impl DoubleEndedIterator for FileIter {
    fn next_back(&mut self) -> Option<Self::Item> {
        let current_before = self.current;
        if current_before == 0 {
            None
        } else {
            self.current -= 1;
            Some(File::try_from(current_before).unwrap())
        }
    }
}
