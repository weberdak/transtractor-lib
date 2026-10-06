use crate::configs::db::ConfigDB;
use crate::parsers::flows::layout_to_text_items::layout_to_text_items;
use crate::parsers::flows::pdf_to_text_items::pdf_to_text_items;
use crate::parsers::flows::text_items_to_debug::text_items_to_debug_with_benchmark;
use crate::parsers::flows::text_items_to_layout::text_items_to_layout;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data_with_benchmark;
use crate::python::exceptions::{ConfigLoadError, ParseError, SpecError};
use crate::python::utils;
use crate::structs::{Benchmark, Spec, TextItem};
use pdfsink_rs::PdfDocument;
use pyo3::exceptions::PyRuntimeError;
use pyo3::prelude::*;
use std::io::Write;

/// Helper to convert Python PDF file path to Rust text items
fn py_pdf_path_to_text_items(py_pdf_path: &Bound<'_, PyAny>) -> PyResult<Vec<TextItem>> {
    let rust_pdf_path = py_pdf_path.extract::<String>()?;
    let pdf_document = PdfDocument::open(&rust_pdf_path)
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to open PDF document: {}", e)))?;
    pdf_to_text_items(&pdf_document)
        .map_err(|e| PyRuntimeError::new_err(format!("Failed to convert PDF to text items: {}", e)))
}

/// Helper to write Rust string to text file to path specified by Python caller.
fn str_to_file(content: String, py_file_path: &Bound<'_, PyAny>) -> PyResult<()> {
    let rust_file_path = py_file_path.extract::<String>()?;
    let mut file = std::fs::File::create(&rust_file_path).map_err(|e| {
        PyRuntimeError::new_err(format!(
            "Failed to create file at {}: {}",
            rust_file_path, e
        ))
    })?;
    file.write_all(content.as_bytes()).map_err(|e| {
        PyRuntimeError::new_err(format!(
            "Failed to write to file at {}: {}",
            rust_file_path, e
        ))
    })?;
    Ok(())
}

/// Helper to read string from text file to path specified by Python caller.
fn file_to_str(py_file_path: &Bound<'_, PyAny>) -> PyResult<String> {
    let rust_file_path = py_file_path.extract::<String>()?;
    std::fs::read_to_string(&rust_file_path).map_err(|e| {
        PyRuntimeError::new_err(format!("Failed to read file at {}: {}", rust_file_path, e))
    })
}

#[pyclass]
#[derive(Default)]
pub struct LibParser {
    db: ConfigDB,
    /// Last deprecation warnings from config loading
    last_deprecation_warnings: Vec<String>,
}

#[pymethods]
impl LibParser {
    /// Create a new Parser instance
    #[new]
    pub fn new() -> Self {
        Self {
            db: ConfigDB::new(),
            last_deprecation_warnings: Vec::new(),
        }
    }

    /// Get deprecation warnings from the last loaded configuration
    pub fn get_deprecation_warnings(&self) -> Vec<String> {
        self.last_deprecation_warnings.clone()
    }

    /// Register configuration file, update the StatementTyper and return any
    /// deprecation warnings.
    pub fn register_config_from_json(
        &mut self,
        py_config_json_path: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let json_str = file_to_str(py_config_json_path)?;

        // Clear previous warnings
        self.last_deprecation_warnings.clear();

        // Register and get deprecation warnings
        match self.db.register_from_str_with_warnings(&json_str) {
            Ok(warnings) => {
                self.last_deprecation_warnings = warnings;
                Ok(())
            }
            Err(e) => Err(ConfigLoadError::new_err(e)),
        }
    }

