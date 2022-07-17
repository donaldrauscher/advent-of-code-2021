
fn main() {
    let height_map = include_str!("../input.txt")
        .lines()
        .map(|row| {
            row
                .chars()
                .into_iter()
                .map(|c| c.to_string().parse::<usize>().unwrap())
                .collect::<Vec<usize>>()
        })
        .collect::<Vec<_>>();

    let n_rows = height_map.len();
    let n_cols = height_map.iter().next().unwrap().len();

    let risk_level =(0..n_rows)
        .into_iter()
        .map(|r| {
            (0..n_cols)
                .into_iter()
                .map(|c| {
                    let lp = low_point(r, c, &height_map);
                    if lp == height_map[r][c] {
                        lp + 1
                    } else {
                        0
                    }
                })
                .sum::<usize>()
        })
        .sum::<usize>();

    println!("Total risk level: {}", risk_level);
}

fn low_point(r: usize, c: usize, height_map: &Vec<Vec<usize>>) -> usize {
    let n_rows = height_map.len();
    let n_cols = height_map.iter().next().unwrap().len();

    let row_start = if r == 0 { 0 } else { r - 1 };
    let row_end = if r == (n_rows - 1) { r } else { r + 1 };
    let col_start = if c == 0 { 0 } else { c - 1 };
    let col_end = if c == (n_cols - 1) { c } else { c + 1 };

    let low_point = height_map[row_start..=row_end]
        .into_iter()
        .map(|r| r[col_start..=col_end].into_iter().min().unwrap())
        .min()
        .unwrap();

    return *low_point;
}
