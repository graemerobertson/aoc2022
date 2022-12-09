use std::collections::HashSet;
use std::fs::File;
use std::io::{self, BufRead, BufReader};

#[derive(Debug, Hash, PartialEq, Eq, Clone)]
struct Point {
    x: i32,
    y: i32,
}

fn new_tail_position(head: &Point, tail: &Point) -> Point {
    let mut new_tail: Point = tail.clone();
    if (head.x - tail.x).abs() > 1 && (head.y - tail.y).abs() > 1 {
        new_tail = Point {
            x: (head.x + tail.x) / 2,
            y: (head.y + tail.y) / 2,
        };
    } else if (head.x - tail.x).abs() > 1 {
        new_tail = Point {
            x: tail.x + (head.x - tail.x) / 2,
            y: head.y,
        };
    } else if (head.y - tail.y).abs() > 1 {
        new_tail = Point {
            x: head.x,
            y: tail.y + (head.y - tail.y) / 2,
        };
    }
    new_tail
}

pub(crate) fn day09() {
    let f: File = File::open("data/day09.txt").unwrap();
    let reader: BufReader<File> = BufReader::new(f);
    let input_data: Vec<String> = reader.lines().collect::<io::Result<Vec<String>>>().unwrap();
    let mut set_of_tail_points: HashSet<Point> = HashSet::new();
    let mut knots: Vec<Point> = vec![Point { x: 0, y: 0 }; 10];
    set_of_tail_points.insert(Point { x: 0, y: 0 });
    for line in input_data {
        let direction_indicator = line.split(' ').next().unwrap();
        let distance = line.split(' ').last().unwrap().parse::<i32>().unwrap();
        let unit_move = match direction_indicator {
            "R" => Point { x: 1, y: 0 },
            "L" => Point { x: -1, y: 0 },
            "U" => Point { x: 0, y: 1 },
            "D" => Point { x: 0, y: -1 },
            _ => panic!(),
        };
        for _ in 0..distance {
            let new_head = Point {
                x: knots[0].x + unit_move.x,
                y: knots[0].y + unit_move.y,
            };
            knots[0] = new_head;
            for tail_index in 1..10 {
                knots[tail_index] = new_tail_position(
                    knots.get(tail_index - 1).unwrap(),
                    knots.get(tail_index).unwrap(),
                );
            }
            set_of_tail_points.insert(knots.last().unwrap().clone());
        }
    }
    println!(
        "Number of points visited by the tail is {}",
        set_of_tail_points.len()
    );
}
