use prettytable::{Attr, Cell, Row, Table, color, format};

use crate::{Baris, Kolom, Tabel, TipeBaris};

fn mapper(tipe: &TipeBaris) -> (String, &str) {
    let (l, c, r) = ("l", "c", "r");

    match tipe {
        TipeBaris::Int(v) => (v.to_string(), r),
        TipeBaris::Str(v) => (v.clone(), l),
        TipeBaris::Float(v) => (v.to_string(), r),
        TipeBaris::Date(v) => (v.to_string(), r),
        TipeBaris::Enum { variant } => (variant.clone(), c),
        TipeBaris::Null => ("-".to_string(), c),
    }
}

pub fn kolom_to_tabel(kolom: Vec<Kolom>) -> Tabel {
    let mut tabel = Tabel::new();
    tabel.kolom.extend(kolom);
    tabel
}

pub fn baris_to_tabel(baris: Vec<Baris>) -> Tabel {
    let mut tabel = Tabel::new();
    tabel.baris.extend(baris);
    tabel
}

pub fn show(tabel: &Tabel) {
    // println!("Project: minidb row-oriented.");
    let mut pt = Table::new();
    pt.set_format(*format::consts::FORMAT_BOX_CHARS);

    if !tabel.kolom.is_empty() {
        let kolom = &tabel.kolom;
        pt = register_kolom(pt, &kolom);
    }

    if !tabel.baris.is_empty() {
        let baris = &tabel.baris;
        pt = register_baris(pt, &baris);
    }

    pt.printstd();
    println!();
}

//print struct kolom
fn register_kolom(pt: Table, kolom: &Vec<Kolom>) -> Table {
    let mut pt = pt;

    let cell: Vec<Cell> = kolom
        .iter()
        .map(|k| {
            match k.primary_key {
                true => {
                    Cell::new(&format!("*{}", &k.nama))
                        .style_spec("c")
                        .with_style(Attr::Bold)
                        // .with_style(Attr::Italic(true))
                        // .with_style(Attr::Underline(true))
                        .with_style(Attr::ForegroundColor(color::YELLOW))
                }
                false => Cell::new(&k.nama).style_spec("c").with_style(Attr::Bold),
            }
        })
        .collect();
    pt.add_row(Row::new(cell));
    pt
}

// print struct Baris
fn register_baris(pt: Table, baris: &Vec<Baris>) -> Table {
    let mut pt = pt;

    for v in baris {
        let cell: Vec<Cell> = v
            .tipe
            .iter()
            .map(|k| {
                let tipe = mapper(&k);
                Cell::new(&tipe.0).style_spec(tipe.1)
            })
            .collect();

        pt.add_row(Row::new(cell));
    }

    pt
}
