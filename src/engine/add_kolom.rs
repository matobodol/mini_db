use crate::{Kolom, Tabel, TipeBaris};

use std::collections::hash_map::Entry;

pub fn add_kolom(tabel: &mut Tabel, kolom: Kolom) -> Result<(), String> {
    let index = tabel.kolom.len();

    match tabel.index_kolom.entry(kolom.nama.clone()) {
        Entry::Occupied(_) => {
            return Err("DuplicateKolomName".into());
        }
        Entry::Vacant(e) => {
            tabel.kolom.push(kolom);
            e.insert(index);
        }
    }

    // tambah Null ke semua baris lama
    for baris in &mut tabel.baris {
        baris.tipe.push(TipeBaris::Null);
    }

    Ok(())
}

pub fn update_nama_kolom(
    tabel: &mut Tabel,
    new_name: &str,
    select_kolom: &str,
) -> Result<(), String> {
    // ambil index kolom target dari hashmap
    let idx = *tabel
        .index_kolom
        .get(select_kolom)
        .ok_or("TargetKolomNotFound")?;

    // cek duplikat
    if tabel.index_kolom.contains_key(new_name) {
        return Err("KolomNameIsDuplicate".into());
    }

    // update nama di Vec
    tabel.kolom[idx].nama = new_name.to_string();

    // rename key di HashMap (remove + insert)
    tabel.index_kolom.remove(select_kolom);
    tabel.index_kolom.insert(new_name.to_string(), idx);

    Ok(())
}
