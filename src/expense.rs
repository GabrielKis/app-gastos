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

// Function to extract matched fields from the input string
fn extract_fields(input: &str) -> Option<ExpenseData> {
    // Define the regular expression pattern
    let re = Regex::new(r"^(\d{2}\s[A-Z]{3})\s(.+)\s(\d{1,3},\d{2})$").unwrap();

    // Match the input string against the regular expression
    if let Some(captures) = re.captures(input) {
        // Extract matched groups
        let date = captures.get(1).unwrap().as_str().to_string();
        let income = captures.get(2).unwrap().as_str().to_string();
        let value = captures.get(3).unwrap().as_str().to_string();

        // Return a struct with the matched fields
        Some(ExpenseData { date, income, value })
    } else {
        None
    }
}

fn csv_date_format(csv_struct: &mut CSVData, date: &str, year: &str) {
    let aux_date_str: Vec<&str> = date.split_whitespace().collect();
    let month_number: &str;

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
    csv_struct.date = format!("{}-{}-{}", year, month_number, aux_date_str[0]);
}

fn expense_to_csv(expense_data: &mut Vec<ExpenseData>) -> Vec<String> {
    // Add date
    //let mut csv_data_vec: Vec<CSVData> = Vec::new();
    let mut csv_data = CSVData::default();
    let mut csv_string: String;
    let mut csv_str_vector: Vec<String> = vec![];
    csv_str_vector.push("Data,Descrição,Categoria,Custo,Divide,Lice,Gabs\n".to_string());

    for expense in expense_data {
        csv_date_format(&mut csv_data, &expense.date, "2024");
        csv_data.income = expense.income.clone();
        csv_data.group = "Geral".to_string();
        csv_data.value = format!("\"{}\"", expense.value.clone());
        csv_data.division = "\"0,5\"".to_string();
        //csv_data.from = format!("{}", csv_data.value);
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
        //println!("{}", csv_string);
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

pub fn read_income_pdf(filename: &str, expense_data: &mut Vec<ExpenseData>) {
    // This function should read all PDF file and save the informations
    // on the "expense" struct on a vector.

    // TODO: handle errors when parsing the PDF
    let bytes = std::fs::read(filename).unwrap();
    let pdf_read_text = pdf_extract::extract_text_from_mem(&bytes).unwrap(); // TODO: handle errors on the read - use match
    for line in pdf_read_text.lines() {
        if let Some(fields) = extract_fields(line) {
            expense_data.push(fields);
        }
    }
}

pub fn convert_incomes_to_csv(filename: &str, expense_data: &mut Vec<ExpenseData>) -> io::Result<()> {
    let csv_expenses_vec: Vec<String> = expense_to_csv(expense_data);
    write_vector_to_file(filename, csv_expenses_vec)?;

    Ok(())
}
