use mini_db::{Baris, Kolom, Tabel, TipeBaris, TipeKolom, display::show, engine::insert_baris};

fn main() {
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
        ]),
        Baris::new(&[
            TipeBaris::Int(2),
            TipeBaris::Str("joni".into()),
            TipeBaris::Str("jl mangga dua".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
        ]),
        Baris::new(&[
            TipeBaris::Int(3),
            TipeBaris::Str("jani".into()),
            TipeBaris::Str("jl mangga tiga".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
        ]),
        Baris::new(&[
            TipeBaris::Int(3),
            TipeBaris::Str("jana".into()),
            TipeBaris::Str("jl mangga tiga".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
        ]),
        Baris::new(&[
            TipeBaris::Int(3),
            TipeBaris::Str("jene".into()),
            TipeBaris::Str("jl mangga tiga".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
        ]),
    ]);

    /* TES LOGIC */

    insert_baris(
        &mut tabel,
        Baris::new(&[
            TipeBaris::Int(77),
            TipeBaris::Str("Romeo Gadungan".into()),
            TipeBaris::Str("Jl. gelondongan".into()),
            TipeBaris::Enum {
                variant: "Active".into(),
            },
        ]),
    )
    .unwrap();
    insert_baris(
        &mut tabel,
        Baris::new(&[
            TipeBaris::Int(77),
            TipeBaris::Str("Juliet Gadungan".into()),
            TipeBaris::Str("Jl. gelondongan".into()),
            TipeBaris::Enum {
                variant: "Progress".into(),
            },
        ]),
    )
    .unwrap();
    insert_baris(
        &mut tabel,
        Baris::new(&[
            TipeBaris::Int(77),
            TipeBaris::Str("Juliet Gadungan".into()),
            TipeBaris::Str("Jl. gelondongan".into()),
            TipeBaris::Enum {
                variant: "Pending".into(),
            },
        ]),
    )
    .unwrap();
    show(&tabel);

    // hapus baris kolom id n dengan nilai x
    tabel.delete_baris("Id", TipeBaris::Int(1));

    // hapus kolom
    tabel.delete_kolom(&["Alamat", "Enum"]);

    tabel.delete_kemunculan_baris("Id", TipeBaris::Int(3));

    show(&tabel);

    tabel.add_kolom(&[
        Kolom::new("Alamat", TipeKolom::Str),
        Kolom::new("Status", TipeKolom::Str),
    ]);
    show(&tabel);
}
