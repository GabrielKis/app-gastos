mod expense;
use std::path::Path;
use std::fs;
use std::env;

use expense::PDFTypeFile;
use slint::{format, SharedString};
slint::include_modules!();

fn execute_conversion(pdf_filename: SharedString, csv_filename: SharedString, pdf_file: &PDFTypeFile) -> Result<SharedString, SharedString> {
    // check if PDF File exists and is a file
    if !Path::new(pdf_filename.as_str()).is_file() {
        return Err(format!("PDF File not found: {}", pdf_filename));
    }

    // check if CSV name is valid (not empty)
    if csv_filename.is_empty() {
        return Err(format!("Output CSV File without name. Give the CSV Output file a name"));
    }

    // check if csv path is valid (if not, create it)
    let csv_output_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("output").join("csv");
    match fs::create_dir_all(csv_output_path.clone()) {
        Ok(_) => {}, // Do nothing
        Err(e) => return Err(format!("Not possible to create CSV output directory: {}", e))
    }

    // join CSV file name
    let csv_expense_file_path_bind = csv_output_path.join(csv_filename.as_str());
    let csv_expense_file_path = csv_expense_file_path_bind.to_str();

    let mut expenses_on_pdf: Vec<expense::ExpenseData> = Vec::new();
    expense::read_income_pdf(&pdf_filename.as_str(), &mut expenses_on_pdf, pdf_file);
    let _ = expense::convert_incomes_to_csv(&csv_expense_file_path.unwrap(), &mut expenses_on_pdf, pdf_file);

    Ok(format!("Output CSV File available on: {}", csv_expense_file_path.unwrap()))
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    // On button click, execute PDF conversion
    ui.on_converte_fatura(move |pdf_filename, csv_filename| {

        let result_msg: String;
        let pdf_bill_type:PDFTypeFile = expense::PDFTypeFile::ItauPdf;

        match execute_conversion(pdf_filename, csv_filename, &pdf_bill_type) {
            Ok(s) => result_msg = format!("Success: {}", s).to_string(),
            Err(s) => result_msg = format!("Error: {}", s).to_string(),
        }
        let ui = ui_handle.unwrap();
        ui.set_result_msg(result_msg.into());
    });

    // Set default file path
    let mut execution_path = match env::current_exe() {
        Ok(exe_path) => exe_path,
        Err(_) => std::path::PathBuf::new(),
    };

    for _ in 0..3 {
        match execution_path.parent() {
            Some(exe_path) => execution_path = exe_path.to_path_buf(),
            None => {},// Do nothing
        }
    }

    let path_string = match execution_path.canonicalize() {
        Ok(path) => path.to_string_lossy().to_string(),
        Err(_) => String::new(),
    };
    ui.set_execution_path(path_string.into());

    // Execute SLINT
    ui.run()
}
