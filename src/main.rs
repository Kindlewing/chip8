use lc3_vm::vm::VM;
use std::io;

fn assemble() {
    todo!("Write assembler");
}

fn main() -> Result<(), io::Error> {
    let mut vm = VM::new();
    let program_path = "./obj/hello.obj";
    vm.load_to_memory(program_path)?;
    vm.run();
    Ok(())
}
