extern crate cc;

fn main() {
    cc::Build::new()
        .flag("-Wall")
        .flag("-Wimplicit-fallthrough")
        .file("libwrc/src/wrc/wrc.c")
        .file("libwrc/src/eth/eth.c")
        .file("libwrc/src/arp/arp.c")
        .file("libwrc/src/ip/ip.c")
        .file("libwrc/src/tcp/tcp.c")
        .file("libwrc/src/udp/udp.c")
        .file("libwrc/src/utils/utils.c")
        .compile("libwrc");
}
