fn main() {
    let n = 10;
    println!("F({n}) = {:?}", fibonacci(n));
}

fn fibonacci(n: u32) -> Option<u64> {
    if n == 0 {
        return Some(0);
    }

    let (mut previous, mut current) = (0_u64, 1_u64);
    for _ in 1..n {
        let next = previous.checked_add(current)?;
        previous = current;
        current = next;
    }

    Some(current)
}

#[cfg(test)]
mod tests {
    use super::fibonacci;

    #[test]
    fn computes_fibonacci_numbers_and_reports_overflow() {
        assert_eq!(fibonacci(0), Some(0));
        assert_eq!(fibonacci(1), Some(1));
        assert_eq!(fibonacci(10), Some(55));
        assert_eq!(fibonacci(93), Some(12_200_160_415_121_876_738));
        assert_eq!(fibonacci(94), None);
    }
}
