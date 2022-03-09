use rand::Rng;

#[derive(Copy, Clone)]
struct Point {
    x: f64,
    y: f64,
}

struct Square {
    top: Point,
    bottom: Point,
}

impl Square {
    fn new(top: Point, side: f64) -> Self {
        Square {
            top: top,
            bottom: Point {
                x: top.x + side,
                y: top.y - side,
            }
        }
    }
}

fn overlap(sq1: &Square, sq2: &Square) -> bool {
    // If one square is on left side of other
    if sq1.top.x >= sq2.bottom.x || sq2.top.x >= sq1.bottom.x {
        return false;
    }
    // If one square is above other
    if sq1.bottom.y >= sq2.top.y || sq2.bottom.y >= sq1.top.y {
        return false;
    }
    return true;
}

fn main() {
    let square1 = Square::new(Point {x: 3., y: 6.}, 3.);
    let square2 = Square::new(Point {x: 5., y: 5.}, 4.);
    let result = overlap(&square1, &square2);
    println!("{}", result);

    let mut rng = rand::thread_rng();
    // Randomly create 100 squares
    let mut squares: Vec<Square> = Vec::new();
    for _ in 0..100 {
        let side = rng.gen_range(0., 1000.);
        squares.push(Square::new(Point { x: rng.gen_range(0., 1000.), y: rng.gen_range(0., 1000.) }, side));
    }

    for i in 0..100 {
        for j in i+1..100 {
            let overlap = overlap(&squares[j], &squares[i]);
            if overlap {
                println!("{}-th square overlap to {}-th square", i, j);
            }   
        }
    }
}

