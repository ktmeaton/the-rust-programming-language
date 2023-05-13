use std::collections::HashMap;
use std::io::{Error, ErrorKind};

//use log::debug;

use itertools::Itertools;
use eyre::Report;
use petgraph::graph::{Graph, NodeIndex};
use petgraph::dot::{Dot, Config};
use petgraph::visit::Dfs;
use petgraph::Direction;

use std::fs::File;
use std::io::Write;

use csv;

// serde
use serde;
use serde_json;

#[derive(serde::Deserialize, Debug)]
struct LineageNotesRow<'a> {
    lineage: &'a str,
    _description: &'a str,
}

#[derive(Debug)]
pub struct Phylogeny {
    pub graph: Graph<String, isize>,
    pub order: Vec<String>,
    pub lookup: HashMap<String, NodeIndex>,
}

impl Phylogeny {
    pub fn new() -> Self {
        Phylogeny { 
            graph: Graph::new(),
            order : Vec::new(),
            lookup: HashMap::new(), 
        }
    }

    pub fn build_graph(&mut self, dataset_name: &String, dataset_tag: &String, dataset_dir: &String)  -> Result<(), Report> {

        let mut graph_data: HashMap<String, Vec<String>> = HashMap::new();
        if dataset_name == "sars-cov-2" {
            if dataset_tag == "nightly" {
                (graph_data, self.order) = create_graph_data(dataset_dir).unwrap();
            }

        }

        // ------------------------------------------------------------------------
        // Construct Graph

        // Add root node
        let name = "root".to_string();
        let id = self.graph.add_node(name.clone());
        self.lookup.insert(name, id);

        // Add descendants
        for name in &self.order{
            let id = self.graph.add_node(name.clone());
            self.lookup.insert(name.clone(), id.clone());
            let parents = &graph_data[&name.clone()];
            for parent in parents {
                let parent_id = self.lookup[&parent.clone()];
                self.graph.add_edge(parent_id, id, 1);
            }
        
        }

        Ok(())
    }

    pub fn export_graph(&self, dataset_dir: &String)  -> Result<(), Report> {


        // Export graph to dot file
        let graph_path = format!("{}/graph.dot", dataset_dir);
        let mut graph_output = format!("{}", Dot::with_config(
            &self.graph, 
            &[Config::EdgeNoLabel]
        ));
        // Set horizontal
        graph_output = str::replace(&graph_output, "digraph {", "digraph {\n    rankdir=\"LR\";");

        let mut graph_file = File::create(graph_path).unwrap();    
        graph_file.write_all(&graph_output.as_bytes()).expect("Failed to write graph file.");

        Ok(())

    }

    pub fn get_descendants(&self, name: &String) -> Result<Vec<String>, Report> {

        let mut descendants = Vec::new();

        // Find the node that matches the name
        let node = self.get_node(name).expect(format!["Couldn't find node name in phylogeny: {}", name].as_str());
        // Construct a depth-first-search (Dfs)
        let mut dfs = Dfs::new(&self.graph, node);
        // Skip over self
        dfs.next(&self.graph);
        // Iterate over descendants
        while let Some(nx) = dfs.next(&self.graph) {
            // Get node name 
            let nx_name = self.get_name(&nx).unwrap();
            descendants.push(nx_name);
        }

        Ok(descendants)

    }

    pub fn get_ancestors(&mut self, name: &String) -> Option<Vec<Vec<String>>> {

        let mut ancestors = Vec::new();

        // Construct a backwards depth-first-search (Dfs)
        self.graph.reverse();
        let node = self.get_node(&name).unwrap();
        let mut dfs = Dfs::new(&self.graph, node);

        // Walk to the root, there might be multiple paths (recombinants)
        let mut path_nodes = Vec::new();
        let mut prev_name = String::new();

        // Skip self
        dfs.next(&self.graph);
        while let Some(nx) = dfs.next(&self.graph) {
            // Get node name 
            let nx_name = self.get_name(&nx).unwrap();

            // If the previous node name was root, that means we topped
            // out the search in the last iter, but still have alternate 
            // recombinant paths to deal with
            if prev_name == "root" {

                // Add the topped out path to our list of paths
                path_nodes.reverse();
                ancestors.push(path_nodes.clone());

                // Initialize vector for new paths
                //path_nodes = Vec::new();

                // Recursive search, swap graph back and forth
                self.graph.reverse();
                let nx_ancestors = self.get_ancestors(&nx_name).unwrap();
                for ancestor_nodes in &nx_ancestors {
                    ancestors.push(ancestor_nodes.clone());
                }
                self.graph.reverse();

            }

            path_nodes.push(nx_name.clone());

            prev_name = nx_name;

        }

        if prev_name == "root" {
            path_nodes.reverse();
            ancestors.push(path_nodes.clone());            
        }
   
        // Restore original graph order   

        self.graph.reverse();
        Some(ancestors)
    }

