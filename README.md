# 📚 Soroban Books Management System
[![Network: Testnet](https://img.shields.io/badge/Network-Testnet-blue)](https://stellar.expert/explorer/testnet)
[![SDK: Soroban](https://img.shields.io/badge/SDK-Soroban-black)](https://soroban.stellar.org)
[![Language: Rust](https://img.shields.io/badge/Language-Rust-orange)](https://www.rust-lang.org)

Smart Contract berbasis Rust yang dideploy di jaringan **Stellar Soroban**. Kontrak ini memungkinkan manajemen data buku (CRUD) secara terdesentralisasi dengan penyimpanan data langsung di blockchain (on-chain).

---

## 🖼️ Tampilan Hasil Run
*(Pastikan file gambar sudah kamu upload ke folder project dengan nama 'screenshot.png')*

<p align="center">
  <img src="screenshot1.png" alt="Preview Terminal Soroban" width="700" style="border-radius: 10px; border: 1px solid #ddd;">
  <img src="screenshot2.png" alt="Preview Terminal Soroban" width="700" style="border-radius: 10px; border: 1px solid #ddd;">
</p>

---

## 📝 Identitas Kontrak
| Properti | Detail |
| :--- | :--- |
| **Contract ID** | `CAOQIJITAXNJUEJ6SERB5M6FC35BTH5ATTJUZQPKT2AMWQGHB2VV5KAH` |
| **Source Account** | `salsabillanajwa` |
| **Network** | Testnet |
| **WASM Hash** | `f12bdb5bb97706d8d55c1c2cbf4ebf9b4c29600e4569f67c1f57b81a65a7c142` |

---

## ⚙️ Fitur & Fungsi
Kontrak ini menggunakan fitur `env.storage().instance()` untuk penyimpanan data permanen.

| Fungsi | Deskripsi | Parameter |
| :--- | :--- | :--- |
| `create_book` | Menambah buku baru | `title`, `category`, `isbn` |
| `get_books` | Mengambil semua buku | `id`  |
| `get_book_by_id` | Cari buku secara spesifik | `id` |
| `update_book` | Edit info buku | `id`, `new_title`, `new_category`, `new_isbn` |
| `delete_book` | Hapus buku dari storage | `id` |

---
