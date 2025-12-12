use std::collections::HashMap;

pub const SAMPLE_OUTPUT: i64 = 5;

/*
aaa: you hhh
you: bbb ccc
bbb: ddd eee
ccc: ddd eee fff
ddd: ggg
eee: out
fff: out
ggg: out
hhh: ccc fff iii
iii: out

*/

struct Node<'a> {
    name: &'a str,
    edges: Vec<&'a str>,
    depends_on: Vec<&'a str>,
    times_reached: usize,
    reach_count: usize,
}
impl<'a> Node<'a> {
    fn new(name: &'a str) -> Self {
        Node {
            name,
            edges: Vec::new(),
            depends_on: Vec::new(),
            times_reached: 0,
            reach_count: 0,
        }
    }
}

pub fn run(inp: &str) -> i64 {
    unsafe {
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

        // clear out any nodes with zero depends_on except "you"
        loop {
            let mut names = graph.keys().cloned().collect::<Vec<&str>>();
            let mut next_names = Vec::new();
            let mut made_change = false;
            for name in names.iter() {
                if *name == "you" {
                    continue;
                }
                let node = graph.get(name).unwrap_unchecked();
                if node.depends_on.is_empty() {
                    made_change = true;
                    // println!("Removing node {}", name);
                    let dependents = node.edges.clone();
                    graph.remove(name);
                    for dep_name in dependents {
                        let dep_node = graph.get_mut(dep_name).unwrap_unchecked();
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

        // remove you.depends_on
        graph.get_mut("you").unwrap_unchecked().depends_on.clear();

        let mut to_visit = vec![("you", 1)];
        // do modified bfs
        while !to_visit.is_empty() {
            let mut next_visit = Vec::new();
            for &(node_name, reach_count) in &to_visit {
                let node = &mut graph.get_mut(node_name).unwrap_unchecked();
                node.reach_count += reach_count;
                node.times_reached += 1;
                if node.times_reached >= node.depends_on.len() {
                    for &dep in &node.edges {
                        next_visit.push((dep, node.reach_count));
                    }
                }
            }
            to_visit = next_visit;
        }

        // get reach_count of `out`
        graph.get("out").unwrap_unchecked().reach_count as i64
    }
}
