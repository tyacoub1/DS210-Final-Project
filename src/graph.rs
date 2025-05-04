use petgraph::graph::Graph;
use petgraph::Undirected;
use std::collections::HashMap;

pub struct FlightGraph {
    pub graph: Graph<String, (), Undirected>,
}

impl FlightGraph {
    pub fn load_from_csv(path: &str) -> Result<Self, Box<dyn std::error::Error>> {
        let mut graph = petgraph::Graph::<String, (), petgraph::Undirected>::new_undirected();
        let mut node_map = HashMap::new();

        let mut rdr = csv::Reader::from_path(path)?;
        for result in rdr.records() {
            let record = result?;
            let source = record[0].to_string();
            let dest = record[1].to_string();

            let src_index = *node_map.entry(source.clone()).or_insert_with(|| {
                graph.add_node(source.clone())
            });

            let dst_index = *node_map.entry(dest.clone()).or_insert_with(|| {
                graph.add_node(dest.clone())
            });

            graph.add_edge(src_index, dst_index, ());
        }

        Ok(FlightGraph { graph })
    }

    pub fn is_connected(&self, src: &str, dst: &str) -> bool {
        let node_map: HashMap<String, _> = self.graph.node_indices()
            .map(|i| (self.graph[i].clone(), i))
            .collect();

        let src_index = match node_map.get(src) {
            Some(&i) => i,
            None => return false,
        };

        let dst_index = match node_map.get(dst) {
            Some(&i) => i,
            None => return false,
        };

        petgraph::algo::has_path_connecting(&self.graph, src_index, dst_index, None)
    }

    pub fn print_top_connected(&self, top_n: usize) {
        let mut airport_connections: Vec<(String, usize)> = self.graph.node_indices()
            .map(|i| (self.graph[i].clone(), self.graph.neighbors(i).count()))
            .collect();

        airport_connections.sort_by(|a, b| b.1.cmp(&a.1)); 

        println!("\nTop {} Connected Airports:", top_n);
        for (airport, connections) in airport_connections.into_iter().take(top_n) {
            println!("{} with {} connections", airport, connections);
        }
    }

    pub fn airport_with_most_connections(&self) -> Option<String> {
        let mut max_connections = 0;
        let mut airport_with_max = None;

        for node in self.graph.node_indices() {
            let connections = self.graph.neighbors(node).count();
            if connections > max_connections {
                max_connections = connections;
                airport_with_max = Some(self.graph[node].clone());
            }
        }

        airport_with_max
    }

    pub fn total_airports(&self) -> usize {
        self.graph.node_count()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use petgraph::graph::Graph;
    use petgraph::Undirected;

    #[test]
    fn test_graph_creation_and_connection() {
        let mut graph = Graph::<String, (), Undirected>::new_undirected();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        graph.add_edge(a, b, ());

        let fg = FlightGraph { graph };
        assert!(fg.is_connected("A", "B"));
        assert!(!fg.is_connected("A", "C"));
    }

    #[test]
    fn test_top_connected_airports() {
        let mut graph = Graph::<String, (), Undirected>::new_undirected();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());
        let d = graph.add_node("D".to_string());

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());
        graph.add_edge(c, d, ());
        graph.add_edge(a, c, ());

        let fg = FlightGraph { graph };
        fg.print_top_connected(2);  
    }

    #[test]
    fn test_airport_with_most_connections() {
        let mut graph = Graph::<String, (), Undirected>::new_undirected();
        let a = graph.add_node("A".to_string());
        let b = graph.add_node("B".to_string());
        let c = graph.add_node("C".to_string());
        let d = graph.add_node("D".to_string());

        graph.add_edge(a, b, ());
        graph.add_edge(b, c, ());
        graph.add_edge(c, d, ());
        graph.add_edge(a, c, ());

        let fg = FlightGraph { graph };
        let airport = fg.airport_with_most_connections();
        assert_eq!(airport, Some("C".to_string()));
    }

    #[test]
    fn test_total_airports() {
        let mut graph = Graph::<String, (), Undirected>::new_undirected();
        graph.add_node("A".to_string());
        graph.add_node("B".to_string());
        graph.add_node("C".to_string());

        let fg = FlightGraph { graph };
        assert_eq!(fg.total_airports(), 3);
    }
}