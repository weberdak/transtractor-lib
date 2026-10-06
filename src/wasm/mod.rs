use crate::configs::db::ConfigDB;
use crate::parsers::flows::layout_to_text_items::layout_to_text_items;
use crate::parsers::flows::pdf_to_text_items::pdf_to_text_items;
use crate::parsers::flows::text_items_to_debug::text_items_to_debug_with_benchmark;
use crate::parsers::flows::text_items_to_layout::text_items_to_layout;
use crate::parsers::flows::text_items_to_statement_data::text_items_to_statement_data_with_benchmark;
use crate::structs::{Benchmark, BenchmarkMicros, Spec, StatementData, TextItem, Transaction};
use pdfsink_rs::PdfDocument;
use serde::Serialize;
use serde_wasm_bindgen::to_value;
use wasm_bindgen::prelude::*;

#[derive(Serialize)]
struct JsTransaction {
    date: i64,
    index: usize,
    description: String,
    amount: f64,
    balance: f64,
    account_number: String,
}

#[derive(Serialize)]
struct JsStatementData {
    transactions: Vec<JsTransaction>,
    benchmark: BenchmarkMicros,
}

#[wasm_bindgen(js_name = Parser)]
pub struct WasmParser {
    db: ConfigDB,
    last_deprecation_warnings: Vec<String>,
}

#[wasm_bindgen(js_class = Parser)]
impl WasmParser {
    #[wasm_bindgen(constructor)]
    pub fn new() -> Self {
        Self {
            db: ConfigDB::new(),
            last_deprecation_warnings: Vec::new(),
        }
    }

    #[wasm_bindgen(js_name = getDeprecationWarnings)]
    pub fn get_deprecation_warnings(&self) -> Box<[JsValue]> {
        self.last_deprecation_warnings
            .iter()
            .cloned()
            .map(JsValue::from)
            .collect::<Vec<_>>()
            .into_boxed_slice()
    }

    #[wasm_bindgen(js_name = loadConfigFromJson)]
    pub fn load_config_from_json(&mut self, config_json: String) -> Result<(), JsValue> {
        self.last_deprecation_warnings.clear();
        let warnings = self
            .db
            .register_from_str_with_warnings(&config_json)
            .map_err(|e| JsValue::from_str(&e))?;
        self.last_deprecation_warnings = warnings;
        Ok(())
    }

