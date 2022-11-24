use crate::{
    cards::Cards,
    strategy::{Announcement, PlayerId},
};

pub fn update_hand_on_announcement(pid: PlayerId, ann: Announcement, hand: &mut Cards) {
    match ann {
        Announcement::Action {
            player_asked,
            asked_for,
            ..
        } if player_asked == pid => {
            *hand = hand.remove(asked_for.in_all_suits());
        }
        Announcement::GotBook { player, book } if player == pid => {
            *hand = hand.remove(book.in_all_suits());
        }
        _ => {}
    }
}
