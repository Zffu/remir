use criterion::{Criterion, criterion_group, criterion_main};
use remir::{
    builders::{
        build_argument_grab, build_call, build_conditional_branch, build_const_int,
        build_const_string, build_int_compare, build_math_op_int, build_ret,
    },
    misc::{CompareOperator, MathOperator},
    module::Module,
    values::{ValueType, int::SSAIntValue},
    writer::InstructionWriter,
};

fn hello_world() {
    let mut m = Module::new("test".into());

    let printf = m.create_function("printf".into(), vec![ValueType::new_any_pointer()], None);

    let main = m.create_function("main".into(), vec![], None);
    m.move_function(main.clone());

    let block = m.create_block("entry".into()).unwrap();

    m.move_end(block, main);

    let str = build_const_string(&mut m, "Hello World\n".to_string()).unwrap();

    let _ = build_call(&mut m, printf, vec![str.into()], false, false, false).unwrap();
}

fn fib() {
    let mut m = Module::new("fib".to_string());

    let fib = m.create_function(
        "fib".into(),
        vec![ValueType::Int(false, 32)],
        Some(ValueType::Int(false, 32)),
    );

    m.move_function(fib.clone());

    let _ = m.create_function("main".into(), vec![], None);

    let entry = m.create_block("entry".into()).unwrap();
    let then = m.create_block("then".into()).unwrap();
    let else_block = m.create_block("else".into()).unwrap();

    m.move_end(entry, fib.clone());

    let arg = SSAIntValue::try_from(build_argument_grab(&mut m, 0).unwrap()).unwrap();

    let two = build_const_int(&mut m, 2, 32, false).unwrap();
    let one = build_const_int(&mut m, 1, 32, false).unwrap();

    let cond =
        build_int_compare(&mut m, arg.clone(), two.clone(), CompareOperator::Lt, false).unwrap();

    build_conditional_branch(&mut m, cond, then.clone(), else_block.clone()).unwrap();

    m.move_end(then.clone(), fib.clone());
    build_ret(&mut m, Some(arg.clone().into()));

    m.move_end(else_block.clone(), fib.clone());

    let n1 = build_math_op_int(
        &mut m,
        arg.clone(),
        one,
        MathOperator::Sub,
        false,
        false,
        false,
        false,
    )
    .unwrap();

    let n2 = build_math_op_int(
        &mut m,
        arg.clone(),
        two,
        MathOperator::Sub,
        false,
        false,
        false,
        false,
    )
    .unwrap();

    let fib1 = build_call(&mut m, fib.clone(), vec![n1.into()], true, false, false)
        .unwrap()
        .unwrap();

    let fib1 = SSAIntValue::try_from(fib1).unwrap();

    let fib2 = build_call(&mut m, fib.clone(), vec![n2.into()], true, false, false)
        .unwrap()
        .unwrap();

    let fib2 = SSAIntValue::try_from(fib2).unwrap();

    let sum = build_math_op_int(
        &mut m,
        fib1,
        fib2,
        MathOperator::Add,
        false,
        false,
        false,
        false,
    )
    .unwrap();

    build_ret(&mut m, Some(sum.into()));
}

fn benchmark_hello_world(c: &mut Criterion) {
    c.bench_function("hello_world", |b| {
        b.iter(|| hello_world());
    });
}

fn benchmark_fib(c: &mut Criterion) {
    c.bench_function("fib", |b| {
        b.iter(|| fib());
    });
}

criterion_group!(remir_benches, benchmark_hello_world, benchmark_fib);
criterion_main!(remir_benches);