    /// Process a layout string and return statement data as a Python object of type
    /// AccountData.
    pub fn py_layout_path_to_py_statement_data(
        &self,
        py_layout_path: &Bound<'_, PyAny>,
    ) -> PyResult<Py<PyAny>> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let py_layout_str = file_to_str(py_layout_path)?;
        let text_items = layout_to_text_items(&py_layout_str).map_err(PyRuntimeError::new_err)?;
        benchmark.pdf_extractor.pause();
        let data =
            text_items_to_statement_data_with_benchmark(&self.db, &text_items, &mut benchmark)
                .map_err(ParseError::new_err)?;
        utils::rust_statement_data_to_py_statement_data(&data)
    }

    /// Process a PDF file path from Python caller and write layout file.
    pub fn py_pdf_path_to_layout(
        &self,
        py_pdf_path: &Bound<'_, PyAny>,
        py_layout_path: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let text_items = py_pdf_path_to_text_items(py_pdf_path)?;
        let layout_str = text_items_to_layout(&text_items).map_err(PyRuntimeError::new_err)?;
        str_to_file(layout_str, py_layout_path)
    }

    /// Process a PDF file path from Python caller and write debug file.
    pub fn py_pdf_path_to_debug(
        &self,
        py_pdf_path: &Bound<'_, PyAny>,
        py_debug_path: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let text_items = py_pdf_path_to_text_items(py_pdf_path)?;
        benchmark.pdf_extractor.pause();
        let debug_str = text_items_to_debug_with_benchmark(&self.db, &text_items, &mut benchmark)
            .map_err(ParseError::new_err)?;
        str_to_file(debug_str, py_debug_path)
    }

    /// Process a layout string from Python caller and write debug file.
    pub fn py_layout_path_to_debug(
        &self,
        py_layout_path: &Bound<'_, PyAny>,
        py_debug_path: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let py_layout_str = file_to_str(py_layout_path)?;
        let text_items = layout_to_text_items(&py_layout_str).map_err(PyRuntimeError::new_err)?;
        benchmark.pdf_extractor.pause();
        let debug_str = text_items_to_debug_with_benchmark(&self.db, &text_items, &mut benchmark)
            .map_err(ParseError::new_err)?;
        str_to_file(debug_str, py_debug_path)
    }

    /// Process a PDF file path from Python caller and return a Python AccountData object.
    pub fn py_pdf_path_to_py_statement_data(
        &self,
        py_pdf_path: &Bound<'_, PyAny>,
    ) -> PyResult<Py<PyAny>> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let text_items = py_pdf_path_to_text_items(py_pdf_path)?;
        benchmark.pdf_extractor.pause();
        let data =
            text_items_to_statement_data_with_benchmark(&self.db, &text_items, &mut benchmark)
                .map_err(ParseError::new_err)?;
        utils::rust_statement_data_to_py_statement_data(&data)
    }

    /// Process a PDF file path from Python caller and return a JSON spec string.
    pub fn py_pdf_path_to_spec(
        &self,
        py_pdf_path: &Bound<'_, PyAny>,
        py_spec_path: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let text_items = py_pdf_path_to_text_items(py_pdf_path)?;
        let spec = Spec::new(&self.db, text_items).map_err(ParseError::new_err)?;
        let spec_str = spec.to_json().map_err(|e| {
            PyRuntimeError::new_err(format!("Failed to convert Spec to JSON string: {}", e))
        })?;
        str_to_file(spec_str, py_spec_path)
    }

    /// Process a layout file path from Python caller and return a JSON spec string.
    pub fn py_layout_path_to_spec(
        &self,
        py_layout_path: &Bound<'_, PyAny>,
        py_spec_path: &Bound<'_, PyAny>,
    ) -> PyResult<()> {
        let py_layout_str = file_to_str(py_layout_path)?;
        let text_items = layout_to_text_items(&py_layout_str).map_err(PyRuntimeError::new_err)?;
        let spec = Spec::new(&self.db, text_items).map_err(ParseError::new_err)?;
        let spec_str = spec.to_json().map_err(|e| {
            PyRuntimeError::new_err(format!("Failed to convert Spec to JSON string: {}", e))
        })?;
        str_to_file(spec_str, py_spec_path)
    }

    /// Validate a JSON spec file from Python caller and return any validation errors.
    pub fn py_spec_path_to_validate(&self, py_spec_path: &Bound<'_, PyAny>) -> PyResult<()> {
        let spec_str = file_to_str(py_spec_path)?;
        let spec = Spec::from_json(&spec_str).map_err(|e| {
            PyRuntimeError::new_err(format!("Failed to parse Spec from JSON string: {}", e))
        })?;
        spec.validate(&self.db).map_err(SpecError::new_err)
    }
}
