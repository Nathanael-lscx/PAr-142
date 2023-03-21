pub mod decking;
pub mod kuhn;
pub mod algorithm;

use kuhn::{Node, ActionSpace};
use decking::Deck;

fn main() {
    let deck = Deck::new();
    println!("{:?}", deck);

    let game = Node::new(false, 0, vec![ActionSpace::BET, ActionSpace::CHECK], Vec::new(), deck);
    println!("{:?}", game);
    println!("{:?}", game.information_set());

    let mut cfr = algorithm::CounterfactualRegretMinimization::new(&game, true); 

    // cfr.update_strategy_sum(cfr.root.information_set(), cfr.root.actions[0], 1f32);
    // println!("strategy_sum: {:?}", cfr.strategy_sum);

    // cfr.compute_nash();
    // println!("nash: {:?}", cfr.nash);
    // println!("strategy_sum: {:?}", cfr.strategy_sum);

    let nb_iteration = 1000i16;
    for _ in 0..nb_iteration {
        cfr.cfr_utility_rec(&game, vec![1f32, 1f32]);
    }

    cfr.compute_nash();
    println!("strategy_sum: {:?}", cfr.strategy_sum);
    let value = cfr.get_value_of_node(&game);
    println!("value: {:?}", value);

    // is_chance: bool, to_move: Option<u8>, actions: Vec<ActionSpace>, actions_history: Vec<ActionSpace>, cards: Vec<Card>
}
