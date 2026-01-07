use crate::{Kolom, Tabel, TipeBaris, core_any_kolom, core_flag_kolom, core_position_kolom};

pub fn add_kolom(tabel: &mut Tabel, kolom: Vec<Kolom>) -> Result<(), String> {
    // VALIDASI KOLOM

    for kol in kolom {
        // cek nama kolom duplicate
        if core_any_kolom(tabel, |k| k.nama == kol.nama) {
            return Err("DuplicateKolomName".to_string());
        };

        // cek primary_key yg aktif saat ini
        // dan ubah ke false
        if kol.primary_key {
            core_flag_kolom(tabel, |k| k.primary_key = false);
        }

        // push kolom baru
        tabel.kolom.push(kol.clone());
    }

    // tambah nilai null
    // pada baris sebelumnya yg kosong
    for len in &mut tabel.baris {
        if len.tipe.len() < tabel.kolom.len() {
            len.tipe.push(TipeBaris::Null);
        }
    }

    Ok(())
}

pub fn update_nama_kolom(
    tabel: &mut Tabel,
    select_kolom: &str,
    new_name: &str,
) -> Result<(), String> {
    // cek target kolom is exist
    core_position_kolom(tabel, |k| k.nama == select_kolom)
        .ok_or_else(|| "TargetKolomNotFound".to_string())?;

    // cek dupicate new name
    if core_any_kolom(tabel, |k| k.nama == new_name) {
        return Err("KolomNameIsDuplicate".to_string());
    }

    // update name
    core_flag_kolom(tabel, |k| {
        if k.nama == select_kolom.to_string() {
            k.nama = new_name.to_string()
        }
    });

    Ok(())
}
