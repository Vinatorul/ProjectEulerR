/* [info]
[authors]
Kuvaev Alexander <alexander@kuvaev.me>
[name]
Multiples of 3 and 5
[statement]
If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3, 5, 6 and 9. The sum of these multiples is 23.
Find the sum of all the multiples of 3 or 5 below N.
[input]
2
10
100
[end] */

use std::io;

fn main() {
    let mut l1 = String::new();
    io::stdin().read_line(&mut l1).unwrap();
    let l1: u32 = l1.trim().parse().unwrap();
    for x in 0..l1 {
        let mut l2 = String::new();
        io::stdin().read_line(&mut l2).unwrap();
        let mut l2: i64 = l2.trim().unwrap();
        l2 -= 1;
        let mut sum = 0;
        let mut c: i64 = l2/3;
        sum += 3*(c+1)*c/2;
        c = l2/5;
        sum += 5*(c+1)*c/2;
        c = l2/15;
        sum -= 15*(c+1)*c/2;      
        println!("{}", sum);
    }
}