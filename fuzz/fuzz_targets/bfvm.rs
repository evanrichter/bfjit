#![no_main]

use libfuzzer_sys::fuzz_target;

fuzz_target!(|data: (bool, &str, &[u8])| {
    let (opt, src, input) = data;
    fuzz(opt, src, input);
});

fn fuzz(opt: bool, src: &str, input: &[u8]) -> Option<()> {
    let mut ir = bfjit::bfir::compile(src).ok()?;
    if opt {
        bfjit::bfir::optimize(&mut ir);
    }

    let (code, start) = bfjit::BfVM::fuzz_compile(&ir).ok()?;

    let memory = vec![0; bfjit::BfVM::MEMORY_SIZE].into_boxed_slice();
    let mut vm = bfjit::BfVM::fuzz_new(
        code,
        start,
        memory,
        Box::new(input),
        Box::new(std::io::sink()),
    );

    vm.run().ok()
}
