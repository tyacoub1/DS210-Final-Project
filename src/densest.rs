use crate::graph::FlightGraph;
use std::collections::HashSet;

pub fn approximate_densest_subgraph(flight_graph: &FlightGraph) -> (f64, HashSet<petgraph::graph::NodeIndex>) {
    let mut graph = flight_graph.graph.clone(); 
    let mut subgraph_nodes: HashSet<_> = graph.node_indices().collect();
    let mut best_subgraph = subgraph_nodes.clone();
    let mut max_density = 0.0;

    while !subgraph_nodes.is_empty() {
        let edge_count = subgraph_nodes
            .iter()
            .map(|&n| graph.edges(n).count())
            .sum::<usize>() as f64
            / 2.0;

        let node_count = subgraph_nodes.len() as f64;
        let density = if node_count > 0.0 {
            edge_count / node_count
        } else {
            0.0
        };

        if density > max_density {
            max_density = density;
            best_subgraph = subgraph_nodes.clone();
        }

        if let Some(&lowest) = subgraph_nodes
            .iter()
            .min_by_key(|&&n| graph.edges(n).count())
        {
            subgraph_nodes.remove(&lowest);
            graph.remove_node(lowest);
        } else {
            break;
        }
    }

    println!("\nDensest Subgraph Approximation:");
    println!(
        "Max density ≈ {:.2} with {} nodes",
        max_density,
        best_subgraph.len()
    );

    (max_density, best_subgraph)
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::Graph;

    #[test]
    fn test_densest_subgraph_not_empty() {
        let mut fg = FlightGraph {
            graph: Graph::new_undirected(),
        };

        let a = fg.graph.add_node("A".to_string());
        let b = fg.graph.add_node("B".to_string());
        let c = fg.graph.add_node("C".to_string());

        fg.graph.add_edge(a, b, ());
        fg.graph.add_edge(a, c, ());

        let (density, nodes) = approximate_densest_subgraph(&fg);
        assert!(density > 0.0);
        assert!(!nodes.is_empty());
    }
}