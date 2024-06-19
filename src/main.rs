mod expense;
use std::path::Path;
use std::fs;

use regex::Error;
use slint::{format, SharedString};
slint::include_modules!();

fn execute_conversion(pdf_filename: SharedString, csv_filename: SharedString) -> Result<SharedString, SharedString> {
    // check if PDF File exists and is a file
    if !Path::new(pdf_filename.as_str()).is_file() {
        return Err(format!("Arquivo PDF não encontrado: {}", pdf_filename));
    }

    // check if CSV name is valid (not empty)
    if csv_filename.is_empty() {
        return Err(format!("Arquivo CSV sem nome. Defina o nome do arquivo CSV de saída"));
    }

    // check if csv path is valid (if not, create it)
    let csv_output_path = Path::new(env!("CARGO_MANIFEST_DIR")).join("output").join("csv");
    match fs::create_dir_all(csv_output_path.clone()) {
        Ok(_) => {} // Do nothing,
        Err(e) => return Err(format!("Não foi possível criar o diretório de saida do CSV: {}", e))
    }

    let mut csv_expense_file_path = String::new();
    match csv_output_path.join(csv_filename.as_str()).to_str() {
        Some(csv_file) => csv_expense_file_path = csv_file.to_string(),
        None => return Err(format!("Não foi possível criar o arquivo CSV")),
    }

    let mut expenses_on_pdf: Vec<expense::ExpenseData> = Vec::new();
    expense::read_income_pdf(&pdf_filename.as_str(), &mut expenses_on_pdf);
    let _ = expense::convert_incomes_to_csv(&csv_expense_file_path, &mut expenses_on_pdf);

    Ok(format!("Arquivo CSV gerado em: {}", csv_expense_file_path))
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    //let ui_handle = ui.as_weak();
    ui.on_converte_fatura(move |pdf_filename, csv_filename| {

        match execute_conversion(pdf_filename, csv_filename) {
            Ok(s) => println!("DEU CERTO: {}", s),
            Err(s) => println!("Deu errado: {}", s),
        }
        //let ui = ui_handle.unwrap();
        //ui.set_results(result.into());
    });

    ui.run()
}
