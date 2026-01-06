use crate::{Kolom, Tabel, TipeBaris, core_any_kolom, engine::flag_switcher};

pub fn add_kolom(tabel: &mut Tabel, kolom: Vec<Kolom>) -> Result<(), String> {
    // VALIDASI KOLOM

    for kol in kolom {
        // cek nama kolom duplicate
        if core_any_kolom(tabel, |k| k.nama == kol.nama) {
            return Err("DuplicateKolomName".to_string());
        };

        if kol.primary_key {
            flag_switcher(tabel);
        }

        tabel.kolom.push(kol.clone());
    }

    // buat kolom
    // tabel.kolom.extend(kolom.to_vec());

    // tambah nilai null
    // pada baris sebelumnya yg kosong
    for len in &mut tabel.baris {
        if len.tipe.len() < tabel.kolom.len() {
            len.tipe.push(TipeBaris::Null);
        }
    }

    Ok(())
}
