use std::collections::HashSet;

#[derive(Debug, Clone)]
enum OperationType {
    DivideA,
    BitwiseXorB,
    Modulus,
    Jump,
    BitwiseXorBC,
    Write,
    DivideB,
    DivideC,
}

#[derive(Debug, Clone)]
pub struct Operation {
    operation: OperationType,
    operand: u64,
}

#[derive(Debug, Clone)]
pub struct Computer {
    register_a: u64,
    register_b: u64,
    register_c: u64,

    program_input: String,
    program: Vec<Operation>,
}

#[derive(Debug, PartialEq, Eq, Hash)]
pub struct ComputerKey {
    register_a: u64,
    register_b: u64,
    register_c: u64,
    pointer: usize,
    output: String,
}

impl ComputerKey {
    fn new(a: u64, b: u64, c: u64, pointer: usize, output: String) -> Self {
        Self {
            register_a: a,
            register_b: b,
            register_c: c,
            pointer,
            output,
        }
    }
}

pub type OutputCache = HashSet<ComputerKey>;

fn extract_register_value(input: &str) -> u64 {
    input.split(" ").last().unwrap().parse().unwrap()
}

fn parse_instructions(input: &str) -> (String, Vec<Operation>) {
    let mut result = vec![];

    let mut operation: Option<OperationType> = None;

    let instruction_part = input.split(" ").last().unwrap();

    instruction_part.split(",").for_each(|input| {
        let value = input.parse().unwrap();
        if operation.is_some() {
            result.push(Operation {
                operation: operation.clone().unwrap(),
                operand: value,
            });

            operation = None;
            return;
        }

        operation = Some(match value {
            0 => OperationType::DivideA,
            1 => OperationType::BitwiseXorB,
            2 => OperationType::Modulus,
            3 => OperationType::Jump,
            4 => OperationType::BitwiseXorBC,
            5 => OperationType::Write,
            6 => OperationType::DivideB,
            _ => OperationType::DivideC,
        })
    });

    (instruction_part.to_string(), result)
}

impl Computer {
    pub fn from_string(input: &str) -> Self {
        let mut lines = input.split('\n');
        let a = lines.nth(0).unwrap();
        let b = lines.nth(0).unwrap();
        let c = lines.nth(0).unwrap();
        let instructions = lines.nth(1).unwrap();

        let (program_input, program) = parse_instructions(instructions);

        Self {
            register_a: extract_register_value(a),
            register_b: extract_register_value(b),
            register_c: extract_register_value(c),
            program_input,
            program,
        }
    }
}

fn divide(numerator: u64, combo: u64) -> u64 {
    let denominator = (2 as u64).pow(combo as u32);
    (numerator as f64 / denominator as f64).floor() as u64
}

fn modulus(value: u64, modulus: u64) -> u64 {
    value % modulus
}

fn bitwise_xor(value_a: u64, value_b: u64) -> u64 {
    value_a ^ value_b
}

fn concat_output(output: &Vec<u64>) -> String {
    output
        .iter()
        .map(|value| value.to_string())
        .collect::<Vec<String>>()
        .join(",")
}

impl Computer {
    pub fn set_register_a(&mut self, value: u64) {
        self.register_a = value;
    }

    pub fn is_output_copy(&mut self, cache: Option<&mut OutputCache>) -> bool {
        let (output, moves) = self.output_with_cache(cache.as_deref());
        let matcher = &self.program_input;

        let result = output.eq(matcher);

        if !result && cache.is_some() {
            let cache = cache.unwrap();
            moves.into_iter().for_each(|key| {
                cache.insert(key);
            });
        }

        result
    }

    fn combo_operand(&self, operand: &u64) -> Option<u64> {
        match operand {
            0 | 1 | 2 | 3 => Some(*operand),
            4 => Some(self.register_a),
            5 => Some(self.register_b),
            6 => Some(self.register_c),
            _ => None,
        }
    }

    pub fn output_with_cache(&mut self, cache: Option<&OutputCache>) -> (String, Vec<ComputerKey>) {
        let mut output: Vec<u64> = vec![];
        let mut pointer = 0;
        let mut moves = vec![];

        while let Some(computation) = self.program.get(pointer) {
            let Operation { operation, operand } = computation;
            let combo = self.combo_operand(operand);

            let Computer {
                register_a,
                register_b,
                register_c,
                ..
            } = self;

            let key = ComputerKey::new(
                *register_a,
                *register_b,
                *register_c,
                pointer,
                concat_output(&output),
            );

            match cache {
                None => {}
                Some(cache) => {
                    if cache.contains(&key) {
                        return (String::new(), moves);
                    }
                }
            }

            moves.push(key);

            pointer += 1;

            match operation {
                OperationType::DivideA => self.register_a = divide(*register_a, combo.unwrap()),
                OperationType::BitwiseXorB => self.register_b = bitwise_xor(*register_b, *operand),
                OperationType::Modulus => self.register_b = modulus(combo.unwrap(), 8),
                OperationType::Jump => match register_a {
                    0 => {}
                    _ => pointer = *operand as usize,
                },
                OperationType::BitwiseXorBC => {
                    self.register_b = bitwise_xor(*register_b, *register_c)
                }
                OperationType::Write => output.push(modulus(combo.unwrap(), 8)),
                OperationType::DivideB => self.register_b = divide(*register_a, combo.unwrap()),
                OperationType::DivideC => self.register_c = divide(*register_a, combo.unwrap()),
            }
        }

        (concat_output(&output), moves)
    }

    pub fn output(&mut self) -> String {
        self.output_with_cache(None).0
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn determines_result_of_operations() {
        // If register C contains 9, the program 2,6 would set register B to 1.
        let input = "Register A: 0\nRegister B: 0\nRegister C: 9\n\nProgram: 2,6";
        let mut computer = Computer::from_string(input);
        computer.output();
        assert_eq!(computer.register_b, 1);

        // If register A contains 10, the program 5,0,5,1,5,4 would output 0,1,2.
        let input = "Register A: 10\nRegister B: 0\nRegister C: 0\n\nProgram: 5,0,5,1,5,4";
        let mut computer = Computer::from_string(input);
        assert_eq!(computer.output(), "0,1,2");

        // If register A contains 2024, the program 0,1,5,4,3,0 would output 4,2,5,6,7,7,7,7,3,1,0 and leave 0 in register A.
        let input = "Register A: 2024\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0";
        let mut computer = Computer::from_string(input);
        assert_eq!(computer.output(), "4,2,5,6,7,7,7,7,3,1,0");
        assert_eq!(computer.register_a, 0);

        // If register B contains 29, the program 1,7 would set register B to 26.
        let input = "Register A: 0\nRegister B: 29\nRegister C: 0\n\nProgram: 1,7";
        let mut computer = Computer::from_string(input);
        computer.output();
        assert_eq!(computer.register_b, 26);

        // If register B contains 2024 and register C contains 43690, the program 4,0 would set register B to 44354.
        let input = "Register A: 0\nRegister B: 2024\nRegister C: 43690\n\nProgram: 4,0";
        let mut computer = Computer::from_string(input);
        computer.output();
        assert_eq!(computer.register_b, 44354);
    }

    #[test]
    fn determines_value_after_running_program() {
        let input = "Register A: 729\nRegister B: 0\nRegister C: 0\n\nProgram: 0,1,5,4,3,0";

        let mut computer = Computer::from_string(input);

        assert_eq!(computer.program.len(), 3);

        assert_eq!(&computer.output(), "4,6,3,5,6,3,5,2,1,0")
    }
}
