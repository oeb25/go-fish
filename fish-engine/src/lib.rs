use std::{
    fmt::{Debug, Display},
    ops::Range,
};

use itertools::Itertools;
use log::info;
use serde::Serialize;

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Card(pub u64);

impl Card {
    pub fn all() -> impl Iterator<Item = Card> {
        (0..52).map(Card)
    }

    fn in_all_suits(self) -> Cards {
        let base = self.0 % 13;

        Cards::empty()
            .add(Card(base + 13 * 0))
            .add(Card(base + 13 * 1))
            .add(Card(base + 13 * 2))
            .add(Card(base + 13 * 3))
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = "🂡🂢🂣🂤🂥🂦🂧🂨🂩🂪🂫🂬🂭🂮🂱🂲🂳🂴🂵🂶🂷🂸🂹🂺🂻🂼🂽🂾🃁🃂🃃🃄🃅🃆🃇🃈🃉🃊🃋🃌🃍🃎🃑🃒🃓🃔🃕🃖🃗🃘🃙🃚🃛🃜🃝🃞";

        return write!(f, "{}", cards.chars().nth(self.0 as _).unwrap());

        let face = self.0 % 13;
        let suit = self.0 / 13;

        let face = "A123456789JQK".chars().nth(face as _).unwrap();
        let suit = "♣♥♦♠".chars().nth(suit as _).unwrap();

        write!(f, "{suit}{face}")
    }
}

#[derive(Default, Clone, Copy)]
pub struct Cards {
    cards: u64,
}

impl Serialize for Cards {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.iter().collect_vec().serialize(serializer)
    }
}

impl Cards {
    pub fn empty() -> Self {
        Cards { cards: 0 }
    }
    pub fn all() -> Self {
        Cards {
            cards: (1 << 53) - 1,
        }
    }
    pub fn is_empty(self) -> bool {
        self.cards == 0
    }
    pub fn add(self, c: Card) -> Self {
        Cards {
            cards: self.cards | (1 << c.0),
        }
    }
    pub fn remove(self, c: Card) -> Self {
        let mut cards = self.cards;
        cards &= !(1 << c.0);

        Cards { cards }
    }
    pub fn intersection(self, cs: Cards) -> Self {
        Cards {
            cards: self.cards & cs.cards,
        }
    }
    pub fn has(self, c: Card) -> bool {
        !self.intersection(Cards::empty().add(c)).is_empty()
    }
    pub fn num(self) -> u32 {
        self.cards.count_ones() as _
    }
    pub fn choose_random(self) -> Option<Card> {
        fn choose(r: Range<u64>) -> u64 {
            let len = r.end - r.start;

            let mut dest = [0; (u64::BITS / u8::BITS) as usize];

            getrandom::getrandom(&mut dest);
            r.start + (u64::from_be_bytes(dest) % len)
        }

        if self.is_empty() {
            return None;
        }

        let i = choose(0..self.num() as u64);

        self.iter().nth(i as _)
    }
    pub fn iter(&self) -> impl Iterator<Item = Card> + '_ {
        (0..52).filter_map(|idx| {
            if (1 << idx) & self.cards != 0 {
                Some(Card(idx))
            } else {
                None
            }
        })
    }
}

impl Debug for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iter().format(","))
    }
}
impl Display for Cards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.iter().format(","))
    }
}

fn pick<T>(xs: &[T]) -> &T {
    let mut dest = [0; (usize::BITS / u8::BITS) as usize];

    getrandom::getrandom(&mut dest);
    &xs[(usize::from_be_bytes(dest) % xs.len())]
}

#[derive(Debug, Clone, Serialize)]
pub struct Action {
    ask_who: PlayerId,
    ask_for: Card,
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Announcement {
    pub player_asking: PlayerId,
    pub player_asked: PlayerId,
    pub asked_for: Cards,
    pub response: Response,
}

#[derive(Debug, Clone, Copy, Serialize)]
#[serde(tag = "type")]
pub enum Response {
    GoFish,
    TakeThese { count: u32 },
}

#[derive(Debug, Default, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize)]
pub struct PlayerId(pub u32);

pub struct Context {
    players: Vec<PlayerId>,
}

pub trait Strategy: Debug {
    fn deal_card(&mut self, ctx: &mut Context, card: Card);
    fn action(&mut self, ctx: &mut Context) -> Option<Action>;
    fn react(&mut self, ctx: &mut Context, res: Announcement);
}

#[derive(Debug, Default, Serialize)]
pub struct Random {
    hand: Cards,
}

impl Strategy for Random {
    fn deal_card(&mut self, ctx: &mut Context, card: Card) {
        self.hand = self.hand.add(card);
    }

    fn action(&mut self, ctx: &mut Context) -> Option<Action> {
        let ask_who = *pick(&ctx.players);

        Some(Action {
            ask_who,
            ask_for: self.hand.choose_random()?,
        })
    }

    fn react(&mut self, ctx: &mut Context, res: Announcement) {}
}

#[derive(Debug, Serialize)]
pub struct Player {
    hand: Cards,
    pairs: Vec<Cards>,
    #[serde(skip)]
    strategy: Box<dyn Strategy>,
}

impl Player {
    pub fn new(strategy: Box<dyn Strategy>) -> Player {
        Player {
            hand: Cards::empty(),
            pairs: vec![],
            strategy,
        }
    }

    pub fn deal_card(&mut self, ctx: &mut Context, c: Card) {
        self.strategy.deal_card(ctx, c);
        self.hand = self.hand.add(c);
    }

