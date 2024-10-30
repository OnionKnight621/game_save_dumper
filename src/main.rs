use std::io;
use std::fs;
use std::path::Path;
use std::thread;
use std::time::Duration;
use chrono::Local;

fn main() {
    // ask for folder with game saves
    println!("Enter folder path with game saves (like: S:\\games\\save):");
    let save_source_folder_path = read_input();

    // check if file exists
    if !Path::new(&save_source_folder_path).exists() {
        println!("Folder with game saves not found!");
        return;
    }

    println!("Enter saves destination folder path (like: C:\\Desktop\\save_dumps):");
    let save_destination_folder_path = read_input();

    // create destination folder if not exists
    if !Path::new(&save_destination_folder_path).exists() {
        fs::create_dir_all(&save_destination_folder_path).expect("Failed to create destination folder!");
    }

    println!("Enter interval (like: 10):");
    let dump_interval_minutes = read_input();
    let dump_interval = if dump_interval_minutes.is_empty() {
        10
    } else {
        dump_interval_minutes.trim().parse::<u64>().unwrap_or(10)
    };

    println!("Max dumps count (like: 10):");
    let max_dumps_count = read_input();
    let max_dumps = if max_dumps_count.is_empty() {
        10
    } else {
        max_dumps_count.trim().parse::<u64>().unwrap_or(10)
    };

    println!("Dumping saves every {} minutes, max {} dumps", dump_interval, max_dumps_count);
    
    loop {
        if let Err(e) = copy_save_file(&save_source_folder_path, &save_destination_folder_path) {
            println!("Failed to copy save file: {}", e);
        }

        if let Err(e) = manage_dumps(&save_destination_folder_path, max_dumps) {
            println!("Failed to manage dumps: {}", e);
        }

        thread::sleep(Duration::from_secs(dump_interval * 60));
    }
}

fn read_input() -> String {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    input.trim().to_string()
}

fn copy_save_file(save_path: &str, dump_folder: &str) -> Result<(), std::io::Error> {
    let timestamp = Local::now().format("%Y%m%d%H%M%S");
    let save_file_name = Path::new(save_path).file_name().unwrap();
    let new_file_name = format!("{}_{}", timestamp, save_file_name.to_string_lossy());
    let new_file_path = Path::new(dump_folder).join(new_file_name);
    fs::copy(save_path, new_file_path)?;
    Ok(())
}

fn manage_dumps(dump_folder: &str, max_dumps: u64) -> Result<(), std::io::Error> {
    let mut entries: Vec<_> = fs::read_dir(dump_folder)?
        .filter_map(Result::ok)
        .filter(|e| e.path().is_file())
        .collect();

    // Сортуємо файли за часом створення
    entries.sort_by_key(|e| fs::metadata(e.path()).unwrap().created().unwrap());

    // Якщо кількість файлів перевищує максимальну, видаляємо найстаріші
    while entries.len() > max_dumps.try_into().unwrap() {
        if let Some(entry) = entries.first() {
            fs::remove_file(entry.path())?;
            entries.remove(0);
        }
    }
    Ok(())
}

