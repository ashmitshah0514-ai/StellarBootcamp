#![no_std]
use soroban_sdk::{contract, contractimpl, Env, Symbol, symbol_short, Address, Vec};

#[contract]
pub struct EarnQuests;

#[contractimpl]
impl EarnQuests {

    // Add a new quest (admin use)
    pub fn add_quest(env: Env, quest_id: Symbol, reward: i128) {
        env.storage().instance().set(&quest_id, &reward);
    }

    // Complete a quest and earn reward
    pub fn complete_quest(env: Env, user: Address, quest_id: Symbol) {
        let reward: i128 = env
            .storage()
            .instance()
            .get(&quest_id)
            .unwrap_or(0);

        let key = (user.clone(), symbol_short!("reward"));

        let current_reward: i128 = env
            .storage()
            .instance()
            .get(&key)
            .unwrap_or(0);

        env.storage().instance().set(&key, &(current_reward + reward));
    }

    // Check total rewards earned by user
    pub fn get_rewards(env: Env, user: Address) -> i128 {
        let key = (user, symbol_short!("reward"));

        env.storage().instance().get(&key).unwrap_or(0)
    }
}