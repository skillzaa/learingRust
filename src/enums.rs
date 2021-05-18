#[derive(Debug)]
enum IdAddressType {
    Ip4,
    Ip6,
}
#[derive(Debug)]
struct Connection {
    id_address : IdAddressType,
    id : u32,
}
fn main(){

    let four = IdAddressType::Ip4;
    let six = IdAddressType::Ip6;

    println!("IdAddressType ip4  {:#?}", four);
    println!("IdAddressType ip6  {:#?}", six);
    //---------------------------------------
    let my_connection = Connection {
        id_address: IdAddressType::Ip4,
        id: 55,
    };
    println!("conenction struct   {:#?}", my_connection);
}