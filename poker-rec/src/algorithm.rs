use crate::kuhn::*;
use rand::seq::SliceRandom;
use std::collections::HashMap;

#[derive(Debug)]
pub struct CounterfactualRegretMinimization {
    pub regret_sum: HashMap<InformationSet, HashMap<ActionSpace, f64>>,
    pub strategy_sum: HashMap<InformationSet, HashMap<ActionSpace, f64>>,
    pub strategy: HashMap<InformationSet, HashMap<ActionSpace, f64>>,
    pub nash: HashMap<InformationSet, HashMap<ActionSpace, f64>>,
    // counter: usize,
}

impl CounterfactualRegretMinimization {
    pub fn new() -> Self {
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
            // counter: 0usize,
        }
    }

    pub fn update_strategy(&mut self, inf_set: InformationSet) -> () {
        let mut sum = 0f64;
        for value in self.regret_sum[&inf_set].values() {
            if value > &0f64 {
                sum += value;
            }
        }

        for action in self.regret_sum[&inf_set].keys() {
            if sum == 0f64 {
                let prob = 1f64 / self.regret_sum[&inf_set].len() as f64;
                self.strategy
                    .entry(inf_set.clone())
                    .or_insert(HashMap::new())
                    .insert(action.clone(), prob);
            } else {
                let sum_of_regret_for_action = self.regret_sum[&inf_set][action];
                let prob = if sum_of_regret_for_action > 0f64 {
                    sum_of_regret_for_action / sum
                } else {
                    0f64
                };
                self.strategy
                    .entry(inf_set.clone())
                    .or_insert(HashMap::new())
                    .insert(action.clone(), prob);
            }
        }
    }

    pub fn update_strategy_sum(
        &mut self,
        inf_set: InformationSet,
        action: ActionSpace,
        prob: f64,
    ) -> () {
        *self
            .strategy_sum
            .entry(inf_set)
            .or_insert(HashMap::new())
            .entry(action)
            .or_insert(0f64) += prob;
    }

    pub fn update_regret_sum(
        &mut self,
        inf_set: InformationSet,
        action: ActionSpace,
        regret: f64,
    ) -> () {
        *self
            .regret_sum
            .entry(inf_set)
            .or_insert(HashMap::new())
            .entry(action)
            .or_insert(0f64) += regret;
    }

    pub fn compute_nash(&mut self) -> () {
        for (inf_set, strat_sum) in self.strategy_sum.iter() {
            let sum: f64 = strat_sum.values().sum();
            let nash_map = self.nash.entry(inf_set.clone()).or_insert(HashMap::new());
            for (action, prob) in strat_sum.iter() {
                *nash_map.entry(*action).or_insert(0f64) = prob / sum;
            }
        }
    }

    pub fn get_value_of_root(&self, root: &Root) -> Vec<f64> {
        let mut value = vec![0f64, 0f64];
        let nb_children = root.children.len();
        for child in root.children.iter() {
            let add_vec = self.get_value_of_node(child);
            value = value
                .iter()
                .zip(add_vec.iter())
                .map(|(x, y)| x + y / nb_children as f64)
                .collect();
        }
        return value;
    }

    pub fn get_value_of_node(&self, node: &Node) -> Vec<f64> {
        if node.is_terminal() {
            let value = node.evaluation().iter().map(|x| *x as f64).collect();
            return value;
        } else {
            let mut value = vec![0f64, 0f64];
            for (a, child) in node.children.iter() {
                let prob = self.nash[&node.information_set()][a];
                let add_vec: Vec<f64> = self
                    .get_value_of_node(child)
                    .iter()
                    .map(|x| x * prob)
                    .collect();
                value = value
                    .iter()
                    .zip(add_vec.iter())
                    .map(|(x, y)| x + y)
                    .collect();
            }
            return value;
        }
    }

    pub fn cfr_utility(&mut self, root: &Root) {
        // Choose a random child from the vector of children
        let node = root.children.choose(&mut rand::thread_rng()).unwrap();
        // let node = &root.children[self.counter];
        // self.counter = (self.counter + 1) % root.children.len();
        self.cfr_utility_rec(node, vec![1f64, 1f64]);
    }

    pub fn cfr_utility_rec(&mut self, node: &Node, reach_probs: Vec<f64>) -> Vec<f64> {
        // reach_probs is a vector containing the probability of player i to reach the game_state
        // under the asumption that he is trying to reach it. Say othe
        if node.is_terminal() {
            node.evaluation().iter().map(|x| *x as f64).collect()
        } else {
            let mut child_cfr_utilities = HashMap::new();
            let mut values = vec![0f64, 0f64];
            for action in node.actions.clone() {
                let mut new_reach_probs = reach_probs.clone();
                for (i, x) in new_reach_probs.iter_mut().enumerate() {
                    if i != node.to_move as usize {
                        *x *= *self
                            .strategy
                            .entry(node.information_set().clone())
                            .or_insert(HashMap::new())
                            .entry(action.clone())
                            .or_insert(1f64 / node.actions.len() as f64);
                    }
                }

                let child_cfr_utility =
                    self.cfr_utility_rec(&node.children[&action], new_reach_probs);
                values = values
                    .iter()
                    .zip(child_cfr_utility.iter())
                    .map(|(x, y)| x + y * self.strategy[&node.information_set()][&action])
                    .collect();

                child_cfr_utilities.insert(action, child_cfr_utility);
            }

            for action in node.actions.clone() {
                let action_cfr_regret = reach_probs[node.to_move as usize]
                    * (child_cfr_utilities[&action][node.to_move as usize]
                        - values[node.to_move as usize]);
                self.update_regret_sum(node.information_set(), action.clone(), action_cfr_regret);
                self.update_strategy_sum(
                    node.information_set(),
                    action,
                    self.strategy[&node.information_set()][&action],
                )
            }

            self.update_strategy(node.information_set());
            return values;
        }
    }
}
