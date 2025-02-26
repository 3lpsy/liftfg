use rayon::prelude::*;
use std::collections::BTreeMap;
use std::collections::HashSet;
use std::env;
use std::fs::{self, File};
use std::io::{self, Write};
use std::path::{Path, PathBuf};
use std::process::Command;
use std::sync::Mutex;

// scuffed doc generator for AI doom loop

const DEFAULT_FILTERS: [&str; 7] = [
    "dioxus",
    "seaorm",
    "sqlx",
    "serde",
    "validator",
    "tauri",
    "fg",
];

fn main() -> io::Result<()> {
    println!("[*] Generating markdown");
    let filters = get_filters_from_args();
    let filter_fn = filters.join("-");
    println!("[*] Filters: {:?}", filters);
    let project_root = find_project_root()?;
    println!("[*] Project root: {}", project_root.display());
    let deps = get_all_dependencies(&project_root)?;
    let filtered_deps = filter_dependencies(deps, &filters);
    println!("[*] Found {} filtered dependencies", filtered_deps.len());

    if filtered_deps.is_empty() {
        println!("[!] No dependencies matched the filters.");
        return Ok(());
    }

    // Create temp directory before parallel processing
    let temp_dir = project_root.join("target/temp-docs");
    fs::create_dir_all(&temp_dir)?;

    // First, vendor all dependencies at once
    let mut cmd = Command::new("cargo");
    cmd.current_dir(&project_root).args(&[
        "vendor",
        "--versioned-dirs",
        &format!("{}", temp_dir.display()),
    ]);
    println!("[*] Downloading dependency sources");
    cmd.output()?;

    // Use a Mutex to collect docs from all dependencies safely in parallel
    let all_docs = Mutex::new(String::new());
    all_docs.lock().unwrap().push_str("\n# Documentation\n\n");

    // Process dependencies in parallel
    filtered_deps.par_iter().for_each(|dep| {
        let dep_docs = match generate_full_dependency_docs(dep, &temp_dir) {
            Ok(docs) => {
                format!("{}\n\n---\n\n", docs)
            }
            Err(e) => {
                let parts: Vec<&str> = dep.split(':').collect();
                let name = parts[0];
                println!("[!] Error generating docs for {}: {}", name, e);
                format!(
                    "## {}\n\n**Error generating documentation:** {}\n\n---\n\n",
                    name, e
                )
            }
        };

        // Safely append to the all_docs string
        if let Ok(mut docs) = all_docs.lock() {
            docs.push_str(&dep_docs);
        }
    });

    let output_path = project_root.join(format!("target/{}.md", filter_fn));

    if let Some(parent) = output_path.parent() {
        fs::create_dir_all(parent)?;
    }

    let mut file = File::create(&output_path)?;
    file.write_all(all_docs.lock().unwrap().as_bytes())?;

    println!(
        "[*] Documentation markdown generated at: {}",
        output_path.display()
    );

    Ok(())
}

fn get_filters_from_args() -> Vec<String> {
    // Check if any command line arguments were provided
    let args: Vec<String> = env::args().skip(1).collect();

    if args.is_empty() {
        // Use default filters
        DEFAULT_FILTERS.iter().map(|&s| s.to_string()).collect()
    } else {
        // Use provided filters
        args
    }
}

fn filter_dependencies(deps: HashSet<String>, filters: &[String]) -> Vec<String> {
    // Filter and sort dependencies
    let mut filtered_deps: Vec<String> = deps
        .into_iter()
        .filter(|dep| {
            let name = dep.split(':').next().unwrap_or(""); // Get name part before version
            filters.iter().any(|filter| name.starts_with(filter))
        })
        .collect();

    filtered_deps.sort();
    filtered_deps
}

fn find_project_root() -> io::Result<PathBuf> {
    // Start from current directory and walk up until we find a Cargo.toml
    let mut current_dir = env::current_dir()?;

    loop {
        let cargo_toml_path = current_dir.join("Cargo.toml");
        if cargo_toml_path.exists() {
            return Ok(current_dir);
        }

        // Move up one directory
        if !current_dir.pop() {
            break;
        }
    }

    // If we didn't find a Cargo.toml, use the current directory
    Ok(env::current_dir()?)
}

