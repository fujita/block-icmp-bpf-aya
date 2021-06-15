This is just an example of how to write BPF (XDP) application with [aya](https://github.com/alessandrod/aya), purely Rust library for BPF.

The other Rust libraries for BPF, [libbpf-rs](https://github.com/libbpf/libbpf-rs) and [RedBPF](https://github.com/foniod/redbpf) internally use the canonical C library, [libbpf](https://github.com/libbpf/libbpf).

The following command attaches the XDP code that drops ICMP packets and prints messages to the common trace_pipe (/sys/kernel/debug/tracing/trace_pipe).
```bash
$ cargo build
$ sudo ./target/debug/block-icmp-bpf --dev eth0
```

You can build a static binary easily if you prefer.
```bash
$ cargo build --target=x86_64-unknown-linux-musl
$ ldd target/x86_64-unknown-linux-musl/debug/block-icmp-bpf-aya
	statically linked
```
