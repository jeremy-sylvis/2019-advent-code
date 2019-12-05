use nalgebra::geometry::Point2;
use std::str::FromStr;

pub fn parse_points(directions: &Vec<&str>) -> Vec<Point2<f32>> {
    const DIRECTION_UP: char = 'U';
    const DIRECTION_RIGHT: char = 'R';
    const DIRECTION_DOWN: char = 'D';
    const DIRECTION_LEFT: char = 'L';

    let mut last_point: Point2<f32> = Point2::new(0.0,0.0);

    let mut points: Vec<Point2<f32>> = Vec::new();
    // Push our origin point
    points.push(last_point);

    for step in directions {
        let char_bytes: &[u8] = step.as_bytes();
        // Get direction from the text
        let direction_char: char = char_bytes[0] as char;
        
        // Get the actual magnitude value from the text
        let mut magnitude_text: String = String::new();
        for index in 1..char_bytes.len() {
            let character_byte: u8 = char_bytes[index];
            let character: char = character_byte as char;
            magnitude_text.push(character);
        }
        
        let value: f32 = f32::from_str(&magnitude_text).unwrap();

        // Use the direction we're moving in relation to last point, plus the value, to calculate the new point

        // Notes: This is a bit gross.
        // In dotnet-land, I could use a lambda expression to describe the property access I'd do on the fly and re-use it later, e.g.
        // in this expression I could identify if I'd be modifying X or Y and just run that on a point object.
        // Here... I don't know how to pull off a property expression like that.
        // This code is highly duplicated as a result.
        let x_value: f32;
        let y_value: f32;
        match direction_char {
            // Positive Y
            DIRECTION_UP => {
                x_value = last_point.coords.x;
                y_value = last_point.coords.y + value;
            },
            // Positive X
            DIRECTION_RIGHT => {
                x_value = last_point.coords.x + value;
                y_value = last_point.coords.y;
            },
            // Negative Y
            DIRECTION_DOWN => {
                x_value = last_point.coords.x;
                y_value = last_point.coords.y - value;
            },
            // Negative X
            DIRECTION_LEFT => {
                x_value = last_point.coords.x - value;
                y_value = last_point.coords.y;
            },
            // Unknown - panic 'o clock
            _ => {
                panic!("Unknown direction {}", direction_char);
            }
        }

        let new_point: Point2<f32> = Point2::new(x_value, y_value);
        last_point = new_point;
        points.push(new_point);
    }

    return points;
}

#[test]
fn simple_up_testcase() {
    let directions: Vec<&str> = vec!["U50"];

    let points: Vec<Point2<f32>> = parse_points(&directions);

    let origin: Point2<f32> = points[0];
    assert_eq!(0.0, origin.coords.x);
    assert_eq!(0.0, origin.coords.y);

    let point: Point2<f32> = points[1];
    assert_eq!(0.0, point.coords.x);
    assert_eq!(50.0, point.coords.y);
}

#[test]
fn simple_right_testcase() {
    let directions: Vec<&str> = vec!["R50"];

    let points: Vec<Point2<f32>> = parse_points(&directions);

    let origin: Point2<f32> = points[0];
    assert_eq!(0.0, origin.coords.x);
    assert_eq!(0.0, origin.coords.y);

    let point: Point2<f32> = points[1];
    assert_eq!(50.0, point.coords.x);
    assert_eq!(0.0, point.coords.y);
}

#[test]
fn simple_down_testcase() {
    let directions: Vec<&str> = vec!["D50"];

    let points: Vec<Point2<f32>> = parse_points(&directions);

    let origin: Point2<f32> = points[0];
    assert_eq!(0.0, origin.coords.x);
    assert_eq!(0.0, origin.coords.y);

    let point: Point2<f32> = points[1];
    assert_eq!(0.0, point.coords.x);
    assert_eq!(-50.0, point.coords.y);
}

#[test]
fn simple_left_testcase() {
    let directions: Vec<&str> = vec!["L50"];

    let points: Vec<Point2<f32>> = parse_points(&directions);

    let origin: Point2<f32> = points[0];
    assert_eq!(0.0, origin.coords.x);
    assert_eq!(0.0, origin.coords.y);

    let point: Point2<f32> = points[1];
    assert_eq!(-50.0, point.coords.x);
    assert_eq!(0.0, point.coords.y);
}

#[test]
fn return_to_origin_testcase() {
    let directions: Vec<&str> = vec!["U50", "R50", "D100", "L100", "U50", "R50"];

    let points: Vec<Point2<f32>> = parse_points(&directions);

    let origin: Point2<f32> = points[0];
    assert_eq!(0.0, origin.coords.x);
    assert_eq!(0.0, origin.coords.y);

    let first_up: Point2<f32> = points[1];
    assert_eq!(0.0, first_up.coords.x);
    assert_eq!(50.0, first_up.coords.y);

    let first_right: Point2<f32> = points[2];
    assert_eq!(50.0, first_right.coords.x);
    assert_eq!(50.0, first_right.coords.y);

    let first_down: Point2<f32> = points[3];
    assert_eq!(50.0, first_down.coords.x);
    assert_eq!(-50.0, first_down.coords.y);

    let first_left: Point2<f32> = points[4];
    assert_eq!(-50.0, first_left.coords.x);
    assert_eq!(-50.0, first_down.coords.y);

    let last: Point2<f32> = points[points.len() - 1];
    assert_eq!(0.0, last.coords.x);
    assert_eq!(0.0, last.coords.y);
}