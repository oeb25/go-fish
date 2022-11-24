use core::num;
use std::collections::HashMap;
pub use std::fmt::Debug;

use serde::Serialize;
use typeshare::typeshare;

use crate::{
    cards::{Card, Rank, Ranks},
    random_strat::Random,
    wiki_strat::Wiki,
    Player,
};

pub trait Strategy: Debug {
    fn init(pid: PlayerId) -> Self;
    fn deal_card(&mut self, ctx: &Context, card: Card);
    fn action(&mut self, ctx: &Context) -> Option<Action>;
    fn react(&mut self, ctx: &Context, res: Announcement);
}

pub enum StratBuilder {
    Random,
    Wiki,
}

impl StratBuilder {
    pub fn init(self, pid: PlayerId) -> Strat {
        match self {
            StratBuilder::Random => Strat::Random(Random::init(pid)),
            StratBuilder::Wiki => Strat::Wiki(Wiki::init(pid)),
        }
    }
}

#[derive(Debug)]
pub enum Strat {
    Random(Random),
    Wiki(Wiki),
}

impl Strat {
    pub fn random() -> StratBuilder {
        StratBuilder::Random
    }
    pub fn wiki() -> StratBuilder {
        StratBuilder::Wiki
    }
    pub fn deal_card(&mut self, ctx: &Context, card: Card) {
        match self {
            Strat::Random(s) => s.deal_card(ctx, card),
            Strat::Wiki(s) => s.deal_card(ctx, card),
        }
    }

    pub fn action(&mut self, ctx: &Context) -> Option<Action> {
        match self {
            Strat::Random(s) => s.action(ctx),
            Strat::Wiki(s) => s.action(ctx),
        }
    }

    pub fn react(&mut self, ctx: &Context, a: Announcement) {
        match self {
            Strat::Random(s) => s.react(ctx, a),
            Strat::Wiki(s) => s.react(ctx, a),
        }
    }
}

#[derive(Debug, Clone, Copy)]
pub struct PublicPlayerInfo {
    pub cards_on_hand: usize,
    pub books: Ranks,
}
impl Default for PublicPlayerInfo {
    fn default() -> Self {
        Self {
            cards_on_hand: Default::default(),
            books: Ranks::empty(),
        }
    }
}

#[derive(Debug)]
pub struct Context {
    players: Vec<PublicPlayerInfo>,
}

impl Context {
    pub fn new(num_players: usize) -> Self {
        Context {
            players: vec![Default::default(); num_players],
        }
    }
    pub fn update(&mut self, players: &[Player]) {
        for (p, c) in players.iter().zip(self.players.iter_mut()) {
            *c = PublicPlayerInfo {
                cards_on_hand: p.hand.num() as _,
                books: p.books,
            }
        }
    }
    pub fn players<'s>(&'s self) -> impl Iterator<Item = (PlayerId, PublicPlayerInfo)> + 's {
        self.players
            .iter()
            .enumerate()
            .map(|(i, p)| (PlayerId(i as _), *p))
    }
}

#[derive(Debug, Clone, Serialize)]
pub struct Action {
    pub ask_who: PlayerId,
    pub ask_for: Rank,
}

#[typeshare]
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum Announcement {
    Action {
        player_asking: PlayerId,
        player_asked: PlayerId,
        asked_for: Rank,
        response: Response,
    },
    GotBook {
        player: PlayerId,
        book: Rank,
    },
}

#[typeshare]
#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "type", content = "content")]
pub enum Response {
    GoFish,
    TakeThese { count: u32 },
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct PlayerId(pub u32);
