fn main() {
    const INPUT: i32 = 265149;

    let mut low_bound: i32 = 1;
    let mut up_bound: i32 = 9;
    let mut i: i32 = 1;
    let mut j: i32 = 2;

    while !(INPUT > low_bound && INPUT <= up_bound) {
        low_bound += 8 * i;
        up_bound += 8 * j;
        i += 1;
        j += 1;
    }

    println!("low: {} high: {}", low_bound, up_bound);

    let least_steps: i32 = (up_bound - low_bound) / 8;

    println!("Least steps: {}", least_steps);

    let mut left_corner: i32 = 1;
    let mut right_corner: i32 = 1;

    for i in 0..least_steps {
        left_corner = left_corner + (6 + (8 * i));
        right_corner = right_corner + (2 + (8 * i));
    }

    println!("Right corner: {} Left Corner: {}", right_corner, left_corner);

    let left_center: i32 = left_corner - least_steps;
    let bottom_center: i32 = left_corner + least_steps;
    let right_center: i32 = right_corner - least_steps;
    let top_center: i32 = right_corner + least_steps;

    let mut left_diff: i32 = INPUT - left_center;
    let mut bottom_diff: i32 = INPUT - bottom_center;
    let mut right_diff: i32 = INPUT - right_center;
    let mut top_diff: i32 = INPUT - top_center;

    if left_diff < 0 {left_diff = -1 * left_diff;}
    if bottom_diff < 0 {bottom_diff = -1 * bottom_diff;}
    if right_diff < 0 {right_diff = -1 * right_diff;}
    if top_diff < 0 {top_diff = -1 * top_diff;}

    let mut total_steps: i32 = 0;
    if left_diff < bottom_diff && left_diff < right_diff && left_diff < top_diff {
        println!("left_diff is lowest");
        total_steps = left_diff + least_steps;
    }
    if bottom_diff < left_diff && bottom_diff < right_diff && bottom_diff < top_diff {
        println!("bottom_diff is lowest");
        println!("{}, {}", bottom_center, bottom_diff);
        total_steps = bottom_diff + least_steps;
    }
    if right_diff < left_diff && right_diff < bottom_diff && right_diff < top_diff {total_steps = right_diff + least_steps;}
    if top_diff < left_diff && top_diff < bottom_diff && top_diff < right_diff {total_steps = top_diff + least_steps;}

    println!("Total Steps: {}", total_steps);

}