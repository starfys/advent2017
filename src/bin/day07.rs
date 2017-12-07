#[macro_use]
extern crate nom;
use nom::{alpha, digit, IResult};
use std::collections::{HashSet, HashMap};
use std::io;
use std::io::prelude::*;
use std::str;
use std::str::FromStr;

// Struct that represents the information in each line
#[derive(Debug)]
struct Node<'a> {
    name: &'a str,
    weight: u32,
    children: Vec<&'a str>,
}
// Struct that represent's information for a node
#[derive(Clone)]
struct NodeInfo {
    weight: u32,
    total_weight: u32,
    children: Vec<String>,
}
// Calculates the total weight for each node
fn calculate_total_weight(mut graph: &mut HashMap<String, NodeInfo>, root: String) -> u32 {
    // Get the current node
    let cur_node: NodeInfo = graph.get(&root).unwrap().clone();
    // Used to sum up weight of children
    let mut child_weight: u32 = 0;
    // Iterate over the current node's children
    for node in cur_node.children {
        child_weight += calculate_total_weight(&mut graph, node);
    }
    // Now that we have assumed all children have their weight calculated
    graph.get_mut(&root).unwrap().total_weight += child_weight;
    // Return the total weight
    graph.get(&root).unwrap().total_weight
}
/// Parses a line from day 7's input into a node's description
named!(parse_node<Node>, 
    do_parse!(
        // Read the node's name
        name: map_res!(
            alpha,
            std::str::from_utf8
        ) >>
        // " ("
        tag!(" (") >>
        // Read in the weight
        weight: map_res!(
            map_res!(
                digit,
                std::str::from_utf8
            ),
            FromStr::from_str
        ) >>
        // ")"
        tag!(")") >>
        // Read in the child nodes
        children: alt_complete!(
            // Parse " -> a, b, c"
            do_parse!(
                // Parse the arrow with spaces
                tag!(" -> ") >>
                // Parse out the list of children 
                children: separated_list_complete!(
                    // Seperated by ", "
                    tag!(", "),
                    // Any alphanumeric string
                    map_res!(
                        alpha,
                        std::str::from_utf8
                    )
                ) >>
                // Return the value from the sub parser
                (children)
            ) |
            // If we can't find tag, we probably hit an EOF here, so just return empty vector
            value!(vec![])
        ) >>
        (Node { name: name, weight: weight, children: children })
   ) 
);

fn main() {
    // Get a handle on stdin
    let stdin = io::stdin();
    // Read lines from stdin
    // Keep them here, so the &strs later refer to something. Zero copy ftw
    let lines: Vec<String> = stdin.lock().lines().map(|line| line.unwrap()).collect();
    // Parse the lines as nodes
    let nodes: Vec<Node> = lines
        .iter()
        .map(|line| {
            // Parse the line
            match parse_node(line.as_bytes()) {
                IResult::Done(b"", node) => node,
                _ => panic!("Failed to parse node"),
            }
        })
        .collect();
    // Keep track of which nodes are children
    let mut child_nodes: HashSet<String> = HashSet::new();
    // Create a graph
    let mut graph: HashMap<String, NodeInfo> = HashMap::new();
    // Figure out which nodes are children
    for node in &nodes {
        // Add the node to the graph
        graph.insert(
            node.name.to_string(),
            NodeInfo {
                weight: node.weight,
                total_weight: node.weight,
                children: node.children.iter().map(|name| name.to_string()).collect(),
            },
        );
        // Add all of the node's children to the children list
        for child in &node.children {
            child_nodes.insert(child.to_string());
        }
        // While we're at it, create a graph from the nodes
    }
    // Iterate over the nodes again to find which node is the root
    let mut root_node: String = String::from("");
    for node in &nodes {
        if !child_nodes.contains(&node.name.to_string()) {
            root_node = node.name.to_string();
            break;
        }
    }
    // Print what we found
    if root_node == "" {
        panic!("Failed to find root node")
    } else {
        println!("Part 1: {}", root_node);
    }
    // Use the graph and the root node to solve the puzzle
    calculate_total_weight(&mut graph, root_node);
    // Now, iterate over the graph's children's weight
}
