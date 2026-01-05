use crate::{Kolom, Tabel, TipeBaris, TipeKolom, core_any_kolom, core_position_kolom};

pub fn add_kolom(tabel: &mut Tabel, nama: &str, tipe: TipeKolom) -> Result<(), String> {
    if core_any_kolom(tabel, |k| k.nama == nama) {
        return Err("DuplicateKolomName".to_string());
    };

    let kolom = Kolom::new(nama, tipe);

    tabel.kolom.push(kolom);
    let _ = core_position_kolom(tabel, |k| k.nama == nama)
        .ok_or_else(|| "Gagal menambah kolom".to_string())?;

    for len in &mut tabel.baris {
        if len.tipe.len() < tabel.kolom.len() {
            len.tipe.push(TipeBaris::Null);
        }
    }

    Ok(())
}
