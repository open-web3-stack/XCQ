use polkavm::{Caller, Config, Linker};

struct HostFunctions;

impl poc_executor::XcqExecutorContext for HostFunctions {
    fn register_host_functions<T>(&mut self, linker: &mut Linker<T>) {
        linker
            .func_wrap("host_call", move |caller: Caller<_>, ptr: u32| -> u32 {
                let mut data = [0u8];
                let data = caller.read_memory_into_slice(ptr, &mut data).unwrap();
                println!("host_call: {:?}", data);
                (data[0] + 1) as u32
            })
            .unwrap();
        linker
            .func_wrap("call_sbrk_indirectly", move |mut caller: Caller<_>, size: u32| -> u32 {
                caller.sbrk(size).unwrap_or(0)
            })
            .unwrap();
        linker
            .func_wrap(
                "host_write",
                move |mut caller: Caller<_>, src: u32, size: u32, dst: u32| {
                    let data = caller.read_memory_into_vec(src, size).unwrap();
                    caller.write_memory(dst, &data).unwrap();
                },
            )
            .unwrap();
    }
}

fn main() {
    env_logger::init();

    let raw_blob = include_bytes!("../../../output/poc-guest.polkavm");

    let config = Config::from_env().unwrap();

    let mut executor: poc_executor::XcqExecutor<HostFunctions> = poc_executor::XcqExecutor::new(config, HostFunctions);
    let res = executor.execute(raw_blob, &[0u8]).unwrap();
    println!("Result: {:?}", res);

    let res = executor.execute(raw_blob, &[1u8, 40u8]).unwrap();
    println!("Result: {:?}", res);
}
