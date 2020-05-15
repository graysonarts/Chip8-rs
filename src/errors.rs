use crate::opcodes::OpCode;

#[derive(Debug, PartialEq)]
pub(crate) enum RuntimePanic {
	UnknownOpcode(OpCode)
}

impl std::fmt::Display for RuntimePanic {
	fn fmt(&self, _: &mut std::fmt::Formatter) -> std::result::Result<(), std::fmt::Error> { todo!() }
}

impl std::error::Error for RuntimePanic {

}