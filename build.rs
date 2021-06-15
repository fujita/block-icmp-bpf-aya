use std::fs::create_dir_all;
use std::io::Write;
use std::process::Command;

fn main() {
    let output = Command::new("bpftool")
        .arg("btf")
        .arg("dump")
        .arg("file")
        .arg("/sys/kernel/btf/vmlinux")
        .arg("format")
        .arg("c")
        .output()
        .unwrap();
    let mut f = std::fs::File::create("./src/bpf/vmlinux.h").unwrap();
    f.write_all(&output.stdout).unwrap();

    create_dir_all("./src/bpf/.output").unwrap();
    Command::new("clang")
        .arg("-O2")
        .arg("-g")
        .arg("-target")
        .arg("bpf")
        .arg("-c")
        .arg("./src/bpf/block-icmp.bpf.c")
        .arg("-o")
        .arg("./src/bpf/.output/block-icmp.bpf.o")
        .output()
        .unwrap();
}
