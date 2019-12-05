pub fn parse_directions(text: &str) -> Vec<&str> {
    let segments: Vec<&str> = text.trim_end().split(',').collect();

    return segments;
}

#[test]
fn first_testcase() {
    let text: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72,U62,R66,U55,R34,D71,R55,D58,R83";

    let directions: Vec<&str> = parse_directions(text);
    
    let expected_directions: Vec<&str> = vec!["R75", "D30", "R83", "U83", "L12", "D49", "R71", "U7", "L72", "U62", "R66", "U55", "R34", "D71", "R55", "D58", "R83"];
    assert_eq!(directions, expected_directions);
}

#[test]
fn second_testcase() {
    let text: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51,U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";

    let directions: Vec<&str> = parse_directions(text);
    
    let expected_directions: Vec<&str> = vec!["R98", "U47", "R26", "D63", "R33", "U87", "L62", "D20", "R33", "U53", "R51",
        "U98", "R91", "D20", "R16", "D67", "R40", "U7", "R15", "U6", "R7"];
    assert_eq!(directions, expected_directions);
}