use crate::{Tabel, core_flag_kolom};

pub fn set_primary(tabel: &mut Tabel, to_pk: &str) -> Result<(), String> {
    let &index_pk = tabel
        .index_kolom
        .get(to_pk)
        .ok_or_else(|| format!("Kolom '{}' Not Found.", to_pk))?;

    core_flag_kolom(tabel, |k| k.flag.primary_key = false);
    tabel.kolom[index_pk].flag.primary_key = true;

    tabel.index_pk.clear();
    tabel.index_pk.insert(to_pk.to_string(), index_pk);

    return Ok(());
}
