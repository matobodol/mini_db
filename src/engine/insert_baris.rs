use crate::{Baris, Tabel};

pub fn insert_baris(tabel: &mut Tabel, baris: Baris) -> Result<(), String> {
    tabel.baris.push(baris);

    Ok(())
}
