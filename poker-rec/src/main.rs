pub mod decking;
pub mod kuhn;

use kuhn::{Node, ActionSpace};
use decking::Deck;

fn main() {
    let deck = Deck::new();
    println!("{:?}", deck);

    let game = Node::new(false, 0, vec![ActionSpace::BET, ActionSpace::CHECK], Vec::new(), deck);
    println!("{:?}", game);

    // is_chance: bool, to_move: Option<u8>, actions: Vec<ActionSpace>, actions_history: Vec<ActionSpace>, cards: Vec<Card>
}
