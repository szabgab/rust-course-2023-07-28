fn main() {
    //println!("Hello, world! again");
    //print!("Hello, world!\n");
    //println!("again");
    //eprintln!("error Hello, world! again");
    //eprint!("error Hello, world! again");

    let name = "Foo";
    println!("Hello");
    println!("Hello {}", name);
    println!("Hello {name}");
    dbg!(name);
    let age = 21;
    let out = format!("Hello {name} are you more than {age} old?");
}
