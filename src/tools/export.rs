use std::error::Error;
use csv::WriterBuilder;
use serde::de::DeserializeOwned;
use serde::Serialize;

pub fn save_csv<T: DeserializeOwned + Serialize>(data: Vec<T>) -> Result<(), Box<dyn Error>> {
    let mut wtr = WriterBuilder::new()
                    .delimiter(b';')
                    .double_quote(true)
                    .escape(b'\\')
                    .from_writer(std::io::stdout());
    for item in data {
        wtr.serialize(item)?;
    }
    wtr.flush()?;
    Ok(())
}
