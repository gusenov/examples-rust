struct S {
    s: Option<String>,
}

fn main() {
    let s1 = S { s: Some(String::from("hello")) };

    if let Some(str) = s1.s {
        println!("{}", str);
    }

    if let Some(str) = s1.s {
    // Value used after being moved [E0382]

        println!("{}", str);
    }

    let s2 = Some(String::from("hello"));

    if let Some(str) = s2 {
        println!("{}", str);
    }

    if let Some(str) = s2 {
    // Value used after being moved [E0382]

        println!("{}", str);
    }

}
