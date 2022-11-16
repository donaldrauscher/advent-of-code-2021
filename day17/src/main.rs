fn trace(x_velocity_initial: isize,
         y_velocity_initial: isize,
         x_box: &Vec<isize>,
         y_box: &Vec<isize>) -> Option<(isize, isize)> {
    let mut success: bool = false;

    let mut y: isize = y_velocity_initial;
    let mut x: isize = x_velocity_initial;

    let mut y_velocity: isize = y_velocity_initial;
    let mut x_velocity: isize = x_velocity_initial;

    loop {
        if (y >= y_box[0]) && (y <= y_box[1]) && (x >= x_box[0]) && (x <= x_box[1]) {
            success = true;
            break;
        }
        if (y < y_box[0]) || (x >= x_box[1]) {
            break
        }
        y_velocity -= 1;
        x_velocity = if x_velocity == 0 { 0 } else { x_velocity - 1 };
        y += y_velocity;
        x += x_velocity;
    }

    if success {
        return Some((x_velocity_initial, y_velocity_initial));
    } else {
        return None;
    }
}

fn main() {
    let x_y = include_str!("../input.txt")
        .trim()
        .trim_start_matches("target area: x=")
        .split_once(", y=")
        .unwrap();

    let x_box = x_y.0
        .splitn(2, "..")
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    let y_box = x_y.1
        .splitn(2, "..")
        .map(|n| n.parse::<isize>().unwrap())
        .collect::<Vec<_>>();

    // symmetry!  the probe will always return to y=0
    // find max velocity that keeps it in the box and back-out altitude
    let y_max_velocity = -y_box[0] - 1;
    let y_max_altitude = y_max_velocity * (y_max_velocity + 1) / 2;
    println!("Max altitude = {}", y_max_altitude);

    let y_min_velocity = y_box[0];
    let x_max_velocity = x_box[1];

    // use quadratic formula to determine minimum x velocity that gets us to start of box
    let x_min_velocity: isize = ((-1.0_f64 + ((1 + 4*2*x_box[0]) as f64).sqrt())/2.0_f64).ceil() as isize;

    let mut initial_velocities: Vec<(isize, isize)> = Vec::new();
    for x in x_min_velocity..=x_max_velocity {
        for y in y_min_velocity..=y_max_velocity {
            if let Some((xx, yy)) = trace(x, y, &x_box, &y_box) {
                initial_velocities.push((xx, yy));
            }
        }
    }

    println!("Number of initial velocities = {:?}", initial_velocities.len());
}
