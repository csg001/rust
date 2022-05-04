enum test_enum {
    A(i32),
    B(f32),
    C(String),
}
fn main() {
    let mut c = vec![
        test_enum::A(3),
        test_enum::B(3.0),
        test_enum::B(3.6),
        test_enum::C("T".to_string()),
    ];
    //let mut v=vec![0,502,-502];
    for i in &mut c {
        //*i +=50;
        match i {
            test_enum::A(a) => println!("{}",a),
            test_enum::B(b) => println!("{}", b),
            test_enum::C(c) => println!("{}", c),
        }
    }
}