    pub fn get_common_ancestor(&mut self, name_1: &String, name_2: &String) -> Option<String> {

        let common_ancestor = "root".to_string();

        let mut name_1_ancestors = self.get_ancestors(name_1).unwrap();
        println!("name_1_ancestors: {:?}", name_1_ancestors);
        let mut name_2_ancestors = self.get_ancestors(name_2).unwrap();
        println!("name_2_ancestors: {:?}", name_2_ancestors); 

        // // Case 1, neither are recombinants
        // if name_1_ancestors.len() == 1  && name_2_ancestors.len() == 1{

        //     //name_1_ancestors = name_1_ancestors[0];
        //     // name_2_ancestors = name_2_ancestors[0];
            
        //     // let it = name_1_ancestors.iter().zip(name_2_ancestors.iter());

        //     // for (i, (n_1, n_2)) in it.enumerate() {
        //     //     println!("{}: ({:?}, {:?})", i, n_1, n_2);
        //     // }
        // }

        Some(common_ancestor)
    }

    pub fn get_node(&self, name: &String) -> Option<NodeIndex> {

        if self.lookup.contains_key(name) {
            let node = self.lookup[name];
            return Some(node)

        }

        None

    }

    pub fn get_name(&self, node: &NodeIndex) -> Option<String> {

        for (name, node_l) in &self.lookup {
            if node == node_l {
               return Some(name.clone())
            }
        }

        None

    }
}    


pub fn download_lineage_notes(dataset_dir: &String) -> Result<String, Report> {
    // https://raw.githubusercontent.com/cov-lineages/pango-designation/master/lineage_notes.txt
    let lineage_notes_path = format!("{}/lineage_notes.txt", dataset_dir);
    Ok(lineage_notes_path)
}

pub fn download_alias_key(dataset_dir: &String) -> Result<String, Report> {
    // https://raw.githubusercontent.com/cov-lineages/pango-designation/master/pango_designation/alias_key.json

    let alias_key_path = format!("{}/alias_key.json", dataset_dir);
    Ok(alias_key_path)
}

pub fn import_alias_key(dataset_dir: &String) -> Result<HashMap<String, Vec<String>>, Report> {

    let alias_key_path = download_alias_key(dataset_dir).expect("Couldn't download alias_key from url.");
    let alias_key_str = std::fs::read_to_string(alias_key_path).expect("Couldn't read alias_key file.");
    let alias_key_val: serde_json::Value = serde_json::from_str(&alias_key_str).expect("Couldn't convert alias_key to json");
    let alias_key_raw: serde_json::Map<String, serde_json::Value> = alias_key_val
        .as_object()
        .expect("Couldn't convert alias_key json to json Map")
        .clone();

    // This should probably be a custom deserializer, but I don't know how to do that yet
    let mut alias_key: HashMap<String, Vec<String>> = HashMap::new();

    for (alias, lineage) in &alias_key_raw{

        let mut lineage_paths: Vec<String> = Vec::new();

        // Consistify the alias key types
        match lineage.as_array(){
            // If array, this is a recombinant alias with multiple parents.            
            Some(parents) => {
                for parent in parents {
                    let parent = parent.as_str().expect("Couldn't convert parent to str.");
                    // Strip the wildcard asterisks from lineage name
                    let parent_clean = str::replace(parent, "*", "");
                    lineage_paths.push(parent_clean);
                }
            },
            // Otherwise, it might be a string
            None => {
                let mut lineage_path = lineage
                    .as_str()
                    .expect("Couldn't convert lineage to str.")
                    .to_string();
                // If there is not lineage_path (ex. "" for A, B), set to self
                if lineage_path == "" {
                    lineage_path = alias.clone();
                }
                lineage_paths.push(lineage_path);
            }
        }

        alias_key.insert(alias.clone(), lineage_paths);
    
    }

    Ok(alias_key)
}

pub fn create_graph_data(dataset_dir: &String) -> Result<(HashMap<String, Vec<String>>, Vec<String>), Report> {

    // ------------------------------------------------------------------------
    // Download and import data

    // Import the alias key
    let alias_key = import_alias_key(dataset_dir).unwrap();

    // Import the lineage notes
    let lineage_notes_path = download_lineage_notes(dataset_dir).expect("Couldn't download lineage notes from url.");
    let mut reader = csv::ReaderBuilder::new().delimiter(b'\t').from_path(lineage_notes_path)?;
    let _headers = reader.headers()?;

    // ------------------------------------------------------------------------
    // Map parent-child relationships

    let mut graph_data: HashMap<String, Vec<String>> = HashMap::new();
    let mut graph_order: Vec<String> = Vec::new();

    for result in reader.records() {
        let record = result?;

        let row: LineageNotesRow = record.deserialize(None)?; 
        let lineage = row.lineage.to_string();
            
        // Lineages that start with '*' have been withdrawn
        if lineage.starts_with("*"){continue;}

        let parents = get_lineage_parents(lineage.clone(), &alias_key).unwrap();

        graph_order.push(lineage.to_string().clone());
        graph_data.insert(lineage.clone(), parents.clone());

    }

    Ok((graph_data, graph_order))

}

