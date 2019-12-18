use nalgebra::geometry::Point2;
use std::cmp;

struct LineSegment<'a,'b> {
    point_a: &'a Point2<f32>,
    point_b: &'b Point2<f32>
}

pub fn get_points_of_intersection(first_path: &Vec<Point2<f32>>, second_path: &Vec<Point2<f32>>) -> Vec<Point2<f32>> {
    let mut results: Vec<Point2<f32>> = Vec::new();
    
    // Nested iteration. Offset the end index by 2 to account for the fact that this is a list of the elements used to make segments (2 per segment),
    // rather than segments themselves.
    for first_index in 0..first_path.len() - 2 {
        let first_point_a: Point2<f32> = first_path[first_index];
        let first_point_b: Point2<f32> = first_path[first_index + 1];
        let first_segment = LineSegment {
            point_a: &first_point_a,
            point_b: &first_point_b
        };

        for second_index in 0..second_path.len() - 2 {
            let second_point_a: Point2<f32> = second_path[second_index];
            let second_point_b: Point2<f32> = second_path[second_index + 1];
        
            let second_segment = LineSegment{
                point_a: &second_point_a,
                point_b: &second_point_b
            };

            let intersection_result: Option<Point2<f32>> = get_point_of_intersection(&first_segment, &second_segment);           
            match intersection_result {
                Some(point) => {
                    println!("Segment A {}->{}\tSegment B {}->{}\tIntersection at {}", first_point_a, first_point_b, second_point_a, second_point_b, point);
                    let mut does_point_exist: bool = false;
                    for existing_point in results.iter() {
                        if existing_point.coords.x == point.coords.x && existing_point.coords.y == point.coords.y {
                            does_point_exist = true;
                        }
                    }

                    if !does_point_exist {
                        results.push(point);
                    }
                },
                None => {
                    //println!("Segment A {}->{}\tSegment B {}->{}\tNo intersection", first_point_a, first_point_b, second_point_a, second_point_b);
                    continue;
                }
            }
        }
    }

    return results;
}

#[test]
fn site_testcase() {
    let first_path_points: Vec<Point2<f32>> = vec![
        Point2::new(0.0, 0.0),
        Point2::new(0.0, 7.0),
        Point2::new(6.0, 7.0),
        Point2::new(6.0, 3.0),
        Point2::new(2.0, 3.0)
    ];

    let second_path_points: Vec<Point2<f32>> = vec![
        Point2::new(0.0, 0.0),
        Point2::new(8.0, 0.0),
        Point2::new(8.0, 5.0),
        Point2::new(3.0, 5.0),
        Point2::new(3.0, 2.0)
    ];

    let result: Vec<Point2<f32>> = get_points_of_intersection(&first_path_points, &second_path_points);
    assert_eq!(2, result.len());
}

