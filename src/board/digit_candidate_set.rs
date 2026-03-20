use crate::board::digit::Digit;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct DigitCandidateSet(u16);

impl DigitCandidateSet {
    pub const ALL: Self = Self(0b_0000_0011_1111_1110);
    pub const NONE: Self = Self(0);

    pub const fn of(digit: Digit) -> Self {
        return Self(1 << digit.as_u8());
    }

    pub const fn contains(self, digit: Digit) -> bool {
        return (self.0 >> digit.as_u8()) & 0b1 == 0b1
    }

    pub const fn add(self, digit: Digit) -> Self {
        return Self(self.0 | (1 << digit.as_u8()));
    }

    pub const fn remove(self, digit: Digit) -> Self {
        return Self(self.0 & !(1 << digit.as_u8()));
    }

    pub const fn candidates_count(self) -> u32 {
        return self.0.count_ones();
    }

    pub const fn is_empty(self) -> bool {
        return self.0 == 0;
    }

    pub const fn is_solved(self) -> bool {
        return self.candidates_count() == 1;
    }

    pub fn solved_digit(self) -> Option<Digit> {
        if self.is_solved() {
            return Some(Digit::from(match self.0.trailing_zeros() {
                1 => Digit::ONE,
                2 => Digit::TWO,
                3 => Digit::THREE,
                4 => Digit::FOUR,
                5 => Digit::FIVE,
                6 => Digit::SIX,
                7 => Digit::SEVEN,
                8 => Digit::EIGHT,
                9 => Digit::NINE,
                _ => unreachable!(),
            }));
        }

        return None;
    }
}

