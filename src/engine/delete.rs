use crate::*;

/*
 * HAPUS KOLOM
 */

pub fn delete_kolom(tabel: &mut Tabel, klm: Vec<&str>) -> Result<Vec<Kolom>, String> {
    let mut removed = Vec::new();

    for kolom in klm {
        let index_kolom = core_position_kolom(tabel, |k| k.nama == kolom)
            .ok_or_else(|| "KolomNotFound".to_string())?;

        if core_any_kolom(tabel, |k| k.nama == kolom && k.flag.primary_key) {
            return Err("ThisKolomIsPrimaryKey".into());
        }

        let kolom = core_find_kolom(tabel, |k| k.nama == kolom)
            .ok_or_else(|| "KolomNotFound".to_string())?;
        removed.push(kolom.clone());

        // remove baris kolom
        for row in &mut tabel.baris {
            row.tipe.remove(index_kolom);
        }

        // remove kolom
        tabel.kolom.remove(index_kolom);
    }
    Ok(removed)
}

/*
 * HAPUS BARIS
 */
// hapus kemunculan pertama
pub fn delete_baris(tabel: &mut Tabel, kolom: &str, nilai: TipeBaris) -> Result<Baris, String> {
    let index_kolom = core_position_kolom(tabel, |k| k.nama == kolom)
        .ok_or_else(|| "err: kolom tidak ditemukan".to_string())?;

    let pattern = core_get_match(index_kolom, &nilai);
    let index_baris = core_position_baris(tabel, pattern)
        .ok_or_else(|| format!("nilai {:?} pada kolom {} tidak ditemukan", &nilai, kolom))?;

    let removed = tabel.baris.remove(index_baris);
    Ok(removed)
}

// hapus semua kemunculan
pub fn delete_kemunculan_baris(
    tabel: &mut Tabel,
    kolom: &str,
    nilai: &TipeBaris,
) -> Result<Vec<Baris>, String> {
    let index_kolom = core_position_kolom(tabel, |k| k.nama == kolom)
        .ok_or_else(|| "kolom tidak ditemukan".to_string())?;

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
        return Err("nilai tidak ditemukan".to_string());
    }

    Ok(removed)
}
