#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, Env, Address, Vec};

#[derive(Clone)]
#[contracttype]
pub struct Item {
    pub seller: Address,
    pub price: i128,
    pub sold: bool,
}

#[derive(Clone)]
#[contracttype]
pub enum DataKey {
    Items,
}

#[contract]
pub struct Marketplace;

#[contractimpl]
impl Marketplace {

    // List an item
    pub fn list_item(env: Env, seller: Address, price: i128) {
        seller.require_auth();

        let mut items: Vec<Item> = env
            .storage()
            .instance()
            .get(&DataKey::Items)
            .unwrap_or(Vec::new(&env));

        let item = Item {
            seller,
            price,
            sold: false,
        };

        items.push_back(item);
        env.storage().instance().set(&DataKey::Items, &items);
    }

    // Buy item
    pub fn buy_item(env: Env, buyer: Address, index: u32) {
        buyer.require_auth();

        let mut items: Vec<Item> = env
            .storage()
            .instance()
            .get(&DataKey::Items)
            .unwrap();

        let mut item = items.get(index).unwrap();

        if item.sold {
            panic!("Already sold");
        }

        item.sold = true;
        items.set(index, item);

        env.storage().instance().set(&DataKey::Items, &items);
    }

    // Get all items
    pub fn get_items(env: Env) -> Vec<Item> {
        env.storage()
            .instance()
            .get(&DataKey::Items)
            .unwrap_or(Vec::new(&env))
    }
}