mod expense;
use std::path::Path;
use std::fs;

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
        Ok(_) => {}, // Do nothing,
        Err(e) => return Err(format!("Não foi possível criar o diretório de saida do CSV: {}", e))
    }

    // join CSV file name
    let csv_expense_file_path_bind = csv_output_path.join(csv_filename.as_str());
    let csv_expense_file_path = csv_expense_file_path_bind.to_str();

    let mut expenses_on_pdf: Vec<expense::ExpenseData> = Vec::new();
    expense::read_income_pdf(&pdf_filename.as_str(), &mut expenses_on_pdf);
    let _ = expense::convert_incomes_to_csv(&csv_expense_file_path.unwrap(), &mut expenses_on_pdf);

    Ok(format!("Arquivo CSV gerado em: {}", csv_expense_file_path.unwrap()))
}

fn main() -> Result<(), slint::PlatformError> {
    let ui = AppWindow::new()?;

    let ui_handle = ui.as_weak();
    ui.on_converte_fatura(move |pdf_filename, csv_filename| {

        let result_msg: String;
        match execute_conversion(pdf_filename, csv_filename) {
            Ok(s) => result_msg = format!("Sucesso: {}", s).to_string(),
            Err(s) => result_msg = format!("Erro: {}", s).to_string(),
        }
        let ui = ui_handle.unwrap();
        ui.set_result_msg(result_msg.into());
    });

    ui.run()
}
