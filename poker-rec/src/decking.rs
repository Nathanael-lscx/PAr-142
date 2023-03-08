/*     
    Cette bibliothèque crée la structure du jeu de carte (Deck), elle implémente des fonctions
    permettant d'afficher un jeu et de créer des mains aléatoires.
*/    
use rand::{thread_rng, seq::SliceRandom};


// Création de la classe du jeu


// Création de la structure du jeu de carte
#[derive(Clone, Debug)]
pub struct Deck{
    pub d: Vec<Card>,
}

// Implémentation du jeu de carte dans la structure
impl Deck{
    // Lors de la création de la structure on crée un jeu de cartes mélangé
    pub fn new() -> Deck{
        // La liste vide
        let mut cardset:Vec<Card> = Vec::new();

        // On itère dans les numéros puis dans les couleurs
        for v in 1..4{
            cardset.push(Card{
                val: v,
            })
        }
    
        // On mélange
        cardset.shuffle(&mut thread_rng());

        // On renvoie la structure
        Deck{
            d: cardset,
        }
    }

    // Pour sortir une carte, on sort la première
    pub fn get_card(&mut self) -> Card{
        self.d.pop().unwrap()
    }
}


// Structure de la carte
#[derive(Copy, Clone, Debug)]
pub struct Card{
    pub val: u8,
}


// Création de la carte et de ses fonctions d'affichage
impl Card{
    pub fn format_card_str(&self) -> String{
        // format!("{{'Value': '{:?}'}}", self.val)
        format!("Val : {:?}", self.val)
    }
}
