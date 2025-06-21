use std::env;
enum FileSize {
    Bytes(u64),
    Kilobytes(u64),
    Megabytes(u64),
    Gigabytes(u64),
}
// Implement FileSize trait for FileSize enum
impl ToString for FileSize {
    fn to_string(&self) -> String {
        match self {
            FileSize::Bytes(bytes) => format!("{} bytes", bytes),
            FileSize::Kilobytes(kb) => format!("{} kilobytes", kb),
            FileSize::Megabytes(mb) => format!("{} megabytes", mb),
            FileSize::Gigabytes(gb) => format!("{} gigabytes", gb),
        }
    }
}

#[derive(Debug)]
struct FileSizeFormatter {
    //size: u64,
}

impl FileSizeFormatter {

    fn format_size(size: u64) -> String {

        let bytes_str = FileSize::Bytes(size).to_string();
        let kb_str = FileSize::Kilobytes(size / 1000).to_string();
        let mb_bytes = FileSize::Megabytes(size / 1_000_000).to_string();
        let gb_bytes = FileSize::Gigabytes(size / 1_000_000_000).to_string();
        format!("Sizes {{ bytes: \"{}\" kilobytes: \"{}\"  megabytes: \"{}\"  gigabytes: \"{}\"}}", 
                bytes_str, kb_str, mb_bytes, gb_bytes) 
    }
}



fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        panic!("Invalid input format. Expected format: '<size> <unit>'");
    }
    let string_size = args[1].clone();
    // string_size has the format of "24 mb"
    // Split the string into two parts
    let parts: Vec<&str> = string_size.split_whitespace().collect();
    // The first part is the size, and the second part is the unit
    let size: u64 = parts[0].parse().unwrap();
    let unit = parts[1].to_lowercase();
    // Convert the size to bytes based on the unit
    let size_in_bytes = match unit.as_str() {
        "bytes" | "b" => size,
        "kilobytes" | "kb" => size * 1_000,
        "megabytes" | "mb" => size * 1_000_000,
        "gigabytes" | "gb" => size * 1_000_000_000,
        _ => panic!("Unknown unit: {}", unit),
    };

    let result = FileSizeFormatter::format_size(size_in_bytes);
    println!("{}", result)
}