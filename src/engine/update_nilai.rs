use crate::{
    Baris, Tabel, TipeBaris, core_find_baris, core_get_match, core_position_baris,
    core_position_kolom,
};

pub fn update_nilai(
    tabel: &mut Tabel,
    select_kolom: &str,
    select_nilai: TipeBaris,
    target_kolom: &str,
    target_nilai: TipeBaris,
) -> Result<Vec<Baris>, String> {
    // Indexing Selected
    let index_select_kolom = core_position_kolom(tabel, |k| k.nama == select_kolom)
        .ok_or_else(|| "WhereKolomNotFound")?;

    let index_select_nilai =
        core_position_baris(tabel, core_get_match(index_select_kolom, &select_nilai))
            .ok_or_else(|| "WhereNilaiMissMatch".to_string())?;

    // indexing target
    let index_target_kolom = core_position_kolom(tabel, |t| t.nama == target_kolom)
        .ok_or_else(|| "TargetNotFound".to_string())?;

    let mut changed = Vec::new();

    let old_value = core_find_baris(tabel, core_get_match(index_select_kolom, &select_nilai))
        .ok_or_else(|| "nilai lama tidak ditemukan".to_string())?;
    changed.push(old_value.clone());

    // execute update nilai
    tabel.baris[index_select_nilai].tipe[index_target_kolom] = target_nilai.clone();

    let new_value = core_find_baris(tabel, core_get_match(index_target_kolom, &target_nilai))
        .ok_or_else(|| "nilai baru tidak ditemukan".to_string())?;
    changed.push(new_value.clone());

    // inset information to changed
    changed[0]
        .tipe
        .push(TipeBaris::Str("<- OLD VALUE".to_string()));
    changed[1]
        .tipe
        .push(TipeBaris::Str("<- NEW VALUE".to_string()));

    Ok(changed)
}
