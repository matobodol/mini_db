use rustyline::Editor;
use rustyline::history::DefaultHistory;

use crate::{Baris, Kolom, Tabel, TipeBaris, parse_to_tipe_baris, parse_to_tipe_kolom};
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
    println!("9. Update Null ");
    println!("0. Keluar\n");
}
pub fn run() {
    let data_file = default_data_file();
    let data_file = &data_file.to_string_lossy();
    // Editor rustyline versi 17.x
    let mut rl: Editor<(), DefaultHistory> = Editor::new().unwrap();

    let mut tabel = match load_from_file(data_file) {
        Ok(file_is_loaded) => file_is_loaded,
        Err(_) => Tabel::new(),
    };

    loop {
        let input = rl.readline("mini_db >> ").unwrap();
        match input.trim() {
            "" => tabel.show(),
            "reset" => tabel.reset_baris(),
            "hard-reset" => tabel.reset_tabel(),
            "m" | "menu" | "opsi" => opsi(),
            "0" => {
                save_to_file(&tabel, data_file).unwrap();
                println!("Keluar..");
                break;
            }
            "1" => {
                // Tambah kolom
                let nama = rl.readline("Nama kolom: ").unwrap();
                let tipe_str = rl
                    .readline("Tipe kolom (int/float/str/date/enum): ")
                    .unwrap();

                let tipe = match parse_to_tipe_kolom(&tipe_str) {
                    Ok(v) => v,
                    Err(e) => {
                        println!("{}", e);
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
                    let nilai = match parse_to_tipe_baris(&input, &kolom.tipe) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            TipeBaris::Null
                        }
                    };
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
                    let baris_nilai = match parse_to_tipe_baris(&nilai, &k.tipe) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            TipeBaris::Null
                        }
                    };
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
                    let baris_nilai = match parse_to_tipe_baris(&nilai, &k.tipe) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            TipeBaris::Null
                        }
                    };
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
                    let select_nilai = match parse_to_tipe_baris(&select_nilai_input, &kol.tipe) {
                        Ok(v) => v,
                        Err(e) => {
                            println!("{}", e);
                            TipeBaris::Null
                        }
                    };

                    let target_kolom = rl.readline("Nama kolom yang akan diubah: ").unwrap();
                    let target_kol_ref = tabel.kolom.iter().find(|c| c.nama == target_kolom);

                    if let Some(target_kol) = target_kol_ref {
                        let new_nilai_input = rl.readline("Masukkan nilai baru: ").unwrap();
                        let new_nilai =
                            match parse_to_tipe_baris(&new_nilai_input, &target_kol.tipe) {
                                Ok(v) => v,
                                Err(e) => {
                                    println!("{}", e);
                                    TipeBaris::Null
                                }
                            };

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
            "9" => {
                for baris in tabel.baris.iter_mut() {
                    for (i, tipe) in baris.tipe.iter_mut().enumerate() {
                        if tipe != &TipeBaris::Null {
                            continue;
                        }

                        let kolom = &tabel.kolom[i];

                        let prompt = format!("{}: ({:?}): ", kolom.nama, kolom.tipe);
                        let input = rl.readline(&prompt).unwrap();

                        let nilai = match parse_to_tipe_baris(&input, &kolom.tipe) {
                            Ok(v) => v,
                            Err(e) => {
                                println!("{}", e);
                                TipeBaris::Null
                            }
                        };

                        *tipe = nilai;
                    }
                }
            }
            _ => println!("Pilihan tidak valid!\n"),
        }
    }
}
