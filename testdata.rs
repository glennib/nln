#!/usr/bin/env -S cargo +nightly -Zscript
//! This is a cargo script for generating test data for benchmarking.
//!
//! Run with `./testdata.rs` (requires nightly)
//!
//! See https://rust-lang.github.io/rfcs/3424-cargo-script.html
//!
//! ```cargo
//! [package]
//! edition = "2024"
//! [profile.dev]
//! opt-level = 3
//! ```

use std::fs::{self, File};
use std::io::{self, Write};
use std::path::Path;
use std::thread::spawn;

#[derive(Debug, Clone, Copy)]
enum Size {
    Small,
    Medium,
    Large,
    Huge,
}

impl Size {
    fn as_str(&self) -> &'static str {
        match self {
            Size::Small => "small",
            Size::Medium => "medium",
            Size::Large => "large",
            Size::Huge => "huge",
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum FileVariant {
    NoTrailing,
    ManyTrailing,
    ManyTrailingThenContent,
}

impl FileVariant {
    fn as_str(&self) -> &'static str {
        match self {
            FileVariant::NoTrailing => "no_trailing",
            FileVariant::ManyTrailing => "many_trailing",
            FileVariant::ManyTrailingThenContent => "many_trailing_then_content",
        }
    }
}

struct SizeConfig {
    trailing_count: usize,
    line_count: usize,
}

impl Size {
    fn config(&self) -> SizeConfig {
        match self {
            Size::Small => SizeConfig {
                trailing_count: 1000,
                line_count: 0,
            },
            Size::Medium => SizeConfig {
                trailing_count: 2000,
                line_count: 0,
            },
            Size::Large => SizeConfig {
                trailing_count: 10000,
                line_count: 10000,
            },
            Size::Huge => SizeConfig {
                trailing_count: 1000000,
                line_count: 15000000,
            },
        }
    }
}

fn generate_newlines(writer: &mut impl Write, count: usize) -> io::Result<()> {
    for _ in 0..count {
        writer.write_all(b"\n")?;
    }
    Ok(())
}

fn generate_file(size: Size, variant: FileVariant, output_path: &Path) -> io::Result<()> {
    let config = size.config();
    let mut file = File::create(output_path)?;

    // Generate Lorem ipsum lines
    for i in 1..=config.line_count {
        writeln!(
            file,
            "Line {}: Lorem ipsum dolor sit amet, consectetur adipiscing elit",
            i
        )?;
    }

    // Write final content (no newline)
    file.write_all(b"Last line")?;

    // Handle trailing newlines based on variant
    match variant {
        FileVariant::ManyTrailing => {
            generate_newlines(&mut file, config.trailing_count)?;
        }
        FileVariant::ManyTrailingThenContent => {
            generate_newlines(&mut file, config.trailing_count)?;
            file.write_all(b"Final content")?;
        }
        FileVariant::NoTrailing => {}
    }

    Ok(())
}

fn generate_for_size(size: Size, testdata_dir: &Path) -> io::Result<()> {
    println!("Generating {} files...", size.as_str());

    let threads: Vec<_> = [
        FileVariant::NoTrailing,
        FileVariant::ManyTrailing,
        FileVariant::ManyTrailingThenContent,
    ]
    .into_iter()
    .map(|variant| {
        let filename = format!("{}_{}.txt", size.as_str(), variant.as_str());
        let path = testdata_dir.join(filename);
        spawn(move || generate_file(size, variant, &path))
    })
    .collect();
    for thread in threads {
        thread.join().unwrap()?;
    }
    Ok(())
}

fn main() -> io::Result<()> {
    let testdata_dir = Path::new("testdata");

    // Create testdata directory if it doesn't exist
    fs::create_dir_all(testdata_dir)?;

    println!("Generating test data files in testdata/...\n");

    // Edge cases
    println!("Generating edge case files...");

    // File with only newlines
    let mut file = File::create(testdata_dir.join("only_newlines.txt"))?;
    generate_newlines(&mut file, 5000)?;

    // Mixed line endings (CRLF and LF)
    let mut file = File::create(testdata_dir.join("mixed_line_endings.txt"))?;
    file.write_all(b"Line 1\r\n")?;
    file.write_all(b"Line 2\n")?;
    file.write_all(b"Line 3\r\n")?;
    generate_newlines(&mut file, 100)?;
    file.write_all(b"Line 4\n")?;
    for _ in 0..100 {
        file.write_all(b"\r\n")?;
    }

    // Empty file
    File::create(testdata_dir.join("empty.txt"))?;

    // Generate all size files
    for size in [Size::Small, Size::Medium, Size::Large, Size::Huge] {
        generate_for_size(size, testdata_dir)?;
    }

    println!("\nTest data files generated!");
    println!("\nFile sizes:");

    // Display file sizes
    let entries = fs::read_dir(testdata_dir)?;
    let mut files: Vec<_> = entries
        .filter_map(|entry| entry.ok())
        .filter(|entry| entry.path().is_file())
        .collect();

    files.sort_by_key(|entry| entry.file_name());

    for entry in files {
        let metadata = entry.metadata()?;
        let size = metadata.len();
        let size_str = if size > 1_000_000_000 {
            format!("{:.1}G", size as f64 / 1_000_000_000.0)
        } else if size > 1_000_000 {
            format!("{:.1}M", size as f64 / 1_000_000.0)
        } else if size > 1_000 {
            format!("{:.1}K", size as f64 / 1_000.0)
        } else {
            format!("{}", size)
        };
        println!("{:>8}  {}", size_str, entry.file_name().to_string_lossy());
    }

    Ok(())
}
