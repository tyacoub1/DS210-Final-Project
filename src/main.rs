mod graph;
mod centrality;
mod densest;

use graph::FlightGraph;

fn main() {
    println!("✈️  Starting Flight Graph Analysis...");

    match FlightGraph::load_from_csv("data/routes.csv") {
        Ok(graph) => {
            println!("✅ Loaded {} airports", graph.total_airports());

            graph.print_top_connected(5);

            if let Some(hub) = graph.airport_with_most_connections() {
                println!("🌍 Hub with most connections: {hub}");
            }

            println!("🔎 Checking if ATL is connected to JFK...");
            println!("➡️  Connected? {}", graph.is_connected("ATL", "JFK"));

            centrality::print_centrality(&graph);
            densest::approximate_densest_subgraph(&graph);
        }
        Err(e) => {
            eprintln!("❌ Failed to load graph: {}", e);
        }
    }
}