use std::collections::HashMap;
use std::io;

struct Item {
    id: u32,
    name: String,
    quantity: i32,
}

fn display_stock_as_stack(warehouse: &HashMap<u32, Item>) {
    let mut stack: Vec<(&u32, &Item)> = warehouse.iter().collect();
    stack.reverse();

    println!("Stok Barang (Stack):");
    let mut count = 1;
    for (id, item) in stack.iter() {
        println!("{}. ID: {}, Nama: {}, Kuantitas: {}", count, id, item.name, item.quantity);
        count += 1;
    }
}

fn display_stock_as_queue(warehouse: &HashMap<u32, Item>) {
    let queue: Vec<(&u32, &Item)> = warehouse.iter().collect();

    println!("Stok Barang (Queue):");
    let mut count = 1;
    for (id, item) in queue.iter() {
        println!("{}. ID: {}, Nama: {}, Kuantitas: {}", count, id, item.name, item.quantity);
        count += 1;
    }
}

fn main() {
    println!("   SELAMAT DATANG DI GUDANG KITA");

    let mut warehouse: HashMap<u32, Item> = HashMap::new();
    let mut current_id: u32 = 1; // ID awal untuk barang yang akan ditambahkan

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

                let new_item = Item {
                    id: current_id,
                    name: item_name.clone(),
                    quantity,
                };

                warehouse.insert(current_id, new_item);
                println!("Barang '{}' dengan ID {} sejumlah {} berhasil ditambahkan.", item_name, current_id, quantity);
                current_id += 1; // Increment ID untuk barang selanjutnya
            }
            2 => {
                println!("Stok Barang:");
                display_stock_as_stack(&warehouse);
                display_stock_as_queue(&warehouse);
            }
            3 => {
                println!("Masukkan ID barang yang akan diubah:");
                let mut input_id = String::new();
                io::stdin().read_line(&mut input_id).expect("Gagal membaca baris");
                let input_id: u32 = match input_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("ID tidak valid. Masukkan ID yang valid.");
                        continue;
                    }
                };

                if let Some(item) = warehouse.get_mut(&input_id) {
                    println!("Masukkan jumlah barang baru untuk '{}':", item.name);
                    let mut new_quantity = String::new();
                    io::stdin().read_line(&mut new_quantity).expect("Gagal membaca baris");
                    let new_quantity: i32 = match new_quantity.trim().parse() {
                        Ok(num) => num,
                        Err(_) => {
                            println!("Jumlah tidak valid. Masukkan angka yang valid.");
                            continue;
                        }
                    };
                    item.quantity = new_quantity;
                    println!("Barang '{}' berhasil diubah.", item.name);
                } else {
                    println!("Barang dengan ID {} tidak ditemukan.", input_id);
                }
            }
            4 => {
                println!("Masukkan ID barang yang akan dihapus:");
                let mut input_id = String::new();
                io::stdin().read_line(&mut input_id).expect("Gagal membaca baris");
                let input_id: u32 = match input_id.trim().parse() {
                    Ok(num) => num,
                    Err(_) => {
                        println!("ID tidak valid. Masukkan ID yang valid.");
                        continue;
                    }
                };

                if let Some(item) = warehouse.remove(&input_id) {
                    println!("Barang '{}' dengan ID {} berhasil dihapus.", item.name, input_id);
                } else {
                    println!("Barang dengan ID {} tidak ditemukan.", input_id);
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