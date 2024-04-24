# Dicom Reorganizer

**Overview**

This tool provides a powerful and flexible solution for reorganizing DICOM files into a BIDS-compliant structure. Leveraging the performance of Rust and the ease of integration with Python, it efficiently handles large DICOM datasets.

**Key Features**

* **BIDS Compliance:** Ensures reorganized files adhere to the Brain Imaging Data Structure (BIDS) standard, promoting data sharing and interoperability.
* **Rust-Powered Performance:** Benefits from Rust's speed and safe memory management for fast reorganization, especially with large datasets.
* **Python Integration:** Seamlessly integrates into your existing Python analysis workflows using the `pyo3` bridge. 
* **Customization:** Supports flexible configuration through the input CSV file, allowing you to map custom DICOM metadata to BIDS elements.
* **Parallel Processing:** Efficiently reorganizes multiple DICOM files concurrently using Rayon.

**Requirements**

1. **Rust:** Installation via `rustup` is recommended (https://www.rust-lang.org/tools/install).
2. **Python:** A suitable Python environment.
3. **Dependencies:**
   * `pyo3`
   * `rayon`
   * `csv`
   * `anyhow` 

**Installation**

1. **Clone or download the repository:**
   ```bash
   git clone https://github.com/NeuroGranberg/dicom_reorganizer.git
   ```

2. **Install Rust dependencies:**
   ```bash
   cd dicom-reorganizer
   cargo install --path .
   ```

3. **Build the Python package:**
   ```bash
   maturin develop # or maturin build --release
   ```

4. **Install the generated Python package:**
   ```bash
   pip install https://github.com/NeuroGranberg/dicom_reorganizer/blob/main/target/wheels/dicom_reorganizer-0.1.0-cp310-cp310-manylinux_2_34_x86_64.whl  
   ```

### Creating the CSV File

The tool requires a CSV file containing the following information about your DICOM files:

* **subject_name:** Subject or patient name.
* **session_date:** Date of the imaging session.
* **dicom_type:** Type of DICOM data (e.g., 'anat', 'func', 'dwi', etc.).
* **current_path:** Current file path of the DICOM file.

**You can generate this CSV file using a DICOM metadata extraction tool like [Dicom Metadata Extractor](https://neurogranberg.github.io/Nima_Documentation/Dicom%20Extractor/Usage/).**

**Usage**

```python
import dicom_reorganizer

csv_path = 'path/to/your/dicom_metadata.csv'
root_name = 'path/to/bids/dataset' 
columns = {
    "subject_name": "SubjectID",  # Adjust column names as needed
    "session_date": "AcquisitionDate",
    "dicom_type": "DicomType",
    "current_path": "DicomPath",
}
copy_files = True  # Set to False if you want to move files instead of copying

dicom_reorganizer.reorganize_dicoms(csv_path, root_name, columns, copy_files)
```

**Example CSV Structure**

```csv
subject_name,session_date,dicom_type,current_path
sub-01,2023-11-23,anat,/home/user/dicoms/sub-01/T1.dcm
sub-01,2023-11-23,func,/home/user/dicoms/sub-01/rest_bold.dcm
...
```

**Contributing**

I welcome contributions! Feel free to open issues, submit pull requests, or suggest new features.

**License**

This project is licensed under MIT.
