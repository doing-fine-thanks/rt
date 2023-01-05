use ptree::*;
use clap::Parser;
use glob::Pattern;
use ptree::item::StringItem;
use walkdir::{DirEntry, WalkDir};

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::Metadata;

/// For globs
const DEFAULT_PATTERN: &str = "";

/// A simple tree program clone.
#[derive(Parser)]
#[command(name = "rtree")]
#[command(author = "Logan D. <logan.h.g.davis@icloud.com")]
#[command(version = "0.0.1")]
#[command(about = "A simple tree clone.", long_about = None)]
struct Cli {
    /// The root of the directory tree to display.
    #[arg(short, long, default_value=".")]
    root_path: String,

    /// a glob pattern to use for finding files.
    #[arg(short, long)]
    pattern: Option<String>,

    /// a glob pattern for excluding results.
    #[arg(short, long)]
    exclude: Option<String>
}

/// A container for File System items.
/// Whether it is a file or a directory
/// can be found in the metadata field.
#[derive(Debug)]
struct FsItem {
    name: String,
    metadata: Metadata
}


impl Display for FsItem {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", &self.name)
    }
}

impl Clone for FsItem {
    fn clone(&self) -> Self {
        FsItem {
            name: self.name.clone(),
            metadata: self.metadata.clone()
        }
    }
}

/// A FsItem tree structure.
#[derive(Debug)]
struct FsMapNode {
    fs_item: FsItem,
    contents: HashMap<String, FsMapNode>
}


impl FsMapNode {
    fn new(fs_item: FsItem) -> Self {
        FsMapNode {
            fs_item,
            contents: HashMap::new()
        }
    }

    /// Insert child FsItem (IE, a file/subdir to a given directory.
    fn insert_child(&mut self, item: FsItem) {
        self.contents.insert(item.name.clone(), FsMapNode::new(item));
    }

    ///Places item at a path in an FsMapNode pass as seperated segments.
    fn _spelunker(place: &mut FsMapNode, path: &Vec<&str>, index: usize, item: FsItem) {
        if path.len() -1 == index {
            place.insert_child(item.clone());
        } else {
            if let Some(mut map_elem) = place.contents.get_mut(path[index]) {
                FsMapNode::_spelunker(
                    &mut map_elem,
                    path,
                    index + 1,
                    item
                )
            } // log orphaned files?
        }
    }

    /// Insert an FsItem into a FsNodeMap at a given path.
    fn insert_at_path(&mut self, path: &str, item: FsItem) {
        let segments = path.split("/").collect::<Vec<&str>>();
        FsMapNode::_spelunker(
            self,
            &segments,
            1,
            item
        )
    }

    /// Create a tree to display with Ptree.
    fn create_display_tree(&self) -> StringItem {
        StringItem {
            text: self.fs_item.to_string(),
            children: {
                 self.contents.iter().map(|i| i.1.create_display_tree()).collect::<Vec<StringItem>>()
            }
        }
    }

    /// Display FsNodeMap using Ptree.
    fn display_tree(&self) {
        print_tree(&self.create_display_tree()).expect("Unable to print tree!");
    }
}

/// Checks to see if a path is in a list of paths or a sub path.
/// Technically would produce a false positive if the subpath
/// started after the root dir (ie '/src/t/e'.contians('t/e') -> true).
/// but I dont feel like fixing that right now.
fn is_match_or_parent(path: &str, matches: &Vec<&str>) -> bool {
    matches.iter().any(|i| i.contains(path))
}

/// Takes a pattern and list of DirEntrys' and returns
/// the set of paths in those entries that match the pattern.
fn find_matches<'a>(pattern: &Pattern, entries: &'a Vec<DirEntry>) -> Vec<&'a str> {
    let mut matched_files: Vec<&str> = Vec::new();

    for item in entries {
        let path = item.path().to_str().unwrap();
        if pattern.matches(&path) {
            matched_files.push(path);
        }
    }

    matched_files
}


fn main() -> Result<(), Box<dyn std::error::Error>>{
    // Parse cli args.
    let cli = Cli::parse();

    // set up patterns.
    let (match_pattern, find_specific_pattern) = match cli.pattern {
        Some(pattern) => (Pattern::new(&pattern)?, true),
        _ => (Pattern::new(DEFAULT_PATTERN)?, false)
    };

    let (exclude_pattern, exclude_specific_pattern) = match cli.exclude {
        Some(exclude) => (Pattern::new(&exclude)?, true),
        _ => (Pattern::new(DEFAULT_PATTERN)?, false)
    };


    let entries: Vec<DirEntry> = WalkDir::new(&cli.root_path)
        .into_iter()
        .filter_map(|entry| entry.ok())
        .collect();

    // get match references.
    let matched_files = if find_specific_pattern {
        find_matches(&match_pattern, &entries)
    } else {
        Vec::new()
    };

    // make vec iterator for easier index management.
    let mut dir_iter = entries.iter();

    // create the map's root.
    let first = dir_iter.next().expect("No files found!");
    let entry_meta = first.metadata().expect("Unable to parse metadata for files!");

    let root = FsItem{
        name: first.file_name()
            .to_os_string()
            .into_string()
            .unwrap(),
        metadata: entry_meta
    };

    let mut fs_root = FsMapNode::new(root);

    // Create all the roots children.
    while let Some(entry) = dir_iter.next() {

        let entry_meta: Metadata = entry.metadata().expect("No files found!");
        let path = entry.path().to_str().expect("Unable to parse metadata for files!");

        // if we have a search pattern but it doesn't match:
        if !matched_files.is_empty() && !is_match_or_parent(path, &matched_files) {
            continue
        }

        // if it matches the exclusion pattern
        if exclude_specific_pattern && exclude_pattern.matches(path) {
            continue
        }

        fs_root.insert_at_path(
            path,
            FsItem{
                name: entry.file_name()
                    .to_os_string()
                    .into_string()
                    .unwrap(),
                metadata: entry_meta
            }
        ) ;
    };

    // Display the tree.
    fs_root.display_tree();
    Ok(())
}
