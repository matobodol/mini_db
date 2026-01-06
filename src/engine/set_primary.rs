use crate::{Tabel, core_position_kolom};

pub fn set_primary(tabel: &mut Tabel, pk: &str) -> Result<usize, String> {
    let index_pk =
        core_position_kolom(tabel, |k| k.nama == pk).ok_or_else(|| "KolomNotFound".to_string())?;

    flag_switcher(tabel);
    tabel.kolom[index_pk].primary_key = true;

    Ok(index_pk)
}

pub fn flag_switcher(tabel: &mut Tabel) {
    tabel
        .kolom
        .iter_mut()
        .filter(|k| k.primary_key)
        .for_each(|k| k.primary_key = false);
}
