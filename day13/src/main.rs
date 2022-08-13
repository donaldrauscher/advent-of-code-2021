fn main() {
    let (coords, folds) = include_str!("../input.txt").split_once("\n\n").unwrap();

    let folds: Vec<(&str, usize)> = folds
        .lines()
        .map(|l| {
            let (d, i) = l[11..].split_once('=').unwrap();
            (d, i.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut coords: Vec<(usize, usize)> = coords
        .lines()
        .map(|l| {
            let (x, y) = l.split_once(',').unwrap();
            (x.parse::<usize>().unwrap(), y.parse::<usize>().unwrap())
        })
        .collect::<Vec<_>>();

    let mut fold_num = 0;
    for (d, i) in folds {
        fold_num += 1;
        coords = coords
            .iter()
            .filter_map(|(mut x, mut y)| {
                match d {
                    "x" if x > i => Some((2*i - x, y)),
                    "x" if x == i => None,
                    "y" if y > i => Some((x, 2*i - y)),
                    "y" if y == i => None,
                    _ => Some((x, y))
                }
            })
            .collect::<Vec<_>>();
        coords.sort_unstable();
        coords.dedup();
        if fold_num == 1 {
            println!("Number of points after first fold: {}", coords.len());
        }
    }

    println!("{:?}", coords);
    print_sheet(&coords);
}

fn print_sheet(coords: &Vec<(usize, usize)>) {
    let (max_x, max_y) = coords
        .iter()
        .fold((0, 0), |(max_x, max_y), (x, y)| {
            (std::cmp::max(max_x, *x), std::cmp::max(max_y, *y))
        });

    let coords_lookup = coords
        .iter()
        .fold(vec![0; (max_x+1)*(max_y+1)], |mut lookup, (x, y)| {
            lookup[y*max_x + x] = 1;
            lookup
        });

    for y in 0..=max_y {
        let row: String = (0..=max_x)
            .into_iter()
            .map(|x| if coords_lookup[y*max_x + x] > 0 { "#" } else { "." })
            .collect();
        println!("{}", row);
    }
}
