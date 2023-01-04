use ptree::*;
use clap::Parser;
use glob::Pattern;
use ptree::item::StringItem;
use walkdir::WalkDir;

use std::collections::HashMap;
use std::fmt::{Display, Formatter};
use std::fs::Metadata;


/// A simple tree program clone.
#[derive(Parser)]
#[command(name = "rtree")]
#[command(author = "Logan D. <logan.h.g.davis@icloud.com")]
#[command(version = "0.0.1")]
#[command(about = "A simple tree clone.", long_about = None)]
struct Cli {
    /// Name of the person to greet
    #[arg(short, long, default_value=".")]
    root_path: String,

    #[arg(short, long)]
    pattern: Option<String>
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
            FsMapNode::_spelunker(
                &mut place.contents.get_mut(path[index]).expect("Can't index to path"), //todo make better
                path,
                index + 1,
                item
            )
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

fn is_match_or_parent(path: &str, matches: &Vec<String>) -> bool {
    matches.iter().any(|i| i.contains(path))
}

fn main() -> Result<(), Box<dyn std::error::Error>>{ // figure out if better way
    // Parse cli args.
    let cli = Cli::parse();



    let mut matched_files: Vec<String> = Vec::new();
    if let Some(pattern) = cli.pattern {

        let pattern = Pattern::new(&pattern).unwrap();
        let walker = WalkDir::new(&cli.root_path)
            .into_iter()
            .filter_map(|entry| entry.ok());

        for item in walker {
            let path = item.path().to_str().unwrap();
            if pattern.matches(&path) {
                matched_files.push(path.to_string());
            }
        }
    }


    // find all files on their relative paths.
    let mut walker = WalkDir::new(&cli.root_path)
        .into_iter()
        .filter_map(|entry| entry.ok());

    println!("matched files = {:?}", matched_files);

    // create the map's root.
    let first = walker.next().expect("No files found!");
    let entry_meta = first.metadata().unwrap();
    let root = FsItem{
        name: first.file_name()
            .to_os_string()
            .into_string()
            .unwrap(),
        metadata: entry_meta
    };

    let mut fs_root = FsMapNode::new(root);

    // Create all the roots childern.

    while let Some(entry) = walker.next() {
        let entry_meta: Metadata = entry.metadata().unwrap();
        let path = entry.path().to_str().unwrap();

        if !matched_files.is_empty() && !is_match_or_parent(path, &matched_files) {
            continue
        }

        let item =  FsItem{
            name: entry.file_name()
                .to_os_string()
                .into_string()
                .unwrap(),
            metadata: entry_meta
        };
        fs_root.insert_at_path(path, item) ;
    };

    // Display the tree.
    fs_root.display_tree();
    Ok(())

}
