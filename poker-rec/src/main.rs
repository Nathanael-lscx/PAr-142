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

    let mut cfr = algorithm::CounterfactualRegretMinimization::new(true); 

    // cfr.update_strategy_sum(cfr.root.information_set(), cfr.root.actions[0], 1f32);
    // println!("strategy_sum: {:?}", cfr.strategy_sum);

    // cfr.compute_nash();
    // println!("nash: {:?}", cfr.nash);
    // println!("strategy_sum: {:?}", cfr.strategy_sum);

    let nb_iteration = i32::pow(10, 4);
    println!("nb_iteration: {:?}", nb_iteration);
    let mut card = 1f32;
    for _ in 0..nb_iteration {
        let deck = Deck::new();
        card += deck.d[0].val as f32 - deck.d[1].val as f32;
        let game = Node::new(false, 0, vec![ActionSpace::BET, ActionSpace::CHECK], Vec::new(), deck);
        cfr.cfr_utility_rec(&game, vec![1f32, 1f32]);
    }
    card /= 1000f32;
    println!("card: {:?}", card);

    cfr.compute_nash();
    println!("nash_equilibrum: {:?}", cfr.nash);

    let mut value = 0f32;
    let mut card = 1f32;
    for _ in 0..10000 {
        let deck = Deck::new();
        card += deck.d[0].val as f32 - deck.d[1].val as f32;
        let game = Node::new(false, 0, vec![ActionSpace::BET, ActionSpace::CHECK], Vec::new(), deck);
        value += cfr.get_value_of_node(&game)[0];
    }
    card /= 10000f32;
    value /= 10000f32;
    println!("card: {:?}", card);
    println!("value: {:?}", value);

    // is_chance: bool, to_move: Option<u8>, actions: Vec<ActionSpace>, actions_history: Vec<ActionSpace>, cards: Vec<Card>
}
