use aya::programs::{Xdp, XdpFlags};
use aya::{Bpf, Btf};
use clap::{App, Arg};
use std::convert::TryInto;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::{thread, time};

fn main() {
    let code = include_bytes!("bpf/.output/block-icmp.bpf.o");

    let running = Arc::new(AtomicBool::new(true));
    let r = running.clone();
    ctrlc::set_handler(move || {
        r.store(false, Ordering::SeqCst);
    })
    .unwrap();

    let args = App::new("block-icmp-bpf")
        .arg(
            Arg::with_name("DEV")
                .long("dev")
                .takes_value(true)
                .required(true)
                .help("specify device name (e.g, eth0)"),
        )
        .get_matches();
    let dev_name = args.value_of("DEV").unwrap();

    let mut bpf = Bpf::load(&code.to_vec(), Btf::from_sys_fs().ok()).unwrap();
    let program: &mut Xdp = bpf.program_mut("xdp").unwrap().try_into().unwrap();
    program.load().unwrap();
    program.attach(dev_name, XdpFlags::default()).unwrap();

    while running.load(Ordering::SeqCst) {
        thread::sleep(time::Duration::from_secs(1));
    }
}