// Given two linesegments, get the Point of Intersection or None.
// shamelessly adapted from: https://stackoverflow.com/a/14795484
fn get_point_of_intersection(first_segment: &LineSegment, second_segment: &LineSegment) -> Option<Point2<f32>> {
    let s_ba_x: f32 = first_segment.point_b.coords.x - first_segment.point_a.coords.x;
    let s_ba_y: f32 = first_segment.point_b.coords.y - first_segment.point_a.coords.y;
    
    let s_dc_x: f32 = second_segment.point_b.coords.x - second_segment.point_a.coords.x;
    let s_dc_y: f32 = second_segment.point_b.coords.y - second_segment.point_a.coords.y;

    let denominator: f32 = s_ba_x * s_dc_y - s_dc_x * s_ba_y;

    if denominator == 0.0 {
        // This indicates colinear lines
        return None;
    }

    let was_denominator_positive: bool = denominator > 0.0;

    let s_ac_x: f32 = first_segment.point_a.coords.x - second_segment.point_a.coords.x;
    let s_ac_y: f32 = first_segment.point_a.coords.y - second_segment.point_a.coords.y;

    let s_numerator: f32 = s_ba_x * s_ac_y - s_ba_y * s_ac_x;
    let was_s_numerator_negative: bool = s_numerator < 0.0;

    if was_s_numerator_negative && was_denominator_positive {
        // No collision
        return None;
    }

    let t_numerator: f32 = s_dc_x * s_ac_y - s_dc_y * s_ac_x;
    let was_t_numerator_negative: bool = t_numerator < 0.0;

    if was_t_numerator_negative && was_denominator_positive {
        // No collision
        return None;
    }

    if (s_numerator > denominator && was_denominator_positive) || (t_numerator > denominator && was_denominator_positive) {
        // No collision
        return None;
    }

    let t: f32 = t_numerator / denominator;
    let x: f32 = first_segment.point_a.coords.x + (t * s_ba_x);
    let y: f32 = first_segment.point_a.coords.y + (t * s_ba_y);

    let intersection: Point2<f32> = Point2::new(x, y);

    // Ensure the intersection point is actually bounded by the line segments
    // It has to exist on *both* segments.
    let first_segment_min_x: f32 = first_segment.point_a.coords.x.min(first_segment.point_b.coords.x);
    let first_segment_max_x: f32 = first_segment.point_a.coords.x.max(first_segment.point_b.coords.x);
    let second_segment_min_x: f32 = second_segment.point_a.coords.x.min(second_segment.point_b.coords.x);
    let second_segment_max_x: f32 = second_segment.point_a.coords.x.max(second_segment.point_b.coords.x);

    let first_segment_min_y: f32 = first_segment.point_a.coords.y.min(first_segment.point_b.coords.y);
    let first_segment_max_y: f32 = first_segment.point_a.coords.y.max(first_segment.point_b.coords.y);
    let second_segment_min_y: f32 = second_segment.point_a.coords.y.min(second_segment.point_b.coords.y);
    let second_segment_max_y: f32 = second_segment.point_a.coords.y.max(second_segment.point_b.coords.y);

    let does_intersection_exist_on_first_segment: bool = first_segment_min_x <= intersection.coords.x && first_segment_max_x >= intersection.coords.x &&
        first_segment_min_y <= intersection.coords.y && first_segment_max_y >= intersection.coords.y;
    let does_intersection_exist_on_second_segment: bool = second_segment_min_x <= intersection.coords.x && second_segment_max_x >= intersection.coords.x &&
        second_segment_min_y <= intersection.coords.y && second_segment_max_y >= intersection.coords.y;

    if does_intersection_exist_on_first_segment && does_intersection_exist_on_second_segment {
        return Option::from(intersection);
    }
    
    return None;
}

#[test]
fn simple_intersect_testcase_1() {
    let point_a: Point2<f32> = Point2::new(-3.0, 0.0);
    let point_b: Point2<f32> = Point2::new(3.0, 0.0);
    let point_c: Point2<f32> = Point2::new(0.0, 5.0);
    let point_d: Point2<f32> = Point2::new(0.0, -5.0);

    let line_segment_a: LineSegment = LineSegment {
        point_a: &point_a,
        point_b: &point_b
    };

    let line_segment_b: LineSegment = LineSegment {
        point_a: &point_c,
        point_b: &point_d
    };

    let result: Option<Point2<f32>> = get_point_of_intersection(&line_segment_a, &line_segment_b);
    assert_ne!(None, result);
    
    let result_point: Point2<f32> = result.unwrap();
    assert_eq!(0.0, result_point.coords.x);
    assert_eq!(0.0, result_point.coords.y);
}

