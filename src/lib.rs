use libc;

pub const OK: u8 = 0;
pub const ERR: u8 = 1;

pub const MAX_PA: usize = 3;
pub const MAX_IFACE: usize = 10;
pub const MAX_IFNAME: usize = 16;

pub const FILE: u8 = 0;
pub const PRINT: u8 = 1;

#[derive(Debug)]
#[repr(C)]
pub struct wc_sockaddr {
    pub sa_family: libc::sa_family_t,
    pub sa_data: [libc::c_char; 14],
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub enum pa {
    PA_NULL = 0,
    PA_IP,
    PA_ARP,
    PA_TCP,
    PA_UDP,
    PA_ETH
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct wc_pa {
    eth: wc_eth,
    arp: wc_arp,
    ip: wc_ip,
    tcp: wc_tcp,
    udp: wc_udp,
    p: [pa; MAX_PA]
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct wc_eth {
    source: *mut libc::c_uchar,
    dest: *mut libc::c_uchar,
    proto: libc::c_ushort
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct wc_arp {
    hw_t: *mut libc::c_char,
    p_t: *mut libc::c_char,
    hw_len: libc::c_uchar,
    p_len: libc::c_uchar,
    opcode: *mut libc::c_char,
    sender_mac: *mut libc::c_char,
    sender_ip: *mut libc::c_char,
    target_mac: *mut libc::c_char,
    target_ip: *mut libc::c_char,
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct wc_ip {
    version: libc::c_uchar,
    ttl: libc::c_int,
    ihl: libc::c_int,
    tos: libc::c_ushort,
    tl: libc::c_longlong,
    ident: libc::c_short,
    hchs: libc::c_int,
    source: *mut libc::c_uchar,
    dest: *mut libc::c_uchar,
    proto: libc::c_uchar
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct wc_tcp {
    source: libc::c_ushort,
    dest: libc::c_ushort,
    sequence: libc::c_ulong,
    ack_sequnce: libc::c_ulong,
    window: libc::c_ushort,
    checksum: libc::c_ushort,
    flag: libc::c_uchar
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct wc_udp {
    source: libc::c_ushort,
    dest: libc::c_ushort,
    check: libc::c_ushort,
    len: libc::c_ushort
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct wc_iface {
    name: [libc::c_char; MAX_IFNAME],
    mtu: libc::c_ulonglong,
    flag: libc::c_uchar
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct wc_iflist {
    ifc: [wc_iface; MAX_IFACE],
    len: libc::c_uchar
}

#[allow(non_camel_case_types)]
#[derive(Debug)]
#[repr(C)]
pub struct wrc {
    fd: libc::c_int,
    recvl: libc::c_int,
    recvn: libc::c_ulonglong,
    iface: wc_iface,
    ign_pa: pa,
    flag: libc::c_char,
    saddr: wc_sockaddr,
    recv: *mut libc::c_uchar
}

extern "C" {
    pub fn wrc_default(w: *mut wrc) -> libc::c_void;
    pub fn wrc_destroy(w: *mut wrc) -> libc::c_void;

    pub fn wrc_setopts(w: *mut wrc, ifc: wc_iface, p: pa, flag: libc::c_char) -> libc::c_char;
    pub fn wrc_cap(w: *mut wrc, fp: libc::c_uchar, cb: extern "C" fn(wc_pa, *mut libc::FILE)) -> libc::c_char;
    pub fn DEAFULT_CAP(wcp: wc_pa, fp: *mut libc::FILE) -> libc::c_void;
    
    pub fn wrc_get_interfaces() -> wc_iflist;
}

#[cfg(test)]
mod tests {
    use super::*;
    
    #[test]
    fn it_works() {
        let ifc: wc_iflist = unsafe { wrc_get_interfaces() };
        println!("{:?}", ifc);
        assert_eq!(ifc.len, 3)
    }
}
