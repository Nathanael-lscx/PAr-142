pub mod algorithm;
pub mod decking;
pub mod kuhn;

use algorithm::CounterfactualRegretMinimization;
use decking::Deck;
use kuhn::{ActionSpace, Node, Root};

fn train_with_harcoding(nb_iteration: i32, cfr: &mut CounterfactualRegretMinimization) -> Vec<f64> {
    let root: Root = Root::new();

    for _ in 0..nb_iteration {
        cfr.cfr_utility(&root);
    }

    cfr.compute_nash();
    cfr.get_value_of_root(&root)
}

fn train_without_harcoding(
    nb_iteration: i32,
    cfr: &mut CounterfactualRegretMinimization,
) -> Vec<f64> {
    let root: Root = Root::new();

    for _ in 0..nb_iteration {
        let deck = Deck::new();
        let game = Node::new(
            0,
            vec![ActionSpace::BET, ActionSpace::CHECK],
            Vec::new(),
            deck,
        );
        cfr.cfr_utility_rec(&game, vec![1f64, 1f64]);
    }

    cfr.compute_nash();
    cfr.get_value_of_root(&root)
}

fn estime_value(nb_iteration: i32, cfr: &CounterfactualRegretMinimization) -> f64 {
    let mut value = 0f64;
    for _ in 0..nb_iteration {
        let deck = Deck::new();
        let game = Node::new(
            0,
            vec![ActionSpace::BET, ActionSpace::CHECK],
            Vec::new(),
            deck,
        );
        value += cfr.get_value_of_node(&game)[0];
    }

    value / nb_iteration as f64
}

fn main() {
    let mut cfr = algorithm::CounterfactualRegretMinimization::new();

    let nb_iteration = 10000;
    let value = train_with_harcoding(nb_iteration, &mut cfr);
    println!("Value with hardcoding: {:?}", value);

    let mut cfr = algorithm::CounterfactualRegretMinimization::new();

    let nb_iteration = 10000;
    let value = train_without_harcoding(nb_iteration, &mut cfr);
    println!("Value without hardcoding: {:?}", value);

    let nb_mean = 10000;
    let value = estime_value(nb_mean, &cfr);
    println!("Estimed value: {:?}", value);
}
