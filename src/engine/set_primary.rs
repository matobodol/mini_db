use crate::{Tabel, core_flag_kolom, core_position_kolom};

pub fn set_primary(tabel: &mut Tabel, pk: &str) -> Result<usize, String> {
    let index_pk =
        core_position_kolom(tabel, |k| k.nama == pk).ok_or_else(|| "KolomNotFound".to_string())?;

    core_flag_kolom(tabel, |k| k.primary_key = false);
    tabel.kolom[index_pk].primary_key = true;

    Ok(index_pk)
}
