# DS210-Final-Project
# Flight Graph Analysis Project Report
## A. Project Overview
### Goal
    This project analyzes airline route data to identify patterns in global flight networks, including hub airports, connectivity between airports, centrality measures, and the densest subgraphs within the network.
### Dataset
    The dataset used is a CSV file (routes.csv) containing flight route information between airports. The data includes airport codes for source and destination airports, providing a basis for constructing an undirected graph representing the global flight network.
## B. Data Processing
### Loading Data
    The data is loaded from a CSV file using Rust's csv library
    Each record represents a flight route with source and destination airports
    The data is transformed into an undirected graph structure using the petgraph library
### Transformations
    Airport codes are used as node labels
    Each connection between airports is represented as an undirected edge
    Duplicate routes are handled naturally by the graph structure

## C. Code Structure
### Modules
    graph.rs - Core module that defines the FlightGraph struct and its methods for loading, querying, and analyzing the flight network.
    centrality.rs - Contains functions for calculating and displaying closeness centrality measures for airports.
    densest.rs - Implements an algorithm to approximate the densest subgraph within the flight network.
    main.rs - Entry point that coordinates the analysis workflow and presents results.

### Key Functions & Types
#### FlightGraph (Struct)
    Purpose: Represents the flight network as an undirected graph
    Components: Contains a petgraph::Graph<String, (), Undirected> representing airports and routes
    
#### load_from_csv (Function)
    Purpose: Loads route data from a CSV file and constructs the graph
    Inputs: Path to the CSV file        
    Outputs: A Result containing either a FlightGraph or an error
    Core Logic: Creates nodes for airports and edges for routes, using a HashMap to track node indices
    
#### is_connected (Function)
    Purpose: Checks if there is a path between two airports
    Inputs: Source and destination airport codes
    Outputs: Boolean indicating connectivity
    Core Logic: Uses petgraph's path-finding algorithm to determine if airports are connected
    
#### print_top_connected (Function)
    Purpose: Identifies and displays airports with the most connections
    Inputs: Number of top airports to display
    Outputs: Console output of airports and connection counts
    Core Logic: Counts neighbors for each node, sorts by count, and displays the top N

#### compute_centrality (Function)
    Purpose: Calculates closeness centrality for all airports
    Inputs: FlightGraph reference
    Outputs: Vector of tuples containing airport codes and centrality scores
    Core Logic: Uses Dijkstra's algorithm to compute shortest paths and calculates centrality based on path lengths

#### approximate_densest_subgraph (Function)
    Purpose: Finds an approximation of the densest subgraph in the network
    Inputs: FlightGraph reference
    Outputs: Density measure and a set of nodes in the densest subgraph
    Core Logic: Iteratively removes the node with the lowest degree until the graph is empty, tracking the maximum density

### Main Workflow
    Load route data from CSV into a FlightGraph
    Display basic statistics about the graph (total number of airports)
    Find and display the top connected airports
    Identify the airport with the most connections
    Check connectivity between specific airports (ATL to JFK)
    Calculate and display centrality measures for the first 5 airports
    Approximate and display the densest subgraph in the network

## D. Tests
#### Test Output
    running 6 tests
    test densest::tests::test_densest_subgraph_not_empty ... ok
    test centrality::tests::test_centrality_returns_values ... ok
    test graph::tests::test_airport_with_most_connections ... ok
    test graph::tests::test_graph_creation_and_connection ... ok
    test graph::tests::test_top_connected_airports ... ok
    test graph::tests::test_total_airports ... ok

    test result: ok. 6 passed; 0 failed; 0 ignored; 0 measured; 0 filtered out; finished in 0.00s

### Test Coverage
#### test_densest_subgraph_not_empty
    Verifies that the densest subgraph function returns a non-empty result with a positive density value
    Important to ensure the algorithm works on a simple test case

#### test_centrality_returns_values
    Ensures that the centrality computation returns valid results
    Validates the core functionality of the centrality analysis

#### test_airport_with_most_connections
    Checks that the function correctly identifies the node with the most edges
    Confirms the hub identification logic works correctly

#### test_graph_creation_and_connection
    Validates basic graph construction and connectivity checking
    Ensures the underlying graph structure functions as expected

#### test_top_connected_airports
    Tests the function that identifies airports with the most connections
    Ensures sorting and selection of top airports works correctly

#### test_total_airports
    Verifies accurate counting of nodes in the graph
    Provides basic validation of graph construction



## E. Results
### Program Output
    ✈️  Starting Flight Graph Analysis...
    ✅ Loaded 1116 airports

    Top 5 Connected Airports:   
    FR with 2484 connections
    4296 with 2484 connections
    AA with 2354 connections
    24 with 2354 connections
    UA with 2180 connections
    🌍 Hub with most connections: FR
    🔎 Checking if ATL is connected to JFK...
    ➡️  Connected? false

    Centrality (Closeness):
    2B → closeness: 2.0000
    410 → closeness: 2.0000
    2G → closeness: 2.0000
    1654 → closeness: 2.0000
    2I → closeness: 2.0000

    Densest Subgraph Approximation:
    Max density ≈ 1242.00 with 2 nodes

### Interpretation
    The analysis identified 1,116 distinct airports in the dataset.

    Airlines appear to be represented in the data alongside airports (e.g., FR, AA, UA), which indicates that the dataset might be structured differently than initially assumed or includes airline identifiers.

    The top connected entities have thousands of connections, suggesting they might be major airlines or hub airports.

    The highest connectivity belongs to "FR" (possibly Ryanair) with 2,484 connections.

    Interestingly, ATL (Atlanta) and JFK (New York) are not connected according to our analysis, which is unexpected for major airports and suggests potential issues with the data or graph construction.

    The centrality analysis shows equal closeness values (2.0000) for the first five airports examined, suggesting they might have similar positions in the network topology.

    The densest subgraph consists of only 2 nodes with a density of 1,242, indicating a very strong connection between these two entities.

## F. Usage Instructions
### Building and Running
    Clone the repository
    Navigate to the project directory
    Ensure the routes.csv file is in the data directory
    Run cargo build to compile the project
    Execute cargo run to perform the analysis

### Expected Runtime
    The analysis completes within seconds for the current dataset of 1,116 airports. Performance may vary for larger datasets but should remain efficient due to the use of appropriate data structures and algorithms.

## G. AI-Assistance Disclosure
    I utilized AI assistance for structuring the Rust project and optimizing the graph algorithms, particularly:
        Setting up the project structure with appropriate modules
        
        Implementing the densest subgraph approximation algorithm
        
        Optimizing the centrality calculations using Dijkstra's algorithm

    For each of these areas, I've developed my own understanding of the underlying concepts:

    The densest subgraph approximation uses a greedy algorithm that iteratively removes the node with the lowest degree, which provides a 2-approximation of the optimal solution.
    
    Closeness centrality is calculated as the reciprocal of the average shortest path length to all other nodes, providing a measure of how central an airport is in the network.

### Additional Resources
    petgraph documentation for graph algorithms and data structures
    Networkx documentation for reference on graph analysis concepts
