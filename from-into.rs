use std::net::Ipv4Addr;

fn main() {
    ping(Ipv4Addr::new(23,21,68,141));
    ping([66, 146, 219, 98]);
    ping(0xd076eb94_u32);

    println!("{:?}", Ipv4Addr::from([66, 146, 219, 98]));
    println!("{:?}",Ipv4Addr::from(0xd076eb94_u32));

}

fn ping<A>(address: A) where A: Into<Ipv4Addr> {
    println!("{:?}", address.into());
}