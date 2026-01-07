use crate::{Tabel, TipeBaris};

pub fn update_null(tabel: &mut Tabel, nilai: TipeBaris) -> Result<(), String> {
    for baris in tabel.baris.iter_mut() {
        let index = baris.tipe.iter().position(|k| k == &TipeBaris::Null);

        if let Some(v) = index {
            baris.tipe[v] = nilai.clone();
        }
    }

    Ok(())
}
