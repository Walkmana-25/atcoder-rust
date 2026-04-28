use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input!{
        n: usize,
        m: usize,
        f: [usize; n]
    }
    
    // 質問1
    let hm: HashSet<_> = f.iter().collect();
    if hm.iter().count() == f.iter().count() {
        println!("Yes");
    }else {
        println!("No");
    
    }
    
    //質問2
    let mut v: Vec<_> = Vec::new();
    for _ in 0..m {
        v.push(0);
    }
    
    for ff in f {
        v[ff - 1] += 1;
    }
    
    let mut flag = true;
    for vs in v {
        if vs == 0 {
            flag = false;
        }
    }
    
    if flag {println!("Yes")} else {println!("No")}
    
    

}