#[test]
fn double_negative_testcase() {
    let point_a: Point2<f32> = Point2::new(0.0, -2.5);
    let point_b: Point2<f32> = Point2::new(-5.0, -2.5);
    let point_c: Point2<f32> = Point2::new(-2.5, 0.0);
    let point_d: Point2<f32> = Point2::new(-2.5, -5.0);

    let line_segment_a: LineSegment = LineSegment {
        point_a: &point_a,
        point_b: &point_b
    };

    let line_segment_b: LineSegment = LineSegment {
        point_a: &point_c,
        point_b: &point_d
    };

    let result: Option<Point2<f32>> = get_point_of_intersection(&line_segment_a, &line_segment_b);
    assert_ne!(None, result);
    
    let result_point: Point2<f32> = result.unwrap();
    assert_eq!(-2.5, result_point.coords.x);
    assert_eq!(-2.5, result_point.coords.y);
}

#[test]
fn should_not_intersect_testcase() {
    let point_a: Point2<f32> = Point2::new(0.0,0.0);
    let point_b: Point2<f32> = Point2::new(0.0,7.0);

    let first_segment: LineSegment = LineSegment{
        point_a: &point_a,
        point_b: &point_b
    };

    let point_c: Point2<f32> = Point2::new(8.0,0.0);
    let point_d: Point2<f32> = Point2::new(8.0,5.0);

    let second_segment: LineSegment = LineSegment {
        point_a: &point_c,
        point_b: &point_d
    };

    let result = get_point_of_intersection(&first_segment, &second_segment);
    assert_eq!(None, result);
}

pub fn get_taxicab_distance(point: &Point2<f32>) -> f32 {
    return point.coords.x.abs() + point.coords.y.abs();
}

#[test]
fn positive_x_taxicab_distance_testcase() {
    let point: Point2<f32> = Point2::new(1.0, 0.0);
    let taxicab_distance: f32 = get_taxicab_distance(&point);

    assert_eq!(1.0, taxicab_distance);
}

#[test]
fn positive_y_taxicab_distance_testcase() {
    let point: Point2<f32> = Point2::new(0.0, 1.0);
    let taxicab_distance: f32 = get_taxicab_distance(&point);

    assert_eq!(1.0, taxicab_distance);
}

#[test]
fn negative_x_taxicab_distance_testcase() {
    let point: Point2<f32> = Point2::new(-1.0, 0.0);
    let taxicab_distance: f32 = get_taxicab_distance(&point);

    assert_eq!(1.0, taxicab_distance);
}

#[test]
fn negative_y_taxicab_distance_testcase() {
    let point: Point2<f32> = Point2::new(0.0, -1.0);
    let taxicab_distance: f32 = get_taxicab_distance(&point);

    assert_eq!(1.0, taxicab_distance);
}

#[test]
fn double_negative_taxicab_distance_testcase() {
    let point: Point2<f32> = Point2::new(-31.0, -14.0);
    let taxicab_distance: f32 = get_taxicab_distance(&point);

    assert_eq!(45.0, taxicab_distance);
}

// Get the point closest to origin, if any.
pub fn get_point_closest_to_origin_by_taxicab_distance(points: &Vec<Point2<f32>>) -> Option<&Point2<f32>> {
    let mut closest_point: Option<&Point2<f32>> = None;
    let mut closest_distance: f32 = -1.0;

    for point in points {
        // The origin is not a valid point
        if point.coords.x == 0.0 && point.coords.y == 0.0 {
            continue;
        }

        let current_distance = get_taxicab_distance(&point);

        if closest_distance < 0.0 || current_distance < closest_distance {
            closest_distance = current_distance;
            closest_point = Option::from(point);
        }
    }

    return closest_point;
}

#[test]
fn simple_testcase() {
    let points: Vec<Point2<f32>> = vec![Point2::new(1.0, 1.0), Point2::new(2.0, 0.0)];

    let closest_point: &Point2<f32>;
    let result = get_point_closest_to_origin_by_taxicab_distance(&points);
    match result {
        None => panic!("Could not find point closest to origin."),
        Some(value) => closest_point = value
    }

    assert_eq!(1.0, closest_point.coords.x);
    assert_eq!(1.0, closest_point.coords.y);
}