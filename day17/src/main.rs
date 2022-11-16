use std::cmp;

fn main() {
    let x_y = include_str!("../input.txt")
        .trim()
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .unwrap();

    // let x_box = x_y.0
    //     .splitn(2, "..")
    //     .map(|n| n.parse::<isize>().unwrap())
    //     .collect::<Vec<_>>();

    let y_box = x_y.1
        .splitn(2, "..")
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    // symmetry!  the probe will always return to y=0
    // find max velocity that keeps it in the box and back-out altitude

    let y_velocity = -y_box[0] - 1;
    let y_max_altitude = y_velocity * (y_velocity + 1) / 2;
    println!("Max altitude = {}", y_max_altitude);
}
