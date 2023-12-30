use std::collections::{HashMap, VecDeque};

struct Barang {
    id: usize,
    nama: String,
    stok: i32,
}

struct DataBarang {
    data: HashMap<usize, Barang>,
    stack: Vec<usize>,
    queue: VecDeque<usize>,
}

impl DataBarang {
    fn tambah_barang(&mut self, id: usize, nama: String, stok: i32) {
        let barang = Barang { id, nama, stok };
        self.data.insert(id, barang);
        self.stack.push(id);
        self.queue.push_back(id);
    }

    fn lihat_stok_stack(&self) {
        println!("Stok barang (urutan stack):");
        for &id in self.stack.iter().rev() {
            if let Some(barang) = self.data.get(&id) {
                println!("ID: {}, Nama: {}, Stok: {}", barang.id, barang.nama, barang.stok);
            }
        }
    }

    fn lihat_stok_queue(&self) {
        println!("Stok barang (urutan queue):");
        for &id in self.queue.iter() {
            if let Some(barang) = self.data.get(&id) {
                println!("ID: {}, Nama: {}, Stok: {}", barang.id, barang.nama, barang.stok);
            }
        }
    }

    fn edit_barang(&mut self, id: usize, nama: String, stok: i32) {
        if let Some(barang) = self.data.get_mut(&id) {
            barang.nama = nama;
            barang.stok = stok;
        }
    }

    fn hapus_barang(&mut self, id: usize) {
        if let Some(barang) = self.data.remove(&id) {
            if let Some(index) = self.stack.iter().position(|&x| x == id) {
                self.stack.remove(index);
            }
            if let Some(index) = self.queue.iter().position(|&x| x == id) {
                self.queue.remove(index);
            }
            println!("Barang dengan ID {} telah dihapus.", barang.id);
        } else {
            println!("Barang dengan ID {} tidak ditemukan.", id);
        }
    }
}

fn main() {
        println!("   SELAMAT DATANG DI GUDANG KITA");
    let mut data_barang = DataBarang {
        data: HashMap::new(),
        stack: Vec::new(),
        queue: VecDeque::new(),
    };

    loop {
        println!("Pilih operasi:");
        println!("1. Tambah barang");
        println!("2. Lihat stok barang (urutan stack)");
        println!("3. Lihat stok barang (urutan queue)");
        println!("4. Edit barang");
        println!("5. Hapus barang");
        println!("6. Keluar");

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Gagal membaca baris");

        match input.trim().parse() {
            Ok(1) => {
                println!("Masukkan ID barang:");
                let mut id_input = String::new();
                std::io::stdin().read_line(&mut id_input).expect("Gagal membaca baris");
                let id: usize = id_input.trim().parse().expect("ID harus berupa angka");

                println!("Masukkan nama barang:");
                let mut nama_input = String::new();
                std::io::stdin().read_line(&mut nama_input).expect("Gagal membaca baris");
                let nama = nama_input.trim().to_string();

                println!("Masukkan stok barang:");
                let mut stok_input = String::new();
                std::io::stdin().read_line(&mut stok_input).expect("Gagal membaca baris");
                let stok: i32 = stok_input.trim().parse().expect("Stok harus berupa angka");

                data_barang.tambah_barang(id, nama, stok);
            }
            Ok(2) => {
                data_barang.lihat_stok_stack();
            }
            Ok(3) => {
                data_barang.lihat_stok_queue();
            }
            Ok(4) => {
                println!("Masukkan ID barang yang akan diedit:");
                let mut id_input = String::new();
                std::io::stdin().read_line(&mut id_input).expect("Gagal membaca baris");
                let id: usize = id_input.trim().parse().expect("ID harus berupa angka");

                println!("Masukkan nama baru untuk barang:");
                let mut nama_input = String::new();
                std::io::stdin().read_line(&mut nama_input).expect("Gagal membaca baris");
                let nama = nama_input.trim().to_string();

                println!("Masukkan stok baru untuk barang:");
                let mut stok_input = String::new();
                std::io::stdin().read_line(&mut stok_input).expect("Gagal membaca baris");
                let stok: i32 = stok_input.trim().parse().expect("Stok harus berupa angka");

                data_barang.edit_barang(id, nama, stok);
            }
            Ok(5) => {
                println!("Masukkan ID barang yang akan dihapus:");
                let mut id_input = String::new();
                std::io::stdin().read_line(&mut id_input).expect("Gagal membaca baris");
                let id: usize = id_input.trim().parse().expect("ID harus berupa angka");

                data_barang.hapus_barang(id);
            }
            Ok(6) => {
                println!("Keluar dari aplikasi.");
                break;
            }
            _ => {
                println!("Pilihan tidak valid, coba lagi.");
            }
        }
    }
}
