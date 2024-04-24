use pyo3::prelude::*;
use pyo3::types::PyDict;
use rayon::prelude::*;
use csv::ReaderBuilder;
use std::fs::{self, copy};
use std::path::Path;
use anyhow::Result;

#[pyfunction]
fn reorganize_dicoms(csv_path: String, root_name: String, columns: &PyDict, copy_files: Option<bool>) -> PyResult<()> {
    let subject_name_column = columns.get_item("subject_name").unwrap().extract::<&str>().unwrap();
    let session_date_column = columns.get_item("session_date").unwrap().extract::<&str>().unwrap();
    let dicom_type_column = columns.get_item("dicom_type").unwrap().extract::<&str>().unwrap();
    let current_path_column = columns.get_item("current_path").unwrap().extract::<&str>().unwrap();

    let should_copy = copy_files.unwrap_or(true); // Default to true if not specified

    let mut rdr = ReaderBuilder::new().from_path(csv_path).unwrap();
    let headers = rdr.headers().unwrap().clone();

    rdr.records().par_bridge().for_each(|result| {
        let record = result.unwrap();
        let subject_name = record.get(headers.iter().position(|h| h == subject_name_column).unwrap()).unwrap();
        let session_date = record.get(headers.iter().position(|h| h == session_date_column).unwrap()).unwrap();
        let dicom_type = record.get(headers.iter().position(|h| h == dicom_type_column).unwrap()).unwrap();
        let current_path = record.get(headers.iter().position(|h| h == current_path_column).unwrap()).unwrap();

        // Add 'sub-' and 'ses-' prefixes
        let new_path = format!("{}/sub-{}/ses-{}/{}/", root_name, subject_name, session_date, dicom_type);
        fs::create_dir_all(&new_path).unwrap();

        // Option to copy or move
        let destination_path = Path::new(&new_path).join(Path::new(current_path).file_name().unwrap());
        if should_copy {
            copy(current_path, &destination_path).unwrap();
        } else {
            fs::rename(current_path, destination_path).unwrap();
        }
    });

    Ok(())
}

#[pymodule]
fn dicom_reorganizer(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(reorganize_dicoms, m)?)?;
    Ok(())
}