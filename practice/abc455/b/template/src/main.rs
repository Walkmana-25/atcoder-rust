use proconio::input;


fn main() {
    input!{
        h: usize,
        w: usize,
        s: [String; h]
    }
    
    let mut ans: i32 = 0;
    
    for h1 in 1..h+1 {
        for h2 in h1..h+1 {
            for w1 in 1..w + 1 {
                for w2 in w1..w + 1 {
                    let mut flag: bool = true;
                    
                        for i in h1..h2 + 1 {
                            for j in  w1..w1 + 1{
                                let a = s[i - 1].chars().nth(j - 1).unwrap();
                                let b = s[h1 + h2 - i - 1].chars().nth(w1 + w2 - j - 1).unwrap();
                                
                                if a != b {
                                    flag = false;
                                }
                            }
                            
                        }
                    if flag {
                        ans += 1;
                    }
                }
            }
            
        }
        
    }
    
    println!("{ans}");

}
