use serde::{Deserialize, Serialize};

#[derive(Deserialize, Default, Debug, Clone)]
pub struct FormatData {
    value: String,
    unit: String,
}

impl Serialize for FormatData {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        (&self.value, &self.unit).serialize(serializer)
    }
}

impl FormatData {
    pub fn new(value: u64, unit: &str) -> FormatData {
        FormatData {
            value: value.to_string(),
            unit: unit.to_string(),
        }
    }

    pub fn new_with_precision(value: f64, unit: &str, precision: u8) -> FormatData {
        let precision = precision as usize;
        FormatData {
            value: format!("{:.precision$}", value),
            unit: unit.to_string(),
        }
    }
}

pub struct Formator {
    precision: u8,
}

impl Formator {
    pub fn new() -> Formator {
        Formator { precision: 1 }
    }

    pub fn format_from_byte(&self, value: u64) -> FormatData {
        let unit = "B";
        if value > 1024 {
            self.format_from_kilo_byte(value as f64 / 1024.0)
        } else {
            FormatData::new(value, unit)
        }
    }

    pub fn format_from_kilo_byte(&self, value: f64) -> FormatData {
        let unit = "KiB";
        if value > 1024.0 {
            self.format_from_mega_byte(value / 1024.0)
        } else {
            FormatData::new_with_precision(value, unit, self.precision)
        }
    }

    pub fn format_from_mega_byte(&self, value: f64) -> FormatData {
        let unit = "MiB";
        if value > 1024.0 {
            self.format_from_giga_byte(value / 1024.0)
        } else {
            FormatData::new_with_precision(value, unit, self.precision)
        }
    }

    pub fn format_from_giga_byte(&self, value: f64) -> FormatData {
        let unit = "GiB";
        if value > 1024.0 {
            self.format_from_tera_byte(value / 1024.0)
        } else {
            FormatData::new_with_precision(value, unit, self.precision)
        }
    }

    pub fn format_from_tera_byte(&self, value: f64) -> FormatData {
        let unit = "TiB";
        if value > 1024.0 {
            self.format_from_peta_byte(value / 1024.0)
        } else {
            FormatData::new_with_precision(value, unit, self.precision)
        }
    }

    pub fn format_from_peta_byte(&self, value: f64) -> FormatData {
        let unit = "PiB";
        if value > 1024.0 {
            self.format_from_eta_byte(value / 1024.0)
        } else {
            FormatData::new_with_precision(value, unit, self.precision)
        }
    }

    pub fn format_from_eta_byte(&self, value: f64) -> FormatData {
        let unit = "EiB";
        FormatData::new_with_precision(value / 1024.0, unit, self.precision)
    }
}

pub trait Convert<V> {
    fn convert(&self) -> V;
}