    #[wasm_bindgen(js_name = load)]
    pub fn load(&mut self, config_json_path: String) -> Result<(), JsValue> {
        self.last_deprecation_warnings.clear();
        let content = std::fs::read_to_string(&config_json_path).map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to read config file at {}: {}",
                config_json_path, e
            ))
        })?;

        let warnings = self
            .db
            .register_from_str_with_warnings(&content)
            .map_err(|e| JsValue::from_str(&e))?;
        self.last_deprecation_warnings = warnings;
        Ok(())
    }

    #[wasm_bindgen(js_name = parse)]
    pub fn parse(&self, pdf_path: String) -> Result<JsValue, JsValue> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let doc = PdfDocument::open(&pdf_path).map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to open PDF document at {}: {}",
                pdf_path, e
            ))
        })?;
        let items = pdf_to_text_items(&doc)
            .map_err(|e| JsValue::from_str(&format!("Failed to extract text items: {}", e)))?;
        benchmark.pdf_extractor.pause();
        let data = text_items_to_statement_data_with_benchmark(&self.db, &items, &mut benchmark)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse statement data: {}", e)))?;
        statement_data_to_js(&data)
    }

    #[wasm_bindgen(js_name = parseBytes)]
    pub fn parse_bytes(&self, pdf_bytes: &[u8]) -> Result<JsValue, JsValue> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let doc = PdfDocument::from_bytes(pdf_bytes).map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to open PDF document from byte input: {}",
                e
            ))
        })?;
        let items = pdf_to_text_items(&doc)
            .map_err(|e| JsValue::from_str(&format!("Failed to extract text items: {}", e)))?;
        benchmark.pdf_extractor.pause();
        let data = text_items_to_statement_data_with_benchmark(&self.db, &items, &mut benchmark)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse statement data: {}", e)))?;
        statement_data_to_js(&data)
    }

    #[wasm_bindgen(js_name = parseLayout)]
    pub fn parse_layout(&self, layout_path: String) -> Result<JsValue, JsValue> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let content = std::fs::read_to_string(&layout_path).map_err(|e| {
            JsValue::from_str(&format!(
                "Failed to read layout file at {}: {}",
                layout_path, e
            ))
        })?;
        let items = layout_to_text_items(&content)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse layout text: {}", e)))?;
        benchmark.pdf_extractor.pause();
        let data = text_items_to_statement_data_with_benchmark(&self.db, &items, &mut benchmark)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse statement data: {}", e)))?;
        statement_data_to_js(&data)
    }

    #[wasm_bindgen(js_name = parseLayoutText)]
    pub fn parse_layout_text(&self, layout_text: String) -> Result<JsValue, JsValue> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let items = layout_to_text_items(&layout_text)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse layout text: {}", e)))?;
        benchmark.pdf_extractor.pause();
        let data = text_items_to_statement_data_with_benchmark(&self.db, &items, &mut benchmark)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse statement data: {}", e)))?;
        statement_data_to_js(&data)
    }

    #[wasm_bindgen(js_name = layout)]
    pub fn layout(&self, pdf_path: String, output_file: String) -> Result<(), JsValue> {
        let items = pdf_path_to_text_items(&pdf_path)?;
        let layout_str = text_items_to_layout(&items)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate layout output: {}", e)))?;
        write_str_to_file(&layout_str, &output_file)
    }

    #[wasm_bindgen(js_name = layoutBytes)]
    pub fn layout_bytes(&self, pdf_bytes: &[u8]) -> Result<String, JsValue> {
        let items = pdf_bytes_to_text_items(pdf_bytes)?;
        text_items_to_layout(&items)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate layout output: {}", e)))
    }

    #[wasm_bindgen(js_name = debug)]
    pub fn debug(&self, pdf_path: String, output_file: String) -> Result<(), JsValue> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let items = pdf_path_to_text_items(&pdf_path)?;
        benchmark.pdf_extractor.pause();
        let debug_str = text_items_to_debug_with_benchmark(&self.db, &items, &mut benchmark)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate debug output: {}", e)))?;
        write_str_to_file(&debug_str, &output_file)
    }

    #[wasm_bindgen(js_name = debugLayout)]
    pub fn debug_layout(&self, layout_path: String, output_file: String) -> Result<(), JsValue> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let items = layout_path_to_text_items(&layout_path)?;
        benchmark.pdf_extractor.pause();
        let debug_str = text_items_to_debug_with_benchmark(&self.db, &items, &mut benchmark)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate debug output: {}", e)))?;
        write_str_to_file(&debug_str, &output_file)
    }

    #[wasm_bindgen(js_name = spec)]
    pub fn spec(&self, pdf_path: String, output_file: String) -> Result<(), JsValue> {
        let items = pdf_path_to_text_items(&pdf_path)?;
        let spec_str = text_items_to_spec_json(&self.db, items)?;
        write_str_to_file(&spec_str, &output_file)
    }

    #[wasm_bindgen(js_name = specLayout)]
    pub fn spec_layout(&self, layout_path: String, output_file: String) -> Result<(), JsValue> {
        let items = layout_path_to_text_items(&layout_path)?;
        let spec_str = text_items_to_spec_json(&self.db, items)?;
        write_str_to_file(&spec_str, &output_file)
    }

    #[wasm_bindgen(js_name = validateSpec)]
    pub fn validate_spec(&self, spec_path: String) -> Result<(), JsValue> {
        let spec_str = std::fs::read_to_string(&spec_path).map_err(|e| {
            JsValue::from_str(&format!("Failed to read spec file at {}: {}", spec_path, e))
        })?;
        self.validate_spec_text(spec_str)
    }

    #[wasm_bindgen(js_name = debugBytes)]
    pub fn debug_bytes(&self, pdf_bytes: &[u8]) -> Result<String, JsValue> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let items = pdf_bytes_to_text_items(pdf_bytes)?;
        benchmark.pdf_extractor.pause();
        text_items_to_debug_with_benchmark(&self.db, &items, &mut benchmark)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate debug output: {}", e)))
    }

    #[wasm_bindgen(js_name = debugLayoutText)]
    pub fn debug_layout_text(&self, layout_text: String) -> Result<String, JsValue> {
        let mut benchmark = Benchmark::new();
        benchmark.total.start();
        benchmark.pdf_extractor.start();
        let items = layout_to_text_items(&layout_text)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse layout text: {}", e)))?;
        benchmark.pdf_extractor.pause();
        text_items_to_debug_with_benchmark(&self.db, &items, &mut benchmark)
            .map_err(|e| JsValue::from_str(&format!("Failed to generate debug output: {}", e)))
    }

    #[wasm_bindgen(js_name = specBytes)]
    pub fn spec_bytes(&self, pdf_bytes: &[u8]) -> Result<String, JsValue> {
        let items = pdf_bytes_to_text_items(pdf_bytes)?;
        text_items_to_spec_json(&self.db, items)
    }

    #[wasm_bindgen(js_name = specLayoutText)]
    pub fn spec_layout_text(&self, layout_text: String) -> Result<String, JsValue> {
        let items = layout_to_text_items(&layout_text)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse layout text: {}", e)))?;
        text_items_to_spec_json(&self.db, items)
    }

    #[wasm_bindgen(js_name = validateSpecText)]
    pub fn validate_spec_text(&self, spec_json: String) -> Result<(), JsValue> {
        let spec = Spec::from_json(&spec_json)
            .map_err(|e| JsValue::from_str(&format!("Failed to parse spec JSON: {}", e)))?;
        spec.validate(&self.db)
            .map_err(|e| JsValue::from_str(&format!("Spec validation failed: {}", e)))
    }
}

