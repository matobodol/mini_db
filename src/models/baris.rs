use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum TipeBaris {
    Int(i64),
    Str(String),
    Float(f64),
    Date(chrono::NaiveDate),
    Enum { variant: String },
    Null,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Baris {
    pub tipe: Vec<TipeBaris>,
}

impl Baris {
    pub fn new(tipe: Vec<TipeBaris>) -> Self {
        Self {
            tipe: tipe.to_vec(),
        }
    }
}
