use crate::{Baris, Kolom, Tabel, TipeBaris};

// ==== KOLOM
// dipakai untuk membuat kolom baru
pub fn core_any_kolom<F>(tabel: &Tabel, f: F) -> bool
where
    F: Fn(&Kolom) -> bool,
{
    tabel.kolom.iter().any(f)
}

// dipakai untuk menghapus kolom
pub fn core_position_kolom<F>(tabel: &Tabel, f: F) -> Option<usize>
where
    F: Fn(&Kolom) -> bool,
{
    tabel.kolom.iter().position(f)
}

// gunakan ini untuk mendapatkan data kolom (Kolom)
pub fn core_find_kolom<F>(tabel: &Tabel, f: F) -> Option<&Kolom>
where
    F: Fn(&Kolom) -> bool,
{
    tabel.kolom.iter().find(|k| f(k))
}

//======== BARIS
// core cek nilai pada baris
// index nilai di dalam baris
// membutuhkan index_kolom sebagai patokan
pub fn core_get_match(index_kolom: usize, nilai: &TipeBaris) -> impl Fn(&Baris) -> bool + '_ {
    move |b| b.tipe.get(index_kolom).is_some_and(|v| v == nilai)
}

pub fn core_position_baris<F>(tabel: &Tabel, f: F) -> Option<usize>
where
    F: Fn(&Baris) -> bool,
{
    tabel.baris.iter().position(f)
}

pub fn core_find_baris<F>(tabel: &Tabel, f: F) -> Option<&Baris>
where
    F: Fn(&Baris) -> bool,
{
    tabel.baris.iter().find(|b| f(b))
}