fn pdf_path_to_text_items(pdf_path: &str) -> Result<Vec<TextItem>, JsValue> {
    let doc = PdfDocument::open(pdf_path).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to open PDF document at {}: {}",
            pdf_path, e
        ))
    })?;
    pdf_to_text_items(&doc)
        .map_err(|e| JsValue::from_str(&format!("Failed to extract text items: {}", e)))
}

fn pdf_bytes_to_text_items(pdf_bytes: &[u8]) -> Result<Vec<TextItem>, JsValue> {
    let doc = PdfDocument::from_bytes(pdf_bytes).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to open PDF document from byte input: {}",
            e
        ))
    })?;
    pdf_to_text_items(&doc)
        .map_err(|e| JsValue::from_str(&format!("Failed to extract text items: {}", e)))
}

fn layout_path_to_text_items(layout_path: &str) -> Result<Vec<TextItem>, JsValue> {
    let content = std::fs::read_to_string(layout_path).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to read layout file at {}: {}",
            layout_path, e
        ))
    })?;
    layout_to_text_items(&content)
        .map_err(|e| JsValue::from_str(&format!("Failed to parse layout text: {}", e)))
}

fn text_items_to_spec_json(db: &ConfigDB, items: Vec<TextItem>) -> Result<String, JsValue> {
    let spec = Spec::new(db, items)
        .map_err(|e| JsValue::from_str(&format!("Failed to build spec: {}", e)))?;
    spec.to_json()
        .map_err(|e| JsValue::from_str(&format!("Failed to serialize spec to JSON: {}", e)))
}

fn write_str_to_file(content: &str, output_file: &str) -> Result<(), JsValue> {
    std::fs::write(output_file, content).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to write to file at {}: {}",
            output_file, e
        ))
    })
}

fn statement_data_to_js(data: &StatementData) -> Result<JsValue, JsValue> {
    let js_data = JsStatementData::from(data);
    to_value(&js_data).map_err(|e| {
        JsValue::from_str(&format!(
            "Failed to serialize statement data to JavaScript value: {}",
            e
        ))
    })
}

impl From<&Transaction> for JsTransaction {
    fn from(value: &Transaction) -> Self {
        Self {
            date: value.date,
            index: value.index,
            description: value.description.clone(),
            amount: value.amount,
            balance: value.balance,
            account_number: value.account_number.clone(),
        }
    }
}

impl From<&StatementData> for JsStatementData {
    fn from(value: &StatementData) -> Self {
        Self {
            transactions: value.transactions.iter().map(JsTransaction::from).collect(),
            benchmark: value.benchmark.as_micros(),
        }
    }
}

impl Default for WasmParser {
    fn default() -> Self {
        Self::new()
    }
}
