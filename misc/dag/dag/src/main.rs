use std::collections::{HashMap, VecDeque};

#[derive(Debug)]
struct Edge(usize, usize);

fn main() {
    let edges: Vec<Edge> = vec![
        Edge(0, 1), Edge(1, 2), Edge(1, 3),
        Edge(0, 8),
        Edge(3, 4), Edge(4, 5), Edge(0, 5), Edge(1, 5)
    ];

    let mut adjacency_list: HashMap<usize, Vec<usize>> = HashMap::new();
    let mut indegree: HashMap<usize, usize> = HashMap::new();
    let mut outdegree: HashMap<usize, usize> = HashMap::new();

    for edge in &edges {
        // Ensure both nodes exist in maps
        adjacency_list.entry(edge.0).or_default();
        adjacency_list.entry(edge.1).or_default();
        indegree.entry(edge.0).or_default();
        indegree.entry(edge.1).or_default();
        outdegree.entry(edge.0).or_default();
        outdegree.entry(edge.1).or_default();

        // Add outgoing edge (0 -> 1)
        adjacency_list.get_mut(&edge.0).unwrap().push(edge.1);

        // Increment in-degree for target node
        *indegree.get_mut(&edge.1).unwrap() += 1;
        *outdegree.get_mut(&edge.0).unwrap() += 1;
    }

    let mut topo_order: Vec<usize> = Vec::new();
    let mut stack_node: VecDeque<usize> = VecDeque::new();

    // Start with nodes that have 0 incoming dependencies
    for (&node, &deps) in &indegree {
        if deps == 0 {
            stack_node.push_back(node);
        }
    }

    let mut count_deps = indegree.clone();
    while let Some(n) = stack_node.pop_front() {
        topo_order.push(n);

        // Decrement dependency counts for outgoing neighbors
        if let Some(neighbors) = adjacency_list.get(&n) {
            for &neighbor in neighbors {
                let deps = count_deps.get_mut(&neighbor).unwrap();
                *deps -= 1;

                // Push to queue ONLY when dependencies hit zero
                if *deps == 0 {
                    stack_node.push_back(neighbor);
                }
            }
        }
    }

    println!("topo_order -> {:?}", topo_order);
    println!("Indegree -> {:?}", indegree);
    print!("Outdegree -> {:?}", outdegree);
}