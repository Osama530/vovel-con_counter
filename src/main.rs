
fn main() {
let input = String::from("osama qamar");

let mut v_count = 0;
let mut c_count = 0;
for c in input.chars() {
    match c {
        'a' | 'e' | 'i' | 'o'| 'u' |
        'A' | 'E'| 'I' | 'O' | 'U'    => v_count+=1,
                                   _ => c_count+=1
 
    };
};
println!("Vovels :{}",v_count);
println!("consonents :{}",c_count);










//  let hello = String::from("السلام عليكم");
//  for c in hello.bytes(){
//      println!("{}",c);
//  }
//  println!("{}",hello.len());
}
