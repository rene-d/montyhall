fn main() {
    // play 1
    println!("Hello, world!");

    // play 2
    let a = 1;
    println!("a = {}", a);
    println!("a = {a}");

    // play 3
    for i in 0..3 {
        println!("Hello, world! n° {}", i + 1);
    }

    // play 4
    print!("b =");
    let mut b = 1u64;
    loop {
        if b > 5000 {
            break;
        }
        print!(" 0x{b:x}");
        b *= 2;
    }
    println!();

    // play 5
    let s = "hello";
    println!("s = «{}» length={}", s, s.len());

    // play 6
    let mut s = s.to_string();
    s += " world!";
    println!("s = «{}» length={}", s, s.len());
    affiche(s); // déplacer avant le println! 🤷‍♀️
}

fn affiche(s: String) {
    println!("chaîne de caractères: {:?}", s);
}
