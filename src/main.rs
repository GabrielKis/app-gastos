mod expense;
use std::path::Path;

fn main() {
    let project_path = Path::new(env!("CARGO_MANIFEST_DIR"));
    let tests_pdf_path = project_path.join("tests").join("test_pdfs");
    let tests_csv_path = project_path.join("tests").join("test_csv");
    let pdf_expense_filename = "Nubank_2024-06-22.pdf";
    let csv_expense_filename = pdf_expense_filename.replace(".pdf", ".csv");

    let mut pdf_expense_file_path = String::new();
    let mut csv_expense_file_path = String::new();

    match tests_pdf_path.join(pdf_expense_filename).to_str() {
        Some(pdf_file) => pdf_expense_file_path = pdf_file.to_string(),
        None => println!("Error"),
    }

    match tests_csv_path.join(csv_expense_filename).to_str() {
        Some(csv_file) => csv_expense_file_path = csv_file.to_string(),
        None => println!("Error"),
    }

    let mut expenses_on_pdf: Vec<expense::ExpenseData> = Vec::new();
    expense::read_income_pdf(&pdf_expense_file_path, &mut expenses_on_pdf);
    let _ = expense::convert_incomes_to_csv(&csv_expense_file_path, &mut expenses_on_pdf);
    println!("Arquivo CSV gerado em: {}", csv_expense_file_path);
}
