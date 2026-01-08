use std::collections::HashMap;

use serde::{Deserialize, Serialize};

use crate::{
    Baris, Kolom, TipeBaris,
    display::{baris_to_tabel, kolom_to_tabel, show},
    engine::{
        add_kolom, delete_baris, delete_kemunculan_baris, delete_kolom, insert_baris, set_primary,
        update_nama_kolom, update_nilai, update_null,
    },
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Tabel {
    pub kolom: Vec<Kolom>,
    pub index_kolom: HashMap<String, usize>,
    pub baris: Vec<Baris>,
    pub index_pk: HashMap<String, usize>,
}

impl Tabel {
    pub fn new() -> Self {
        Self {
            kolom: Vec::new(),
            index_kolom: HashMap::new(),
            baris: Vec::new(),
            index_pk: HashMap::new(),
        }
    }

    pub fn reset_tabel(&mut self) {
        self.kolom = Vec::new();
        self.index_kolom = HashMap::new();
        self.baris = Vec::new();
    }
    pub fn reset_baris(&mut self) {
        self.baris = Vec::new();
    }

    pub fn update_null(&mut self, nilai: TipeBaris) -> Result<(), String> {
        update_null(self, nilai)?;

        Ok(())
    }

    pub fn show(&self) {
        println!("kolom : {:?}", self.index_kolom);
        println!("idx pk: {:?}", self.index_pk);
        println!("\nproject: minidb row-oriented.");
        show(self);
    }

    // set primary key
    pub fn set_primary_key(&mut self, pk: &str) -> Result<(), String> {
        set_primary(self, pk)?;

        Ok(())
    }

    // add kolom
    pub fn add_kolom(&mut self, kolom: Kolom) -> Result<(), String> {
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
        let removed = delete_baris(self, kolom, &nilai)?;
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
        set_kolom: &str,
        set_nilai: TipeBaris,
        where_kolom: &str,
        where_nilai: &TipeBaris,
    ) -> Result<(), String> {
        let changed = update_nilai(self, set_kolom, set_nilai, where_kolom, where_nilai)?;
        println!("\nBaris changed in Column: {}..", set_kolom);
        show(&baris_to_tabel(vec![changed.0, changed.1]));

        Ok(())
    }

    pub fn update_kolom_name(
        &mut self,
        set_new_name: &str,
        where_kolom_name: &str,
    ) -> Result<(), String> {
        update_nama_kolom(self, set_new_name, where_kolom_name)?;

        Ok(())
    }
}
