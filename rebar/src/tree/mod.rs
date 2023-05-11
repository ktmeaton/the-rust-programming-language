use std::collections::HashMap;

use itertools::Itertools;
use eyre::Report;
use id_tree::{Node,NodeId,Tree, TreeBuilder, InsertBehavior::*};
use csv;
use serde;
use serde_json;
use log::debug;

#[derive(serde::Deserialize, Debug)]
struct LineageNotesRow<'a> {
    lineage: &'a str,
    _description: &'a str,
}

// Specifically for SARS-CoV-2
pub fn build_tree() -> Result<(Tree<String>, HashMap<String, id_tree::NodeId>), Report>{


    // TBD: Download alias key: https://raw.githubusercontent.com/cov-lineages/pango-designation/master/pango_designation/alias_key.json
    let alias_key_path = "dataset/sars-cov-2-latest/alias_key.json";
    let alias_key_str = std::fs::read_to_string(alias_key_path).unwrap();
    let alias_key_val: serde_json::Value = serde_json::from_str(&alias_key_str).unwrap();
    let alias_key: serde_json::Map<String, serde_json::Value> = alias_key_val.as_object().unwrap().clone();

    // TBD: Download lineage notes: https://raw.githubusercontent.com/cov-lineages/pango-designation/master/lineage_notes.txt
    let lineage_notes_path = "dataset/sars-cov-2-latest/lineage_notes.txt";
    let mut reader = csv::ReaderBuilder::new()
        .delimiter(b'\t')
        .from_path(lineage_notes_path)?;

    // Retrive headers
    let _headers = reader.headers()?;

    // ------------------------------------------------------------------------
    // Map parent-child relationships

    let mut tree_data: HashMap<String, String> = HashMap::new();
    let mut tree_order: Vec<String> = Vec::new();

    // Read records
    for result in reader.records() {
        let record = result?;

        let row: LineageNotesRow = record.deserialize(None)?; 
        let lineage = row.lineage.clone();
            
        // Lineages that start with '*' have been withdrawn
        if lineage.starts_with("*"){continue;}

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
        // By default, set parent to prefix
        let mut parent = lineage_prefix.to_string();

        // Option 1. If prefix same as lineage, psarent is either root or X
        //           Ex. A,B => root
        //           Ex. XBB => X
        if lineage_prefix == lineage {
            if lineage.starts_with("X") {
                parent = "X".to_string();
            } else {
                parent = "root".to_string();
            }
        } 

        // Option 2. 3+ parts to lineage levels
        //              Ex. A.2.3 = "A.2"
        if lineage_level >= 3 {
            parent = lineage_parts[0..lineage_level-1].join(".");
        }
        // Option 3. Check for alias
        //   Ex. A.1 = "A", C.1 = "B.1.1.1"
        else if alias_key.contains_key(&lineage_prefix){
            let alias = alias_key[&lineage_prefix].clone();
            if alias != "" {

                // X lineage has multiple parents as vector
                // Check for recursion
                if lineage_prefix.starts_with("X"){
                    //let alias = alias[0].clone();
                    //println!("\t{:?}", alias);
                } else {
                    // Use as_str to remove extra double quotes
                    parent = alias.as_str().unwrap().to_string();

                    // If parent is not in the tree, we need to collapse the alias
                    // by shrinking the levels
                    // BC.1 = B.1.1.529.1.1.1
                    // Partial collapse down
                    if !tree_order.contains(&parent){
                        let parent_parts = parent.split(".").collect::<Vec<_>>();
                       
                        for i in (0..parent_parts.len()).rev(){
                            let parent_subset = parent_parts[0..i].iter().join(".");
                            if tree_order.contains(&parent_subset){
                                parent = parent_subset;
                                break
                            }
                        }
                    }
                }
            }
        }

        tree_data.insert(lineage.to_string().clone(), parent.clone());
        tree_order.push(lineage.to_string().clone());

    }

    // ------------------------------------------------------------------------
    // Construct Tree

    let mut tree: Tree<String> = TreeBuilder::new()
        .with_node_capacity(tree_order.len())
        .build();

    let mut name_to_id = HashMap::new();

    // Add root node
    let name = "root".to_string();
    let id = tree.insert(Node::new(name.clone()), AsRoot).unwrap();
    name_to_id.insert(name, id.clone());

    // Add 'X' recombinant node
    let name = "X".to_string();
    let parent = "root".to_string();    
    let parent_id = &name_to_id[&parent];
    let id: NodeId = tree.insert(Node::new(name.clone()), UnderNode(&parent_id)).unwrap();
    name_to_id.insert(name, id.clone());    

    for name in tree_order{
        let parent = tree_data[&name].clone();
        let parent_id = &name_to_id[&parent];
        let id: NodeId = tree.insert(Node::new(name.clone()), UnderNode(&parent_id)).unwrap();
        name_to_id.insert(name, id.clone());
    }

    // For debugging
    let mut s = String::new();
    tree.write_formatted(&mut s).unwrap();
    debug!("\n{}", s);    

    Ok((tree, name_to_id))

}

pub fn to_newick(tree: Tree<String>, name_to_id: HashMap<String, id_tree::NodeId>) -> Result<(), Report> {
    println!("to_newick");

    let root_id = &name_to_id[&"root".to_string()];

    let newick = String::new();
    let newick = String::from("(A,B)root;");
    for node in tree.traverse_pre_order(&root_id).unwrap() {
        println!("{}", node.data());

        if node.data() == "XB" {
            break
        }
    }

    println!("{newick}");


    Ok(())
}

