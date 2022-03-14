use dotenv::dotenv;
use std::env;

use ftp::FtpStream;
use std::fs;
use std::fs::File;
use std::io::Cursor;
use std::io::Read;
use std::path::PathBuf;
use std::time::SystemTime;

// Function to get the file path as string
fn get_file_path(file: PathBuf) -> String {
    let output = file.into_os_string().into_string().unwrap();
    output
}

// Function to get the correct filename
fn get_filename(file: &PathBuf) -> String {
    let prepare = file.file_name().unwrap();
    let output = prepare.to_os_string().into_string().unwrap();
    output
}

pub fn upload() {
    dotenv().expect("Failed to read .env file");

    // Create a connection to an FTP server and authenticate to it
    let mut ftp_stream =
        FtpStream::connect(&env::var("FTP_ADDRESS").expect("FTP_ADDRESS must be set")).unwrap();
    let _ = ftp_stream
        .login(
            &env::var("FTP_USER").expect("FTP_USER must be set"),
            &env::var("FTP_PASSWORD").expect("FTP_PASSWORD must be set"),
        )
        .unwrap();

    // Change into a new directory, relative to the one we are currently in
    let _ = ftp_stream
        .cwd(&env::var("DIRECTORY_TO_UPLOAD").expect("DIRECTORY_TO_UPLOAD must be set"))
        .unwrap();

    // Get the current directory that the client will be reading from and writing to
    println!("Current directory: {}", ftp_stream.pwd().unwrap());

    let mut recent_file = SystemTime::UNIX_EPOCH;
    let mut correct_file = PathBuf::new();
    // Find then stores (PUT) a file from the client to the holders working directory of the server
    for entry in fs::read_dir(".\\holders").unwrap() {
        // Time to campare the SystemTime if it changing have to get the correct path
        let recent_file_control = recent_file;
        let entry = entry.unwrap();
        let path = entry.path();
        // Get the metadata when the iterated file created
        let created = path.metadata().unwrap().created().unwrap();
        // If the iterated file created later that will be the recent file
        recent_file = recent_file.max(created);

        // Getting the path of the most recent file
        if recent_file != recent_file_control {
            correct_file = path;
        } else {
            ()
        }
    }
    // Don't change the order of this 2 variable, because the second have to consume it since PathBuf doesn't have Copy trait implemented
    let filename = get_filename(&correct_file);
    let file_to_send = get_file_path(correct_file);

    // Reading the file in
    let mut data = String::new();
    let mut file = File::open(&file_to_send).expect("Unable to open file");
    file.read_to_string(&mut data)
        .expect("Unable to read string");
    let mut reader = Cursor::new(data.as_bytes());

    // Sending the file to the FTP stream
    let _ = ftp_stream.put(&filename, &mut reader);
    println!("Successfully uploaded the file");

    // Terminate the connection to the server
    let _ = ftp_stream.quit();
}
