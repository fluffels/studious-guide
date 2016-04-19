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
}

