fn main() {
    let a = 23;
    //let b: i16 = 10;
    //let c = 2.3;
    let b = 19;
    let c = a + b;
    dbg!(c);
    let d = a / b;
    dbg!(d);

    let f = a as f32 / b as f32;
    dbg!(f);
}
