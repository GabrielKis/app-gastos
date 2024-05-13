mod expense;
//fn read_pdf_extract(filename: &str) {
//    let bytes = std::fs::read(filename).unwrap();
//    let out = pdf_extract::extract_text_from_mem(&bytes).unwrap();
//    println!("{}", out);
//}

// regular expression: ^\d{2}\s[A-Z]{3}\s[a-zA-z\s]*\d{1,3},\d{2}$
// match this thing: 

fn main() {
    let pdf_file = "tests/test_pdfs/Nubank_2024-04-20.pdf";
    let mut expenses_on_pdf: Vec<expense::ExpenseData> = Vec::new();
    expense::read_income_pdf(&pdf_file, &mut expenses_on_pdf);
    let _ = expense::convert_incomes_to_csv("tests/test_csv/Nubank_2024-04-20.csv", &mut expenses_on_pdf);
}
