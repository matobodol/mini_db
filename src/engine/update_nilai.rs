use crate::{Baris, Tabel, TipeBaris, core_get_match, core_position_baris};

pub fn update_nilai(
    tabel: &mut Tabel,
    set_kolom: &str,
    set_nilai: TipeBaris,
    where_kolom: &str,
    where_nilai: &TipeBaris,
) -> Result<(Baris, Baris), String> {
    let &where_kolom_idx = tabel
        .index_kolom
        .get(where_kolom)
        .ok_or_else(|| "WhereKolomNotFound".to_string())?;

    let &set_kolom_idx = tabel
        .index_kolom
        .get(set_kolom)
        .ok_or_else(|| "SetKolomNotFound".to_string())?;

    let predicate = core_get_match(where_kolom_idx, where_nilai);
    let baris_idx =
        core_position_baris(tabel, predicate).ok_or_else(|| "WhereNilaiMismatch".to_string())?;

    let mut old_row = tabel.baris[baris_idx].clone();
    old_row.tipe.push(TipeBaris::Str("<- OLD_VALUE".into()));

    tabel.baris[baris_idx].tipe[set_kolom_idx] = set_nilai;

    let mut new_row = tabel.baris[baris_idx].clone();
    new_row.tipe.push(TipeBaris::Str("<- NEW_VALUE".into()));

    Ok((new_row, old_row))
}
