use std::convert::TryInto;

pub fn run_program(stack: &mut Vec<u32>) {
    const OPCODE_ADD: usize = 1;
    const OPCODE_MULTIPLY: usize = 2;
    const OPCODE_END: usize = 99;

    let mut index: usize = 0;
    while index < std::usize::MAX {
        let opcode: usize = stack[index].try_into().unwrap();

        match opcode {
            OPCODE_ADD => add(stack, index),
            OPCODE_MULTIPLY => multiply(stack, index),
            // If we're done, just exit.
            OPCODE_END => return,
            // Not 100% sure what this is. Fall-through?
            _ => return
        }

        // Operation done - advance index.
        index += 4;
    }
}

fn add(stack: &mut Vec<u32>, index: usize) {
    let first_operand_index: usize = (index + 1).try_into().unwrap();
    let first_operand_position: usize = stack[first_operand_index].try_into().unwrap();

    let second_operand_index: usize = (index + 2).try_into().unwrap();
    let second_operand_position: usize = stack[second_operand_index].try_into().unwrap();

    let result_index: usize = (index + 3).try_into().unwrap();
    let result_position: usize = stack[result_index].try_into().unwrap();

    let first_operand: u32 = stack[first_operand_position].try_into().unwrap();
    let second_operand: u32 = stack[second_operand_position].try_into().unwrap();
    let result: u32 = first_operand + second_operand;
    stack[result_position] = result.try_into().unwrap();
}

fn multiply(stack: &mut Vec<u32>, index: usize) {
    let first_operand_index: usize = (index + 1).try_into().unwrap();
    let first_operand_position: usize = stack[first_operand_index].try_into().unwrap();

    let second_operand_index: usize = (index + 2).try_into().unwrap();
    let second_operand_position: usize = stack[second_operand_index].try_into().unwrap();

    let result_index: usize = (index + 3).try_into().unwrap();
    let result_position: usize = stack[result_index].try_into().unwrap();

    let first_operand: u32 = stack[first_operand_position].try_into().unwrap();
    let second_operand: u32 = stack[second_operand_position].try_into().unwrap();
    let result: u32 = first_operand * second_operand;
    stack[result_position] = result.try_into().unwrap();
}

#[test]
fn simple_addition_test() {
    let mut input_stack: Vec<u32> = vec![1,0,0,0,99];
    let expected_stack: Vec<u32> = vec![2,0,0,0,99];

    run_program(&mut input_stack);

    assert_eq!(input_stack, expected_stack);
}

#[test]
fn simple_multiplication_test() {
    let mut input_stack: Vec<u32> = vec![2,3,0,3,99];
    let expected_stack: Vec<u32> = vec![2,3,0,6,99];

    run_program(&mut input_stack);

    assert_eq!(input_stack, expected_stack);
}

#[test]
fn less_simple_multiplication_test() {
    let mut input_stack: Vec<u32> = vec![2,4,4,5,99,0];
    let expected_stack: Vec<u32> = vec![2,4,4,5,99,9801];

    run_program(&mut input_stack);

    assert_eq!(input_stack, expected_stack);
}

#[test]
fn complicated_test() {
    let mut input_stack: Vec<u32> = vec![1,1,1,4,99,5,6,0,99];
    let expected_stack: Vec<u32> = vec![30,1,1,4,2,5,6,0,99];

    run_program(&mut input_stack);

    assert_eq!(input_stack, expected_stack);
}