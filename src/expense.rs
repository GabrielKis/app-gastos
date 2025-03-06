use regex::Regex;
use std::fs::File;
use std::io::{self, Write};

#[derive(Debug, Default, Clone)]
pub struct ExpenseData {
    date: String,
    income: String,
    value: String,
}

#[derive(Debug, Default)]
struct CSVData {
    date: String,
    income: String,
    group: String,
    value: String,
    division: String,
    from: String,
    to: String,
}

// This class should handle all expense info
// read data from PDF
// write data on CSV according to definition given by new routine
// sum all expenses

const NUBANK_EXPENSE_REGEX: &str = r"^(\d{2}\s[A-Z]{3})\s(.+)\sR\$\s(\d{1,3},\d{2})$";
const ITAU_EXPENSE_REGEX: &str = r"^(\d{2}\/\d{2})\s(.+?)\s(\d{1,3}(?:\.\d{3})*,\d{2})($|\s{1})";

#[derive(PartialEq, Clone, Copy)]
pub enum PDFTypeFile {
    ItauPdf,
    NubankPdf,
}

// Function to extract matched fields from the input string
fn extract_fields(input: &str, pdf_type: &PDFTypeFile) -> Option<ExpenseData> {
    // Define the regular expression pattern

    let re: Regex = match pdf_type {
        PDFTypeFile::NubankPdf => Regex::new(NUBANK_EXPENSE_REGEX).unwrap(),
        PDFTypeFile::ItauPdf => Regex::new(ITAU_EXPENSE_REGEX).unwrap(),
    };

    // Match the input string against the regular expression
    if let Some(captures) = re.captures(input) {
        // Extract matched groups
        let date = captures.get(1).unwrap().as_str().to_string();
        let income = captures.get(2).unwrap().as_str().to_string();
        let value = captures.get(3).unwrap().as_str().to_string().replace(".", "");

        // Return a struct with the matched fields
        Some(ExpenseData { date, income, value })
    } else {
        None
    }
}

fn csv_date_format(csv_struct: &mut CSVData, date: &str, year: &str, pdf_type: &PDFTypeFile) {
    let aux_date_str: Vec<&str>;
    let month_number: &str;
    if pdf_type == &PDFTypeFile::ItauPdf {
        aux_date_str = date.split('/').collect();
        month_number = aux_date_str[1];
    } else {
        aux_date_str = date.split_whitespace().collect();
        match aux_date_str[1] {
            "JAN" => month_number = "01",
            "FEV" => month_number = "02",
            "MAR" => month_number = "03",
            "ABR" => month_number = "04",
            "MAI" => month_number = "05",
            "JUN" => month_number = "06",
            "JUL" => month_number = "07",
            "AGO" => month_number = "08",
            "SET" => month_number = "09",
            "OUT" => month_number = "10",
            "NOV" => month_number = "11",
            "DEZ" => month_number = "12",
            _ => month_number = "00",
        }
    }

    csv_struct.date = format!("{}-{}-{}", year, month_number, aux_date_str[0]);
}

fn expense_to_csv(expense_data: &mut Vec<ExpenseData>, pdf_type: &PDFTypeFile) -> Vec<String> {
    let mut csv_data = CSVData::default();
    let mut csv_string: String;
    let mut csv_str_vector: Vec<String> = vec![];
    csv_str_vector.push("Data,Descrição,Categoria,Custo,Divide,Person_A,Person_B\n".to_string());

    for expense in expense_data {
        csv_date_format(&mut csv_data, &expense.date, "2025", pdf_type);
        csv_data.income = expense.income.clone();
        csv_data.group = "Geral".to_string();
        csv_data.value = format!("\"{}\"", expense.value.clone());
        csv_data.division = "\"0,5\"".to_string();
        csv_data.from = "=D3*(1-E3)".to_string();
        csv_data.to = "=D3*E3".to_string();
        //csv_data.to = format!("-{}", csv_data.value);

        csv_string = format!("{},{},{},{},{},{},{}",
                            csv_data.date,
                            csv_data.income,
                            csv_data.group,
                            csv_data.value,
                            csv_data.division,
                            csv_data.from,
                            csv_data.to
                        );
        csv_str_vector.push(csv_string.clone());
    }
    return csv_str_vector;
}

fn write_vector_to_file(file_path: &str, strings: Vec<String>) -> io::Result<()> {
    // Open the file in write-only mode, returns `io::Result<File>`
    let mut file = File::create(file_path)?;

    // Iterate over the vector and write each string to the file
    for s in strings {
        writeln!(file, "{}", s)?; // Write each string followed by a newline
    }

    Ok(())
}

pub fn read_income_pdf(filename: &str, expense_data: &mut Vec<ExpenseData>, pdf_type: &PDFTypeFile) {
    // This function should read all PDF file and save the informations
    // on the "expense" struct on a vector.

    // TODO: handle errors when parsing the PDF
    let bytes = std::fs::read(filename).unwrap();
    let pdf_read_text = pdf_extract::extract_text_from_mem(&bytes).unwrap(); // TODO: handle errors on the read - use match
    let mut fields: Option<ExpenseData>;
    for line in pdf_read_text.lines() {
        fields = extract_fields(line, pdf_type);
        if fields.is_some() {
            expense_data.push(fields.unwrap());
        }

        // On Itau PDF, stop reading after this line:
        if line.contains("Total dos lançamentos atuais") && pdf_type == &PDFTypeFile::ItauPdf {
            return;
        }
    }
}


pub fn convert_incomes_to_csv(filename: &str, expense_data: &mut Vec<ExpenseData>, pdf_type: &PDFTypeFile) -> io::Result<()> {
    let csv_expenses_vec: Vec<String> = expense_to_csv(expense_data, pdf_type);
    write_vector_to_file(filename, csv_expenses_vec)?;

    Ok(())
}
