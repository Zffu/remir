use criterion::{Criterion, criterion_group, criterion_main};
use remir::{
    builders::{build_call, build_const_string},
    module::Module,
    values::ValueType,
    writer::InstructionWriter,
};
use remir_llvm::{LLVMBridge, build_llvm};

fn hello_world_llvm() {
    let mut m = Module::new("test".into());

    let printf = m.create_function("printf".into(), vec![ValueType::new_any_pointer()], None);

    let main = m.create_function("main".into(), vec![], None);
    let main_ref = m.get_function(&main);

    let block = main_ref.append_block(&mut m, "entry".into());

    m.move_end(block, main);

    let str = build_const_string(&mut m, "Hello World\n".to_string()).unwrap();

    let call = build_call(&mut m, printf, vec![str.into()], false, false, false).unwrap();

    let mut bridge = LLVMBridge::new();

    build_llvm(&mut bridge, &mut m).unwrap();
}

fn benchmark_hello_world_llvm(c: &mut Criterion) {
    c.bench_function("hello_world_llvm", |b| {
        b.iter(|| hello_world_llvm());
    });
}

criterion_group!(llvm_benches, benchmark_hello_world_llvm);
criterion_main!(llvm_benches);
