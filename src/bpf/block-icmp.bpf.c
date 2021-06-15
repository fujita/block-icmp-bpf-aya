#include "vmlinux.h"
#include <bpf/bpf_helpers.h>
#include <bpf/bpf_tracing.h>

SEC("xdp")
int xdp_icmp_c(struct xdp_md *ctx)
{
	void *data = (void *)(long)ctx->data;
	void *data_end = (void *)(long)ctx->data_end;

	struct iphdr *ip = data + sizeof(struct ethhdr);
	if ((void *)(ip + 1) > data_end)
		return XDP_PASS;

	if (ip->protocol == IPPROTO_ICMP)
	{
		bpf_printk("PING");
		return XDP_DROP;
	}
	return XDP_PASS;
}

char _license[] SEC("license") = "GPL";
