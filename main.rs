use std::fmt;

struct Vector {
    w: i32,
    x: i32,
    y: i32,
    z: i32,
}

impl Vector {
    fn new() -> Vector {
        Vector {w: 0, x: 0, y: 0, z: 0}
    }
}

impl fmt::Display for Vector {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "({}, {}, {}, {})", self.w, self.x, self.y, self.z)
    }
}

fn main() {
    let axiom = "F";
    let mut result = axiom;
    let rules = [("F", "FF")];

    for rule in &rules {
        if result == rule.0 {
            result = rule.1
        }
    }

    println!("Result: {}", result);

    let v = Vector::new();
    println!("Vector: {}", v);
}

