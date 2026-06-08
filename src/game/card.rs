use std::ops::RangeInclusive;

pub type Card = u8;
pub const RANK_MIN: u8 = 0;
pub const RANK_MAX: u8 = 9;
pub const RANKS: RangeInclusive<u8> = RANK_MIN ..= RANK_MAX;
pub const NUM_RANKS: usize = (RANK_MAX - RANK_MIN) as usize + 1;

pub const NUM_COPIES: usize = 4;
pub const DECK_SIZE: usize = NUM_RANKS * NUM_COPIES;