/*!
    Module containing things related to the bytecode VM.
*/

// External imports
use caxlang_proc::Display;

/// Basically the match expression, but for opcodes on bytes.
macro_rules! opcode_match {
    ($byte:expr, $( $opcode:expr => $code:block ),*) => {
        $(
            if $byte == $opcode.into() $code
        )*
    }
}

/// Basically the match expression, but for operands of opcodes on bytes.
macro_rules! operand_match {
    ($byte:expr, $( $operand:expr => $code:block ),*) => {
        $(
            if $byte == $operand.into() $code
        )*
    }
}

/// An operation code for the VM.
/// 
/// Bytecode example: `ADD [reg_name] [reg_name]`
#[derive(Debug)]
pub enum OpCode {
    Add = 0,
    Sub
}

/// An register for a VM.
/// 
/// Bytecode example: `MOV [register] [operand_one]`
#[derive(Debug)]
pub enum Register {
    R0 = 0,
    R1,
    R2,
    R3,
    R4,
    R5,
    R6,
    R7
}

impl Into<Byte> for Register {
    fn into(self) -> Byte {
        self as Byte
    }
}

/// A more readable alias for a u8.
type Byte = u8;

impl Into<Byte> for OpCode {
    fn into(self) -> Byte {
        self as Byte
    }
}

/// A chunk that the VM can read.
#[derive(Display)]
pub struct Chunk {
    /// Each VM instruction is 3 bytes long;  
    /// one byte for opcode, two bytes for operands (params)
    pub bytes: [Byte; 3]
}

impl Chunk {
    /// Create new chunk from bytes.
    pub fn new(bytes: [Byte; 3]) -> Self {
        Self { bytes }
    }

    /// Parses an operand into a string.  
    /// Operand index '0' is one index after opcode.
    pub fn parse_operand(&self, operand_index: usize) -> &'static str {
        operand_match!{self.bytes[operand_index + 1],
            Register::R0 => { return "R0" },
            Register::R1 => { return "R1" },
            Register::R2 => { return "R2" },
            Register::R3 => { return "R3" },
            Register::R4 => { return "R4" },
            Register::R5 => { return "R5" },
            Register::R6 => { return "R6" },
            Register::R7 => { return "R7" }
        };

        // This will never get returned. This line is just for compiler.
        ""
    }

    /// Returns a representation  
    /// of this chunk as plain text.
    pub fn dissasemble(&self) -> String {
        // Create variables to hold string repr of chunk
        let mut opcode_str: &'static str = "";
        let operand_one: &'static str;
        let operand_two: &'static str;

        // Get string version of operand one and two.
        operand_one = self.parse_operand(0);
        operand_two = self.parse_operand(1);

        opcode_match!{self.bytes[0],
            OpCode::Add => { opcode_str = "Add"; },
            OpCode::Sub => { opcode_str = "Sub"; }
        }

        // Return string repr of current chunk
        format!("{} {} {}", opcode_str, operand_one, operand_two).to_string()
    }
}

impl std::fmt::Debug for Chunk {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.dissasemble())
    }
}

/// The VM that performs actions based on given bytecode chunks.
pub struct VM {
    chunks: Vec<Chunk>
}

impl VM {
    /// Returns a new VM given a list of chunks.
    pub fn new(chunks: Vec<Chunk>) -> Self {
        Self { chunks }
    }
}

#[cfg(test)]
mod tests {
    /// Use outside scope
    use super::*;

    #[test]
    fn test_chunk_assemble() {
        let chunk = Chunk::new([OpCode::Add.into(), 1, 2]);

        println!("{:?}", &chunk.bytes);
    }

    #[test]
    fn test_chunk_dissasemble() {
        let chunk = Chunk::new([OpCode::Add.into(), 1, 2]);

        println!("{}", &chunk);
    }
}
