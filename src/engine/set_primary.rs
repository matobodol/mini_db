use crate::{Tabel, core_position_kolom};

pub fn set_primary(tabel: &mut Tabel, pk: &str) -> Result<(), String> {
    let index_pk =
        core_position_kolom(tabel, |k| k.nama == pk).ok_or_else(|| "KolomNotFound".to_string())?;

    tabel.kolom[index_pk].primary_key = true;

    for (i, v) in tabel.kolom.iter_mut().enumerate() {
        if i != index_pk {
            v.primary_key = false
        }
    }

    Ok(())
}
