/*
    Cette bibliothèque crée la structure du jeu de carte (Deck), elle implémente des fonctions
    permettant d'afficher un jeu et de créer des mains aléatoires.
*/
use rand::{seq::SliceRandom, thread_rng};
// use strum::IntoEnumIterator;
use strum_macros::EnumIter;

// Création de la structure du jeu de carte
#[derive(Clone, Debug, PartialEq, Eq)]
pub struct Deck {
    pub d: Vec<Card>,
}

// Implémentation du jeu de carte dans la structure
impl Deck {
    // Lors de la création de la structure on crée un jeu de cartes mélangé
    pub fn new() -> Deck {
        // La liste vide
        let mut cardset: Vec<Card> = Vec::new();

        // On itère dans les numéros puis dans les couleurs
        for v in 1..4 {
            cardset.push(Card { val: v })
        }

        // On mélange
        cardset.shuffle(&mut thread_rng());

        // On renvoie la structure
        Deck { d: cardset }
    }

    // Pour sortir une carte, on sort la première
    pub fn get_card(&mut self) -> Card {
        self.d.pop().unwrap()
    }

    pub fn hardcode(&mut self, conf: &Decks) {
        let cardset: Vec<Card>;

        match conf {
            &Decks::QK => {
                cardset = vec![Card { val: 2 }, Card { val: 3 }, Card { val: 1 }];
            }
            &Decks::JK => {
                cardset = vec![Card { val: 1 }, Card { val: 3 }, Card { val: 2 }];
            }
            &Decks::JQ => {
                cardset = vec![Card { val: 1 }, Card { val: 2 }, Card { val: 3 }];
            }
            &Decks::KQ => {
                cardset = vec![Card { val: 3 }, Card { val: 2 }, Card { val: 1 }];
            }
            &Decks::KJ => {
                cardset = vec![Card { val: 3 }, Card { val: 1 }, Card { val: 2 }];
            }
            &Decks::QJ => {
                cardset = vec![Card { val: 2 }, Card { val: 1 }, Card { val: 3 }];
            }
        }

        self.d = cardset;
    }
}

// Structure de la carte
#[derive(Copy, Clone, Debug, PartialEq, Hash)]
pub struct Card {
    pub val: u8,
}
impl Eq for Card {}

// Création de la carte et de ses fonctions d'affichage
impl Card {
    pub fn format_card_str(&self) -> String {
        // format!("{{'Value': '{:?}'}}", self.val)
        format!("Val : {:?}", self.val)
    }
}

#[derive(EnumIter, Hash, PartialEq, Eq)]
pub enum Decks {
    QK,
    JK,
    JQ,
    KQ,
    KJ,
    QJ,
}
