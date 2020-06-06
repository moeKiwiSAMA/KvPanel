use virt::connect::Connect;

fn main() {
    if let Ok(mut conn) = Connect::open("test://default") {
        assert_eq!(Ok(0), conn.close());
    }
}
