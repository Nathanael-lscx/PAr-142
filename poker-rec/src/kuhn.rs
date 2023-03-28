/*
    Fichier implémentant le kuhn poker récursif
*/

use crate::decking::{Card, Deck, Decks};
use std::collections::HashMap;
use strum::IntoEnumIterator;
// use strum_macros::EnumIter;

#[derive(PartialEq, Hash, Clone, Copy, Debug)]
pub enum ActionSpace {
    BET,
    CHECK,
    CALL,
    FOLD,
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
    pub to_move: u8,
    pub cards: Deck,

    pub actions_history: Vec<ActionSpace>,
    pub actions: Vec<ActionSpace>,
    pub children: HashMap<ActionSpace, Node>,
}

impl Node {
    pub fn new(
        to_move: u8,
        actions: Vec<ActionSpace>,
        actions_history: Vec<ActionSpace>,
        cards: Deck,
    ) -> Self {
        let mut children = HashMap::new();
        let mover: u8;

        mover = (to_move + 1) % 2;

        fn nextactions(a: ActionSpace, actions_hist: &Vec<ActionSpace>) -> Vec<ActionSpace> {
            if (actions_hist.len() == 0) && a == ActionSpace::BET {
                vec![ActionSpace::FOLD, ActionSpace::CALL]
            } else if (actions_hist.len() == 0) && a == ActionSpace::CHECK {
                vec![ActionSpace::CHECK, ActionSpace::BET]
            } else if (actions_hist.last().unwrap() == &ActionSpace::CHECK) && a == ActionSpace::BET
            {
                vec![ActionSpace::FOLD, ActionSpace::CALL]
            } else {
                Vec::new()
            }
        }

        for a in actions.clone() {
            let mut history = actions_history.clone();
            let next_actions = nextactions(a, &history);
            history.push(a);
            children.insert(
                a,
                Node::new(
                    // &self,
                    mover,
                    next_actions,
                    history,
                    cards.clone(),
                ),
            );
        }

        Node {
            // parent: parent,
            // is_chance: is_chance,
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
            if self.actions_history.last().unwrap() == &ActionSpace::FOLD {
                if self.to_move == 0 {
                    vec![1, -1]
                } else {
                    vec![-1, 1]
                }
            } else if self.actions_history.last().unwrap() == &ActionSpace::CHECK {
                if self.cards.d[0].val > self.cards.d[1].val {
                    vec![1, -1]
                } else {
                    vec![-1, 1]
                }
            } else if self.actions_history.last().unwrap() == &ActionSpace::CALL {
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

pub struct Root {
    children: HashMap<Decks, Node>,
}

impl Root {
    pub fn new() -> Root {
        let mut mychildren = HashMap::new();

        for mydeck in Decks::iter() {
            let mut mycards = Deck::new();
            mycards.hardcode(&mydeck);
            mychildren.insert(mydeck, Node::new(0, Vec::new(), Vec::new(), mycards));
        }

        Root {
            children: mychildren,
        }
    }
}
