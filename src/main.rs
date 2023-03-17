fn main() {
   let mut a = 1;
   change(&mut a);

   println!("{}", a);
}

fn change(a: &mut i32) {
    println!("{:p}", a);
    *a = 2;
}