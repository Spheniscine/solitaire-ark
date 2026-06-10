use std::ops::Range;

use serde::{Deserialize, Serialize};
use serde_tuple::{Deserialize_tuple, Serialize_tuple};
use strum::IntoEnumIterator;
use strum_macros::EnumIter;

use crate::game::{Card, DECK_SIZE};

#[derive(Copy, Clone, Serialize, Deserialize, Debug, PartialEq, Eq, EnumIter)]
pub enum DepotRole {
    FreeCell,
    Tableau,
    Shadow, // "Shadow" depots mirror regular depots and are where completed stacks go; they will be rendered on top of the regular depots
}

pub const NUM_FREECELLS: usize = 4;
pub const NUM_TABLEAUS: usize = 8;
pub const NUM_SHADOW: usize = NUM_FREECELLS + NUM_TABLEAUS;
pub const NUM_DEPOTS: usize = NUM_FREECELLS + NUM_TABLEAUS + NUM_SHADOW;

impl DepotRole {
    pub const fn number_of(&self) -> usize {
        match self {
            DepotRole::FreeCell => NUM_FREECELLS,
            DepotRole::Tableau => NUM_TABLEAUS,
            DepotRole::Shadow => NUM_SHADOW,
        }
    }

    pub const fn offset(&self) -> usize {
        use DepotRole::*;
        match self {
            FreeCell => 0,
            Tableau => NUM_FREECELLS,
            Shadow => NUM_FREECELLS + NUM_TABLEAUS,
        }
    }

    pub const fn range(&self) -> Range<usize> {
        self.offset() .. self.offset() + self.number_of()
    }

    pub fn role_and_subindex(i: usize) -> Option<(DepotRole, usize)> {
        for role in Self::iter() {
            if role.range().contains(&i) {
                return Some((role, i - role.offset()))
            }
        }
        None
    }

    pub fn role(i: usize) -> Option<DepotRole> {
        Self::role_and_subindex(i).map(|x| x.0)
    }

    pub fn id(&self, i: usize) -> usize {
        self.offset() + i
    }

    pub fn is_face_up(&self) -> bool {
        *self != DepotRole::Shadow
    }
}

#[derive(Copy, Clone, Serialize_tuple, Deserialize_tuple, Debug, PartialEq, Eq)]
pub struct BoardPos {
    pub depot_index: usize,
    pub card_index: usize,
}

impl BoardPos {
    pub fn new(depot_index: usize, card_index: usize) -> Self {
        Self { depot_index, card_index }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub enum AnimationAct {
    Move(Vec<Card>, BoardPos, BoardPos),
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct Board {
    pub depots: Vec<Vec<Card>>,
    pub selected: Option<BoardPos>,
    pub animation_acts: Vec<AnimationAct>,
}

impl Board {
    pub fn empty() -> Self {
        Self {
            depots: vec![vec![]; NUM_DEPOTS],
            selected: None,
            animation_acts: vec![],
        }
    }

    pub fn from_deal(deal: &[Card]) -> Self {
        use DepotRole::*;
        assert_eq!(deal.len(), DECK_SIZE);

        let mut res = Self::empty();
        for (&card, depot) in deal.iter().zip(std::iter::repeat(Tableau.range()).flatten()) {
            res.depots[depot].push(card);
        }

        res
    }

    pub fn do_move(&mut self, pos1: BoardPos, pos2: BoardPos) {
        self.selected = None;
        let cards = self.depots[pos1.depot_index].drain(pos1.card_index ..).collect();
        self.animation_acts.push(
            AnimationAct::Move(cards, pos1, pos2)
        );
    }

    pub fn advance_actions(&mut self) {
        for act in self.animation_acts.drain(..) {
            match act {
                AnimationAct::Move(cards, _pos1, pos2) => {
                    self.depots[pos2.depot_index].extend(cards);
                },
            }
        }
    }

    // pub fn top_pos(&self, depot: usize) -> BoardPos {
    //     BoardPos::new(depot, self.depots[depot].len())
    // }
}