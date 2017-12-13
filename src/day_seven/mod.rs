#[cfg(test)]
mod tests;
mod input;

use std::collections::HashSet;

pub enum Node {
    Leaf(String, i32),
    Node(String, i32, Vec<String>),
}

impl Node {
    fn key(&self) -> String {
        match *self {
            Node::Leaf(ref name, _) => name.to_owned(),
            Node::Node(ref name, _, _) => name.to_owned(),
        }
    }
}

impl From<Vec<String>> for Node {
    fn from(v: Vec<String>) -> Node {
        if v.len() > 2 {
            Node::Node(
                v[0].clone(),
                0,
                v[3..v.len()]
                    .into_iter()
                    .map(|x| strip_trailing_comma(x).to_owned())
                    .collect(),
            )
        } else {
            Node::Leaf(v[0].clone(), 0)
        }
    }
}

fn strip_trailing_comma(s: &str) -> &str {
    s.split(",").collect::<Vec<_>>()[0]
}

pub struct Tower {
    nodes: Vec<Node>,
}

impl Tower {
    fn new(nodes: Vec<Node>) -> Tower {
        Tower { nodes }
    }

    fn first(&self) -> &Node {
        &self.nodes[0]
    }

    fn root(&self) -> &Node {
        let names = self.nodes
            .iter()
            .map(|x| x.key())
            .collect::<HashSet<String>>();
        let edges = self.nodes
            .iter()
            .filter_map(|x| match *x {
                Node::Leaf(_, _) => None,
                Node::Node(_, _, ref edges) => Some(edges.clone()),
            })
            .flat_map(|x| x)
            .collect::<HashSet<String>>();

        let names_that_are_not_edges = names.difference(&edges).collect::<Vec<_>>();
        let name = names_that_are_not_edges
            .get(0)
            .expect("no difference")
            .to_owned();
        self.nodes
            .iter()
            .find(|x| x.key() == *name)
            .expect("didn't find it")
    }
}

pub fn parse_input(input: &str) -> Tower {
    let nodes = input
        .to_owned()
        .lines()
        .map(|x| x.split(' ').map(|x| x.to_owned()).collect::<Vec<String>>())
        .map(Node::from)
        .collect();

    Tower::new(nodes)
}
