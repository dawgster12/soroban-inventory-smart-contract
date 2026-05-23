# 📦 Inventory Smart Contract (Soroban - Stellar Blockchain)

Smart Contract ini adalah aplikasi Inventory Management terdesentralisasi (dApp) yang dibuat menggunakan Rust + Soroban SDK dan berjalan di Stellar Blockchain (Testnet).

Project ini menggantikan sistem database tradisional dengan blockchain, sehingga data lebih aman, transparan, dan tidak bisa dimodifikasi sembarangan.

---

# 🚀 Fitur Aplikasi

- ➕ Menambahkan barang ke inventory
- 📦 Melihat semua barang
- ❌ Menghapus barang berdasarkan ID
- 🔼 Mengupdate stok barang

---

# 🧱 Struktur Data

Setiap item inventory disimpan dalam struktur berikut:

pub struct Item {
    id: u64,
    name: String,
    quantity: u32,
}

Penjelasan:
- id = identitas unik barang
- name = nama barang
- quantity = jumlah stok barang

---

# ⚙️ Smart Contract Functions

1. add_item(env: Env, name: String, quantity: u32)
   → Menambahkan barang baru ke inventory

2. get_items(env: Env) -> Vec<Item>
   → Mengambil semua data barang

3. delete_item(env: Env, id: u64)
   → Menghapus barang berdasarkan ID

4. update_stock(env: Env, id: u64, new_quantity: u32)
   → Mengubah jumlah stok barang

---

# 🛠️ Teknologi yang Digunakan

- Rust (no_std)
- Soroban SDK
- Stellar Blockchain
- WebAssembly (WASM)
- Testnet Network

---

# ⚙️ Cara Build Project

cargo build --target wasm32-unknown-unknown --release

Hasil build akan berada di:
target/wasm32-unknown-unknown/release/

---

# 🌐 Setup Network Testnet

soroban network add testnet \
--rpc-url https://soroban-testnet.stellar.org \
--network-passphrase "Test SDF Network ; September 2015"

---

# 👛 Generate Wallet

soroban keys generate alice

Funding wallet:
soroban keys fund alice --network testnet

---

# 🚀 Deploy Smart Contract

soroban contract deploy \
--wasm target/wasm32-unknown-unknown/release/inventory_contract.wasm \
--source alice \
--network testnet

Output:
CONTRACT_ID: CAxxxxxxxxxxxxxxxx

---

# 🧪 Cara Menjalankan (Invoke)

# ➕ Add Item
soroban contract invoke \
--id CONTRACT_ID \
--source alice \
--network testnet \
-- \
add_item \
--name "Keyboard" \
--quantity 10

---

# 📦 Get Items
soroban contract invoke \
--id CONTRACT_ID \
--source alice \
--network testnet \
-- \
get_items

---

# ❌ Delete Item
soroban contract invoke \
--id CONTRACT_ID \
--source alice \
--network testnet \
-- \
delete_item \
--id 123

---

# 🔼 Update Stock
soroban contract invoke \
--id CONTRACT_ID \
--source alice \
--network testnet \
-- \
update_stock \
--id 123 \
--new_quantity 50

---

# 📌 Info Project

Network: Stellar Testnet  
Framework: Soroban SDK  
Language: Rust  
Type: Smart Contract  
Storage: On-chain blockchain storage  

---

# 🎯 Tujuan Project

Project ini dibuat untuk:
- Belajar Smart Contract Development
- CRUD di Blockchain
- Memahami Soroban SDK
- Membuat dApp Inventory sederhana

---

# 👨‍💻 Author

Built with Rust + Soroban + Stellar Blockchain 🚀