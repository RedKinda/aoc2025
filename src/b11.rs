use std::collections::{HashMap, HashSet};

pub const SAMPLE_OUTPUT: i64 = 2;

/*
svr: aaa bbb
aaa: fft
fft: ccc
bbb: tty
tty: ccc
ccc: ddd eee
ddd: hub
hub: fff
eee: dac
dac: fff
fff: ggg hhh
ggg: out
hhh: out
*/

struct Node<'a> {
    name: &'a str,
    edges: Vec<&'a str>,
    depends_on: Vec<&'a str>,
    times_reached: usize,
    reach_count_empty: usize,
    reach_count_dac: usize,
    reach_count_fft: usize,
    reach_count_both: usize,
}
impl<'a> Node<'a> {
    fn new(name: &'a str) -> Self {
        Node {
            name,
            edges: Vec::new(),
            depends_on: Vec::new(),
            times_reached: 0,
            reach_count_empty: 0,
            reach_count_dac: 0,
            reach_count_fft: 0,
            reach_count_both: 0,
        }
    }
}

pub fn run(inp: &str) -> i64 {
    let mut graph: HashMap<&str, Node> = HashMap::new();

    for line in inp.lines() {
        let parts: Vec<&str> = line.split(": ").collect();
        let node = parts[0];
        let edges: Vec<&str> = parts[1].split(' ').collect();

        let e = graph.entry(node).or_insert_with(|| Node::new(node));

        e.edges.extend(edges.iter().cloned());

        for &edge in &edges {
            let dep = graph.entry(edge).or_insert_with(|| Node::new(edge));

            dep.depends_on.push(node);
        }
    }

    // clear out any nodes with zero depends_on except "svr"
    loop {
        let mut names = graph.keys().cloned().collect::<Vec<&str>>();
        let mut next_names = Vec::new();
        let mut made_change = false;
        for name in names.iter() {
            if *name == "svr" {
                continue;
            }
            let node = graph.get(name).unwrap();
            if node.depends_on.is_empty() {
                made_change = true;
                println!("Removing node {}", name);
                let dependents = node.edges.clone();
                graph.remove(name);
                for dep_name in dependents {
                    let dep_node = graph.get_mut(dep_name).unwrap();
                    dep_node.depends_on.retain(|&n| n != *name);
                }
            } else {
                next_names.push(*name);
            }
        }
        if !made_change {
            break;
        }
        names = next_names;
    }

    // remove svr.depends_on
    graph.get_mut("svr").unwrap().depends_on.clear();
    // insert "dummy" node thats a source for svr
    let mut dummynode = Node::new("dummy");
    dummynode.reach_count_empty = 1;
    graph.insert("dummy", dummynode);

    let mut to_visit = vec![("svr", "dummy")];
    // do modified bfs
    while !to_visit.is_empty() {
        let mut next_visit = Vec::new();
        for &(node_name, source_node_name) in &to_visit {
            let nodes = graph.get_disjoint_mut([node_name, source_node_name]);
            let mut nodes_iter = nodes.into_iter();
            let node = nodes_iter.next().unwrap().unwrap();
            let source_node = nodes_iter.next().unwrap().unwrap();

            node.times_reached += 1;

            if node_name == "dac" {
                node.reach_count_dac += source_node.reach_count_empty;
                node.reach_count_dac += source_node.reach_count_dac;
                node.reach_count_both += source_node.reach_count_fft;
                node.reach_count_both += source_node.reach_count_both;
            } else if node_name == "fft" {
                node.reach_count_fft += source_node.reach_count_empty;
                node.reach_count_fft += source_node.reach_count_fft;
                node.reach_count_both += source_node.reach_count_dac;
                node.reach_count_both += source_node.reach_count_both;
            } else {
                node.reach_count_empty += source_node.reach_count_empty;
                node.reach_count_dac += source_node.reach_count_dac;
                node.reach_count_fft += source_node.reach_count_fft;
                node.reach_count_both += source_node.reach_count_both;
            }

            if node.times_reached >= node.depends_on.len() {
                for &dep in &node.edges {
                    next_visit.push((dep, node_name));
                }
            }
        }
        to_visit = next_visit;
    }

    // get reach_count of `out`
    graph.get("out").unwrap().reach_count_both as i64
}
