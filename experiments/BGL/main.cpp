#include <iostream>
#include <boost/graph/adjacency_list.hpp>

int main() {
    typedef boost::adjacency_list<boost::vecS, boost::vecS, boost::undirectedS> Graph;

    // Create a graph object
    Graph g;

    // Add vertices (nodes)
    Graph::vertex_descriptor v1 = boost::add_vertex(g);
    Graph::vertex_descriptor v2 = boost::add_vertex(g);

    // Add an edge connecting the two vertices
    boost::add_edge(v1, v2, g);

    std::cout << "Graph successfully created!" << std::endl;
    std::cout << "Number of vertices: " << boost::num_vertices(g) << std::endl;
    std::cout << "Number of edges: " << boost::num_edges(g) << std::endl;

    return 0;
}
