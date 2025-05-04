use crate::graph::FlightGraph;
use petgraph::algo::dijkstra;

pub fn print_centrality(flight_graph: &FlightGraph) {
    println!("\nCentrality (Closeness):");

    for node in flight_graph.graph.node_indices().take(5) {
        let lengths = dijkstra(&flight_graph.graph, node, None, |_| 1);
        let reachable = lengths.len() as f64;
        let sum_dist: f64 = lengths.values().copied().map(|x| x as f64).sum();

        let closeness = if sum_dist > 0.0 {
            reachable / sum_dist
        } else {
            0.0
        };

        println!(
            "{} → closeness: {:.4}",
            &flight_graph.graph[node],
            closeness
        );
    }
}

pub fn compute_centrality(flight_graph: &FlightGraph) -> Vec<(String, f64)> {
    let mut centralities = Vec::new();

    for node in flight_graph.graph.node_indices() {
        let lengths = dijkstra(&flight_graph.graph, node, None, |_| 1);
        let reachable = lengths.len() as f64;
        let sum_dist: f64 = lengths.values().copied().map(|x| x as f64).sum();

        let closeness = if sum_dist > 0.0 {
            reachable / sum_dist
        } else {
            0.0
        };

        centralities.push((flight_graph.graph[node].clone(), closeness));
    }

    centralities
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::FlightGraph;
    use petgraph::Graph;

    #[test]
    fn test_centrality_returns_values() {
        let mut fg = FlightGraph {
            graph: Graph::new_undirected(),
        };

        let a = fg.graph.add_node("A".to_string());
        let b = fg.graph.add_node("B".to_string());
        let c = fg.graph.add_node("C".to_string());

        fg.graph.add_edge(a, b, ());
        fg.graph.add_edge(a, c, ());

        let centrality = compute_centrality(&fg);
        assert!(!centrality.is_empty());
    }
}