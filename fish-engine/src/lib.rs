use log::info;
use serde::Serialize;

pub mod cards;
pub mod common_strat;
pub mod random_strat;
pub mod strategy;
pub mod wiki_strat;

use cards::{Card, Cards, Ranks};
use strategy::{
    Announcement, Context, PlayerId, PublicPlayerInfo, Response, Strat, StratBuilder, Strategy,
};
use typeshare::typeshare;

pub fn pick<T>(xs: &[T]) -> Option<&T> {
    if xs.is_empty() {
        return None;
    }
    xs.get(fastrand::usize(0..xs.len()))

    // let mut dest = [0; (usize::BITS / u8::BITS) as usize];

    // getrandom::getrandom(&mut dest).expect("Failed to generate random number for pick");
    // xs.get(usize::from_be_bytes(dest) % xs.len())
}

fn ranks_to_vec<S>(ranks: &Ranks, serializer: S) -> Result<S::Ok, S::Error>
where
    S: serde::Serializer,
{
    ranks.to_vec().serialize(serializer)
}

#[typeshare]
#[derive(Debug, Serialize)]
pub struct Player {
    pub hand: Cards,
    #[serde(serialize_with = "ranks_to_vec")]
    pub books: Ranks,
    #[serde(skip)]
    pub strategy: Strat,
}

impl Player {
    pub fn new(strategy: Strat) -> Player {
        Player {
            hand: Cards::empty(),
            books: Ranks::empty(),
            strategy,
        }
    }

    pub fn deal_card(&mut self, ctx: &Context, c: Card) {
        self.strategy.deal_card(ctx, c);
        self.hand = self.hand.add(c);
    }

    pub fn take(&mut self, c: Card) {
        self.hand = self.hand.remove(Cards::empty().add(c));
    }
}

#[typeshare]
#[derive(Debug, Serialize)]
pub struct Game {
    starting_cards: u32,
    pub pool: Cards,
    pub players: Vec<Player>,
    pub announcements: Vec<Announcement>,
    pub stage: GameStage,
    #[serde(skip)]
    ctx: Context,
}

#[typeshare]
#[derive(Debug, Serialize, Clone, derive_more::IsVariant)]
#[serde(tag = "name", content = "content")]
pub enum GameStage {
    Dealing { who_next: PlayerId },
    Playing { who_next: PlayerId },
    Done { who_next: PlayerId },
}

