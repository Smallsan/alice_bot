use rand::seq::IteratorRandom;

pub fn generate_bubble_wrap() -> String {
    let mut grid: Vec<&str> = vec![
        "pop", "pop", "pop", "pop", "pop\n", "pop", "pop", "pop", "pop", "pop\n", "pop", "pop",
        "pop", "pop", "pop\n", "pop", "pop", "pop", "pop", "pop\n", "pop", "pop", "pop", "pop",
        "pop",
    ];

    random_position(&mut grid);

    let formatted_grid: String = grid.iter().map(|cell| format!("||{}||", cell)).collect();

    return formatted_grid;
}

fn random_position(grid: &mut Vec<&str>) {
    let mut rng = rand::thread_rng();
    if let Some(i) = (0..grid.len()).choose(&mut rng) {
        grid[i] = "dud";
    }
}
