use std::fs::File;
use std::error::Error;

use serde::de::DeserializeOwned;

pub mod date_serializer {
    use serde::{de::Error, de::DeserializeOwned, Deserialize, Deserializer, Serialize, Serializer};
    use chrono::{DateTime, NaiveDateTime, Utc};

    fn time_to_json(t: NaiveDateTime) -> String {
        DateTime::<Utc>::from_utc(t, Utc).to_rfc3339()
    }

    pub fn serialize<S: Serializer>(time: &NaiveDateTime, serializer: S) -> Result<S::Ok, S::Error> {
        time_to_json(time.clone()).serialize(serializer)
    }
    
    pub fn deserialize<'de, D: Deserializer<'de>>(deserializer: D) -> Result<NaiveDateTime, D::Error> {
        let time: String = Deserialize::deserialize(deserializer)?;
        Ok(NaiveDateTime::parse_from_str(&time, "%Y-%m-%d %H:%M:%S").map_err(D::Error::custom)?)
    }
}

pub fn load_csv<T: DeserializeOwned + Ord>(filename: &str) -> Result<Vec<T>, Box<dyn Error>> {
    let mut items: Vec<T> = Vec::new();

    let file = File::open(filename)?;
    let mut rdr = csv::ReaderBuilder::new()
                    .delimiter(b';')
                    .double_quote(true)
                    .escape(Some(b'\\'))
                    .from_reader(file);
    for record in rdr.deserialize() {
        let p: T = record?;
        items.push(p);
    }
    items.sort();
    Ok(items)
}
