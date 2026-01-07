use chrono::NaiveDate;
use rustyline::Editor;
use rustyline::history::DefaultHistory;

use crate::{Baris, Kolom, Tabel, TipeBaris, TipeKolom};
use std::fs::File;
use std::io::Write;

use std::env;
use std::path::PathBuf;

// ganti const DATA_FILE
fn default_data_file() -> PathBuf {
    let mut path = env::var("HOME")
        .map(PathBuf::from)
        .unwrap_or_else(|_| PathBuf::from("."));
    path.push(".mini_db.json"); // nama file tersembunyi di home
    path
}

fn save_to_file(tabel: &Tabel, path: &str) -> Result<(), String> {
    let json = serde_json::to_string_pretty(tabel).map_err(|e| e.to_string())?;
    let mut file = File::create(path).map_err(|e| e.to_string())?;
    file.write_all(json.as_bytes()).map_err(|e| e.to_string())?;
    Ok(())
}
use std::fs;

fn load_from_file(path: &str) -> Result<Tabel, String> {
    let data = fs::read_to_string(path).map_err(|e| e.to_string())?;
    let tabel: Tabel = serde_json::from_str(&data).map_err(|e| e.to_string())?;
    Ok(tabel)
}
fn opsi() {
    println!("\n=== MiniDB Menu ===");
    println!("1. Tambah Kolom");
    println!("2. Set Primary Key");
    println!("3. Tambah Baris");
    println!("4. Hapus Baris (kemunculan pertama)");
    println!("5. Hapus Baris (semua kemunculan)");
    println!("6. Hapus Kolom");
    println!("7. Update Nilai Baris");
    println!("8. Update Nama Kolom");
    println!("0. Keluar");
}
pub fn run() {
    let data_file = default_data_file();
    let data_file = &data_file.to_string_lossy();
    // Editor rustyline versi 17.x
    let mut rl: Editor<(), DefaultHistory> = Editor::new().unwrap();

    let mut tabel = match load_from_file(data_file) {
        Ok(t) => {
            // println!("Tabel berhasil diload dari '{}'", DATA_FILE);
            t
        }
        Err(_) => {
            // println!("Tidak ada file data, membuat tabel baru.");
            Tabel::new()
        }
    };

    loop {
        let input = rl.readline("Pilih menu: ").unwrap();
        match input.trim() {
            "menu" | "opsi" => opsi(),
            "0" => {
                save_to_file(&tabel, data_file).unwrap();
                println!("Keluar..");
                break;
            }
            "" => tabel.show(),
            "1" => {
                // Tambah kolom
                let nama = rl.readline("Nama kolom: ").unwrap();
                let tipe_str = rl
                    .readline("Tipe kolom (int/float/str/date/enum): ")
                    .unwrap();

                let tipe = match tipe_str.trim().to_lowercase().as_str() {
                    "int" => TipeKolom::Int,
                    "float" => TipeKolom::Float,
                    "str" => TipeKolom::Str,
                    "date" => TipeKolom::Date,
                    "enum" => {
                        let variants_input = rl
                            .readline("Masukkan varian enum, pisahkan dengan koma: ")
                            .unwrap();
                        let v: Vec<String> = variants_input
                            .split(',')
                            .map(|s| s.trim().to_string())
                            .collect();
                        TipeKolom::Enum { variant: v }
                    }
                    _ => {
                        println!("Tipe tidak valid!");
                        continue;
                    }
                };

                let kolom = Kolom::new(&nama, tipe);
                if let Err(e) = tabel.add_kolom(vec![kolom]) {
                    println!("Gagal menambah kolom: {}", e);
                } else {
                    println!("Kolom berhasil ditambah!");
                    save_to_file(&tabel, data_file).unwrap();
                }
            }
            "2" => {
                let pk = rl.readline("Nama kolom sebagai primary key: ").unwrap();
                match tabel.set_primary_key(&pk) {
                    Ok(_) => {
                        println!("Primary key berhasil diset!");
                        save_to_file(&tabel, data_file).unwrap();
                    }
                    Err(e) => println!("Gagal set primary key: {}", e),
                }
            }
            "3" => {
                // Tambah baris
                if tabel.kolom.is_empty() {
                    println!("Belum ada kolom, tambahkan kolom dulu!");
                    continue;
                }

                let mut nilai_baris = Vec::new();
                for kolom in &tabel.kolom {
                    let input = rl
                        .readline(&format!(
                            "Masukkan nilai untuk kolom '{}' ({:?}): ",
                            kolom.nama, kolom.tipe
                        ))
                        .unwrap();
                    let nilai = parse_input(&input, &kolom.tipe);
                    nilai_baris.push(nilai);
                }

                let baris = Baris::new(nilai_baris);
                if let Err(e) = tabel.add_baris(vec![baris]) {
                    println!("Gagal menambah baris: {}", e);
                } else {
                    println!("Baris berhasil ditambah!");
                    save_to_file(&tabel, data_file).unwrap();
                }
            }
            "4" => {
                let kolom = rl.readline("Nama kolom: ").unwrap();
                let nilai = rl.readline("Nilai baris: ").unwrap();
                let tipe_kolom = tabel.kolom.iter().find(|c| c.nama == kolom);
                if let Some(k) = tipe_kolom {
                    let baris_nilai = parse_input(&nilai, &k.tipe);
                    if let Err(e) = tabel.delete_baris(&kolom, baris_nilai) {
                        println!("Gagal hapus baris: {}", e);
                    }
                    save_to_file(&tabel, data_file).unwrap();
                } else {
                    println!("Kolom tidak ditemukan!");
                }
            }
            "5" => {
                let kolom = rl.readline("Nama kolom: ").unwrap();
                let nilai = rl.readline("Nilai baris: ").unwrap();
                let tipe_kolom = tabel.kolom.iter().find(|c| c.nama == kolom);
                if let Some(k) = tipe_kolom {
                    let baris_nilai = parse_input(&nilai, &k.tipe);
                    if let Err(e) = tabel.delete_kemunculan_baris(&kolom, baris_nilai) {
                        println!("Gagal hapus baris: {}", e);
                    }
                    save_to_file(&tabel, data_file).unwrap();
                } else {
                    println!("Kolom tidak ditemukan!");
                }
            }
            "6" => {
                let kolom_input = rl
                    .readline("Nama kolom yang akan dihapus, pisahkan koma: ")
                    .unwrap();
                let kolom_vec: Vec<&str> = kolom_input.split(',').map(|s| s.trim()).collect();
                if let Err(e) = tabel.delete_kolom(kolom_vec) {
                    println!("Gagal hapus kolom: {}", e);
                }
                save_to_file(&tabel, data_file).unwrap();
            }
            "7" => {
                if tabel.kolom.is_empty() {
                    println!("Belum ada kolom, tambahkan kolom dulu!");
                    continue;
                }

                let select_kolom = rl.readline("Nama kolom untuk mencari baris: ").unwrap();
                let select_kolom_ref = tabel.kolom.iter().find(|c| c.nama == select_kolom);

                if let Some(kol) = select_kolom_ref {
                    let select_nilai_input = rl
                        .readline("Nilai yang dicari di kolom tersebut: ")
                        .unwrap();
                    let select_nilai = parse_input(&select_nilai_input, &kol.tipe);

                    let target_kolom = rl.readline("Nama kolom yang akan diubah: ").unwrap();
                    let target_kol_ref = tabel.kolom.iter().find(|c| c.nama == target_kolom);

                    if let Some(target_kol) = target_kol_ref {
                        let new_nilai_input = rl.readline("Masukkan nilai baru: ").unwrap();
                        let new_nilai = parse_input(&new_nilai_input, &target_kol.tipe);

                        match tabel.update_nilai(
                            &select_kolom,
                            select_nilai,
                            &target_kolom,
                            new_nilai,
                        ) {
                            Ok(_) => println!("Nilai berhasil diupdate!"),
                            Err(e) => println!("Gagal update nilai: {}", e),
                        }
                    } else {
                        println!("Kolom target tidak ditemukan!");
                    }
                } else {
                    println!("Kolom pencarian tidak ditemukan!");
                }
                save_to_file(&tabel, data_file).unwrap();
            }
            "8" => {
                let select_kolom = rl.readline("select kolom: ").unwrap();
                let new_name = rl.readline("new_name: ").unwrap();

                match tabel.update_kolom_name(&select_kolom, &new_name) {
                    Ok(_) => println!("Berhasil update nama kolom."),
                    Err(e) => println!("Gagal update nama kolom. {}", e),
                }
            }
            _ => println!("Pilihan tidak valid!"),
        }
    }
}

fn parse_input(input: &str, tipe: &TipeKolom) -> TipeBaris {
    match tipe {
        TipeKolom::Int => input
            .parse::<i64>()
            .map(TipeBaris::Int)
            .unwrap_or(TipeBaris::Null),
        TipeKolom::Float => input
            .parse::<f64>()
            .map(TipeBaris::Float)
            .unwrap_or(TipeBaris::Null),
        TipeKolom::Str => {
            let trimmed = input.trim();
            if trimmed.is_empty() {
                TipeBaris::Null
            } else {
                TipeBaris::Str(trimmed.to_string())
            }
        }
        TipeKolom::Date => NaiveDate::parse_from_str(input, "%Y-%m-%d")
            .map(TipeBaris::Date)
            .unwrap_or(TipeBaris::Null),
        TipeKolom::Enum { variant } => {
            if variant.contains(&input.to_string()) {
                TipeBaris::Enum {
                    variant: input.to_string(),
                }
            } else {
                println!("Nilai enum tidak valid, di-set Null.");
                TipeBaris::Null
            }
        }
    }
}
