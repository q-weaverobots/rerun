use rerun::datatypes::Uuid;
use std::env;
use std::fs;
use std::net::ToSocketAddrs;
use std::path::Path;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        eprintln!("Usage: custom_callback_app <path_to_directory>");
        std::process::exit(1);
    }
    let dir_path = &args[1];

    if !Path::new(dir_path).is_dir() {
        eprintln!("Error: '{}' is not a valid directory.", dir_path);
        std::process::exit(1);
    }

    // Generate a new store ID to clear the memory
    let rec = rerun::RecordingStreamBuilder::new("rerun_example_custom_callback")
        .recording_id("weave") // NEW STORE ID
        .connect_tcp_opts("127.0.0.1:9877".to_socket_addrs()?.next().unwrap(), None)?;

    // Load .rrd files
    let rrd_files: Vec<_> = fs::read_dir(dir_path)?
        .filter_map(Result::ok)
        .filter(|entry| entry.path().extension().map_or(false, |ext| ext == "rrd"))
        .map(|entry| entry.path())
        .collect();

    if rrd_files.is_empty() {
        eprintln!("No .rrd files found in '{}'.", dir_path);
        std::process::exit(1);
    }

    for rrd_path in &rrd_files {
        println!("Loading file: {:?}", rrd_path);
        rec.log_file_from_path(rrd_path, None, false)?;
    }

    println!(
        "Successfully loaded all .rrd files from '{}' into the Rerun viewer.",
        dir_path
    );

    Ok(())
}
