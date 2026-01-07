use crate::{TipeBaris, TipeKolom};

pub fn parse_to_tipe_baris(input: &str, tipe: &TipeKolom) -> Result<TipeBaris, String> {
    let raw = input.trim();

    if raw.is_empty() || raw.eq_ignore_ascii_case("null") {
        return Ok(TipeBaris::Null);
    }

    match tipe {
        TipeKolom::Int => raw
            .parse::<i64>()
            .map(|k| TipeBaris::Int(k))
            .map_err(|_| "type a number.".to_string()),
        TipeKolom::Float => raw
            .parse::<f64>()
            .map(|v| TipeBaris::Float(v))
            .map_err(|_| "type e float.".to_string()),
        TipeKolom::Date => chrono::NaiveDate::parse_from_str(raw, "%Y-%m-%d")
            .map(|v| TipeBaris::Date(v))
            .map_err(|_| "format date harus: YYYY-MM-DD.".to_string()),
        TipeKolom::Enum { variant } => {
            if variant.contains(&raw.to_string()) {
                Ok(TipeBaris::Enum {
                    variant: raw.to_string(),
                })
            } else {
                Err(format!("nilai {} tidak valid. pilihan {:?}", raw, variant))
            }
        }
        TipeKolom::Str => Ok(TipeBaris::Str(raw.to_string())),
    }
}

pub fn parse_to_tipe_kolom(input: &str) -> Result<TipeKolom, String> {
    let raw = input.to_lowercase();

    match raw.trim() {
        "int" => Ok(TipeKolom::Int),
        "str" => Ok(TipeKolom::Str),
        "float" => Ok(TipeKolom::Float),
        "date" => Ok(TipeKolom::Date),
        _ => {
            if raw.starts_with("enum:") {
                let variant = raw
                    .strip_prefix("enum:")
                    .unwrap_or("")
                    .split(',')
                    .map(|s| s.trim())
                    .filter(|s| !s.is_empty())
                    .map(String::from)
                    .collect();

                Ok(TipeKolom::Enum { variant })
            } else {
                Err("TipeNotValid".to_string())
            }
        }
    }
}
