#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (&str, &[u8])| {
    let (src, input) = data;
    fuzz(src, input);
});

fn fuzz(src: &str, input: &[u8]) -> Option<()> {
    let normal = get_output(false, src, input)?;
    let optimized = get_output(true, src, input)?;
    assert_eq!(normal, optimized);

    Some(())
}

fn get_output(opt: bool, src: &str, input: &[u8]) -> Option<Vec<u8>> {
    let mut ir = bfjit::bfir::compile(src).ok()?;

    if opt {
        bfjit::bfir::optimize(&mut ir);
    }

    let (code, start) = bfjit::BfVM::fuzz_compile(&ir).ok()?;

    let memory = vec![0; bfjit::BfVM::MEMORY_SIZE].into_boxed_slice();
    let mut output = Vec::new();
    let mut vm = bfjit::BfVM::fuzz_new(code, start, memory, Box::new(input), Box::new(&mut output));

    vm.run().ok()?;
    drop(vm);

    Some(output)
}
