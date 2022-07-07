// option3.rs
// Make me compile! Execute `rustlings hint option3` for hints

// I AM NOT DONE

struct Point {
    x: i32,
    y: i32,
}

fn main() {
    let y: Option<Point> = Some(Point { x: 100, y: 200 });

    match y {
        // remember that (&) is in pattern while (ref) is not.
        // ref denote that p is borrowed instead of owned.
        Some(ref p) => println!("Co-ordinates are {},{} ", p.x, p.y),
        _ => println!("no match"),
    }
    y; // Fix without deleting this line.
}
