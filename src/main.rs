use std::fs;
use std::io;
use std::io::prelude::*;
use std::cmp::Ordering;
use std::collections::BTreeSet;
use std::path::Path;

const CSV_FILENAME: &str = "kondo.csv";
const FILE_LOG_INTERVAL: u64 = 10_000;
const ONE_GB: f32 = 1_000_000_000.0;
const NONTRIVIAL_SIZE: u64 = 50_000_000;

#[derive(Eq, PartialEq, Debug)]
struct SizedPath {
    size: u64,
    path: String
}

impl Ord for SizedPath {
    fn cmp(&self, other: &SizedPath) -> Ordering {
        match self.size.cmp(&other.size) {
            Ordering::Equal => {
                self.path.cmp(&other.path)
            },
            // Flip the ordering so we're sorted descending by size.
            Ordering::Less => Ordering::Greater,
            Ordering::Greater => Ordering::Less
        }
    }
}

impl PartialOrd for SizedPath {
    fn partial_cmp(&self, other: &SizedPath) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

#[derive(Debug)]
struct Stats {
    nontrivial: BTreeSet<SizedPath>,
    num_files: u64,
    num_dirs: u64,
    num_errors: u64,
    total_size: u64
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            nontrivial: BTreeSet::new(),
            num_files: 0,
            num_dirs: 0,
            num_errors: 0,
            total_size: 0
        }
    }

    fn log_path(&mut self, path: &Path, size: u64) {
        if size >= NONTRIVIAL_SIZE {
            self.nontrivial.insert(SizedPath {
                size,
                path: String::from(path.to_string_lossy())
            });
        }
    }

    pub fn log_dir(&mut self, dir: &Path, size: u64) {
        self.num_dirs += 1;
        self.log_path(dir, size)
    }

    pub fn log_file(&mut self, file: &fs::DirEntry) -> u64 {
        let mut size = 0;
        self.num_files += 1;
        if let Ok(metadata) = file.metadata() {
            size = metadata.len();
            self.total_size += size;
            self.log_path(&file.path(), size);
        } else {
            self.num_errors += 1;
        }
        if self.num_files % FILE_LOG_INTERVAL == 0 {
            println!(
                "Found {} nontrivial files and directories in {:.2}GB.",
                self.nontrivial.len(),
                to_gb(self.total_size)
            );
        }
        size
    }

    pub fn write_csv(&self, path: &Path) -> io::Result<()> {
        println!("Writing {:?}.", path);
        let f = fs::File::create(path)?;
        let mut bw = io::BufWriter::new(f);
        for sp in self.nontrivial.iter() {
            let size = sp.size.to_string();
            bw.write(size.as_bytes())?;
            bw.write(b",")?;
            bw.write(sp.path.as_bytes())?;
            bw.write(b"\n")?;
        }
        Ok(())
    }
}

fn to_gb(size: u64) -> f32 {
    size as f32 / ONE_GB
}

fn iter_dir(path: &Path, stats: &mut Stats) -> io::Result<u64> {
    let mut size = 0;
    for entry_result in fs::read_dir(path)? {
        if let Ok(entry) = entry_result {
            let path = entry.path();
            if path.is_dir() {
                match iter_dir(&path, stats) {
                    Ok(dir_size) => {
                        size += dir_size;
                    },
                    Err(_) => {
                        // Access was denied or the file was busy or something.
                        stats.num_errors += 1;
                    }
                }
            } else {
                size += stats.log_file(&entry);
            }
        } else {
            stats.num_errors += 1;
        }
    }
    stats.log_dir(path, size);
    Ok(size)
}

fn run() -> io::Result<()> {
    let args: Vec<String> = std::env::args().collect();
    let root = match args.get(1) {
        Some(val) => Path::new(val),
        None => {
            println!("usage: {} <dirname>\n", args.get(0).unwrap());
            std::process::exit(1);
        }
    };
    let mut stats = Stats::new();
    iter_dir(&root, &mut stats)?;
    stats.write_csv(Path::new(CSV_FILENAME))?;
    Ok(())
}

fn main() {
    match run() {
        Ok(()) => {
            println!("Done.");
        },
        Err(e) => {
            println!("Error: {}", e);
            std::process::exit(1);
        }
    }
}
