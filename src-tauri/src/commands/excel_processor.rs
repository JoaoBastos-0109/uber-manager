use std::{collections::HashMap, path::Path};
use chrono::{NaiveDate, TimeDelta};
use serde::{Serialize, Deserialize};
use calamine::{Data, DataType, Range, Reader, open_workbook_auto};

#[derive( Serialize, Deserialize)]
pub struct TollEntry{
    pub date: String,
    pub description: String,
    pub category: String,
    pub cost: f64,
}

#[derive( Serialize, Deserialize)]
pub struct PlateSummary{
    pub plate: String,
    pub total_cost: f64,
    pub entries: Vec<TollEntry>,
}

const ALLOWED_EXTENSIONS: &[&str] = &["xlsx", "xls", "csv"];

#[tauri::command]
pub fn process_excel( file_path: String ) -> Result< Vec<PlateSummary>, String> {

    check_file_extension( &file_path )?;

    let range = open_excel_workbook( &file_path )?;

    let data_map = extract_data_from_range(&range)?;

    return Err("error".to_string());

}

#[tauri::command]
pub fn check_file_extension( file_path: &str) -> Result<String, String> {
    let path = Path::new(&file_path);

    let extension = path.extension()
        .and_then(|s| s.to_str())
        .unwrap_or("");

    match extension {
        item if ALLOWED_EXTENSIONS.contains(&item) => Ok( extension.to_string() ),
        _ => Err("Invalid File".into() ),
    }

}

#[tauri::command]
pub fn get_allowed_extensions() -> Vec<String> {
    return ALLOWED_EXTENSIONS.iter().map(|&s| s.to_string()).collect();
}

fn extract_data_from_range( range: &Range<Data> ) -> Result< HashMap<String, PlateSummary>, String> {
    if range.is_empty() {
        return Err("No data found in the provided range".to_string());
    }

    let mut data_map: HashMap<String, PlateSummary> = HashMap::new();

    for row in range.rows().skip(1){
        // column ( row.get() ) starts at 0
        // plate [2], category [3], exit date/time [6], exit locality [7], cost [9]

        let plate = data_type_to_string_value( row.get(2)).unwrap_or("Unknown".to_string());
        let category = data_type_to_string_value(row.get(3)).unwrap_or("Unkown".to_string());
        let date = data_type_to_string_value(row.get(6)).unwrap_or("".to_string());
        let description = data_type_to_string_value(row.get(7)).unwrap_or("".to_string());
        let cost = data_type_to_float_value( row.get(9)).unwrap_or(0.0);
        
        let entry = TollEntry{
            date,
            description,
            category,
            cost
        };
        
        let summary = data_map.entry( plate.clone()).or_insert( PlateSummary {
            plate,
            total_cost: 0.0,
            entries: Vec::new(),
        });

        summary.total_cost += entry.cost;
        summary.entries.push( entry);

    }

    if data_map.is_empty() {
        return Err("No valid data extracted from the provided range".to_string());
    }

    Ok( data_map)
}

fn open_excel_workbook( file_path: &str) -> Result< Range<Data>, String> {
    //Open excel file
    let mut workbook = open_workbook_auto( file_path)
        .map_err(|e| format!("Failed to open file: {}", e))?;

    //Get the first sheet
    let sheet_name = workbook.sheet_names()
        .get(0)
        .ok_or("No sheets found in the workbook")?
        .clone();

    let range = workbook.worksheet_range(&sheet_name)
        .map_err( |e| format!("Error reading sheet: {}", e))?;

    Ok( range)
}

fn data_type_to_date_string( excel_cell: Option<&Data>) -> String {
    match excel_cell {
        Some( Data::Float( f)) => {
            let base_date = NaiveDate::from_ymd_opt(1899, 12, 30).unwrap();
            let days = *f as i64;//gets the total days since base date
            let seconds = ((*f - days as f64) * 86400.0).round() as i64;//gets the fractional part of the day that as passed with second precision

            let date_time = base_date.and_hms_opt(0, 0, 0).unwrap();//adds time to base date ( midinight)
            let final_date = date_time + TimeDelta::days(days) + TimeDelta::seconds(seconds);//gets base date and adds days and seconds on top of it
            return final_date.format("%Y-%m-%d %H:%M:%S").to_string();//simply formats the

        }
        Some( Data::DateTime(dt)) => dt.to_string(),
        Some( Data::String(s)) => s.clone(),
        _ => "".to_string(),
    }
}

fn data_type_to_string_value( excel_cell: Option<&Data>) -> Option<String> {
    excel_cell.and_then(|c| c.get_string().map(|s| s.to_string()))
        .or_else( || excel_cell.and_then( |c| Some( c.to_string() )))
}

fn data_type_to_float_value( excel_cell: Option<&Data>) -> Option<f64> {
    excel_cell.and_then(|c| c.get_float())
}
