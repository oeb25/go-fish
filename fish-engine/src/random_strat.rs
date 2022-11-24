use itertools::Itertools;
use serde::Serialize;

use crate::{
    cards::{Card, Cards},
    common_strat::update_hand_on_announcement,
    strategy::{Action, Announcement, Context, PlayerId, Strategy},
};

#[derive(Debug, Default, Serialize)]
pub struct Random {
    pid: PlayerId,
    hand: Cards,
}

impl Strategy for Random {
    fn init(pid: PlayerId) -> Self {
        Random {
            pid,
            hand: Cards::empty(),
        }
    }

    fn deal_card(&mut self, _ctx: &Context, card: Card) {
        self.hand = self.hand.add(card);
    }

    fn action(&mut self, ctx: &Context) -> Option<Action> {
        let options = ctx
            .players()
            .filter_map(|(pid, p)| {
                if pid != self.pid && p.cards_on_hand > 0 {
                    Some(pid)
                } else {
                    None
                }
            })
            .collect_vec();

        if options.is_empty() {
            log::error!(
                "There were nothing for me ({:?}) to do :( {ctx:?}",
                self.pid
            );
        }

        let ask_who = *crate::pick(&options)?;

        // println!("{ask_who:?}");

        Some(Action {
            ask_who,
            ask_for: self.hand.choose_random()?.rank(),
        })
    }

    fn react(&mut self, _ctx: &Context, ann: Announcement) {
        update_hand_on_announcement(self.pid, ann, &mut self.hand);
    }
}
