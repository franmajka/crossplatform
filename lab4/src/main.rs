#[repr(u8)]
enum Condition {
  Unconditional,
  Equal,
  NotEqual,
  LessThan,
  GreaterThan,
  LessThanOrEqual,
  GreaterThanOrEqual,
}

impl Into<u8> for Condition {
  fn into(self) -> u8 {
    self as u8
  }
}

impl From<u8> for Condition {
  fn from(value: u8) -> Self {
    match value {
      0 => Condition::Unconditional,
      1 => Condition::Equal,
      2 => Condition::NotEqual,
      3 => Condition::LessThan,
      4 => Condition::GreaterThan,
      5 => Condition::LessThanOrEqual,
      6 => Condition::GreaterThanOrEqual,
      _ => panic!("Invalid condition"),
    }
  }
}

#[repr(u8)]
enum Instruction {
  /// Store local variable
  /// ### Arguments
  /// * `index`: u8 - index of local variable
  Store,
  /// Store constant to local variable
  /// ### Arguments
  /// * `index`: u8 - index of local variable
  /// * `constant`: u8 - value to store
  StoreConstant,
  /// Load onto operand stack
  /// ### Arguments
  /// * `index`: u8 - index of local variable
  Load,
  /// Loads a constant onto the operand stack
  /// ### Arguments
  /// * `constant`: u8 - value to load
  LoadConstant,
  /// Creates a label at the current position
  /// ### Arguments
  /// * `label`: u8 - label index
  Label,
  /// Jumps to a label and clears the operand stack
  /// ### Arguments
  /// * `condition`: Condition - condition to jump
  /// * `label`: u8 - label index
  Jump,
  /// Adds two values on the operand stack and pushes the result
  Add,
  /// Prints the value on top of the operand stack and pops it
  Print,
  /// Halts the program
  Halt,
}

impl Into<u8> for Instruction {
  fn into(self) -> u8 {
    self as u8
  }
}

impl From<u8> for Instruction {
  fn from(value: u8) -> Self {
    match value {
      0 => Instruction::Store,
      1 => Instruction::StoreConstant,
      2 => Instruction::Load,
      3 => Instruction::LoadConstant,
      4 => Instruction::Label,
      5 => Instruction::Jump,
      6 => Instruction::Add,
      7 => Instruction::Print,
      8 => Instruction::Halt,
      _ => panic!("Invalid instruction"),
    }
  }
}

fn main() {
  /*
    let mut num1: u8 = 0;
    let mut num2: u8 = 1;
    let mut sum: u8;
    for i in 0..10 {
      sum = num1 + num2;
      num1 = num2;
      num2 = sum;
      println!("{sum}");
    }
  */
  let byte_code: &[u8] = &[
    Instruction::StoreConstant.into(), 0, 0,
    Instruction::StoreConstant.into(), 1, 1,
    Instruction::Store.into(), 2,

    Instruction::StoreConstant.into(), 3, 0,
    Instruction::Label.into(), 0,
    Instruction::Load.into(), 3,
    Instruction::LoadConstant.into(), 10,
    Instruction::Jump.into(), Condition::GreaterThanOrEqual.into(), 1,

    Instruction::Load.into(), 0,
    Instruction::Load.into(), 1,
    Instruction::Add.into(),
    Instruction::Store.into(), 2,

    Instruction::Load.into(), 1,
    Instruction::Store.into(), 0,

    Instruction::Load.into(), 2,
    Instruction::Store.into(), 1,

    Instruction::Load.into(), 2,
    Instruction::Print.into(),

    Instruction::Load.into(), 3,
    Instruction::LoadConstant.into(), 1,
    Instruction::Add.into(),
    Instruction::Store.into(), 3,
    Instruction::Jump.into(), Condition::Unconditional.into(), 0,

    Instruction::Label.into(), 1,
    Instruction::Halt.into(),
  ];

  let mut local_variables = [0; 256];
  let mut operand_stack = [0; 3];
  let mut stack_pointer: usize = 0;
  let mut labels = [u8::MAX; 256];

  let instructions_length = [2u8, 3, 2, 2, 2, 3, 1, 1, 1];

  let mut instruction_pointer = 0;

  'outer: loop {
    let instruction: Instruction = byte_code[instruction_pointer].into();
    match instruction {
      Instruction::Store => {
        let index = byte_code[instruction_pointer + 1];
        let value = operand_stack[0];
        local_variables[index as usize] = value;
        stack_pointer = 0;
      },
      Instruction::StoreConstant => {
        let index = byte_code[instruction_pointer + 1];
        let constant = byte_code[instruction_pointer + 2];
        local_variables[index as usize] = constant;
      },
      Instruction::Load => {
        let index = byte_code[instruction_pointer + 1];
        let value = local_variables[index as usize];
        operand_stack[stack_pointer] = value;
        stack_pointer += 1;
      },
      Instruction::LoadConstant => {
        let constant = byte_code[instruction_pointer + 1];
        operand_stack[stack_pointer] = constant;
        stack_pointer += 1;
      },
      Instruction::Label => {
        let label = byte_code[instruction_pointer + 1];
        labels[label as usize] = (instruction_pointer + 2) as u8;
      },
      Instruction::Jump => {
        let condition = byte_code[instruction_pointer + 1].into();
        let label = byte_code[instruction_pointer + 2];

        let a = operand_stack[0];
        let b = operand_stack[1];
        stack_pointer = 0;

        let will_jump = match condition {
          Condition::Unconditional => true,
          Condition::Equal => a == b,
          Condition::NotEqual => a != b,
          Condition::LessThan => a < b,
          Condition::GreaterThan => a > b,
          Condition::LessThanOrEqual => a <= b,
          Condition::GreaterThanOrEqual => a >= b,
        };

        if will_jump {
          let new_instruction_pointer = labels[label as usize] as usize;
          if new_instruction_pointer != u8::MAX as usize {
            instruction_pointer = new_instruction_pointer;
            continue;
          }

          loop {
            if byte_code[instruction_pointer] == Instruction::Label.into() {
              let new_label = byte_code[instruction_pointer + 1];
              instruction_pointer += 2;
              labels[new_label as usize] = instruction_pointer as u8;
              if new_label == label {
                continue 'outer;
              }
            } else {
              instruction_pointer += instructions_length[byte_code[instruction_pointer] as usize] as usize;
            }
          }

        }
      },
      Instruction::Add => {
        let a = operand_stack[0];
        let b = operand_stack[1];
        operand_stack[0] = a + b;
        stack_pointer = 0;
      },
      Instruction::Print => {
        let value = operand_stack[0];
        println!("{}", value);
        stack_pointer = 0;
      },
      Instruction::Halt => {
        break;
      },
    }

    instruction_pointer += instructions_length[instruction as usize] as usize;
  }
}
