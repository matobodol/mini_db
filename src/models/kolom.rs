use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum TipeKolom {
    Int,
    Str,
    Float,
    Date,
    Enum { variant: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Flag {
    pub primary_key: bool,
    pub increment: bool,
    pub nullable: bool,
}

impl Flag {
    pub fn default() -> Self {
        Self {
            primary_key: false,
            increment: false,
            nullable: false,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Kolom {
    pub nama: String,
    pub tipe: TipeKolom,
    pub flag: Flag,
}
impl Kolom {
    pub fn new(nama: &str, tipe: TipeKolom) -> Self {
        Self {
            nama: nama.to_string(),
            tipe: tipe,
            flag: Flag::default(),
        }
    }

    pub fn set_primary_key(mut self) {
        self.flag.primary_key = true;
    }
}
