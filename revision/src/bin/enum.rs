fn main(){
    enum IPAddrKind {
        v4(u8,u8,u8,u8),
        v6(String)
    }

    struct IPAddr {
        kind: IPAddrKind,
        address : String
    }
    let home = IPAddr {
        kind : IPAddrKind::v4(1,2,3,4),
        address : String::from("127.0.0.1")
    };
    let loopback = IPAddr{
        kind : IPAddrKind::v6("IP6"),
        address : String::from("197.0.0.8")
    };
}