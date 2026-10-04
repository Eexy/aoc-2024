use std::{
    collections::HashSet,
    fs::File,
    io::{BufRead, BufReader},
};

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Plot {
    plot_type: char,
    x: usize,
    y: usize,
}

const DIRECTIONS: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, 1), (0, -1)];

fn find_next_plots(plot: &Plot, plots: &HashSet<Plot>) -> Vec<Plot> {
    DIRECTIONS
        .iter()
        .filter_map(|direction| {
            let x = plot.x.checked_add_signed(direction.0 as isize)?;
            let y = plot.y.checked_add_signed(direction.1 as isize)?;

            Some(Plot {
                plot_type: plot.plot_type,
                x,
                y,
            })
        })
        .filter(|possible_plot| plots.contains(possible_plot))
        .collect()
}

fn create_region(plot: Plot, plots: &HashSet<Plot>) -> (HashSet<Plot>, usize) {
    let mut regions = HashSet::new();
    regions.insert(plot);

    let mut next_plots = find_next_plots(&plot, plots);
    let mut number_of_connected_sides = next_plots.len();
    while let Some(next_plot) = next_plots.pop() {
        if !regions.contains(&next_plot) {
            let mut nexts = find_next_plots(&next_plot, plots);
            number_of_connected_sides += nexts.len();
            next_plots.append(&mut nexts);
            regions.insert(next_plot);
        }
    }

    (regions, number_of_connected_sides)
}

fn main() -> Result<(), String> {
    let file = File::open("input.txt").map_err(|err| format!("Unable to read file: {err}"))?;
    let reader = BufReader::new(file);
    let mut plots: HashSet<Plot> = HashSet::new();

    for (x, line) in reader.lines().enumerate() {
        let line = line.map_err(|err| format!("unable to read line: {err}"))?;
        let chars = line.chars();
        for (y, ch) in chars.enumerate() {
            plots.insert(Plot {
                plot_type: ch,
                x,
                y,
            });
        }
    }

    let mut visited: HashSet<Plot> = HashSet::new();
    let mut result = 0i64;
    for plot in plots.iter() {
        if visited.contains(plot) {
            continue;
        }
        let region = create_region(*plot, &plots);
        visited.extend(region.0.iter());
        result += region.0.len() as i64 * (region.0.len() * 4 - region.1) as i64;
    }

    dbg!(result);

    Ok(())
}
