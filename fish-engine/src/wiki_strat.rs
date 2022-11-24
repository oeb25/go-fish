use itertools::Itertools;
use serde::Serialize;

use crate::{
    cards::{Card, Cards, Rank},
    common_strat::update_hand_on_announcement,
    strategy::{Action, Announcement, Context, PlayerId, Strategy},
};

#[derive(Debug, Serialize)]
pub struct Wiki {
    pid: PlayerId,
    hand: Cards,

    next_rank: Rank,
}

impl Strategy for Wiki {
    fn init(pid: PlayerId) -> Self {
        Wiki {
            pid,
            hand: Cards::empty(),
            next_rank: Rank::RA,
        }
    }

    fn deal_card(&mut self, _ctx: &Context, card: Card) {
        self.next_rank = card.rank();
        self.hand = self.hand.add(card);
    }

    fn action(&mut self, ctx: &Context) -> Option<Action> {
        let ask_who = *crate::pick(
            &ctx.players()
                .filter_map(|(pid, p)| {
                    if pid != self.pid && p.cards_on_hand > 0 {
                        Some(pid)
                    } else {
                        None
                    }
                })
                .collect_vec(),
        )?;

        while self
            .hand
            .intersection(self.next_rank.in_all_suits())
            .is_empty()
        {
            self.next_rank = self.next_rank.next();
        }

        let ask_for = self.next_rank;
        self.next_rank = self.next_rank.next();

        Some(Action { ask_who, ask_for })
    }

    fn react(&mut self, _ctx: &Context, ann: Announcement) {
        update_hand_on_announcement(self.pid, ann, &mut self.hand);
    }
}
