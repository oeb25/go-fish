use std::{
    fmt::{Debug, Display},
    ops::Range,
};

use bitflags::bitflags;
use itertools::Itertools;
use serde::Serialize;
use strum::IntoEnumIterator;
use strum_macros::{EnumIter, FromRepr};
use typeshare::typeshare;

#[derive(Debug, Clone, Copy, Serialize, EnumIter, FromRepr)]
#[repr(u64)]
pub enum Rank {
    RA = 0,
    R2 = 1,
    R3 = 2,
    R4 = 3,
    R5 = 4,
    R6 = 5,
    R7 = 6,
    R8 = 7,
    R9 = 8,
    R10 = 9,
    RJ = 10,
    RQ = 11,
    RK = 12,
}
bitflags! {
    pub struct Ranks: u64 {
        const RA = 1 << Rank::RA as u64;
        const R2 = 1 << Rank::R2 as u64;
        const R3 = 1 << Rank::R3 as u64;
        const R4 = 1 << Rank::R4 as u64;
        const R5 = 1 << Rank::R5 as u64;
        const R6 = 1 << Rank::R6 as u64;
        const R7 = 1 << Rank::R7 as u64;
        const R8 = 1 << Rank::R8 as u64;
        const R9 = 1 << Rank::R9 as u64;
        const R10 = 1 << Rank::R10 as u64;
        const RJ = 1 << Rank::RJ as u64;
        const RQ = 1 << Rank::RQ as u64;
        const RK = 1 << Rank::RK as u64;
    }
}
impl From<Rank> for Ranks {
    fn from(val: Rank) -> Self {
        Ranks::from_bits(1 << val as u64).unwrap()
    }
}
impl Ranks {
    pub fn iter(self) -> impl Iterator<Item = Rank> {
        Rank::iter().filter(move |&r| self.intersects(r.into()))
    }
    pub fn to_vec(&self) -> Vec<Rank> {
        self.iter().collect()
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
#[repr(u64)]
pub enum Suit {
    Spades = 0,
    Hearths = 1,
    Diamonds = 2,
    Clubs = 3,
}
bitflags! {
    pub struct Suits: u64 {
        const SPADES = 1 << Suit::Spades as u64;
        const HEARTHS = 1 << Suit::Hearths as u64;
        const DIAMONDS = 1 << Suit::Diamonds as u64;
        const CLUBS = 1 << Suit::Clubs as u64;
    }
}

#[derive(Debug, Clone, Copy, Serialize)]
pub struct Card(pub u64);

impl Card {
    pub fn new(rank: Rank, suit: Suit) -> Self {
        Card(rank as u64 + suit as u64 * 13)
        // Card(1 << (rank as u64) << (suit as u64 * 13))
    }

    pub fn all() -> impl Iterator<Item = Card> {
        (0..52).map(Card)
    }

    pub fn rank(self) -> Rank {
        match self.0 % 13 {
            0 => Rank::RA,
            1 => Rank::R2,
            2 => Rank::R3,
            3 => Rank::R4,
            4 => Rank::R5,
            5 => Rank::R6,
            6 => Rank::R7,
            7 => Rank::R8,
            8 => Rank::R9,
            9 => Rank::R10,
            10 => Rank::RJ,
            11 => Rank::RQ,
            12 => Rank::RK,
            _ => unreachable!(),
        }
    }

    pub fn suit(self) -> Suit {
        match self.0 / 13 {
            0 => Suit::Spades,
            1 => Suit::Hearths,
            2 => Suit::Diamonds,
            3 => Suit::Clubs,
            _ => unreachable!(),
        }
    }
}

impl Rank {
    pub fn in_all_suits(self) -> Cards {
        Cards::empty()
            .add(Card::new(self, Suit::Spades))
            .add(Card::new(self, Suit::Hearths))
            .add(Card::new(self, Suit::Diamonds))
            .add(Card::new(self, Suit::Clubs))
    }
    pub fn next(self) -> Rank {
        match self {
            Rank::RA => Rank::R2,
            Rank::R2 => Rank::R3,
            Rank::R3 => Rank::R4,
            Rank::R4 => Rank::R5,
            Rank::R5 => Rank::R6,
            Rank::R6 => Rank::R7,
            Rank::R7 => Rank::R8,
            Rank::R8 => Rank::R9,
            Rank::R9 => Rank::R10,
            Rank::R10 => Rank::RJ,
            Rank::RJ => Rank::RQ,
            Rank::RQ => Rank::RK,
            Rank::RK => Rank::RA,
        }
    }
}

impl Display for Card {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let cards = "🂡🂢🂣🂤🂥🂦🂧🂨🂩🂪🂫🂬🂭🂮🂱🂲🂳🂴🂵🂶🂷🂸🂹🂺🂻🂼🂽🂾🃁🃂🃃🃄🃅🃆🃇🃈🃉🃊🃋🃌🃍🃎🃑🃒🃓🃔🃕🃖🃗🃘🃙🃚🃛🃜🃝🃞";

        write!(f, "{}", cards.chars().nth(self.0 as _).unwrap())

        // let face = self.0 % 13;
        // let suit = self.0 / 13;

        // let face = "A123456789JQK".chars().nth(face as _).unwrap();
        // let suit = "♣♥♦♠".chars().nth(suit as _).unwrap();

        // write!(f, "{suit}{face}")
    }
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
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
            cards: (1 << 52) - 1,
        }
    }
    pub fn all_bounded(n: u64) -> Self {
        assert!((0..=13).contains(&n));

        let mask = (1 << n) - 1;
        let cards = mask | (mask << 13) | (mask << (13 * 2)) | (mask << (13 * 3));

        Cards { cards }
    }
    pub fn is_empty(self) -> bool {
        self.cards == 0
    }
    pub fn add(self, c: Card) -> Self {
        Cards {
            cards: self.cards | (1 << c.0),
        }
    }
    pub fn remove(self, cs: Cards) -> Self {
        Cards {
            cards: self.cards & !cs.cards,
        }
    }
    pub fn remove_one(self, c: Card) -> Self {
        let mut cards = self.cards;
        cards &= !(1 << c.0);

        Cards { cards }
    }
    pub fn intersection(self, cs: Cards) -> Self {
        Cards {
            cards: self.cards & cs.cards,
        }
    }
    pub fn union(self, cs: Cards) -> Self {
        Cards {
            cards: self.cards | cs.cards,
        }
    }
    pub fn has(self, c: Card) -> bool {
        !self.intersection(Cards::empty().add(c)).is_empty()
    }
    pub fn num(self) -> u32 {
        assert_eq!(self.cards.count_ones() as usize, self.iter().count());

        self.cards.count_ones() as _
    }
    pub fn choose_random(self) -> Option<Card> {
        // fn choose(r: Range<u64>) -> u64 {
        //     let len = r.end - r.start;

        //     let mut dest = [0; (u64::BITS / u8::BITS) as usize];

        //     getrandom::getrandom(&mut dest).expect("Failed to generate random numbers");
        //     r.start + (u64::from_be_bytes(dest) % len)
        // }

        if self.is_empty() {
            return None;
        }

        let i = fastrand::u64(0..self.num() as u64);

        // let i = choose(0..self.num() as u64);

        if let Some(c) = self.iter().nth(i as _) {
            Some(c)
        } else {
            todo!("{self:?} {}", self.num())
        }
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
