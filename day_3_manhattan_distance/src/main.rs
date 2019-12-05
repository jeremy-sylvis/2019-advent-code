extern crate nalgebra;

use nalgebra::geometry::Point2;

pub mod point_parser;
pub mod direction_parser;
pub mod geometry_math;

use std::io;
use std::io::prelude::*;

fn main() {
    // debug hack
    // line_calculator::simple_intersect_testcase_1();
    // return;

    let stdin = io::stdin();
    let stdout = io::stdout();

    println!("Please enter the directions:");

    // Read in the line
    let mut buffer = String::new();
    match stdin.read_line(&mut buffer) {
        Ok(n) => println!("Read {} bytes.", n),
        Err(error) => println!("Encountered error: {}", error)
    };

    // Translate to a set of directions
    let directions: Vec<&str> = direction_parser::parse_directions(&buffer);

    // Translate to a set of points
    let route_points: Vec<Point2<f32>> = point_parser::parse_points(&directions);

    
}

#[test]
fn distance_check_testcase_1() {
    let first_path: &str = "R75,D30,R83,U83,L12,D49,R71,U7,L72";
    let first_directions: Vec<&str> = direction_parser::parse_directions(first_path);
    let first_points: Vec<Point2<f32>> = point_parser::parse_points(&first_directions);

    let second_path: &str = "U62,R66,U55,R34,D71,R55,D58,R83";
    let second_directions: Vec<&str> = direction_parser::parse_directions(second_path);
    let second_points: Vec<Point2<f32>> = point_parser::parse_points(&second_directions);

    // for point in &points {
    //     println!("{}", *point);
    // }

    let intersections: Vec<Point2<f32>> = geometry_math::get_points_of_intersection(&first_points, &second_points);
    let closest_point: &Point2<f32>;
    let result = geometry_math::get_point_closest_to_origin_by_taxicab_distance(&intersections);
    match result {
        None => panic!("Could not find close point."),
        Some(value) => closest_point = value
    };

    let taxicab_distance: f32 = geometry_math::get_taxicab_distance(&closest_point);
    assert_eq!(159.0, taxicab_distance);
}

#[test]
fn distance_check_tescase_2() {
    let first_path: &str = "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51";
    let first_directions: Vec<&str> = direction_parser::parse_directions(first_path);
    let first_points: Vec<Point2<f32>> = point_parser::parse_points(&first_directions);

    let second_path: &str = "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7";
    let second_directions: Vec<&str> = direction_parser::parse_directions(second_path);
    let second_points: Vec<Point2<f32>> = point_parser::parse_points(&second_directions);

    // for point in &points {
    //     println!("{}", *point);
    // }

    let intersections: Vec<Point2<f32>> = geometry_math::get_points_of_intersection(&first_points, &second_points);
    let closest_point: &Point2<f32>;
    let result = geometry_math::get_point_closest_to_origin_by_taxicab_distance(&intersections);
    match result {
        None => panic!("Could not find close point."),
        Some(value) => closest_point = value
    };

    let taxicab_distance: f32 = geometry_math::get_taxicab_distance(&closest_point);
    assert_eq!(135.0, taxicab_distance);
}

#[test]
fn site_simple_testcase() {
    let first_path: &str = "R8,U5,L5,D3";
    let first_directions: Vec<&str> = direction_parser::parse_directions(first_path);
    let first_points: Vec<Point2<f32>> = point_parser::parse_points(&first_directions);
    assert_eq!(5, first_points.len());

    let second_path: &str = "U7,R6,D4,L4";
    let second_directions: Vec<&str> = direction_parser::parse_directions(second_path);
    let second_points: Vec<Point2<f32>> = point_parser::parse_points(&second_directions);
    assert_eq!(5, second_points.len());

    let intersections: Vec<Point2<f32>> = geometry_math::get_points_of_intersection(&first_points, &second_points);
    assert_eq!(2, intersections.len());
}