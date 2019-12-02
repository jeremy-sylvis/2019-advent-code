use math::round;

pub fn get_fuel_required_for_mass(mass: i32) -> i32 {
    let third_of_mass: f64 = (mass / 3).into();
    let rounded_third_of_mass: i32 = round::floor(third_of_mass, 0) as i32;
    let fuel_required: i32 = rounded_third_of_mass - 2;
    return fuel_required;
}

#[test]
fn fuel_required_for_12_is_2() {
    let fuel_required: i32 = get_fuel_required_for_mass(12);
    assert_eq!(fuel_required, 2);
}

#[test]
fn fuel_required_for_14_is_2() {
    let fuel_required: i32 = get_fuel_required_for_mass(14);
    assert_eq!(fuel_required, 2);
}

#[test]
fn fuel_required_for_1969_is_654() {
    let fuel_required: i32 = get_fuel_required_for_mass(1969);
    assert_eq!(fuel_required, 654);
}

#[test]
fn fuel_required_for_mass_of_100756_is_33583() {
    let fuel_required: i32 = get_fuel_required_for_mass(100756);
    assert_eq!(fuel_required, 33583);
}