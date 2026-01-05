use crate::{Kolom, Tabel, TipeKolom, core_any_kolom};

pub fn add_kolom(tabel: &mut Tabel, nama: &str, tipe: TipeKolom) -> Result<(), String> {
    if core_any_kolom(tabel, |k| k.nama == nama) {
        return Err("DuplicateKolomName".to_string());
    };

    let kolom = Kolom::new(nama, tipe);

    tabel.kolom.push(kolom);

    Ok(())
}
