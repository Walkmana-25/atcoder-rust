use proconio::input;

fn main() {
    input!{
        l: i32,
        r: i32
    }
    let ans = r - l + 1;
    println!("{ans}");

}
