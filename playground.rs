use std::collections::HashMap;
use std::io;

fn main() {
        println!("   SELAMAT DATANG DI GUDANG KITA");
        
    let mut warehouse: HashMap<String, i32> = HashMap::new();

    loop {
        println!("Pilih operasi:");
        println!("1. Tambah barang");
        println!("2. Lihat stok barang");
        println!("3. Edit barang");
        println!("4. Hapus barang");
        println!("5. Keluar");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Gagal membaca baris");
        let choice: u32 = match choice.trim().parse() {
            Ok(num) => num,
            Err(_) => {
                println!("Masukan tidak valid. Masukkan nomor yang valid.");
                continue;
            }
        };

        match choice {
            1 => {
                println!("Masukkan nama barang:");
                let mut item_name = String::new();
                io::stdin().read_line(&mut item_name).expect("Gagal membaca baris");
                let item_name = item_name.trim().to_string();

                println!("Masukkan jumlah barang:");
                let mut quantity = String::new();
                io::stdin().read_line(&mut quantity).expect("Gagal membaca baris");
                let quantity: i32 = match quantity.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("Jumlah tidak valid. Masukkan angka yang valid.");
                        continue;
                    }
                };

                warehouse.entry(item_name.clone()).and_modify(|e| *e += quantity).or_insert(quantity);
                println!("Barang '{}' sejumlah {} berhasil ditambahkan.", item_name, quantity);
            }
            2 => {
                println!("Stok Barang:");
                let mut count = 1;
                for (item, quantity) in &warehouse {
                    println!("{}. {}: {}", count, item, quantity);
                    count += 1;
                }
            }
            3 => {
                println!("Masukkan nama barang yang akan diubah:");
                let mut item_name = String::new();
                io::stdin().read_line(&mut item_name).expect("Gagal membaca baris");
                let item_name = item_name.trim().to_string();

                if let Some(quantity) = warehouse.get_mut(&item_name) {
                    println!("Masukkan jumlah barang baru:");
                    let mut new_quantity = String::new();
                    io::stdin().read_line(&mut new_quantity).expect("Gagal membaca baris");
                    let new_quantity: i32 = match new_quantity.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Jumlah tidak valid. Masukkan angka yang valid.");
                            continue;
                        }
                    };
                    *quantity = new_quantity;
                    println!("Barang {} berhasil diubah.", item_name);
                } else {
                    println!("Barang tidak ditemukan.");
                }
            }
            4 => {
                println!("Masukkan nama barang yang akan dihapus:");
                let mut item_name = String::new();
                io::stdin().read_line(&mut item_name).expect("Gagal membaca baris");
                let item_name = item_name.trim().to_string();

                if let Some(_) = warehouse.remove(&item_name) {
                    println!("Barang {} berhasil dihapus.", item_name);
                } else {
                    println!("Barang tidak ditemukan.");
                }
            }
            5 => {
                println!("Terima kasih!");
                break;
            }
            _ => {
                println!("Pilihan tidak valid. Masukkan nomor yang valid.");
            }
        }
    }
}
