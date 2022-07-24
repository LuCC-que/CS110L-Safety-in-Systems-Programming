//example 1:
// fn main() {
//     let mut s = String::from("hello");
//     let ref1 = &s;
//     let ref2 = &ref1;
//     let ref3 = &ref2;
//     //as the s has been borrowed cant sign a new value to it
//     s = String::from("goodbye");

//     //as the ref has been borrowed, pass it to another function
//     //compile may suspect if it will get changed or not
//     println!("{}", ref3.to_uppercase());
// }

//example 2:
// fn drip_drop() -> &String {
//     let s = String::from("hello world!");
//     //dangling pointer
//     //in more rust terminology
//     //it surpasses the life cycle of this variable
//     return &s;
// }

fn main() {
    let s1 = String::from("hello");
    let mut v = Vec::new();
    v.push(s1);

    //not working until here
    //it is trying to take the
    //ownership if the string v[0]
    //which is impossible for the string
    let s2: String = v[0];
    println!("{}", s2);
}
