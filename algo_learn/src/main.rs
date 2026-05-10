/*
假设你是一位很棒的家长，想要给你的孩子们一些小饼干。但是每个孩子最多只能给一块饼干。

对每个孩子 i，都有一个胃口值 g[i]，这是能让孩子们满足胃口的饼干的最小尺寸；并且每块饼干 j，都有一个尺寸 s[j]。如果 s[j] >= g[i]，我们可以将这个饼干 j 分配给孩子 i，这个孩子会得到满足。

设计算法尽可能满足越多数量的孩子，并输出这个最大数值。
输入格式

    第一行：孩子数 m 和饼干数 n
    第二行：m 个整数，表示孩子们的胃口值
    第三行：n 个整数，表示每块饼干的尺寸

输出格式

一个整数，表示最多能满足的孩子数量。
*/
use std::io;
fn main() {
    println!("请输入孩子数m和饼干数n:");
    let mut m = String::new();
    io::stdin().read_line(&mut m).expect("读取失败");
}

fn BSort<T: Ord>(arr: &mut [T]) {
    let len = arr.len();
    for i in 0..len {
        let mut swapped = false;
        for j in 0..len - 1 - i {
            if arr[j] > arr[j + 1] {
                arr.swap(j, j + 1);
                swapped = true;
            }
        }
        if !swapped {
            break;
        }
    }
}
