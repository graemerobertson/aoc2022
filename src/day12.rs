use std::cmp::Ordering;
use std::collections::BinaryHeap;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

/*
 * I couldn't be bothered to re-invent Dijkstra's algorithm, so I've literally
 * just copied it from https://doc.rust-lang.org/std/collections/binary_heap/index.html#examples
 * and moved on with my life.
 */

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    cost: usize,
    position: usize,
}

// The priority queue depends on `Ord`.
// Explicitly implement the trait so the queue becomes a min-heap
// instead of a max-heap.
impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        // Notice that the we flip the ordering on costs.
        // In case of a tie we compare positions - this step is necessary
        // to make implementations of `PartialEq` and `Ord` consistent.
        other
            .cost
            .cmp(&self.cost)
            .then_with(|| self.position.cmp(&other.position))
    }
}

// `PartialOrd` needs to be implemented as well.
impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

// Each node is represented as a `usize`, for a shorter implementation.
#[derive(Debug)]
struct Edge {
    node: usize,
    cost: usize,
}

// Dijkstra's shortest path algorithm.

// Start at `start` and use `dist` to track the current shortest distance
// to each node. This implementation isn't memory-efficient as it may leave duplicate
// nodes in the queue. It also uses `usize::MAX` as a sentinel value,
// for a simpler implementation.
fn shortest_path(adj_list: &[Vec<Edge>], start: usize, goal: usize) -> Option<usize> {
    // dist[node] = current shortest distance from `start` to `node`
    let mut dist: Vec<_> = (0..adj_list.len()).map(|_| usize::MAX).collect();

    let mut heap = BinaryHeap::new();

    // We're at `start`, with a zero cost
    dist[start] = 0;
    heap.push(State {
        cost: 0,
        position: start,
    });

    // Examine the frontier with lower cost nodes first (min-heap)
    while let Some(State { cost, position }) = heap.pop() {
        // Alternatively we could have continued to find all shortest paths
        if position == goal {
            return Some(cost);
        }

        // Important as we may have already found a better way
        if cost > dist[position] {
            continue;
        }

        // For each node we can reach, see if we can find a way with
        // a lower cost going through this node
        for edge in &adj_list[position] {
            let next = State {
                cost: cost + edge.cost,
                position: edge.node,
            };

            // If so, add it to the frontier and continue
            if next.cost < dist[next.position] {
                heap.push(next);
                // Relaxation, we have now found a better way
                dist[next.position] = next.cost;
            }
        }
    }

    // Goal not reachable
    None
}

fn build_graph(
    grid: &[Vec<u32>],
    number_of_rows: usize,
    number_of_columns: usize,
    graph: &mut Vec<Vec<Edge>>,
) {
    for row in 0..number_of_rows {
        for column in 0..number_of_columns {
            let mut edges: Vec<Edge> = Vec::new();
            if (row > 0) && (grid[row - 1][column] <= grid[row][column] + 1) {
                edges.push(Edge {
                    node: number_of_columns * (row - 1) + column,
                    cost: 1,
                });
            }
            if (column > 0) && (grid[row][column - 1] <= grid[row][column] + 1) {
                edges.push(Edge {
                    node: number_of_columns * (row) + column - 1,
                    cost: 1,
                });
            }
            if (row < number_of_rows - 1) && (grid[row + 1][column] <= grid[row][column] + 1) {
                edges.push(Edge {
                    node: number_of_columns * (row + 1) + column,
                    cost: 1,
                });
            }
            if (column < number_of_columns - 1) && (grid[row][column + 1] <= grid[row][column] + 1)
            {
                edges.push(Edge {
                    node: number_of_columns * (row) + column + 1,
                    cost: 1,
                });
            }
            graph.push(edges);
        }
    }
}

pub(crate) fn day12() {
    let f: File = File::open("data/day12.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();

    let number_of_rows: usize = input_data.len();
    let number_of_columns: usize = input_data.get(0).unwrap().len();
    let mut grid: Vec<Vec<u32>> = vec![vec![0; number_of_columns]; number_of_rows];
    let mut low_points: Vec<usize> = vec![];
    let mut start_point: usize = 0;
    let mut end_point: usize = 0;

    for (i, line) in input_data.iter().enumerate() {
        for (j, point) in line.chars().enumerate() {
            if point == 'S' {
                grid[i][j] = 'a' as u32;
                start_point = i * number_of_columns + j;
            } else if point == 'E' {
                grid[i][j] = 'z' as u32;
                end_point = i * number_of_columns + j;
            } else {
                if point == 'a' {
                    low_points.push(i * number_of_columns + j);
                }
                grid[i][j] = point as u32;
            }
        }
    }

    let mut graph: Vec<Vec<Edge>> = Vec::new();
    build_graph(&grid, number_of_rows, number_of_columns, &mut graph);

    println!(
        "Most direct trail from start to highest point takes {} moves",
        shortest_path(&graph, start_point, end_point).unwrap()
    );

    println!(
        "Most direct trail from any low point to the highest point takes {} moves",
        low_points
            .iter()
            .filter_map(|p| shortest_path(&graph, *p, end_point))
            .min()
            .unwrap()
    );
}
