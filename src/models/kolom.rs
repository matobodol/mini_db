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
pub struct Kolom {
    pub nama: String,
    pub tipe: TipeKolom,
    pub primary_key: bool,
    /* flag: Flag {primarykey: bool, increment: bool} */
}
impl Kolom {
    pub fn new(nama: &str, tipe: TipeKolom) -> Self {
        Self {
            nama: nama.to_string(),
            tipe: tipe,
            primary_key: false,
        }
    }

    pub fn set_primary_key(mut self) -> Self {
        self.primary_key = true;
        self
    }
}
