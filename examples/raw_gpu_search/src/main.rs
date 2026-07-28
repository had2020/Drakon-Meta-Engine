use fitness_parameters::*;
use wgpu_accelerated::*;

fn main() {
    const TESTS_N: usize = 1;
    let requirements: fitness_parameters::Requirements<TESTS_N> = Requirements {
        reg0_inputs: [67; TESTS_N],
        reg1_inputs: [126; TESTS_N],
        reg2_inputs: [0; TESTS_N],
        reg3_inputs: [0; TESTS_N],
        reg0_expected: [193; TESTS_N],
        reg1_expected: [0; TESTS_N],
        reg2_expected: [0; TESTS_N],
        reg3_expected: [0; TESTS_N],
        reg0_ignore: [IgnoreMask::Checked; TESTS_N],
        reg1_ignore: [IgnoreMask::Ignored; TESTS_N],
        reg2_ignore: [IgnoreMask::Ignored; TESTS_N],
        reg3_ignore: [IgnoreMask::Ignored; TESTS_N],
    };

    let wgpu_wrapper = WgpuWrapper::init();

    let resulting_program = gpu_raw_search(&wgpu_wrapper, &requirements);

    println!("{:?}", resulting_program);
}
