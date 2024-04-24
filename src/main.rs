mod functions;
use functions::{Graph, ListOfEdges, read_graph};
use crate::functions::pagerank;

fn main() {
    let (n, edges): (usize, ListOfEdges) = read_graph("C:/Users/ldunn/DS210/DS210 Rust HWs/hw10/q1/src/pagerank_data.txt");
    let graph = Graph::create_directed(n, &edges);
    pagerank(&graph);
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_total(){ //dependencies approx for equality
        let data = vec![(0,3), (1,3), (2,3), (3,4), (4,3), (4,0)];
        let (n, edges) = (5, data);
        let graph = Graph::create_directed(n, &edges);
        let counts = pagerank(&graph).clone();
        let ranks: Vec<(usize, &usize)> = counts.iter().enumerate().collect();
        let mut total: f32 = 0.0;
        for (_, &count) in ranks.iter() { //adding all of the rank estimations because they should equal 1 together
            total += (count as f32) / (100.0 * (n as f32));
        }
        assert_eq!(true, approx::abs_diff_eq!(1.0, total));
    }
}