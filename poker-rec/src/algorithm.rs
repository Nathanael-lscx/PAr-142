use std::collections::HashMap;
use crate::kuhn::*;

#[derive(Debug)]
pub struct CounterfactualRegretMinimization {
    // pub root: Node,
    pub regret_sum: HashMap<InformationSet, HashMap<ActionSpace, f32>>,
    pub strategy_sum: HashMap<InformationSet, HashMap<ActionSpace, f32>>,
    pub strategy: HashMap<InformationSet, HashMap<ActionSpace, f32>>,
    pub nash: HashMap<InformationSet, HashMap<ActionSpace, f32>>,
    pub chance_sampling: bool,
}

impl CounterfactualRegretMinimization {
    pub fn new(chance_sampling: bool) -> Self {
        //Initialisation des regrets et des stratégies
        let regret_sum = HashMap::new();
        let strategy_sum = HashMap::new();
        let strategy = HashMap::new();
        let nash = HashMap::new();

        CounterfactualRegretMinimization {
            // root: root,
            regret_sum: regret_sum,
            strategy_sum: strategy_sum,
            strategy: strategy,
            nash: nash,
            chance_sampling: chance_sampling,
        }
    }

    pub fn update_strategy (&mut self, inf_set: InformationSet) -> () {
        let mut sum = 0f32;
        for value in self.regret_sum[&inf_set].values() {
            if value > &0f32 {
                sum += value;
            }
        }
        
        for action in self.regret_sum[&inf_set].keys() {
            if sum == 0f32 {
                let prob = 1f32 / self.regret_sum[&inf_set].len() as f32;
                self.strategy.entry(inf_set.clone()).or_insert(HashMap::new()).insert(action.clone(), prob);
            }
            else {
                let sum_of_regret_for_action = self.regret_sum[&inf_set][action];
                let prob = if sum_of_regret_for_action > 0f32 {sum_of_regret_for_action / sum} else {0f32};
                self.strategy.entry(inf_set.clone()).or_insert(HashMap::new()).insert(action.clone(), prob);
            }
        }
        
    }

    pub fn update_strategy_sum(&mut self, inf_set: InformationSet, action: ActionSpace, prob: f32) -> () {
        *self.strategy_sum.entry(inf_set).or_insert(HashMap::new()).entry(action).or_insert(0f32) += prob;
    }

    pub fn update_regret_sum(&mut self, inf_set: InformationSet, action: ActionSpace, regret: f32) -> () {
        *self.regret_sum.entry(inf_set).or_insert(HashMap::new()).entry(action).or_insert(0f32) += regret;
    }

    pub fn compute_nash(&mut self) -> () {
        for (inf_set, strat_sum) in self.strategy_sum.iter() {
            let sum : f32 = strat_sum.values().sum();
            for (action, prob) in strat_sum.iter() {
                let nash_map = self.nash.entry(inf_set.clone()).or_insert(HashMap::new());
                *nash_map.entry(*action).or_insert(0f32) = prob / sum;
            }
        }
    }

    pub fn get_value_of_node(&self, node: &Node) -> Vec<f32> {
        if node.is_terminal() {
            let value = node.evaluation().iter().map(|x| *x as f32).collect();
            return value;
        }
        else {
            let mut value = vec![0f32, 0f32];
            for (a, child) in node.children.iter() {
                let prob = self.nash[&node.information_set()][a];
                let add_vec: Vec<f32> = self.get_value_of_node(child).iter().map(|x| x * prob).collect();
                value = value.iter().zip(add_vec.iter()).map(|(x, y)| x + y).collect();
            }
            return value;
        }
    }

    pub fn cfr_utility_rec(&mut self, node: &Node, reach_probs: Vec<f32>) -> Vec<f32> {
        // reach_probs is a vector containing the probability of player i to reach the game_state
        // under the asumption that he is trying to reach it. Say othe
        if node.is_terminal() {
            node.evaluation().iter().map(|x| *x as f32).collect()
        }
        else {
            let mut child_cfr_utilities = HashMap::new();
            let mut values = vec![0f32, 0f32];
            for action in node.actions.clone() {
                let mut new_reach_probs = reach_probs.clone();
                for (i, x) in new_reach_probs.iter_mut().enumerate() {
                    if i != node.to_move as usize {
                        *x *= *self.strategy.entry(node.information_set().clone()).or_insert(HashMap::new()).entry(action.clone()).or_insert(1f32 / node.actions.len() as f32);
                    }
                }

                let child_cfr_utility = self.cfr_utility_rec(&node.children[&action], new_reach_probs);
                values = values.iter().zip(child_cfr_utility.iter()).map(|(x, y)| x + y*self.strategy[&node.information_set()][&action]).collect();

                child_cfr_utilities.insert(action, child_cfr_utility);
            }

            for action in node.actions.clone() {
                let action_cfr_regret = reach_probs[node.to_move as usize] * (child_cfr_utilities[&action][node.to_move as usize] - values[node.to_move as usize]);
                self.update_regret_sum(node.information_set(), action.clone(), action_cfr_regret);
                self.update_strategy_sum(node.information_set(), action, self.strategy[&node.information_set()][&action])
            }

            self.update_strategy(node.information_set());
            return values;
        }
    }

    // pub fn run(&mut self, nb_iteration: i16) -> () {
    //     for _ in 0..nb_iteration {
    //         self.cfr_utility_rec(&self.root, vec![1f32, 1f32]);
    //     }
    // }

}