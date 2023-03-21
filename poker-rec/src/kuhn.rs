/*
    Fichier implémentant le kuhn poker récursif
*/

use std::collections::HashMap;
use crate::decking::Deck;
use crate::decking::Card;

#[derive(PartialEq, Hash, Clone, Copy, Debug)]
pub enum ActionSpace {
    BET,
    CHECK,
    CALL,
    FOLD,
    // NONE,
    END,
}
impl Eq for ActionSpace {}

#[derive(PartialEq, Eq, Hash, Debug, Clone)]
pub struct InformationSet {
    card: Card,
    actions_history: Vec<ActionSpace>,
}

#[derive(Debug)]
pub struct Node {
    // pub parent: Option<Box<Node>>,
    //is_chance: bool,
    pub to_move: u8,
    pub cards: Deck,

    pub actions_history: Vec<ActionSpace>,
    pub actions: Vec<ActionSpace>,
    pub children: HashMap<ActionSpace, Node>,
}

impl Node {
    pub fn new(is_chance: bool, to_move: u8, actions: Vec<ActionSpace>, actions_history: Vec<ActionSpace>, cards: Deck) -> Self {
        let mut children = HashMap::new();
        let mover: u8;

        mover = (to_move+1) % 2;
        
        
        fn nextactions(a: ActionSpace, actions_hist: &Vec<ActionSpace>) -> Vec<ActionSpace> {
            if (actions_hist.len() == 0) && a == ActionSpace::BET {
                vec![ActionSpace::FOLD, ActionSpace::CALL]
            } else if (actions_hist.len() == 0) && a == ActionSpace::CHECK {
                vec![ActionSpace::CHECK, ActionSpace::BET]
            } else if (actions_hist.last().unwrap() == &ActionSpace::CHECK) && a == ActionSpace::BET {
                vec![ActionSpace::FOLD, ActionSpace::CALL]
            } else {
                Vec::new()
            } 
        }
        
        
        for a in actions.clone() {
            let mut history = actions_history.clone();
            let next_actions = nextactions(a, &history);
            history.push(a);
            children.insert(a, Node::new(
                // &self,
                is_chance,
                mover,
                next_actions,
                history,
                cards.clone(),
            ));
        }
        
        Node {
            // parent: parent,
            //is_chance: is_chance,
            to_move: to_move,
            cards: cards,
            actions_history: actions_history,
            actions: actions,
            children: children,
        }
    }

    pub fn is_terminal(&self) -> bool {
        self.actions.is_empty()
    }

    pub fn information_set(&self) -> InformationSet {
        let card: Card;
        if self.to_move == 0 {
            card = self.cards.d[0];
        } else {
            card = self.cards.d[1];
        }
        InformationSet {
            card: card,
            actions_history: self.actions_history.clone(),
        }
    }
        
    pub fn evaluation(&self) -> Vec<i16> {
        if self.actions.is_empty() {
            if self.actions_history.last().unwrap()==&ActionSpace::FOLD {
                if self.to_move == 0 {
                    vec![1, -1]
                } else {
                    vec![-1, 1]
                }
            } else if self.actions_history.last().unwrap()==&ActionSpace::CHECK {
                if self.cards.d[0].val > self.cards.d[1].val {
                    vec![1, -1]
                } else {
                    vec![-1, 1]
                }
            } else if self.actions_history.last().unwrap()==&ActionSpace::CALL {
                if self.cards.d[0].val > self.cards.d[1].val {
                    vec![2, -2]
                } else {
                    vec![-2, 2]
                }
            } else {
                Vec::new()
            }
        } else {
            Vec::new()
        }
    }
}
        


