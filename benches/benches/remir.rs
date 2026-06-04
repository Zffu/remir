use criterion::{Criterion, criterion_group, criterion_main};
use remir::{
    builders::{build_call, build_const_string},
    module::Module,
    values::ValueType,
    writer::InstructionWriter,
};

fn hello_world() {
    let mut m = Module::new("test".into());

    let printf = m.create_function("printf".into(), vec![ValueType::new_any_pointer()], None);

    let main = m.create_function("main".into(), vec![], None);
    let main_ref = m.get_function(&main);

    let block = main_ref.append_block(&mut m, "entry".into());

    m.move_end(block, main);

    let str = build_const_string(&mut m, "Hello World\n".to_string()).unwrap();

    let call = build_call(&mut m, printf, vec![str.into()], false, false, false).unwrap();
}

fn benchmark_hello_world(c: &mut Criterion) {
    c.bench_function("hello_world", |b| {
        b.iter(|| hello_world());
    });
}

criterion_group!(remir_benches, benchmark_hello_world);
criterion_main!(remir_benches);
