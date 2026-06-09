use std::time::Duration;

use rand::{Rng, seq::SliceRandom};
use serde::{Deserialize, Serialize};

use crate::game::{Board, BoardPos, Card, DECK_SIZE, DepotRole, NUM_COPIES, NUM_FREECELLS, RANKS, Skin};

/* 
 * Notes:
 * Due to the greater complexity of rules involving "locking" stacks by moving things to shadow depots,
 * there is a separation between move *intents* and actual *moves* that affect the board.
 * This also means the undo stack works differently, with a Vec of usizes,
 * representing the length of history to prune to with each undo.
 */

pub const ANIMATION_DURATION: Duration = Duration::from_millis(200);
pub type AnimationKey = u16;

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct ActionRecord {
    pos1: BoardPos, pos2: BoardPos,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default)]
pub enum ScreenState {
    #[default] Game, 
    Settings, Help,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Default, strum_macros::Display)]
pub enum Difficulty {
    #[default] Easy, 
    Medium, Hard, Expert,
}

impl Difficulty {
    pub fn base_freecells_unlocked(&self) -> usize {
        match self {
            Difficulty::Easy => 4,
            Difficulty::Medium => 3,
            Difficulty::Hard => 2,
            Difficulty::Expert => 1,
        }
    }
}

#[derive(Clone, Serialize, Deserialize, Debug, PartialEq, Eq)]
pub struct GameState {
    pub board: Board,
    pub difficulty: Difficulty,
    pub deal: Vec<Card>,
    #[serde(skip)]
    pub animation_key: AnimationKey, // used for syncing and to provide animator components with cycling keys
    pub history: Vec<ActionRecord>,
    pub undo_stack: Vec<usize>,
    pub already_won: bool,
    pub num_wins: i32,

    pub screen_state: ScreenState,

    pub allow_undo: bool,
    pub auto_play: bool,
    pub skin: Skin,
}

impl GameState {
    pub fn new_deal(rng: &mut impl Rng) -> Vec<Card> {
        let mut deck = std::iter::repeat(RANKS)
            .take(NUM_COPIES).flatten()
            .collect::<Vec<_>>();

        deck.shuffle(rng);
        deck
    }

    pub fn new_game(&mut self, difficulty: Difficulty) {
        let deal = Self::new_deal(&mut rand::rng());
        self.board = Board::from_deal(&deal);
        self.deal = deal;
        self.history.clear();
        self.undo_stack.clear();
        self.already_won = false;
        self.difficulty = difficulty;
        // LocalStorage.save_game_state(&self);
    }

    pub fn init() -> Self {
        let mut res = Self {
            board: Board::empty(),
            difficulty: Difficulty::Easy,
            deal: vec![],
            animation_key: 0,
            history: vec![],
            undo_stack: vec![],
            already_won: false,
            num_wins: 0,
            screen_state: ScreenState::Game,
            allow_undo: true,
            auto_play: true,
            skin: Skin::default(),
        };

        res.new_game(Difficulty::Hard);
        res
    }

    pub fn can_stack(&self, back: Card, front: Card) -> bool {
        back == front
    }

    pub fn can_select(&self, pos: BoardPos) -> bool {
        let depot = pos.depot_index;
        let ord = pos.card_index;

        if ord >= self.board.depots[depot].len() {
            return false;
        }
        let slice = &self.board.depots[depot][ord..];

        let Some(role) = DepotRole::role(depot) else { return false };
        match role {
            DepotRole::FreeCell => { slice.len() <= 1 },
            DepotRole::Tableau => {
                slice.windows(2).all(|w| self.can_stack(w[0], w[1]))
            },
            DepotRole::Shadow => false,
        }
    }

    pub fn is_busy(&self) -> bool {
        self.is_acting()
    }

    pub fn is_acting(&self) -> bool {
        !self.board.animation_acts.is_empty()
    }

    pub fn num_freecells_unlocked(&self) -> usize {
        NUM_FREECELLS.min(
            self.difficulty.base_freecells_unlocked() + 
            DepotRole::Tableau.range().filter(|&d| {
                let s = DepotRole::Shadow.id(d);
                !self.board.depots[s].is_empty()
            }).count()
        )
    }

    pub fn is_won(&self) -> bool {
        false // todo
    }

    pub fn undo_possible(&self) -> bool {
        self.allow_undo && !self.undo_stack.is_empty()
    }

    pub fn onclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }

        if let Some(src) = self.board.selected {
            if pos == src { 
                self.board.selected = None; 
                return;
            }
            if src.depot_index == pos.depot_index && self.can_select(pos) {
                self.board.selected = Some(pos);
                return;
            }

            // let dest = BoardPos::new(pos.depot_index, pos.card_index.wrapping_add(1));
            // if !self.can_move(src, dest) { return; }
            // self.board.do_move(src, dest);
            // self.history.push(ActionRecord { pos1: src, pos2: dest, auto: false });
        } else {
            if self.can_select(pos) {
                self.board.selected = Some(pos);
            }
        }
    }

    pub fn ondoubleclick(&mut self, pos: BoardPos) {
        if self.is_busy() { return; }
        
        // todo
    }

    pub fn advance_animations(&mut self, key: AnimationKey) {
        if key != self.animation_key { return; }
        self.animation_key = self.animation_key.wrapping_add(1);
        
        self.board.advance_actions();

        if self.is_won() {
            if !self.already_won {
                self.num_wins += 1;
                self.already_won = true;
            }
        } else {
            // self.check_auto_moves();
        }

        // if !self.is_busy() { LocalStorage.save_game_state(&self); }
    }
}