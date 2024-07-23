pub mod flags;
pub mod opcode;
pub mod register;
pub mod vm;

pub mod prelude {
    pub use crate::flags::Flags;
    pub use crate::opcode::Opcode;
    pub use crate::register::{MemMapReg, Register};
}
