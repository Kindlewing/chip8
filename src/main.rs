use lc3_vm::prelude::VM;
use std::io;

fn main() -> Result<(), io::Error> {
    let mut vm = VM::new();
    let program_path = "./asm/hello_world.asm";
    vm.load_to_memory(program_path)?;
    vm.run();

    println!(
        "memory: {:?} registers: {:#?}",
        vm.memory.first_chunk::<64>(),
        vm.registers
    );
    Ok(())
}