impl Game {
    pub fn new<const N: usize>(starting_cards: u32, players: [StratBuilder; N]) -> Self {
        let num_players = players.len();
        let ctx = Context::new(players.len());

        Game {
            starting_cards,
            pool: Cards::all(),
            players: players
                .into_iter()
                .enumerate()
                .map(|(pid, s)| Player::new(s.init(PlayerId(pid as _))))
                .collect(),
            announcements: vec![],
            stage: GameStage::Dealing {
                who_next: PlayerId(fastrand::u32(0..num_players as u32)),
            },
            ctx,
        }
    }
    pub fn check_validity(&self) {
        let mut seen = self.pool;

        for p in &self.players {
            if !self.pool.intersection(p.hand).is_empty() {
                panic!("PLAYER HAD CARDS IN HAND THAT WERE SOME WHERE IN THE POOL");
            }
            if !seen.intersection(p.hand).is_empty() {
                panic!("PLAYER HAD CARDS IN HAND THAT WERE SOME WHERE ELSE");
            }
            seen = p.hand.union(seen);

            for r in p.books.iter() {
                let rank = r.in_all_suits();
                if !seen.intersection(rank).is_empty() {
                    panic!("PLAYER HAD CARDS IN HAND THAT WERE SOME WHERE ELSE");
                }
                seen = rank.union(seen);
            }
        }

        assert_eq!(seen, Cards::all());
    }
    pub fn step(&mut self) {
        self.ctx.update(&self.players);

        let prev_count = self.announcements.len();
        self.step_inner();

        for a in &self.announcements[prev_count..] {
            for p in &mut self.players {
                p.strategy.react(&self.ctx, *a);
            }
        }
    }
    fn check_books(&mut self, pid: PlayerId) {
        let p = &mut self.players[pid.0 as usize];

        let hand = p.hand;
        let mut seen = Cards::empty();

        let ranks = hand
            .iter()
            .map(|c| c.rank())
            .fold(Ranks::empty(), |rs, r| rs | r.into());

        for rank in ranks.iter() {
            if rank.in_all_suits().intersection(hand) == rank.in_all_suits() {
                p.hand = p.hand.remove(rank.in_all_suits());
                for c in rank.in_all_suits().iter() {
                    seen = seen.add(c);
                }
                self.announcements.push(Announcement::GotBook {
                    player: pid,
                    book: rank,
                });
                p.books |= rank.into();
            }
        }
    }
    fn step_inner(&mut self) {
        match self.stage.clone() {
            GameStage::Dealing { who_next } => {
                let p: &mut Player = &mut self.players[who_next.0 as usize];

                if self.pool.num() == 0 || p.hand.num() == self.starting_cards {
                    self.stage = GameStage::Playing { who_next };

                    for i in 0..self.players.len() {
                        assert_eq!(
                            self.players[i].hand.num(),
                            self.starting_cards,
                            "Oh no! Player {} only has {} cards. There are {} left in the pool",
                            i,
                            self.players[i].hand.num(),
                            self.pool.num()
                        );

                        self.check_books(PlayerId(i as _));
                    }
                } else {
                    let Some(c) = self.pool.choose_random() else {
                        panic!("did not find any card");
                    };
                    self.pool = self.pool.remove_one(c);
                    p.deal_card(&self.ctx, c);
                    self.stage = GameStage::Dealing {
                        who_next: PlayerId((who_next.0 + 1) % self.players.len() as u32),
                    };
                }
            }
            GameStage::Playing { who_next } => {
                if self.players.iter().all(|p| p.hand.is_empty()) {
                    self.stage = GameStage::Done { who_next };
                    return;
                }

                if self.players.iter().filter(|p| !p.hand.is_empty()).count() == 1 {
                    let (idx, p) = self
                        .players
                        .iter_mut()
                        .enumerate()
                        .find(|(_, p)| !p.hand.is_empty())
                        .unwrap();
                    p.hand = p.hand.union(self.pool);

                    self.pool = Cards::empty();

                    self.check_books(PlayerId(idx as _));

                    return;
                }

                self.ctx.update(&self.players);
                let p: &mut Player = &mut self.players[who_next.0 as usize];

                if p.hand.is_empty() {
                    self.stage = GameStage::Playing {
                        who_next: PlayerId((who_next.0 + 1) % self.players.len() as u32),
                    };
                    return;
                }

                if let Some(action) = p.strategy.action(&self.ctx) {
                    if who_next == action.ask_who {
                        panic!("Tried to ask self for cards!");
                        return;
                    }

                    let rank = action.ask_for;

                    if p.hand.intersection(rank.in_all_suits()).is_empty() {
                        info!(
                            "Asked for a card they didn't have! {:?} asked for {:?}",
                            p, rank
                        );
                        return;
                    }

                    self.ctx.update(&self.players);
                    let asked: &mut Player = &mut self.players[action.ask_who.0 as usize];
                    let response = if asked.hand.intersection(rank.in_all_suits()).is_empty() {
                        if let Some(drawn) = self.pool.choose_random() {
                            self.pool = self.pool.remove_one(drawn);

                            let p: &mut Player = &mut self.players[who_next.0 as usize];
                            p.deal_card(&self.ctx, drawn);

                            self.check_validity();

                            Response::GoFish
                        } else {
                            self.check_validity();

                            Response::GoFish
                        }
                    } else {
                        let had = asked.hand.intersection(rank.in_all_suits());

                        for c in had.iter() {
                            asked.take(c);
                        }
                        let p: &mut Player = &mut self.players[who_next.0 as usize];
                        for c in had.iter() {
                            p.deal_card(&self.ctx, c);
                        }

                        Response::TakeThese { count: had.num() }
                    };

                    let announcement = Announcement::Action {
                        player_asking: who_next,
                        player_asked: action.ask_who,
                        // asked_for: face,
                        asked_for: rank,
                        response,
                    };

                    self.announcements.push(announcement);

                    self.stage = GameStage::Playing {
                        who_next: PlayerId((who_next.0 + 1) % self.players.len() as u32),
                    };

                    self.check_books(who_next);
                } else {
                    info!("Player had no action...");
                }

                let p: &mut Player = &mut self.players[who_next.0 as usize];
                if p.hand.is_empty() {
                    self.stage = GameStage::Playing {
                        who_next: PlayerId((who_next.0 + 1) % self.players.len() as u32),
                    };
                }
            }
            GameStage::Done { .. } => {}
        }
    }
}
