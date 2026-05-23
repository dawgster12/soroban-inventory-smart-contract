#![no_std]
use soroban_sdk::{contract, contractimpl, contracttype, symbol_short, Env, String, Symbol, Vec};

#[contracttype]
#[derive(Clone, Debug)]
pub struct Item {
    id: u64,
    name: String,
    quantity: u32,
}

const INVENTORY_DATA: Symbol = symbol_short!("INVENTORY");

#[contract]
pub struct InventoryContract;

#[contractimpl]
impl InventoryContract {

    // 📦 Ambil semua barang
    pub fn get_items(env: Env) -> Vec<Item> {
        env.storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env))
    }

    // ➕ Tambah barang baru
    pub fn add_item(env: Env, name: String, quantity: u32) -> String {
        let mut items: Vec<Item> = env
            .storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        let item = Item {
            id: env.prng().gen::<u64>(),
            name,
            quantity,
        };

        items.push_back(item);

        env.storage()
            .instance()
            .set(&INVENTORY_DATA, &items);

        String::from_str(&env, "Barang berhasil ditambahkan")
    }

    // ❌ Hapus barang
    pub fn delete_item(env: Env, id: u64) -> String {
        let mut items: Vec<Item> = env
            .storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            if let Some(item) = items.get(i) {
                if item.id == id {
                    items.remove(i);

                    env.storage()
                        .instance()
                        .set(&INVENTORY_DATA, &items);

                    return String::from_str(&env, "Barang berhasil dihapus");
                }
            }
        }

        String::from_str(&env, "Barang tidak ditemukan")
    }

    // 🔼 Update stok barang
    pub fn update_stock(env: Env, id: u64, new_quantity: u32) -> String {
        let mut items: Vec<Item> = env
            .storage()
            .instance()
            .get(&INVENTORY_DATA)
            .unwrap_or(Vec::new(&env));

        for i in 0..items.len() {
            if let Some(mut item) = items.get(i) {
                if item.id == id {
                    item.quantity = new_quantity;
                    items.set(i, item);

                    env.storage()
                        .instance()
                        .set(&INVENTORY_DATA, &items);

                    return String::from_str(&env, "Stok berhasil diupdate");
                }
            }
        }

        String::from_str(&env, "Barang tidak ditemukan")
    }
}

mod test;