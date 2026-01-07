use crate::Kolom;

use std::collections::HashSet;

pub fn is_duplicate_kolom(kolom: &Vec<Kolom>) -> Result<(), String> {
    let mut seen = HashSet::new();

    for k in kolom {
        if !seen.insert(&k.nama) {
            return Err("KolomIsDuplicate".into());
        }
    }

    Ok(())
}
