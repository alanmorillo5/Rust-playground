#[allow(dead_code)]
fn main() {
    // ENUMS 
    let _four = IpAddrKind::V4;
    let _six = IpAddrKind::V6;

    fn _route(_ip_kind: IpAddrKind) {}

    _route(IpAddrKind::V4);
    _route(IpAddrKind::V6);

    // Using structs
    #[allow(dead_code)]
    struct IpArr {
        kind: IpAddrKind,
        address: String
    }

    let _home1 = IpArr {
        kind: IpAddrKind::V4,
        address: String::from("127.0.0.1")
    };

    let _loopback1 = IpArr {
        kind: IpAddrKind::V6,
        address: String::from("::1")
    };

    #[allow(dead_code)]
    enum IpAddr {
        V4(u8, u8, u8, u8),
        V6(String)
    }

    let _home2 = IpAddr::V4(127, 0, 0, 1);
    let _loopback2 = IpAddr::V6(String::from("::1"));
}

#[allow(dead_code)]
enum IpAddrKind {
    V4,
    V6
}