fn get_all_dependencies(project_root: &Path) -> io::Result<HashSet<String>> {
    let mut dependencies = HashSet::new();

    // Run cargo metadata to get structured info about all dependencies
    let output = Command::new("cargo")
        .current_dir(project_root)
        .args(&["metadata", "--format-version=1"])
        .output()?;

    if !output.status.success() {
        return Err(io::Error::new(
            io::ErrorKind::Other,
            format!(
                "cargo metadata failed: {}",
                String::from_utf8_lossy(&output.stderr)
            ),
        ));
    }

    // Parse the JSON output
    let metadata: serde_json::Value = serde_json::from_slice(&output.stdout)
        .map_err(|e| io::Error::new(io::ErrorKind::InvalidData, e))?;

    // Extract package names and versions
    if let Some(packages) = metadata["packages"].as_array() {
        for package in packages {
            if let (Some(name), Some(version)) =
                (package["name"].as_str(), package["version"].as_str())
            {
                // Skip workspace packages - we only want external dependencies
                dependencies.insert(format!("{}:{}", name, version));
            }
        }
    }

    Ok(dependencies)
}

fn generate_full_dependency_docs(dep: &str, temp_dir: &Path) -> io::Result<String> {
    let parts: Vec<&str> = dep.split(':').collect();
    let name = parts[0];
    let version = parts[1];

    println!(
        "[*] Parsing documentation from source for {} v{}",
        name, version
    );

    let mut docs = String::new();
    docs.push_str(&format!("## {}\n\n", name));
    docs.push_str(&format!("**Version:** {}\n\n", version));

    // Source code should already be vendored by the main function
    let dep_dir = temp_dir.join(format!("{}-{}", name, version));

    // Verify that the directory exists
    if !dep_dir.exists() {
        return Err(io::Error::new(
            io::ErrorKind::NotFound,
            format!("Dependency directory not found: {}", dep_dir.display()),
        ));
    }

    let api_docs = extract_api_docs(&dep_dir)?;

    if !api_docs.is_empty() {
        docs.push_str("### API Documentation\n\n");
        docs.push_str(&api_docs);
    }

    Ok(docs)
}

fn extract_api_docs(dep_dir: &Path) -> io::Result<String> {
    let mut docs = String::new();
    // Try to look at the source code to extract doc comments
    let src_dir = dep_dir.join("src");
    if !src_dir.exists() {
        println!(
            "[!] No src at file: {}",
            dep_dir.to_string_lossy().to_string()
        );
        return Ok(docs);
    }

    let mut rust_files = Vec::new();
    collect_rust_files(&src_dir, &mut rust_files)?;
    let file_docs: Mutex<BTreeMap<String, String>> = Mutex::new(BTreeMap::new());

    // Process files in parallel
    rust_files.par_iter().for_each(|file| {
        if let Ok(content) = fs::read_to_string(file) {
            let rel_path = file.strip_prefix(dep_dir).unwrap_or(file);
            let path_str = rel_path.to_string_lossy().to_string();

            let mut file_doc = String::new();
            file_doc.push_str(&format!("#### File: {}\n\n", rel_path.display()));
            // Extract documentation from this file
            file_doc.push_str("```rust\n");

            for line in content.lines() {
                let trimmed = line.trim();
                // Check for public item definitions
                let is_pub_item = trimmed.starts_with("pub fn ")
                    || trimmed.starts_with("pub struct ")
                    || trimmed.starts_with("pub enum ")
                    || trimmed.starts_with("pub trait ")
                    || trimmed.starts_with("pub type ");

                let is_item = is_pub_item
                    || trimmed.starts_with("fn ")
                    || trimmed.starts_with("struct ")
                    || trimmed.starts_with("enum ")
                    || trimmed.starts_with("trait ");
                let mut trimmed = trimmed.to_string();
                if trimmed.ends_with("{") {
                    trimmed.push_str("...}");
                }
                if trimmed.ends_with("(") {
                    trimmed.push_str("...)");
                }
                if is_item || is_pub_item {
                    file_doc.push_str(&format!("{}\n", trimmed));
                }
            }
            file_doc.push_str("```\n\n");
            // Store the file documentation
            if let Ok(mut docs_map) = file_docs.lock() {
                docs_map.insert(path_str, file_doc);
            }
        }
    });

    // Combine all documentation in order
    if let Ok(docs_map) = file_docs.lock() {
        for (_, file_doc) in docs_map.iter() {
            docs.push_str(file_doc);
        }
    }

    Ok(docs)
}

fn collect_rust_files(dir: &Path, files: &mut Vec<PathBuf>) -> io::Result<()> {
    if dir.is_dir() {
        for entry in fs::read_dir(dir)? {
            let entry = entry?;
            let path = entry.path();

            if path.is_dir() {
                collect_rust_files(&path, files)?;
            } else if let Some(extension) = path.extension() {
                if extension == "rs" {
                    files.push(path);
                }
            }
        }
    }

    Ok(())
}
