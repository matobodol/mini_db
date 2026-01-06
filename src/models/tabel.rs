use serde::{Deserialize, Serialize};

use crate::{
    Baris, Kolom, TipeBaris,
    display::{baris_to_tabel, kolom_to_tabel, show},
    engine::{
        add_kolom, delete_baris, delete_kemunculan_baris, delete_kolom, insert_baris, set_primary,
        update_nilai,
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

    // set primary key
    pub fn set_primary_key(&mut self, pk: &str) {
        set_primary(self, pk).unwrap();
    }

    // add kolom
    pub fn add_kolom(&mut self, kolom: &[Kolom]) {
        let kolom = kolom.to_vec();
        for v in kolom {
            add_kolom(self, &v.nama, v.tipe).unwrap();
        }
    }

    // insert baris
    pub fn add_baris(&mut self, baris: &[Baris]) {
        let baris = baris.to_vec();
        for row in baris {
            insert_baris(self, row).unwrap();
        }
    }

    // delete kolom (multyple)
    pub fn delete_kolom(&mut self, kolom: &[&str]) {
        let removed = delete_kolom(self, kolom).unwrap();
        println!("Kolom removed..");
        show(&kolom_to_tabel(removed));
    }

    // delete satu baris (kemunculan pertama)
    pub fn delete_baris(&mut self, kolom: &str, nilai: TipeBaris) {
        let removed = delete_baris(self, kolom, nilai).unwrap();
        println!("Baris removed..");
        show(&baris_to_tabel(vec![removed]));
    }

    // delete baris semua kemunculan
    pub fn delete_kemunculan_baris(&mut self, kolom: &str, nilai: TipeBaris) {
        let removed = delete_kemunculan_baris(self, kolom, &nilai).unwrap();
        println!("Baris removed..");
        show(&baris_to_tabel(removed));
    }

    pub fn update_nilai(
        &mut self,
        where_kolom: &str,
        where_nilai: TipeBaris,
        target_kolom: &str,
        target_nilai: TipeBaris,
    ) {
        let changed =
            update_nilai(self, where_kolom, where_nilai, target_kolom, target_nilai).unwrap();
        println!("Baris changed..");
        show(&baris_to_tabel(changed));
    }
}
