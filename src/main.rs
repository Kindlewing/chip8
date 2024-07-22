use lc3_vm::vm::VM;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut vm = VM::new();
    let program_path = "./asm/hello_world.asm";
    vm.load_to_memory(program_path)?;
    vm.run();
    Ok(())
}
