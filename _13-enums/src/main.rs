fn main() {
    // ENUMS 
    let four = IpAddrKind::V4;
    let six = IpAddrKind::V6;

    fn route(ip_kind: IpAddrKind) {}

    route(IpAddrKind::V4);
    route(IpAddrKind::V6);

    // Using structs
    struct IpArr {
        kind: IpAddrKind,
        address: String
    }

    let home = IpArr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let loopback = IpArr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String)
    };

    let home = IpAddr::V4(127, 0, 0, 1);
    let loopback = IpAddr::V6(String::from("::1"));
}

enum IpAddrKind {
    V4,
    V6
}