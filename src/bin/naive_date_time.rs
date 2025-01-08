use chrono::NaiveDateTime;

fn main() {
    let args = vec!["program", "2025-01-07"]; // サンプル引数
    let native_start_date = NaiveDateTime::parse_from_str(
        &format!("{} 00:00:00", args[1]),
        "%Y-%m-%d %H:%M:%S",
    );
    
    match native_start_date {
        Ok(date) => println!("Parsed date: {}", date),
        Err(e) => eprintln!("Error parsing date: {}", e),
    }
}
