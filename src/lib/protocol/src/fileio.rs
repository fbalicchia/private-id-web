
// MIT License

extern crate common;
extern crate crypto;

use std::{
    collections::HashMap,
    path::Path,
    sync::{Arc, RwLock},
};

use crate::shared::TFeatures;
use serde_json::{Value};

/// load text and update the protocol
pub fn load_data_with_features<T>(
    input_path: T,
    keys: Arc<RwLock<Vec<String>>>,
    features: Arc<RwLock<TFeatures>>,
    num_features: Arc<RwLock<usize>>,
    num_records: Arc<RwLock<usize>>,
) where
    T: AsRef<Path>,
{
    if let (
        Ok(mut input_keys),
        Ok(mut input_features),
        Ok(mut input_num_features),
        Ok(mut input_num_records),
    ) = (
        keys.write(),
        features.write(),
        num_features.write(),
        num_records.write(),
    ) {
        if !input_keys.is_empty() || !input_features.is_empty() {
            info!("Data is not empty, skipping reading the data again")
        } else {
            let (mut keys, mut features) = common::files::transpose_keyed_nums(
                common::files::read_csv_as_keyed_nums(input_path, false),
            );
            assert!(!features.is_empty(), "Empty data features, fail fast, bye!");
            input_keys.extend(keys.drain(..));
            input_features.extend(features.drain(..));

            *input_num_features = input_features.len();
            *input_num_records = input_features[0].len();

            info!(
                "Data initialised with dimensions: cols: {}, rows: {}, keys len: {}",
                input_num_features,
                input_num_records,
                input_keys.len()
            );
        }
    }
}

#[derive(Default, Debug)]
pub struct KeyedCSV {
    pub headers: Vec<String>,
    pub records: HashMap<String, Vec<String>>,
}

impl KeyedCSV {
    /// Returns the keys(first column of the CSV) read from the CSV Input
    pub fn get_plain_keys(&self) -> Vec<String> {
        self.records
            .keys()
            .map(|k| k.to_string())
            .collect::<Vec<String>>()
    }
    /// Returns a writable CSV record padded with empty values for every other column except the key
    /// If there is no other column but key, it adds an empty string instead of the the plain key
    /// for debugging purposes
    pub fn get_empty_record_with_key(
        &self,
        key: String,
        empty_val: Option<&str>,
    ) -> Vec<String> {
        let mut record = vec![key];

        let mut until = self.headers.len();
        if self.headers.is_empty() {
            until = 2;
        }

        for _ in 1..until {
            record.push(String::from(empty_val.unwrap_or("")));
        }
        record
    }
    /// Returns a writable CSV record extended with non-key values from the input CSV
    /// If there is no other column but key, it adds the plain key for debugging purposes
    pub fn get_record_with_keys(&self, enc_key: String, raw_key: &str) -> Vec<String> {
        let mut record = vec![enc_key];
        if let Some(extra_columns) = self.records.get(raw_key) {
            if extra_columns.is_empty() {
                record.push(String::from(raw_key));
            } else {
                record.extend(extra_columns.iter().cloned());
            }
        }
        record
    }
}

pub fn load_data(data: Arc<RwLock<KeyedCSV>>, json_table: &str, has_headers: bool) {
    // Read json object from dynamic str into the expected Vec<Vec> form (previously from a CSV)
    let table: Value = serde_json::from_str(json_table).unwrap();
    let table: &Vec<Value> = table.as_array().unwrap();
    let table_len = table.len();

    let mut lines: Vec<Vec<String>> = vec![vec!["".to_string()]; table.len()];  // -OR- files::read_csv_as_strings(path)
    for (row_num, row) in table.iter().enumerate() {
        println!("Row #{}\t{}", row_num, row);
        lines[row_num] = vec![row.as_str().unwrap().to_string()];
    }

    if let Ok(mut wguard) = data.write() {
        if wguard.records.is_empty() {
            let mut line_it = lines.drain(..);
            if has_headers {
                if let Some(headers) = line_it.next() {
                    wguard.headers = headers;
                }
            }
            for line in line_it {
                if let Some((key, rest)) = line.split_first() {
                    wguard.records.insert(key.to_string(), rest.to_vec());
                }
            }
            let keys_len = wguard.records.len();
            info!(
                "Read {} lines from json (dedup: {} lines)",
                table_len,
                table_len - keys_len
            );
        } else {
            warn!("Attempted to run the protocol after the text was already initialized.")
        }
    }
}
