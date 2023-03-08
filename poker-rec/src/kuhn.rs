/*
    Fichier implémentant le kuhn poker récursif
*/

use std::collections::HashMap;
use crate::decking::Deck;

#[derive(PartialEq, Hash, Clone, Copy, Debug)]
pub enum ActionSpace {
    BET,
    CHECK,
    CALL,
    FOLD,
    NONE,
    END,
}
impl Eq for ActionSpace {}

#[derive(Debug)]
pub struct Node {
    // pub parent: Option<Box<Node>>,
    is_chance: bool,
    to_move: Option<u8>,
    pub cards: Deck,

    pub actions_history: Vec<ActionSpace>,
    actions: Vec<ActionSpace>,
    children: HashMap<ActionSpace, Node>,
}

impl Node {
    pub fn new(is_chance: bool, to_move: Option<u8>, actions: Vec<ActionSpace>, actions_history: Vec<ActionSpace>, cards: Deck) -> Self {
        let mut children = HashMap::new();
        let mover: Option<u8>;

        if to_move.is_some() {
            mover = Some((to_move.unwrap()+1) % 2);
        } else {
            mover = Some(0u8);
        }
        
        fn nextactions(a: ActionSpace, actions_hist: &Vec<ActionSpace>) -> Vec<ActionSpace> {
            if (actions_hist.len() == 0) && a == ActionSpace::BET {
                vec![ActionSpace::FOLD, ActionSpace::CALL]
            } else if (actions_hist.len() == 0) && a == ActionSpace::CHECK {
                vec![ActionSpace::BET]
            } else if (actions_hist.last().unwrap() == &ActionSpace::CHECK) && a == ActionSpace::BET {
                vec![ActionSpace::FOLD, ActionSpace::CALL]
            } else if (a==ActionSpace::CALL) || (a==ActionSpace::FOLD) || (actions_hist.last().unwrap() == &ActionSpace::CHECK && a==ActionSpace::CHECK) {
                vec![ActionSpace::NONE]
            } else {
                vec![ActionSpace::END]
            }
        }
        
        
        for a in actions.clone() {
            let mut history = actions_history.clone();
            let next_actions = nextactions(a, &history);
            history.push(a);
            if next_actions != vec![ActionSpace::END] {
                children.insert(a, Node::new(
                    // &self,
                    is_chance,
                    mover,
                    next_actions,
                    history,
                    cards.clone(),
                ));
            } else {
                println!("{:?}", next_actions);
            }
        }
        
        Node {
            // parent: parent,
            is_chance: is_chance,
            to_move: to_move,
            cards: cards,
            actions_history: actions_history,
            actions: actions,
            children: children,
        }

    }

    // pub fn evaluation()

}


