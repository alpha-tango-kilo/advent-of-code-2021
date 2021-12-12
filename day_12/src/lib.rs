use std::collections::HashSet;
use std::fs;
use std::rc::Rc;

pub fn input_cave_graph() -> CaveGraph {
    let mut nodes = HashSet::new();
    let paths = fs::read_to_string("day_12/input")
        .expect("Failed to read input file")
        .lines()
        .map(|line| {
            let mut split = line.split('-');
            let a = Rc::new(Cave::from(split.next().expect("Bad input")));
            let b = Rc::new(Cave::from(split.next().expect("Bad input")));
            nodes.insert(a.clone());
            nodes.insert(b.clone());
            (a, b)
        })
        .collect::<Vec<_>>();

    let start = nodes
        .iter()
        .find(|cave| cave.name.eq("start"))
        .expect("No start")
        .clone();

    CaveGraph {
        start,
        nodes,
        paths,
    }
}

pub fn format_path(path: &[Rc<Cave>]) -> String {
    let parts = path.iter()
        .map(|cave| cave.name.as_str())
        .collect::<Vec<_>>();
    parts.join(",")
}

#[derive(Debug, Clone, Hash, Eq, PartialEq)]
pub struct Cave {
    pub name: String,
    pub big: bool,
}

impl From<&str> for Cave {
    fn from(s: &str) -> Self {
        let big = s
            .chars()
            .next()
            .expect("Empty string cave")
            .is_ascii_uppercase();
        Cave {
            name: s.to_owned(),
            big,
        }
    }
}

pub struct CaveGraph {
    pub start: Rc<Cave>,
    nodes: HashSet<Rc<Cave>>,
    paths: Vec<(Rc<Cave>, Rc<Cave>)>,
}

impl CaveGraph {
    pub fn connected_to<'a>(
        &'a self,
        cave: &'a Cave,
    ) -> impl Iterator<Item = Rc<Cave>> + 'a {
        self.paths
            .iter()
            .filter_map(move |(a, b)| {
                if a.as_ref() == cave {
                    Some(b)
                } else if b.as_ref() == cave {
                    Some(a)
                } else {
                    None
                }
            })
            .cloned()
    }
}