    pub fn take(&mut self, c: Card) {
        self.hand = self.hand.remove(c);
    }
}

#[derive(Debug, Serialize)]
pub struct Game {
    starting_cards: u32,
    pool: Cards,
    players: Vec<Player>,
    pub announcements: Vec<Announcement>,
    pub stage: GameStage,
}

#[derive(Debug, Serialize)]
#[serde(tag = "name")]
pub enum GameStage {
    Dealing { who_next: PlayerId },
    Playing { who_next: PlayerId },
    Done { who_next: PlayerId },
}

impl Game {
    pub fn new(starting_cards: u32, mut players: Vec<Player>) -> Self {
        info!("Starting with {starting_cards} cards!");

        let mut pool = Cards::all();

        let mut ctx = Context { players: vec![] };

        info!("They are dealt! {players:?}");

        Game {
            starting_cards,
            pool,
            players,
            announcements: vec![],
            stage: GameStage::Dealing {
                who_next: PlayerId(0),
            },
        }
    }
    pub fn step(&mut self) {
        let mut ctx = Context {
            players: (0..self.players.len() as _).map(PlayerId).collect_vec(),
        };

        match &mut self.stage {
            GameStage::Dealing { who_next } => {
                let p: &mut Player = &mut self.players[who_next.0 as usize];

                if self.pool.num() == 0 || p.hand.num() == self.starting_cards {
                    self.stage = GameStage::Playing {
                        who_next: *who_next,
                    };
                } else {
                    let Some(c) = self.pool.choose_random() else {
                        return;
                    };
                    self.pool = self.pool.remove(c);
                    p.deal_card(&mut ctx, c);
                    *who_next = PlayerId((who_next.0 + 1) % self.players.len() as u32);
                }
            }
            GameStage::Playing { who_next } => {
                if self.players.iter().all(|p| p.hand.is_empty()) {
                    self.stage = GameStage::Done {
                        who_next: *who_next,
                    };
                    return;
                }

                let p: &mut Player = &mut self.players[who_next.0 as usize];

                if p.hand.is_empty() {
                    *who_next = PlayerId((who_next.0 + 1) % self.players.len() as u32);
                    return;
                }

                if let Some(action) = p.strategy.action(&mut ctx) {
                    if *who_next == action.ask_who {
                        info!("Whuups! Tried to ask self for card");
                        return;
                    }

                    let face = action.ask_for.in_all_suits();

                    if p.hand.intersection(face).is_empty() {
                        info!("Asked for a card they didn't have!");
                        return;
                    }

                    let asked: &mut Player = &mut self.players[action.ask_who.0 as usize];
                    let announcement = if asked.hand.intersection(face).is_empty() {
                        if let Some(drawn) = self.pool.choose_random() {
                            self.pool = self.pool.remove(drawn);

                            let p: &mut Player = &mut self.players[who_next.0 as usize];
                            p.deal_card(&mut ctx, drawn);
                            // self.players[self.who_next.0 as usize]
                            //     .strategy
                            //     .react(&mut ctx, Announcement::GoFish(drawn));
                            Announcement {
                                player_asking: *who_next,
                                player_asked: action.ask_who,
                                // asked_for: face,
                                asked_for: Cards::empty().add(drawn),
                                response: Response::GoFish,
                            }
                        } else {
                            Announcement {
                                player_asking: *who_next,
                                player_asked: action.ask_who,
                                // asked_for: face,
                                asked_for: Cards::empty().add(action.ask_for),
                                response: Response::GoFish,
                            }
                        }
                    } else {
                        info!("THEY MATCHED!");

                        let had = asked.hand.intersection(face);

                        for c in had.iter() {
                            asked.take(c);
                        }
                        let p: &mut Player = &mut self.players[who_next.0 as usize];
                        for c in had.iter() {
                            p.deal_card(&mut ctx, c);
                        }

                        Announcement {
                            player_asking: *who_next,
                            player_asked: action.ask_who,
                            // asked_for: face,
                            asked_for: had,
                            response: Response::GoFish,
                        }
                    };

                    self.announcements.push(announcement);

                    *who_next = PlayerId((who_next.0 + 1) % self.players.len() as u32);

                    for p in &mut self.players {
                        let hand = p.hand;

                        for c in hand.iter() {
                            if c.in_all_suits().iter().all(|c| hand.has(c)) {
                                for c in c.in_all_suits().iter() {
                                    p.hand = p.hand.remove(c);
                                }
                                p.pairs.push(c.in_all_suits());
                            }
                        }
                    }

                    // info!("{action:?}");
                } else {
                    info!("Player had no action...");
                }

                let p: &mut Player = &mut self.players[who_next.0 as usize];
                if p.hand.is_empty() {
                    *who_next = PlayerId((who_next.0 + 1) % self.players.len() as u32);
                    return;
                }
            }
            GameStage::Done { .. } => {}
        }

        // let mut ctx = Context {
        //     players: (0..self.players.len() as _).map(PlayerId).collect_vec(),
        // };

        // let p: &mut Player = &mut self.players[self.who_next.0 as usize];
        // let action = p.strategy.action(&mut ctx);

        // if self.who_next == action.ask_who {
        //     panic!();
        // }

        // let asked: &mut Player = &mut self.players[self.who_next.0 as usize];
        // if asked.hand.intersection(action.ask_for).is_empty() {
        //     let drawn = self.pool.choose_random();
        //     self.pool = self.pool.remove(drawn);
        //     self.players[self.who_next.0 as usize]
        //         .strategy
        //         .react(&mut ctx, Announcement::GoFish(drawn));
        // } else {
        // }
        // asked.
    }
}
