use day_12::input_cave_graph;

fn main() {
    let cave_system = input_cave_graph();
    let mut paths = vec![vec![cave_system.start.clone()]];
    let mut complete = Vec::new();
    while !paths.is_empty() {
        let pathial = paths.remove(0);
        let current_pos = pathial.last().unwrap();
        if current_pos.name != "end" {
            cave_system
                .connected_to(current_pos)
                .filter(|cave| cave.big || !pathial.contains(cave))
                .for_each(|cave| {
                    let mut next = pathial.clone();
                    next.push(cave);
                    paths.push(next);
                });
        } else {
            complete.push(pathial);
        }
    }
    /*let paths = complete.iter()
        .map(|path| day_12::format_path(path))
        .collect::<Vec<_>>();
    println!("Paths: {:?}", &paths);*/
    println!(
        "There are {} paths that visit small caves at most once",
        complete.len()
    );
}
