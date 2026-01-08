use crate::*;

/*
 * HAPUS KOLOM
 */

pub fn delete_kolom(tabel: &mut Tabel, kolom: Vec<&str>) -> Result<Vec<Kolom>, String> {
    let mut removed = Vec::new();
    let mut indexes = Vec::new();

    //  Validasi & kumpulkan index
    for &nama in &kolom {
        let &idx = tabel
            .index_kolom
            .get(nama)
            .ok_or_else(|| format!("kolom '{}' tidak ditemukan.", nama))?;

        if tabel.kolom[idx].flag.primary_key {
            return Err(format!("ThisKolomIsPrimaryKey: '{}'", nama));
        }

        indexes.push((idx, nama.to_string()));
    }

    //  Urutkan index dari besar ke kecil
    indexes.sort_by(|a, b| b.0.cmp(&a.0));

    //  Hapus kolom
    for (idx, _) in indexes {
        // simpan kolom yang dihapus
        removed.push(tabel.kolom.remove(idx));

        // hapus nilai di setiap baris
        for row in &mut tabel.baris {
            row.tipe.remove(idx);
        }
    }

    //  Rebuild index (WAJIB)
    tabel.index_kolom.clear();
    for (i, kol) in tabel.kolom.iter().enumerate() {
        tabel.index_kolom.insert(kol.nama.clone(), i);
    }

    Ok(removed)
}

/*
 * HAPUS BARIS
 */
// hapus kemunculan pertama
pub fn delete_baris(tabel: &mut Tabel, kolom: &str, nilai: &TipeBaris) -> Result<Baris, String> {
    let &index_kolom = tabel
        .index_kolom
        .get(kolom)
        .ok_or_else(|| format!("kolom '{}' tidak ditemukan.", &kolom))?;

    let pattern = core_get_match(index_kolom, nilai);
    let index_baris = core_position_baris(tabel, pattern)
        .ok_or_else(|| format!("nilai {:?} pada kolom '{}' tidak ditemukan", nilai, kolom))?;

    let removed = tabel.baris.remove(index_baris);
    Ok(removed)
}

// hapus semua kemunculan
pub fn delete_kemunculan_baris(
    tabel: &mut Tabel,
    kolom: &str,
    nilai: &TipeBaris,
) -> Result<Vec<Baris>, String> {
    let &index_kolom = tabel
        .index_kolom
        .get(kolom)
        .ok_or_else(|| format!("kolom '{}' tidak ditemukan.", &kolom))?;

    let pattern = core_get_match(index_kolom, nilai);

    let mut removed = Vec::new();

    let mut i = 0;
    while i < tabel.baris.len() {
        if pattern(&tabel.baris[i]) {
            removed.push(tabel.baris.remove(i));
        } else {
            i += 1;
        }
    }

    if removed.is_empty() {
        return Err(format!(
            "nilai {:?} pada kolom '{}' tidak ditemukan",
            nilai, kolom
        ));
    }

    Ok(removed)
}