pub fn get_lineage_parents(lineage: String, alias_key: &HashMap<String, Vec<String>>) -> Result<Vec<String>, Report> {

    let mut parents: Vec<String> = Vec::new();

    // If Recombinant with multiple parents, if so, it will be in the alias
    // key with the parents listed.
    if alias_key.contains_key(&lineage){
        let lineage_paths = &alias_key[&lineage];
        if lineage_paths.len() > 1 {
            // Dedup in case multiple breakpoints
            parents = lineage_paths.clone().into_iter().unique().collect();
            return Ok(parents);
        }
    }

    // Otherwise, single parent
    let decompress = decompress_lineage(lineage.clone(), &alias_key).unwrap();

    // Ex. BA.5.2 -> ["BA", "5", "2"]
    let decompress_parts = decompress
        .split(".")
        .map(|p| p.to_string())
        .collect::<Vec<_>>();

    // If just 1 part, parent is root (ex. A)
    let mut parent = String::from("root");
    if decompress_parts.len() > 1 {
        parent = decompress_parts[0..(decompress_parts.len() - 1)].join(".");
    }
    
    // Compress the full parent back down with aliases
    parent = compress_lineage(parent.clone(), &alias_key).unwrap();
    parents.push(parent);

    Ok(parents)
}

pub fn compress_lineage(lineage: String, alias_key: &HashMap<String, Vec<String>>) -> Result<String, Report> {

    // By default, set compression level to self
    let mut compress = lineage.clone();

    // Reverse the alias-> lineage path lookup 
    let mut alias_key_rev: HashMap<String,String> = HashMap::new();

    for (alias, lineage_paths) in alias_key {
        // Skip over recombinants with multiple parents, don't need their lookup        
        if lineage_paths.len() > 1 {continue}
        let lineage_path = lineage_paths[0].clone();
        alias_key_rev.insert(lineage_path, alias.clone());
    }

    // Ex. BA.5.2 -> ["BA", "5", "2"]
    let compress_parts = compress
        .split(".")
        .map(|p| p.to_string())
        .collect::<Vec<_>>();    

    if compress_parts.len() > 1 {
        for i in (0..compress_parts.len()).rev(){
            let compress_subset = compress_parts[0..i].join(".");

            if alias_key_rev.contains_key(&compress_subset) {
                compress = alias_key_rev[&compress_subset].clone();
                // Get the suffix that was chopped off in subset
                let compress_suffix = &compress_parts[i..];

                // Add the suffix
                if compress_suffix.len() > 0 {
                    compress = format!["{compress}.{}", compress_suffix.join(".")];
                }
                break
            }
        }
    }
    
    Ok(compress)
}

/// Decompresses a SARS-CoV-2 lineage name.
/// 
/// Convert a compressed, SARS-CoV-2 lineage name into it's full unaliased form. q
/// 
/// # Arguments
/// 
///  * `lineage` | `String` | A String of a SARS-CoV-2 lineage name.
///  * `alias_key` | `&HashMap<String, Vec<String>>` | A mapping of SARS-CoV-2 aliases to decompressed lineage paths.
/// 
/// # Basic usage:
/// 
/// ```
/// decompress_lineage("BA.5.2", alias_key).unwrap();
/// ```
/// 
/// # Example
/// 
/// ```no run
/// let alias_key = import_alias_key(".").unwrap();
/// decompress_lineage("BA.5.2", alias_key).unwrap();
/// ```
pub fn decompress_lineage(lineage: String, alias_key: &HashMap<String, Vec<String>>) -> Result<String, Report> {

    // By default, set full path to lineage
    let mut decompress = lineage.clone();

    // Split lineage into levels, Ex. BA.5.2 = ["BA", "5", "2"]
    // Can be a maximum of 4 levels before aliasing
    let mut lineage_level = 0;
    let mut lineage_parts = vec![String::new(); 4];

    for (i, level) in lineage.split(".").enumerate(){
        lineage_parts[i] = level.to_string();
        lineage_level = i + 1;
    }

    // Get the first letter prefix, Ex. BA.5.2 = "BA"
    let lineage_prefix = lineage_parts[0].clone();
    // If there were multiple parts, get suffix (Ex. "BA" and "5.2")
    let lineage_suffix = lineage_parts[1..lineage_level].join(".");

    // Decompressing logic
    if alias_key.contains_key(&lineage_prefix){
        let lineage_paths = &alias_key[&lineage_prefix];
        // Not multiple recombinant parents
        if lineage_paths.len() == 1 {
            decompress = lineage_paths[0].clone();
            // Add back our suffix numbers
            if lineage_level > 1{
                decompress = format!("{decompress}.{lineage_suffix}"); 
            }
        }
    }

    Ok(decompress)
}

