use crate::arguments::Arguments;
use crate::io::reader;
use anyhow::Result;
use std::collections::{HashMap, HashSet};
use std::io::BufRead;

pub fn day12(args: &Arguments) -> Result<()> {
    let reader = reader(args.src_file.as_ref())?;

    let mut graph = Graph::default();
    for line in reader.lines() {
        let line = line?;
        let line = line;
        graph.add_edge(line);
    }
    let graph = graph;

    let result = if args.part == 1 {
        part1(graph)
    } else {
        part2(graph)
    }?;

    info!("{:?}", result);

    Ok(())
}

#[derive(Debug, Default, Eq, PartialEq, Hash, Clone)]
struct Cave {
    name: String,
    is_big: bool,
}

impl Cave {
    fn new(name: String) -> Cave {
        let is_big = name.to_uppercase() == name;
        Cave { name, is_big }
    }
}

#[derive(Debug, Default)]
struct Graph {
    g: HashMap<String, HashSet<String>>,
    caves: HashMap<String, Cave>,
    start: String,
    end: String,
}

impl Graph {
    fn add_edge(&mut self, edge: String) {
        let mut split: Vec<String> = edge.split("-").map(|s| s.into()).collect();
        let left = split.remove(0);
        let right = split.remove(0);

        {
            self.caves
                .entry(left.clone())
                .or_insert(Cave::new(left.clone()));
        }
        {
            self.caves
                .entry(right.clone())
                .or_insert(Cave::new(right.clone()));
        }

        let node = self.g.entry(left.clone()).or_insert(Default::default());
        node.insert(right.clone());

        let node = self.g.entry(right.clone()).or_insert(Default::default());
        node.insert(left.clone());

        if left == "start" {
            self.start = left.clone();
        } else if left == "end" {
            self.end = left.clone();
        }

        if right == "start" {
            self.start = right.clone();
        } else if right == "end" {
            self.end = right.clone();
        }
    }
}

fn can_visit_part1(_graph: &Graph, visit_counts: &HashMap<String, u64>, next: &Cave) -> bool {
    next.is_big || !visit_counts.contains_key(&next.name)
}

fn can_visit_part2(graph: &Graph, visit_counts: &HashMap<String, u64>, next: &Cave) -> bool {
    if next.is_big {
        return true;
    }

    if next.name == graph.start {
        return false;
    }

    if !visit_counts.contains_key(&next.name) {
        return true;
    }

    for count in visit_counts {
        if *count.1 > 1 {
            return false;
        }
    }

    return true;
}

fn path_to_visit_counts(graph: &Graph, current_path: &Vec<String>) -> HashMap<String, u64> {
    let mut counts = HashMap::new();
    for name in current_path {
        let cave = graph.caves.get(name).unwrap();
        if cave.is_big {
            continue;
        }
        if !counts.contains_key(name) {
            counts.insert(name.clone(), 0u64);
        }
        counts.entry(name.clone()).and_modify(|x| *x += 1);
    }
    counts
}

// could probably be more efficient by not cloning everywhere, and
// by not calculating visit_counts every time, and by adding a cache
// for each node, but this is good enough
fn find_paths<F>(graph: &Graph, current_path: &Vec<String>, can_visit: &F) -> Vec<Vec<String>>
where
    F: Fn(&Graph, &HashMap<String, u64>, &Cave) -> bool,
{
    let start = current_path.last().unwrap();
    if *start == graph.end {
        let mut path = Vec::new();
        path.push(start.clone());
        let mut paths = Vec::new();
        paths.push(path);
        return paths;
    }

    let mut paths = Vec::new();
    let visit_counts = path_to_visit_counts(graph, current_path);
    for next in graph.g.get(start).unwrap() {
        let cave = graph.caves.get(next).unwrap();
        if !can_visit(graph, &visit_counts, cave) {
            continue;
        }

        let mut next_current = current_path.clone();
        next_current.push(next.clone());
        let next_current = next_current;
        let sub_paths = find_paths(graph, &next_current, can_visit);

        for sub_path in sub_paths {
            let mut path = Vec::new();
            path.push(start.clone());
            sub_path.into_iter().for_each(|name| path.push(name));
            paths.push(path);
        }
    }
    paths
}

fn part1(graph: Graph) -> Result<usize> {
    let mut path = Vec::new();
    path.push(graph.start.clone());
    let paths = find_paths(&graph, &path, &can_visit_part1);

    Ok(paths.len())
}

fn part2(graph: Graph) -> Result<usize> {
    let mut path = Vec::new();
    path.push(graph.start.clone());
    let paths = find_paths(&graph, &path, &can_visit_part2);

    Ok(paths.len())
}
