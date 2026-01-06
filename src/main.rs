use chrono::NaiveDate;
use mini_db::{Baris, Kolom, Tabel, TipeBaris, TipeKolom, display::show};

fn main() {
    // EKSPERIMEN BRANCH
    let mut tabel = Tabel::new();

    tabel.add_kolom(&[
        Kolom::new("Id", TipeKolom::Int),
        Kolom::new("Nama", TipeKolom::Str),
        Kolom::new("Alamat", TipeKolom::Str),
        Kolom::new(
            "Enum",
            TipeKolom::Enum {
                variant: vec!["Active".into(), "Pending".into(), "Progress".into()],
            },
        ),
        Kolom::new("Tanggal", TipeKolom::Date),
    ]);

    tabel.set_primary_key("Id");

    tabel.add_baris(&[
        Baris::new(&[
            TipeBaris::Int(1),
            TipeBaris::Str("jono".into()),
            TipeBaris::Str("jl mangga satu".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
            TipeBaris::Date(NaiveDate::parse_from_str("2026-1-5".into(), "%Y-%m-%d").unwrap()),
        ]),
        Baris::new(&[
            TipeBaris::Int(2),
            TipeBaris::Str("joni".into()),
            TipeBaris::Str("jl mangga dua".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
            TipeBaris::Date(NaiveDate::parse_from_str("2026-1-5".into(), "%Y-%m-%d").unwrap()),
        ]),
        Baris::new(&[
            TipeBaris::Int(3),
            TipeBaris::Str("jani".into()),
            TipeBaris::Str("jl mangga tiga".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
            TipeBaris::Date(NaiveDate::parse_from_str("2026-1-5".into(), "%Y-%m-%d").unwrap()),
        ]),
        Baris::new(&[
            TipeBaris::Int(3),
            TipeBaris::Str("jana".into()),
            TipeBaris::Str("jl mangga tiga".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
            TipeBaris::Date(NaiveDate::parse_from_str("2026-1-5".into(), "%Y-%m-%d").unwrap()),
        ]),
        Baris::new(&[
            TipeBaris::Int(3),
            TipeBaris::Str("jene".into()),
            TipeBaris::Str("jl mangga tiga".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
            TipeBaris::Date(NaiveDate::parse_from_str("2026-1-5".into(), "%Y-%m-%d").unwrap()),
        ]),
    ]);

    /* TES LOGIC */

    tabel.add_baris(&[
        Baris::new(&[
            TipeBaris::Int(77),
            TipeBaris::Str("Romeo Gadungan".into()),
            TipeBaris::Str("Jl. gelondongan".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
            TipeBaris::Date(NaiveDate::parse_from_str("2026-1-5".into(), "%Y-%m-%d").unwrap()),
        ]),
        Baris::new(&[
            TipeBaris::Int(77),
            TipeBaris::Str("Juliet Gadungan".into()),
            TipeBaris::Str("Jl. gelondongan".into()),
            TipeBaris::Enum {
                variant: "Progress".into(),
            },
            TipeBaris::Date(NaiveDate::parse_from_str("2026-1-5".into(), "%Y-%m-%d").unwrap()),
        ]),
        Baris::new(&[
            TipeBaris::Int(77),
            TipeBaris::Str("Juliet Gadungan".into()),
            TipeBaris::Str("Jl. gelondongan".into()),
            TipeBaris::Enum {
                variant: "Pending".into(),
            },
            TipeBaris::Date(NaiveDate::parse_from_str("2026-1-5".into(), "%Y-%m-%d").unwrap()),
        ]),
    ]);

    show(&tabel);

    // hapus kolom
    tabel.delete_kolom(&["Alamat"]);

    // hapus baris kolom Id dengan nilai x
    // hanya menghapus kemunculan pertama
    tabel.delete_baris("Id", TipeBaris::Int(1));

    tabel.delete_kemunculan_baris("Id", TipeBaris::Int(3));

    show(&tabel);

    // update nilai
    tabel.update_nilai(
        "Nama",
        TipeBaris::Str("joni".to_string()),
        "Enum",
        TipeBaris::Enum {
            variant: "Pending".into(),
        },
    );

    show(&tabel);

    // tinggal buat lagic update nilai...
    // validasi tipe kolom dan baris
    // schema primary-key
    // validasi duplicat kolom primary
    // enum error
    // parse raw dto + validasi
    // menu interaktif
    // save & load to file
}
