extern mod extra;
 
use std::{int,io,result};
use extra::{net,net_tcp,uv};
 
fn main() {
let addr = extra::net_ip::v4::parse_addr("127.0.0.1");
let port = 8080;
 
let mut counter = 0;
 
loop {
let iotask = &uv::global_loop::get();
let connect_result = net_tcp::connect(copy addr, port, iotask);
assert!(connect_result.is_ok());
let sock = result::unwrap(connect_result);
let buf = net_tcp::socket_buf(sock);
buf.write_str(fmt!("ping %i\n", counter));
let line = buf.read_line();
io::println(fmt!("From server: %s", line));
counter += 1;
unsafe { std::libc::sleep(1); }
}
}
