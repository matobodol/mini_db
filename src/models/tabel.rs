use serde::{Deserialize, Serialize};

use crate::{
    Baris, Kolom, TipeBaris, core_flag_kolom, core_position_kolom,
    display::{baris_to_tabel, kolom_to_tabel, show},
    engine::{
        add_kolom, delete_baris, delete_kemunculan_baris, delete_kolom, insert_baris, set_primary,
        update_nama_kolom, update_nilai,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tabel {
    pub kolom: Vec<Kolom>,
    pub baris: Vec<Baris>,
}

impl Tabel {
    pub fn new() -> Self {
        Self {
            kolom: Vec::new(),
            baris: Vec::new(),
        }
    }

    pub fn show(&self) {
        println!("\nproject: minidb row-oriented.");
        show(self);
    }

    // set primary key
    pub fn set_primary_key(&mut self, pk: &str) -> Result<(), String> {
        set_primary(self, pk)?;

        Ok(())
    }

    // add kolom
    pub fn add_kolom(&mut self, kolom: Vec<Kolom>) -> Result<(), String> {
        add_kolom(self, kolom)?;

        Ok(())
    }

    // insert baris
    pub fn add_baris(&mut self, baris: Vec<Baris>) -> Result<(), String> {
        let baris = baris.to_vec();
        for row in baris {
            insert_baris(self, row)?;
        }

        Ok(())
    }

    // delete kolom (multyple)
    pub fn delete_kolom(&mut self, kolom: Vec<&str>) -> Result<(), String> {
        let removed = delete_kolom(self, kolom)?;
        println!("\nKolom removed..");
        show(&kolom_to_tabel(removed));

        Ok(())
    }

    // delete satu baris (kemunculan pertama)
    pub fn delete_baris(&mut self, kolom: &str, nilai: TipeBaris) -> Result<(), String> {
        let removed = delete_baris(self, kolom, nilai)?;
        println!("\nBaris removed..");
        show(&baris_to_tabel(vec![removed]));

        Ok(())
    }

    // delete baris semua kemunculan
    pub fn delete_kemunculan_baris(&mut self, kolom: &str, nilai: TipeBaris) -> Result<(), String> {
        let removed = delete_kemunculan_baris(self, kolom, &nilai)?;
        println!("\nBaris removed..");
        show(&baris_to_tabel(removed));

        Ok(())
    }

    pub fn update_nilai(
        &mut self,
        select_kolom: &str,
        select_nilai: TipeBaris,
        target_kolom: &str,
        new_nilai: TipeBaris,
    ) -> Result<(), String> {
        let changed = update_nilai(self, select_kolom, select_nilai, target_kolom, new_nilai)?;
        println!("\nBaris changed in Column: {}..", target_kolom);
        show(&baris_to_tabel(changed));

        Ok(())
    }

    pub fn update_kolom_name(&mut self, select_kolom: &str, new_name: &str) -> Result<(), String> {
        update_nama_kolom(self, select_kolom, new_name)?;

        Ok(())
    }
}
