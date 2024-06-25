use std::fs;
use std::io::{BufRead, BufReader};
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use clap::Parser;

#[derive(Parser, Debug)]
#[command(version, about)]
struct Args {
    #[arg(short, long, default_value = "./")]
    directory: String,
}

fn main() {
    let args = Args::parse();
    do_converter(args.directory.as_str());
}

fn do_converter(dir: &str) {
    for file_path in list_video_files(dir).expect("fail to list video files") {
        println!("{}", file_path);
        let new_file_path = create_output_name(file_path.as_ref()).expect("fail to create output name");
        println!("{}", new_file_path);
        let args = &[
            "-i",
            file_path.as_ref(),
            "-c:v",
            "h264_videotoolbox",
            "-profile:v",
            "high",
            new_file_path.as_ref(),
        ];
        run_command("ffmpeg", args).expect("fail to do converter");
    }
}

fn list_video_files<P: AsRef<Path>>(path: P) -> Result<Vec<String>, std::io::Error> {
    let mut video_files = Vec::new();
    for entry in fs::read_dir(path)? {
        let entry = entry?;
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        if let Some(ext) = path.extension() {
            if ext != "mp4" {
                continue;
            }
            if let Some(filename) = path.to_str() {
                video_files.push(filename.to_string());
            }
        }
    }

    Ok(video_files)
}

fn run_command(command: &str, args: &[&str]) -> Result<(), std::io::Error> {
    let mut process = Command::new(command)
        .args(args)
        .stdout(Stdio::piped())
        .spawn()?;

    let stdout = process.stdout.take().expect("failed to open stdout");
    let buf = BufReader::new(stdout);

    for line in buf.lines() {
        println!("{}", line?)
    }

    let status = process.wait()?;

    println!("command exited with status: {}", status);

    Ok(())
}

fn create_output_name(input: &str) -> Result<String, std::io::Error> {
    let path = Path::new(input);

    let mut new_path = PathBuf::new();

    if let Some(parent) = path.parent() {
        new_path.push(parent);
    }

    if let Some(stem) = path.file_stem().and_then(|s| s.to_str()) {
        new_path.push(format!("{}-1", stem));
    }

    if let Some(ext) = path.extension() {
        new_path.set_extension(ext);
    }

    Ok(new_path.to_string_lossy().into_owned())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_files() {}

    #[test]
    fn test_run_command() {
        run_command("ls", &["-alh"]).unwrap();
    }
}
