use day_12::{input_cave_graph, Cave};
use std::rc::Rc;

fn main() {
    let cave_system = input_cave_graph();
    let start_route = Route {
        inner: vec![cave_system.start.clone()],
        been_twice: false,
    };

    let mut paths = vec![start_route];
    let mut complete = Vec::new();
    while !paths.is_empty() {
        let partial = paths.remove(0);
        let current_pos = partial.last();
        if current_pos.name != "end" {
            cave_system
                .connected_to(&current_pos)
                .filter(|cave| partial.can_visit(cave))
                .for_each(|cave| paths.push(partial.go_to(cave)));
        } else {
            complete.push(partial);
        }
    }
    println!(
        "There are {} paths that visit small caves at most once",
        complete.len()
    );
}

#[derive(Debug, Clone)]
struct Route {
    inner: Vec<Rc<Cave>>,
    been_twice: bool,
}

impl Route {
    fn go_to(&self, cave: Rc<Cave>) -> Self {
        let mut me = self.clone();
        if !(me.been_twice || cave.big) {
            me.been_twice |= self.inner.contains(&cave);
        }
        me.inner.push(cave);
        me
    }

    fn can_visit(&self, cave: &Rc<Cave>) -> bool {
        cave.name != "start"
            && (cave.big || !self.been_twice || !self.inner.contains(cave))
    }

    fn last(&self) -> Rc<Cave> {
        self.inner.last().unwrap().clone()
    }
}

#[cfg(test)]
impl ToString for Route {
    fn to_string(&self) -> String {
        day_12::format_path(&self.inner)
    }
}

#[cfg(test)]
mod test {
    use crate::Route;
    use day_12::{Cave, CaveGraph};
    use std::rc::Rc;

    const SMALL_TEST: &str = "start-A\nstart-b\nA-c\nA-b\nb-d\nA-end\nb-end";

    #[test]
    fn small_test() {
        let cave_system = CaveGraph::from(SMALL_TEST);
        let start_route = Route {
            inner: vec![cave_system.start.clone()],
            been_twice: false,
        };

        let mut paths = vec![start_route];
        let mut complete = Vec::new();
        while !paths.is_empty() {
            let partial = paths.remove(0);
            let current_pos = partial.last();
            if current_pos.name != "end" {
                cave_system
                    .connected_to(&current_pos)
                    .filter(|cave| {
                        println!(
                            "Wanting to visit {}\nRoute: {}\nAllowed: {}\n",
                            &cave.name,
                            partial.to_string(),
                            partial.can_visit(cave),
                        );
                        partial.can_visit(cave)
                    })
                    .for_each(|cave| paths.push(partial.go_to(cave)));
            } else {
                complete.push(partial);
            }
        }

        let paths = complete
            .iter()
            .map(|route| day_12::format_path(&route.inner))
            .collect::<Vec<_>>();
        println!("Paths: {:#?}", paths);

        assert_eq!(complete.len(), 36);
    }

    #[test]
    fn a_c_a() {
        let cave_system = CaveGraph::from(SMALL_TEST);
        let end = Rc::new(Cave::from("end"));
        let a = Rc::new(Cave::from("A"));
        let b = Rc::new(Cave::from("b"));
        let c = Rc::new(Cave::from("c"));
        let route = Route {
            inner: vec![a.clone(), c.clone(), a.clone()],
            been_twice: false,
        };
        let connected = cave_system
            .connected_to(&a)
            .filter(|cave| route.can_visit(cave))
            .collect::<Vec<_>>();
        assert_eq!(&connected, &[c.clone(), b.clone(), end.clone()], "At A");

        let route = Route {
            inner: vec![a.clone(), c.clone(), a.clone(), c.clone()],
            been_twice: false,
        };
        let connected = cave_system
            .connected_to(&c)
            .filter(|cave| route.can_visit(cave))
            .collect::<Vec<_>>();
        assert_eq!(&connected, &[a.clone()], "At c");
    }
}
