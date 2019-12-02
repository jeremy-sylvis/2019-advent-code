use math::round;

pub fn get_fuel_required_for_mass(mass: i32) -> i32 {
    let third_of_mass: f64 = (mass / 3).into();
    let rounded_third_of_mass: i32 = round::floor(third_of_mass, 0) as i32;
    let fuel_required: i32 = rounded_third_of_mass - 2;

    // Oh snap, getting recursive

    // If the fuel required was negative, return early
    if fuel_required < 0 {
        return fuel_required;
    }

    // Get the fuel required for our current set of fuel.
    let fuel_required_for_fuel: i32 = get_fuel_required_for_mass(fuel_required);

    // If it was negative, we're done - we have our sum.
    if fuel_required_for_fuel < 0 {
        return fuel_required;
    }

    // Otherwise, calculate the new sum and hand it back.
    let fuel_sum: i32 = fuel_required + fuel_required_for_fuel;
    return fuel_sum;
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
fn fuel_required_for_1969_is_966() {
    let fuel_required: i32 = get_fuel_required_for_mass(1969);
    assert_eq!(fuel_required, 966);
}

#[test]
fn fuel_required_for_mass_of_100756_is_50346() {
    let fuel_required: i32 = get_fuel_required_for_mass(100756);
    assert_eq!(fuel_required, 50346);
}