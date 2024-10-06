use std::fs;
use std::io::{self, Cursor, Read, stdin};
use flate2::read::GzDecoder;
use chrono::Local;
use colored::*;

#[cfg(target_os = "windows")]
fn set_console_title(title: &str) {
    use std::ffi::CString;
    use winapi::um::wincon::SetConsoleTitleA;

    let c_title = CString::new(title).unwrap();
    unsafe { SetConsoleTitleA(c_title.as_ptr()) };
}

#[cfg(not(target_os = "windows"))]
fn set_console_title(title: &str) {
    println!("\x1b]0;{}\x07", title);
}

fn decompress_gz_file(input_path: &str) -> io::Result<Vec<u8>> {
    let file = fs::File::open(input_path)?;
    let mut reader = GzDecoder::new(file);
    let mut buffer = Vec::new();
    reader.read_to_end(&mut buffer)?;
    Ok(buffer)
}

fn get_hex(file_buffer: &[u8], offset: usize) -> String {
    let slice = &file_buffer[offset..offset + 2];
    hex::encode(slice).to_uppercase()
}

fn convert_dds_file(input_buffer: &[u8]) -> Vec<u8> {
    let x = get_hex(input_buffer, 6);
    let y = get_hex(input_buffer, 4);

    let raw_hex = format!("444453207C00000007100200{}0000{}000000000000000000000100000000000000000000000000000000000000000000000000000000000000000000000000000000000000000000002000000004000000445831302000000000FF00000000FF00000000FFFF00000000100000000000000000000000000000000000006300000003000000000000000100000000000000", x, y);

    let hex_buffer = hex::decode(raw_hex).unwrap();

    let mut new_data = hex_buffer.clone();
    new_data.extend_from_slice(&input_buffer[12..]);

    new_data
}

fn convert_png_file(input_path: Vec<u8>, output_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let dds = ddsfile::Dds::read(Cursor::new(&input_path)).unwrap();
    let image = image_dds::image_from_dds(&dds, 0).unwrap();

    image.save(output_path)?;
    println!("{} {}", "[SUCCESS]".bright_green(), output_path);
    Ok(())
}

fn wait_for_keypress() {
    println!("{} Press enter to exit...", "[INFO]".bright_blue());
    let _ = stdin().read_line(&mut String::new());
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Set console title
    set_console_title("ShopImage Converter");

    let input_dir = "./input";
    let output_dir = "./output";

    // Create input directory if it doesn't exist
    fs::create_dir_all(input_dir)?;

    // Create output directory if it doesn't exist
    fs::create_dir_all(output_dir)?;

    // Formatting file name
    let filename: String = Local::now().format("%Y-%m-%d").to_string();
    let mut entries = fs::read_dir(input_dir)?;

    // Check if input directory is empty
    if entries.next().is_none() {
        println!("{} No files found. Copy directly from %localappdata%/Blizzard Entertainment/Overwatch/ShopImages", "[WARNING]".bright_yellow());

        // Wait for user input before exiting
        wait_for_keypress();

        return Ok(());
    }
    else {
        for (index, entry) in entries.enumerate() {
            let file_path = entry?.path();
            let input_path = file_path.to_str().unwrap();
    
            let data = decompress_gz_file(input_path)?;
            let dds = convert_dds_file(&data);
    
            let output_filename = format!("{}/{}_{}.png", output_dir, filename, index);
            
            if let Err(_err) = convert_png_file(dds, &output_filename) {
                println!("{} Failed to convert PNG: {}", "[ERROR]".bright_red(), _err);
            }
        }
    
        // Wait for user input before exiting
        wait_for_keypress();
    
        Ok(())
    }
}
